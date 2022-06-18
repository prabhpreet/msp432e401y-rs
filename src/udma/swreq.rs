#[doc = "Register `SWREQ` writer"]
pub struct W(crate::W<SWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWREQ_SPEC>;
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
impl From<crate::W<SWREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_SWREQ` writer - Channel \\[n\\]
Software Request"]
pub type UDMA_SWREQ_W<'a> = crate::FieldWriter<'a, u32, SWREQ_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Software Request"]
    #[inline(always)]
    pub fn udma_swreq(&mut self) -> UDMA_SWREQ_W {
        UDMA_SWREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Software Request\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](index.html) module"]
pub struct SWREQ_SPEC;
impl crate::RegisterSpec for SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swreq::W](W) writer structure"]
impl crate::Writable for SWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
