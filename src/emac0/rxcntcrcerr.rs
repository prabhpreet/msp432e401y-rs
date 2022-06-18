#[doc = "Register `RXCNTCRCERR` reader"]
pub struct R(crate::R<RXCNTCRCERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCNTCRCERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCNTCRCERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCNTCRCERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCNTCRCERR` writer"]
pub struct W(crate::W<RXCNTCRCERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCNTCRCERR_SPEC>;
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
impl From<crate::W<RXCNTCRCERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCNTCRCERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXCNTCRCERR_RXCRCERR` reader - This field indicates the number of frames received with CRC error"]
pub type EMAC_RXCNTCRCERR_RXCRCERR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RXCNTCRCERR_RXCRCERR` writer - This field indicates the number of frames received with CRC error"]
pub type EMAC_RXCNTCRCERR_RXCRCERR_W<'a> =
    crate::FieldWriter<'a, u32, RXCNTCRCERR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with CRC error"]
    #[inline(always)]
    pub fn emac_rxcntcrcerr_rxcrcerr(&self) -> EMAC_RXCNTCRCERR_RXCRCERR_R {
        EMAC_RXCNTCRCERR_RXCRCERR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with CRC error"]
    #[inline(always)]
    pub fn emac_rxcntcrcerr_rxcrcerr(&mut self) -> EMAC_RXCNTCRCERR_RXCRCERR_W {
        EMAC_RXCNTCRCERR_RXCRCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcntcrcerr](index.html) module"]
pub struct RXCNTCRCERR_SPEC;
impl crate::RegisterSpec for RXCNTCRCERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcntcrcerr::R](R) reader structure"]
impl crate::Readable for RXCNTCRCERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcntcrcerr::W](W) writer structure"]
impl crate::Writable for RXCNTCRCERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCNTCRCERR to value 0"]
impl crate::Resettable for RXCNTCRCERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
