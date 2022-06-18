#[doc = "Register `PCCCM` reader"]
pub struct R(crate::R<PCCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCCCM` writer"]
pub struct W(crate::W<PCCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCCM_SPEC>;
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
impl From<crate::W<PCCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCCCM_P0` reader - CRC and Cryptographic Modules Power Control"]
pub type SYSCTL_PCCCM_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCCCM_P0` writer - CRC and Cryptographic Modules Power Control"]
pub type SYSCTL_PCCCM_P0_W<'a> = crate::BitWriter<'a, u32, PCCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Power Control"]
    #[inline(always)]
    pub fn sysctl_pcccm_p0(&self) -> SYSCTL_PCCCM_P0_R {
        SYSCTL_PCCCM_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Power Control"]
    #[inline(always)]
    pub fn sysctl_pcccm_p0(&mut self) -> SYSCTL_PCCCM_P0_W {
        SYSCTL_PCCCM_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcccm](index.html) module"]
pub struct PCCCM_SPEC;
impl crate::RegisterSpec for PCCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcccm::R](R) reader structure"]
impl crate::Readable for PCCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcccm::W](W) writer structure"]
impl crate::Writable for PCCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCCCM to value 0"]
impl crate::Resettable for PCCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
