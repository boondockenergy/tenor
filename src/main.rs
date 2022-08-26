#![no_std]
#![no_main]

#![feature(alloc_error_handler)]
#![feature(box_syntax)]
//#![feature(default_alloc_error_handler)]


mod uart;
mod timer;
mod interrupt;
//mod bpu;
mod uart_log;
mod dma_test;
mod driver;
mod time;
mod util;

extern crate alloc;
use alloc::boxed::Box;
use driver::{DriverAccess};

core::arch::global_asm!(include_str!("asm/boot.S"));
core::arch::global_asm!(include_str!("asm/trap.S"));

use core::alloc::GlobalAlloc;
use core::fmt::{Write, Display};
use core::panic::PanicInfo;
use core::ptr::write_volatile;

use interrupt::VexRiscvInterrupt;
use litex_pac::{TIMER0, TIMER1};
use riscv::asm::*;
use uart_log::UartLogger;

use riscv as _;
use critical_section::{Mutex, RawRestoreState};

use ansi_rgb::{ Foreground };
use ansi_rgb::{ cyan_blue, green_cyan, blue_magenta };

use log::{LevelFilter, info, error};

use crate::dma_test::DMATest;

use time::timeit;

use linked_list_allocator::LockedHeap;

//use numtoa::NumToA;
use util::{SizeFormat};

#[alloc_error_handler]
fn allocation_error(_: core::alloc::Layout) -> ! {
    info!("Allocation failed");
    panic!();
}


driver_add!(UART, uart::Uart);
driver_add!(TIMER0, timer::Timer<TIMER0>);
driver_add!(TIMER1, timer::Timer<TIMER1>);
driver_add!(DMATEST, dma_test::DMATest);


struct RiscvCriticalSection;

critical_section::set_impl!(RiscvCriticalSection);

unsafe impl critical_section::Impl for RiscvCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let was_active = riscv::register::mstatus::read().mie();
        riscv::interrupt::disable();
        was_active
    }

    unsafe fn release(was_active: RawRestoreState) {
        if was_active {
            riscv::interrupt::enable()
        }
    }
}

// Do an unaligned write to test the trap handler
#[allow(dead_code)]
unsafe fn crash() {
    const ptr: *mut u32 = (0xffffffff) as *mut u32;
    write_volatile(ptr, 0xdeadbeef)
}

static LOGGER: UartLogger = UartLogger;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// These are defined in the linker script (lds/litex-sim.lds)
extern "C" {
    static _heap_start: usize; 
    static _heap_end: usize;
}

pub fn init_heap() {
    unsafe {
        let heap_start = &_heap_start as *const usize as usize;
        let heap_end = &_heap_end as *const usize as usize;
        let heap_size = heap_end - heap_start;
        info!("Heap start: {:p} end: {:p} size: {} MiB",
            heap_start as *mut u8,
            heap_end as *mut u8,
            heap_size / (1024*1024)
        );
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("PANIC {:#?}", info);
    loop {
        unsafe { wfi() };
    }
}

fn putchar(ch: char) {
    const UART_ADDR: *const litex_pac::uart::RegisterBlock = litex_pac::UART::PTR;

    unsafe {
        write_volatile(UART_ADDR as *mut char, ch);
    }
}

#[no_mangle]
#[link_section = ".riscv.trap"]
pub unsafe extern "C" fn machine_trap() {
    
    let mcause = riscv::register::mcause::read();

    match mcause.cause() {
        riscv::register::mcause::Trap::Exception(e) => {
            let mhartid = riscv::register::mhartid::read();
            let mepc = riscv::register::mepc::read();
            let mtval = riscv::register::mtval::read();
            let mstatus = riscv::register::mstatus::read();

            let code = mcause.code();

            info!("EXCEPTION {:#?} code={}", e, code);
            info!("MEPC: 0x{:08X}", mepc);
            info!("MTVAL: 0x{:08X}", mtval);
            info!("MCAUSE: 0x{:08X}", mcause.bits());
            info!("MHARTID: 0x{:08X}", mhartid);

            loop {}
        }

        riscv::register::mcause::Trap::Interrupt(source) => {

            match source {
                riscv::register::mcause::Interrupt::MachineExternal => {
                    // Handle external interrupts from the VexRiscv interrupt controller
                    let pending = VexRiscvInterrupt::pending() as u32;
                    //info!("Pending external interrupt: {}\r\n", pending);

                    if pending & (1u32 << litex_pac::Interrupt::UART as u32) != 0 {
                        UART.access(|d| {
                            d.handle_interrupt();
                        });
                    }
                    if pending & (1u32 << litex_pac::Interrupt::TIMER0 as u32) != 0 {
                        TIMER0.access(|d| {
                            d.handle_interrupt();
                        });
                    }
                    if pending & (1u32 << litex_pac::Interrupt::TIMER1 as u32) != 0 {
                        TIMER1.access(|d| {
                            d.handle_interrupt();
                        });
                    }

                    if pending & (1u32 << litex_pac::Interrupt::DMATEST as u32) != 0 {
                        DMATEST.access(|dma| {
                            dma.interrupt_handler();
                        })
                    }
                }

                _ => {
                    info!("Unhandled CPU interrupt source: {:#?}", source);
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn main() {
    let peripherals = litex_pac::Peripherals::take().unwrap();

    let banner = include_str!("banner.txt");

    // Initialize the UART
    let mut uart = uart::Uart::new(peripherals.UART);
    write!(uart, "{}\r\n", banner.fg(blue_magenta())).unwrap();
    write!(uart, "{}", "Rust RISCV Kernel Booting\r\n".fg(cyan_blue())).unwrap();
    UART.set(uart);

    unsafe {
        log::set_logger_racy(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
            .unwrap();
    }

    info!("Logging initialized");

    init_heap();
    info!("Heap initialized");

    // Initialize timers
    let timer0 = timer::Timer::new(peripherals.TIMER0, 2e6 as usize);
    let timer1= timer::Timer::new(peripherals.TIMER1, 3e6 as usize);
    let dma: DMATest = dma_test::DMATest::new(peripherals.DMATEST);

    // Set the driver instances. From this point on, all drivers are accessed using:
    // DEVICE.access(|driver| { driver.method() })
    TIMER0.set(timer0);
    TIMER1.set(timer1);
    DMATEST.set(dma);

    // Enable interrupts
    unsafe {
        // Enable external interrupts from the VexRiscv interrupt controller
        riscv::register::mie::set_mext();

        // Enable global interrupts
        riscv::interrupt::enable();

        VexRiscvInterrupt::write_mask(usize::MAX);
    }

    info!("Interrupts enabled");

    const NUM: usize = (1024*1024*32);

    let data = Box::new(0);
    let dest = Box::new(0);

    info!("Allocated {:p}, {:p}", data, dest);

    dmatest();

    loop {
        unsafe{ wfi() };
    }
}

fn dmatest() {

    const NUM: usize = (256*1024);
    let mut data = box [0 as u8; NUM];
    let mut dest = box [0 as u8; NUM];

    info!("Starting DMA test {:p} -> {:p}", data, dest);

    let mut count = 0u8;
    loop {
        timeit("CPU buffer fill", || {
            for i in 1..NUM+1 {
                data[(i-1) as usize] = count + i as u8 % 8;
            }
        });

        count += 1;

        timeit("DMA copy", || {
            DMATEST.access(|dma| {
                dma.copy(&*data, &*dest, NUM as usize);
                dma.wait();
                //dma.dump();
            });
        });

        timeit("CPU buffer compare", || {
            assert_eq!(data, dest);
        });
        info!("{}", "Pass".fg(green_cyan()));
    }
}
