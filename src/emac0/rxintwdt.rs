#[doc = "Register `RXINTWDT` reader"]
pub struct R(crate::R<RXINTWDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXINTWDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXINTWDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXINTWDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXINTWDT` writer"]
pub struct W(crate::W<RXINTWDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXINTWDT_SPEC>;
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
impl From<crate::W<RXINTWDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXINTWDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXINTWDT_RIWT` reader - Receive Interrupt Watchdog Timer Count"]
pub type EMAC_RXINTWDT_RIWT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_RXINTWDT_RIWT` writer - Receive Interrupt Watchdog Timer Count"]
pub type EMAC_RXINTWDT_RIWT_W<'a> = crate::FieldWriter<'a, u32, RXINTWDT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn emac_rxintwdt_riwt(&self) -> EMAC_RXINTWDT_RIWT_R {
        EMAC_RXINTWDT_RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn emac_rxintwdt_riwt(&mut self) -> EMAC_RXINTWDT_RIWT_W {
        EMAC_RXINTWDT_RIWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxintwdt](index.html) module"]
pub struct RXINTWDT_SPEC;
impl crate::RegisterSpec for RXINTWDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxintwdt::R](R) reader structure"]
impl crate::Readable for RXINTWDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxintwdt::W](W) writer structure"]
impl crate::Writable for RXINTWDT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXINTWDT to value 0"]
impl crate::Resettable for RXINTWDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
