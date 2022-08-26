#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `busy` reader - BPU Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `busy` writer - BPU Busy"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `idle` reader - BPU Idle"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `idle` writer - BPU Idle"]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `running` reader - BPU Running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `running` writer - BPU Running"]
pub type RUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `done` reader - BPU Done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `done` writer - BPU Done"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BPU Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BPU Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BPU Running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BPU Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPU Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - BPU Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W<1> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 2 - BPU Running"]
    #[inline(always)]
    pub fn running(&mut self) -> RUNNING_W<2> {
        RUNNING_W::new(self)
    }
    #[doc = "Bit 3 - BPU Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<3> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BPU Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
