#[doc = "Register `WAITSTAT` reader"]
pub struct R(crate::R<WAITSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAITSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAITSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAITSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAITSTAT` writer"]
pub struct W(crate::W<WAITSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAITSTAT_SPEC>;
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
impl From<crate::W<WAITSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAITSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_WAITSTAT_WAITREQ` reader - Channel \\[n\\]
Wait Status"]
pub type UDMA_WAITSTAT_WAITREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_WAITSTAT_WAITREQ` writer - Channel \\[n\\]
Wait Status"]
pub type UDMA_WAITSTAT_WAITREQ_W<'a> = crate::FieldWriter<'a, u32, WAITSTAT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Wait Status"]
    #[inline(always)]
    pub fn udma_waitstat_waitreq(&self) -> UDMA_WAITSTAT_WAITREQ_R {
        UDMA_WAITSTAT_WAITREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Wait Status"]
    #[inline(always)]
    pub fn udma_waitstat_waitreq(&mut self) -> UDMA_WAITSTAT_WAITREQ_W {
        UDMA_WAITSTAT_WAITREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Wait-on-Request Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitstat](index.html) module"]
pub struct WAITSTAT_SPEC;
impl crate::RegisterSpec for WAITSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waitstat::R](R) reader structure"]
impl crate::Readable for WAITSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [waitstat::W](W) writer structure"]
impl crate::Writable for WAITSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAITSTAT to value 0"]
impl crate::Resettable for WAITSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
