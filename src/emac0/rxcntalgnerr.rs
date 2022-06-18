#[doc = "Register `RXCNTALGNERR` reader"]
pub struct R(crate::R<RXCNTALGNERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCNTALGNERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCNTALGNERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCNTALGNERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCNTALGNERR` writer"]
pub struct W(crate::W<RXCNTALGNERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCNTALGNERR_SPEC>;
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
impl From<crate::W<RXCNTALGNERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCNTALGNERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXCNTALGNERR_RXALGNERR` reader - This field indicates the number of frames received with alignment (dribble) error"]
pub type EMAC_RXCNTALGNERR_RXALGNERR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RXCNTALGNERR_RXALGNERR` writer - This field indicates the number of frames received with alignment (dribble) error"]
pub type EMAC_RXCNTALGNERR_RXALGNERR_W<'a> =
    crate::FieldWriter<'a, u32, RXCNTALGNERR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with alignment (dribble) error"]
    #[inline(always)]
    pub fn emac_rxcntalgnerr_rxalgnerr(&self) -> EMAC_RXCNTALGNERR_RXALGNERR_R {
        EMAC_RXCNTALGNERR_RXALGNERR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of frames received with alignment (dribble) error"]
    #[inline(always)]
    pub fn emac_rxcntalgnerr_rxalgnerr(&mut self) -> EMAC_RXCNTALGNERR_RXALGNERR_W {
        EMAC_RXCNTALGNERR_RXALGNERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcntalgnerr](index.html) module"]
pub struct RXCNTALGNERR_SPEC;
impl crate::RegisterSpec for RXCNTALGNERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcntalgnerr::R](R) reader structure"]
impl crate::Readable for RXCNTALGNERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcntalgnerr::W](W) writer structure"]
impl crate::Writable for RXCNTALGNERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCNTALGNERR to value 0"]
impl crate::Resettable for RXCNTALGNERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
