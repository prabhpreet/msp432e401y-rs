#[doc = "Register `TXCNTSCOL` reader"]
pub struct R(crate::R<TXCNTSCOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCNTSCOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCNTSCOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCNTSCOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCNTSCOL` writer"]
pub struct W(crate::W<TXCNTSCOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCNTSCOL_SPEC>;
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
impl From<crate::W<TXCNTSCOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCNTSCOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TXCNTSCOL_TXSNGLCOLG` reader - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
pub type EMAC_TXCNTSCOL_TXSNGLCOLG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TXCNTSCOL_TXSNGLCOLG` writer - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
pub type EMAC_TXCNTSCOL_TXSNGLCOLG_W<'a> =
    crate::FieldWriter<'a, u32, TXCNTSCOL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline(always)]
    pub fn emac_txcntscol_txsnglcolg(&self) -> EMAC_TXCNTSCOL_TXSNGLCOLG_R {
        EMAC_TXCNTSCOL_TXSNGLCOLG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of successfully transmitted frames after a single collision in the half-duplex mode"]
    #[inline(always)]
    pub fn emac_txcntscol_txsnglcolg(&mut self) -> EMAC_TXCNTSCOL_TXSNGLCOLG_W {
        EMAC_TXCNTSCOL_TXSNGLCOLG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcntscol](index.html) module"]
pub struct TXCNTSCOL_SPEC;
impl crate::RegisterSpec for TXCNTSCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcntscol::R](R) reader structure"]
impl crate::Readable for TXCNTSCOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcntscol::W](W) writer structure"]
impl crate::Writable for TXCNTSCOL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCNTSCOL to value 0"]
impl crate::Resettable for TXCNTSCOL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
