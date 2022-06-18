#[doc = "Register `SRCCM` reader"]
pub struct R(crate::R<SRCCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCCM` writer"]
pub struct W(crate::W<SRCCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCCM_SPEC>;
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
impl From<crate::W<SRCCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRCCM_R0` reader - CRC and Cryptographic Modules Software Reset"]
pub type SYSCTL_SRCCM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRCCM_R0` writer - CRC and Cryptographic Modules Software Reset"]
pub type SYSCTL_SRCCM_R0_W<'a> = crate::BitWriter<'a, u32, SRCCM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Software Reset"]
    #[inline(always)]
    pub fn sysctl_srccm_r0(&self) -> SYSCTL_SRCCM_R0_R {
        SYSCTL_SRCCM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC and Cryptographic Modules Software Reset"]
    #[inline(always)]
    pub fn sysctl_srccm_r0(&mut self) -> SYSCTL_SRCCM_R0_W {
        SYSCTL_SRCCM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC and Cryptographic Modules Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srccm](index.html) module"]
pub struct SRCCM_SPEC;
impl crate::RegisterSpec for SRCCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srccm::R](R) reader structure"]
impl crate::Readable for SRCCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srccm::W](W) writer structure"]
impl crate::Writable for SRCCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCCM to value 0"]
impl crate::Resettable for SRCCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
