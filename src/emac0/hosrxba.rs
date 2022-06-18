#[doc = "Register `HOSRXBA` reader"]
pub struct R(crate::R<HOSRXBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOSRXBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOSRXBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOSRXBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOSRXBA` writer"]
pub struct W(crate::W<HOSRXBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOSRXBA_SPEC>;
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
impl From<crate::W<HOSRXBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOSRXBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HOSRXBA_CURRXBUFA` reader - Host Receive Buffer Address Pointer"]
pub type EMAC_HOSRXBA_CURRXBUFA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HOSRXBA_CURRXBUFA` writer - Host Receive Buffer Address Pointer"]
pub type EMAC_HOSRXBA_CURRXBUFA_W<'a> = crate::FieldWriter<'a, u32, HOSRXBA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn emac_hosrxba_currxbufa(&self) -> EMAC_HOSRXBA_CURRXBUFA_R {
        EMAC_HOSRXBA_CURRXBUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn emac_hosrxba_currxbufa(&mut self) -> EMAC_HOSRXBA_CURRXBUFA_W {
        EMAC_HOSRXBA_CURRXBUFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Current Host Receive Buffer Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hosrxba](index.html) module"]
pub struct HOSRXBA_SPEC;
impl crate::RegisterSpec for HOSRXBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hosrxba::R](R) reader structure"]
impl crate::Readable for HOSRXBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hosrxba::W](W) writer structure"]
impl crate::Writable for HOSRXBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOSRXBA to value 0"]
impl crate::Resettable for HOSRXBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
