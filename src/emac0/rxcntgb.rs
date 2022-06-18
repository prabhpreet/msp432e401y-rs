#[doc = "Register `RXCNTGB` reader"]
pub struct R(crate::R<RXCNTGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCNTGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCNTGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCNTGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCNTGB` writer"]
pub struct W(crate::W<RXCNTGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCNTGB_SPEC>;
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
impl From<crate::W<RXCNTGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCNTGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXCNTGB_RXFRMGB` reader - This field indicates the number of received good and bad frames"]
pub type EMAC_RXCNTGB_RXFRMGB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RXCNTGB_RXFRMGB` writer - This field indicates the number of received good and bad frames"]
pub type EMAC_RXCNTGB_RXFRMGB_W<'a> = crate::FieldWriter<'a, u32, RXCNTGB_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames"]
    #[inline(always)]
    pub fn emac_rxcntgb_rxfrmgb(&self) -> EMAC_RXCNTGB_RXFRMGB_R {
        EMAC_RXCNTGB_RXFRMGB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad frames"]
    #[inline(always)]
    pub fn emac_rxcntgb_rxfrmgb(&mut self) -> EMAC_RXCNTGB_RXFRMGB_W {
        EMAC_RXCNTGB_RXFRMGB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcntgb](index.html) module"]
pub struct RXCNTGB_SPEC;
impl crate::RegisterSpec for RXCNTGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcntgb::R](R) reader structure"]
impl crate::Readable for RXCNTGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcntgb::W](W) writer structure"]
impl crate::Writable for RXCNTGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCNTGB to value 0"]
impl crate::Resettable for RXCNTGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
