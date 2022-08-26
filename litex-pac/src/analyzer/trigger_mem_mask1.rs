#[doc = "Register `TRIGGER_MEM_MASK1` reader"]
pub struct R(crate::R<TRIGGER_MEM_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_MEM_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_MEM_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_MEM_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGGER_MEM_MASK1` writer"]
pub struct W(crate::W<TRIGGER_MEM_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGGER_MEM_MASK1_SPEC>;
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
impl From<crate::W<TRIGGER_MEM_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGGER_MEM_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trigger_mem_mask` reader - "]
pub type TRIGGER_MEM_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `trigger_mem_mask` writer - "]
pub type TRIGGER_MEM_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIGGER_MEM_MASK1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trigger_mem_mask(&self) -> TRIGGER_MEM_MASK_R {
        TRIGGER_MEM_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn trigger_mem_mask(&mut self) -> TRIGGER_MEM_MASK_W<0> {
        TRIGGER_MEM_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bits 32-63 of `ANALYZER_TRIGGER_MEM_MASK`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger_mem_mask1](index.html) module"]
pub struct TRIGGER_MEM_MASK1_SPEC;
impl crate::RegisterSpec for TRIGGER_MEM_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger_mem_mask1::R](R) reader structure"]
impl crate::Readable for TRIGGER_MEM_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigger_mem_mask1::W](W) writer structure"]
impl crate::Writable for TRIGGER_MEM_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIGGER_MEM_MASK1 to value 0"]
impl crate::Resettable for TRIGGER_MEM_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
