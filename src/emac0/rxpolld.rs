#[doc = "Register `RXPOLLD` writer"]
pub struct W(crate::W<RXPOLLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXPOLLD_SPEC>;
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
impl From<crate::W<RXPOLLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXPOLLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXPOLLD_RPD` writer - Receive Poll Demand"]
pub type EMAC_RXPOLLD_RPD_W<'a> = crate::FieldWriter<'a, u32, RXPOLLD_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand"]
    #[inline(always)]
    pub fn emac_rxpolld_rpd(&mut self) -> EMAC_RXPOLLD_RPD_W {
        EMAC_RXPOLLD_RPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Poll Demand\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpolld](index.html) module"]
pub struct RXPOLLD_SPEC;
impl crate::RegisterSpec for RXPOLLD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rxpolld::W](W) writer structure"]
impl crate::Writable for RXPOLLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXPOLLD to value 0"]
impl crate::Resettable for RXPOLLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
