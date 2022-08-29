#![doc = "Peripheral access API for SOC microcontrollers (generated using svd2rust v0.25.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.25.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn UART();
    fn TIMER0();
    fn TIMER1();
    fn DMATEST();
    fn TMU();
}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 5] = [
    Vector { _handler: UART },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: DMATEST },
    Vector { _handler: TMU },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "BPU"]
pub struct BPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPU {}
impl BPU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bpu::RegisterBlock = 0xf000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bpu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BPU {
    type Target = bpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPU").finish()
    }
}
#[doc = "BPU"]
pub mod bpu;
#[doc = "CTRL"]
pub struct CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRL {}
impl CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ctrl::RegisterBlock = 0xf000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CTRL {
    type Target = ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL").finish()
    }
}
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "DMATEST"]
pub struct DMATEST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMATEST {}
impl DMATEST {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dmatest::RegisterBlock = 0xf000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmatest::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMATEST {
    type Target = dmatest::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMATEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATEST").finish()
    }
}
#[doc = "DMATEST"]
pub mod dmatest;
#[doc = "IDENTIFIER_MEM"]
pub struct IDENTIFIER_MEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDENTIFIER_MEM {}
impl IDENTIFIER_MEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const identifier_mem::RegisterBlock = 0xf000_1800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const identifier_mem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IDENTIFIER_MEM {
    type Target = identifier_mem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IDENTIFIER_MEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDENTIFIER_MEM").finish()
    }
}
#[doc = "IDENTIFIER_MEM"]
pub mod identifier_mem;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0xf000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0").finish()
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer1::RegisterBlock = 0xf000_2800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1").finish()
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "TMU"]
pub struct TMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMU {}
impl TMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tmu::RegisterBlock = 0xf000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TMU {
    type Target = tmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMU").finish()
    }
}
#[doc = "TMU"]
pub mod tmu;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0xf000_3800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART").finish()
    }
}
#[doc = "UART"]
pub mod uart;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "BPU"]
    pub BPU: BPU,
    #[doc = "CTRL"]
    pub CTRL: CTRL,
    #[doc = "DMATEST"]
    pub DMATEST: DMATEST,
    #[doc = "IDENTIFIER_MEM"]
    pub IDENTIFIER_MEM: IDENTIFIER_MEM,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TMU"]
    pub TMU: TMU,
    #[doc = "UART"]
    pub UART: UART,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            BPU: BPU {
                _marker: PhantomData,
            },
            CTRL: CTRL {
                _marker: PhantomData,
            },
            DMATEST: DMATEST {
                _marker: PhantomData,
            },
            IDENTIFIER_MEM: IDENTIFIER_MEM {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TMU: TMU {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
        }
    }
}
