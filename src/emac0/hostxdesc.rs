#[doc = "Register `HOSTXDESC` reader"]
pub struct R(crate::R<HOSTXDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOSTXDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOSTXDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOSTXDESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOSTXDESC` writer"]
pub struct W(crate::W<HOSTXDESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOSTXDESC_SPEC>;
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
impl From<crate::W<HOSTXDESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOSTXDESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HOSTXDESC_CURTXDESC` reader - Host Transmit Descriptor Address Pointer"]
pub type EMAC_HOSTXDESC_CURTXDESC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HOSTXDESC_CURTXDESC` writer - Host Transmit Descriptor Address Pointer"]
pub type EMAC_HOSTXDESC_CURTXDESC_W<'a> =
    crate::FieldWriter<'a, u32, HOSTXDESC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn emac_hostxdesc_curtxdesc(&self) -> EMAC_HOSTXDESC_CURTXDESC_R {
        EMAC_HOSTXDESC_CURTXDESC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn emac_hostxdesc_curtxdesc(&mut self) -> EMAC_HOSTXDESC_CURTXDESC_W {
        EMAC_HOSTXDESC_CURTXDESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Current Host Transmit Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hostxdesc](index.html) module"]
pub struct HOSTXDESC_SPEC;
impl crate::RegisterSpec for HOSTXDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hostxdesc::R](R) reader structure"]
impl crate::Readable for HOSTXDESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hostxdesc::W](W) writer structure"]
impl crate::Writable for HOSTXDESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOSTXDESC to value 0"]
impl crate::Resettable for HOSTXDESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
