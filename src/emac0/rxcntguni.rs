#[doc = "Register `RXCNTGUNI` reader"]
pub struct R(crate::R<RXCNTGUNI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCNTGUNI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCNTGUNI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCNTGUNI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCNTGUNI` writer"]
pub struct W(crate::W<RXCNTGUNI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCNTGUNI_SPEC>;
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
impl From<crate::W<RXCNTGUNI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCNTGUNI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RXCNTGUNI_RXUCASTG` reader - This field indicates the number of received good unicast frames"]
pub type EMAC_RXCNTGUNI_RXUCASTG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_RXCNTGUNI_RXUCASTG` writer - This field indicates the number of received good unicast frames"]
pub type EMAC_RXCNTGUNI_RXUCASTG_W<'a> =
    crate::FieldWriter<'a, u32, RXCNTGUNI_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good unicast frames"]
    #[inline(always)]
    pub fn emac_rxcntguni_rxucastg(&self) -> EMAC_RXCNTGUNI_RXUCASTG_R {
        EMAC_RXCNTGUNI_RXUCASTG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field indicates the number of received good unicast frames"]
    #[inline(always)]
    pub fn emac_rxcntguni_rxucastg(&mut self) -> EMAC_RXCNTGUNI_RXUCASTG_W {
        EMAC_RXCNTGUNI_RXUCASTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcntguni](index.html) module"]
pub struct RXCNTGUNI_SPEC;
impl crate::RegisterSpec for RXCNTGUNI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcntguni::R](R) reader structure"]
impl crate::Readable for RXCNTGUNI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcntguni::W](W) writer structure"]
impl crate::Writable for RXCNTGUNI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCNTGUNI to value 0"]
impl crate::Resettable for RXCNTGUNI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
