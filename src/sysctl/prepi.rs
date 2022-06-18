#[doc = "Register `PREPI` reader"]
pub struct R(crate::R<PREPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREPI` writer"]
pub struct W(crate::W<PREPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREPI_SPEC>;
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
impl From<crate::W<PREPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PREPI_R0` reader - EPI Module Peripheral Ready"]
pub type SYSCTL_PREPI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PREPI_R0` writer - EPI Module Peripheral Ready"]
pub type SYSCTL_PREPI_R0_W<'a> = crate::BitWriter<'a, u32, PREPI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EPI Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prepi_r0(&self) -> SYSCTL_PREPI_R0_R {
        SYSCTL_PREPI_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EPI Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prepi_r0(&mut self) -> SYSCTL_PREPI_R0_W {
        SYSCTL_PREPI_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prepi](index.html) module"]
pub struct PREPI_SPEC;
impl crate::RegisterSpec for PREPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prepi::R](R) reader structure"]
impl crate::Readable for PREPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prepi::W](W) writer structure"]
impl crate::Writable for PREPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREPI to value 0"]
impl crate::Resettable for PREPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
