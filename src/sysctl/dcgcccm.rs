#[doc = "Register `DCGCCCM` reader"]
pub struct R(crate::R<DCGCCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCCCM` writer"]
pub struct W(crate::W<DCGCCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCCCM_SPEC>;
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
impl From<crate::W<DCGCCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCCCM_D0` reader - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCCM_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCCCM_D0` writer - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCCCM_D0_W<'a> = crate::BitWriter<'a, u32, DCGCCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcccm_d0(&self) -> SYSCTL_DCGCCCM_D0_R {
        SYSCTL_DCGCCCM_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcccm_d0(&mut self) -> SYSCTL_DCGCCCM_D0_W {
        SYSCTL_DCGCCCM_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcccm](index.html) module"]
pub struct DCGCCCM_SPEC;
impl crate::RegisterSpec for DCGCCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcccm::R](R) reader structure"]
impl crate::Readable for DCGCCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcccm::W](W) writer structure"]
impl crate::Writable for DCGCCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCCCM to value 0"]
impl crate::Resettable for DCGCCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
