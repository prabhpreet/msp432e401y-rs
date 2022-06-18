#[doc = "Register `TXDLADDR` reader"]
pub struct R(crate::R<TXDLADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDLADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDLADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDLADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDLADDR` writer"]
pub struct W(crate::W<TXDLADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDLADDR_SPEC>;
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
impl From<crate::W<TXDLADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDLADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TXDLADDR_TXDLADDR` reader - Start of Transmit List Base Address"]
pub type EMAC_TXDLADDR_TXDLADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TXDLADDR_TXDLADDR` writer - Start of Transmit List Base Address"]
pub type EMAC_TXDLADDR_TXDLADDR_W<'a> = crate::FieldWriter<'a, u32, TXDLADDR_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List Base Address"]
    #[inline(always)]
    pub fn emac_txdladdr_txdladdr(&self) -> EMAC_TXDLADDR_TXDLADDR_R {
        EMAC_TXDLADDR_TXDLADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List Base Address"]
    #[inline(always)]
    pub fn emac_txdladdr_txdladdr(&mut self) -> EMAC_TXDLADDR_TXDLADDR_W {
        EMAC_TXDLADDR_TXDLADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Transmit Descriptor List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdladdr](index.html) module"]
pub struct TXDLADDR_SPEC;
impl crate::RegisterSpec for TXDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdladdr::R](R) reader structure"]
impl crate::Readable for TXDLADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdladdr::W](W) writer structure"]
impl crate::Writable for TXDLADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDLADDR to value 0"]
impl crate::Resettable for TXDLADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
