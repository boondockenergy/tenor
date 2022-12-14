#[doc = "Register `EV_PENDING` reader"]
pub struct R(crate::R<EV_PENDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EV_PENDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EV_PENDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EV_PENDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EV_PENDING` writer"]
pub struct W(crate::W<EV_PENDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EV_PENDING_SPEC>;
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
impl From<crate::W<EV_PENDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EV_PENDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `zero` reader - `1` if a `zero` event occurred. This Event is triggered on a **falling** edge."]
pub type ZERO_R = crate::BitReader<bool>;
#[doc = "Field `zero` writer - `1` if a `zero` event occurred. This Event is triggered on a **falling** edge."]
pub type ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EV_PENDING_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - `1` if a `zero` event occurred. This Event is triggered on a **falling** edge."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` if a `zero` event occurred. This Event is triggered on a **falling** edge."]
    #[inline(always)]
    pub fn zero(&mut self) -> ZERO_W<0> {
        ZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When a zero event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_pending](index.html) module"]
pub struct EV_PENDING_SPEC;
impl crate::RegisterSpec for EV_PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ev_pending::R](R) reader structure"]
impl crate::Readable for EV_PENDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ev_pending::W](W) writer structure"]
impl crate::Writable for EV_PENDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EV_PENDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
