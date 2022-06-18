#[doc = "Register `HOSRXDESC` reader"]
pub struct R(crate::R<HOSRXDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOSRXDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOSRXDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOSRXDESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOSRXDESC` writer"]
pub struct W(crate::W<HOSRXDESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOSRXDESC_SPEC>;
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
impl From<crate::W<HOSRXDESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOSRXDESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HOSRXDESC_CURRXDESC` reader - Host Receive Descriptor Address Pointer"]
pub type EMAC_HOSRXDESC_CURRXDESC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HOSRXDESC_CURRXDESC` writer - Host Receive Descriptor Address Pointer"]
pub type EMAC_HOSRXDESC_CURRXDESC_W<'a> =
    crate::FieldWriter<'a, u32, HOSRXDESC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn emac_hosrxdesc_currxdesc(&self) -> EMAC_HOSRXDESC_CURRXDESC_R {
        EMAC_HOSRXDESC_CURRXDESC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn emac_hosrxdesc_currxdesc(&mut self) -> EMAC_HOSRXDESC_CURRXDESC_W {
        EMAC_HOSRXDESC_CURRXDESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Current Host Receive Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hosrxdesc](index.html) module"]
pub struct HOSRXDESC_SPEC;
impl crate::RegisterSpec for HOSRXDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hosrxdesc::R](R) reader structure"]
impl crate::Readable for HOSRXDESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hosrxdesc::W](W) writer structure"]
impl crate::Writable for HOSRXDESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOSRXDESC to value 0"]
impl crate::Resettable for HOSRXDESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
