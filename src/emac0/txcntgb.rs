#[doc = "Register `TXCNTGB` reader"]
pub struct R(crate::R<TXCNTGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCNTGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCNTGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCNTGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCNTGB` writer"]
pub struct W(crate::W<TXCNTGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCNTGB_SPEC>;
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
impl From<crate::W<TXCNTGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCNTGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TXCNTGB_TXFRMGB` reader - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
pub type EMAC_TXCNTGB_TXFRMGB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TXCNTGB_TXFRMGB` writer - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
pub type EMAC_TXCNTGB_TXFRMGB_W<'a> = crate::FieldWriter<'a, u32, TXCNTGB_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn emac_txcntgb_txfrmgb(&self) -> EMAC_TXCNTGB_TXFRMGB_R {
        EMAC_TXCNTGB_TXFRMGB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of good and bad frames transmitted, exclusive of retried frames"]
    #[inline(always)]
    pub fn emac_txcntgb_txfrmgb(&mut self) -> EMAC_TXCNTGB_TXFRMGB_W {
        EMAC_TXCNTGB_TXFRMGB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcntgb](index.html) module"]
pub struct TXCNTGB_SPEC;
impl crate::RegisterSpec for TXCNTGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcntgb::R](R) reader structure"]
impl crate::Readable for TXCNTGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcntgb::W](W) writer structure"]
impl crate::Writable for TXCNTGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCNTGB to value 0"]
impl crate::Resettable for TXCNTGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
