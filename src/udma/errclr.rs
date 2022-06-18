#[doc = "Register `ERRCLR` reader"]
pub struct R(crate::R<ERRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRCLR` writer"]
pub struct W(crate::W<ERRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRCLR_SPEC>;
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
impl From<crate::W<ERRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_ERRCLR_ERRCLR` reader - uDMA Bus Error Status"]
pub type UDMA_ERRCLR_ERRCLR_R = crate::BitReader<bool>;
#[doc = "Field `UDMA_ERRCLR_ERRCLR` writer - uDMA Bus Error Status"]
pub type UDMA_ERRCLR_ERRCLR_W<'a> = crate::BitWriter<'a, u32, ERRCLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Bus Error Status"]
    #[inline(always)]
    pub fn udma_errclr_errclr(&self) -> UDMA_ERRCLR_ERRCLR_R {
        UDMA_ERRCLR_ERRCLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Bus Error Status"]
    #[inline(always)]
    pub fn udma_errclr_errclr(&mut self) -> UDMA_ERRCLR_ERRCLR_W {
        UDMA_ERRCLR_ERRCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Bus Error Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errclr](index.html) module"]
pub struct ERRCLR_SPEC;
impl crate::RegisterSpec for ERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errclr::R](R) reader structure"]
impl crate::Readable for ERRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errclr::W](W) writer structure"]
impl crate::Writable for ERRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ERRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
