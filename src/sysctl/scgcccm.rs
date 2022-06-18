#[doc = "Register `SCGCCCM` reader"]
pub struct R(crate::R<SCGCCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCCCM` writer"]
pub struct W(crate::W<SCGCCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCCCM_SPEC>;
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
impl From<crate::W<SCGCCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCCCM_S0` reader - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCCM_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCCCM_S0` writer - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCCCM_S0_W<'a> = crate::BitWriter<'a, u32, SCGCCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcccm_s0(&self) -> SYSCTL_SCGCCCM_S0_R {
        SYSCTL_SCGCCCM_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcccm_s0(&mut self) -> SYSCTL_SCGCCCM_S0_W {
        SYSCTL_SCGCCCM_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcccm](index.html) module"]
pub struct SCGCCCM_SPEC;
impl crate::RegisterSpec for SCGCCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcccm::R](R) reader structure"]
impl crate::Readable for SCGCCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcccm::W](W) writer structure"]
impl crate::Writable for SCGCCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCCCM to value 0"]
impl crate::Resettable for SCGCCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
