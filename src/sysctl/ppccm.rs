#[doc = "Register `PPCCM` reader"]
pub struct R(crate::R<PPCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPCCM` writer"]
pub struct W(crate::W<PPCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPCCM_SPEC>;
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
impl From<crate::W<PPCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPCCM_P0` reader - CRC and Cryptographic Modules Present"]
pub type SYSCTL_PPCCM_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPCCM_P0` writer - CRC and Cryptographic Modules Present"]
pub type SYSCTL_PPCCM_P0_W<'a> = crate::BitWriter<'a, u32, PPCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Present"]
    #[inline(always)]
    pub fn sysctl_ppccm_p0(&self) -> SYSCTL_PPCCM_P0_R {
        SYSCTL_PPCCM_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Present"]
    #[inline(always)]
    pub fn sysctl_ppccm_p0(&mut self) -> SYSCTL_PPCCM_P0_W {
        SYSCTL_PPCCM_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppccm](index.html) module"]
pub struct PPCCM_SPEC;
impl crate::RegisterSpec for PPCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppccm::R](R) reader structure"]
impl crate::Readable for PPCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppccm::W](W) writer structure"]
impl crate::Writable for PPCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPCCM to value 0"]
impl crate::Resettable for PPCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
