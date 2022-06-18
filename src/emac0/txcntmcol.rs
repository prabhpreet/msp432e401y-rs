#[doc = "Register `TXCNTMCOL` reader"]
pub struct R(crate::R<TXCNTMCOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCNTMCOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCNTMCOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCNTMCOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCNTMCOL` writer"]
pub struct W(crate::W<TXCNTMCOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCNTMCOL_SPEC>;
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
impl From<crate::W<TXCNTMCOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCNTMCOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TXCNTMCOL_TXMULTCOLG` reader - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
pub type EMAC_TXCNTMCOL_TXMULTCOLG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TXCNTMCOL_TXMULTCOLG` writer - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
pub type EMAC_TXCNTMCOL_TXMULTCOLG_W<'a> =
    crate::FieldWriter<'a, u32, TXCNTMCOL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline(always)]
    pub fn emac_txcntmcol_txmultcolg(&self) -> EMAC_TXCNTMCOL_TXMULTCOLG_R {
        EMAC_TXCNTMCOL_TXMULTCOLG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after multiple collisions in the half-duplex mode"]
    #[inline(always)]
    pub fn emac_txcntmcol_txmultcolg(&mut self) -> EMAC_TXCNTMCOL_TXMULTCOLG_W {
        EMAC_TXCNTMCOL_TXMULTCOLG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcntmcol](index.html) module"]
pub struct TXCNTMCOL_SPEC;
impl crate::RegisterSpec for TXCNTMCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcntmcol::R](R) reader structure"]
impl crate::Readable for TXCNTMCOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcntmcol::W](W) writer structure"]
impl crate::Writable for TXCNTMCOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCNTMCOL to value 0"]
impl crate::Resettable for TXCNTMCOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
