#[doc = "Register `TXOCTCNTG` reader"]
pub struct R(crate::R<TXOCTCNTG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXOCTCNTG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXOCTCNTG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXOCTCNTG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXOCTCNTG` writer"]
pub struct W(crate::W<TXOCTCNTG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXOCTCNTG_SPEC>;
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
impl From<crate::W<TXOCTCNTG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXOCTCNTG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TXOCTCNTG_TXOCTG` reader - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
pub type EMAC_TXOCTCNTG_TXOCTG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TXOCTCNTG_TXOCTG` writer - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
pub type EMAC_TXOCTCNTG_TXOCTG_W<'a> = crate::FieldWriter<'a, u32, TXOCTCNTG_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline(always)]
    pub fn emac_txoctcntg_txoctg(&self) -> EMAC_TXOCTCNTG_TXOCTG_R {
        EMAC_TXOCTCNTG_TXOCTG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of bytes transmitted, exclusive of preamble, in good frames"]
    #[inline(always)]
    pub fn emac_txoctcntg_txoctg(&mut self) -> EMAC_TXOCTCNTG_TXOCTG_W {
        EMAC_TXOCTCNTG_TXOCTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Transmit Octet Count Good\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txoctcntg](index.html) module"]
pub struct TXOCTCNTG_SPEC;
impl crate::RegisterSpec for TXOCTCNTG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txoctcntg::R](R) reader structure"]
impl crate::Readable for TXOCTCNTG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txoctcntg::W](W) writer structure"]
impl crate::Writable for TXOCTCNTG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXOCTCNTG to value 0"]
impl crate::Resettable for TXOCTCNTG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
