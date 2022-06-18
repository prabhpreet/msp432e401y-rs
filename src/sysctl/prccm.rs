#[doc = "Register `PRCCM` reader"]
pub struct R(crate::R<PRCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRCCM` writer"]
pub struct W(crate::W<PRCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRCCM_SPEC>;
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
impl From<crate::W<PRCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRCCM_R0` reader - CRC and Cryptographic Modules Peripheral Ready"]
pub type SYSCTL_PRCCM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRCCM_R0` writer - CRC and Cryptographic Modules Peripheral Ready"]
pub type SYSCTL_PRCCM_R0_W<'a> = crate::BitWriter<'a, u32, PRCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prccm_r0(&self) -> SYSCTL_PRCCM_R0_R {
        SYSCTL_PRCCM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prccm_r0(&mut self) -> SYSCTL_PRCCM_R0_W {
        SYSCTL_PRCCM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prccm](index.html) module"]
pub struct PRCCM_SPEC;
impl crate::RegisterSpec for PRCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prccm::R](R) reader structure"]
impl crate::Readable for PRCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prccm::W](W) writer structure"]
impl crate::Writable for PRCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRCCM to value 0"]
impl crate::Resettable for PRCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
