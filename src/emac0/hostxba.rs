#[doc = "Register `HOSTXBA` reader"]
pub struct R(crate::R<HOSTXBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOSTXBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOSTXBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOSTXBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOSTXBA` writer"]
pub struct W(crate::W<HOSTXBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOSTXBA_SPEC>;
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
impl From<crate::W<HOSTXBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOSTXBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HOSTXBA_CURTXBUFA` reader - Host Transmit Buffer Address Pointer"]
pub type EMAC_HOSTXBA_CURTXBUFA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HOSTXBA_CURTXBUFA` writer - Host Transmit Buffer Address Pointer"]
pub type EMAC_HOSTXBA_CURTXBUFA_W<'a> = crate::FieldWriter<'a, u32, HOSTXBA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn emac_hostxba_curtxbufa(&self) -> EMAC_HOSTXBA_CURTXBUFA_R {
        EMAC_HOSTXBA_CURTXBUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn emac_hostxba_curtxbufa(&mut self) -> EMAC_HOSTXBA_CURTXBUFA_W {
        EMAC_HOSTXBA_CURTXBUFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Current Host Transmit Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hostxba](index.html) module"]
pub struct HOSTXBA_SPEC;
impl crate::RegisterSpec for HOSTXBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hostxba::R](R) reader structure"]
impl crate::Readable for HOSTXBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hostxba::W](W) writer structure"]
impl crate::Writable for HOSTXBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOSTXBA to value 0"]
impl crate::Resettable for HOSTXBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
