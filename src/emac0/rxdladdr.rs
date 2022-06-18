#[doc = "Register `RXDLADDR` reader"]
pub struct R(crate::R<RXDLADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDLADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDLADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDLADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDLADDR` writer"]
pub struct W(crate::W<RXDLADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDLADDR_SPEC>;
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
impl From<crate::W<RXDLADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDLADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXDLADDR_STRXLIST` reader - Start of Receive List"]
pub type EMAC_RXDLADDR_STRXLIST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RXDLADDR_STRXLIST` writer - Start of Receive List"]
pub type EMAC_RXDLADDR_STRXLIST_W<'a> = crate::FieldWriter<'a, u32, RXDLADDR_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn emac_rxdladdr_strxlist(&self) -> EMAC_RXDLADDR_STRXLIST_R {
        EMAC_RXDLADDR_STRXLIST_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Receive List"]
    #[inline(always)]
    pub fn emac_rxdladdr_strxlist(&mut self) -> EMAC_RXDLADDR_STRXLIST_W {
        EMAC_RXDLADDR_STRXLIST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Descriptor List Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdladdr](index.html) module"]
pub struct RXDLADDR_SPEC;
impl crate::RegisterSpec for RXDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdladdr::R](R) reader structure"]
impl crate::Readable for RXDLADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdladdr::W](W) writer structure"]
impl crate::Writable for RXDLADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDLADDR to value 0"]
impl crate::Resettable for RXDLADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
