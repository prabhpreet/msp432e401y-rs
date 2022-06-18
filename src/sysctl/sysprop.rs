#[doc = "Register `SYSPROP` reader"]
pub struct R(crate::R<SYSPROP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPROP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPROP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPROP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPROP` writer"]
pub struct W(crate::W<SYSPROP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPROP_SPEC>;
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
impl From<crate::W<SYSPROP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPROP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SYSPROP_FPU` reader - FPU Present"]
pub type SYSCTL_SYSPROP_FPU_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SYSPROP_FPU` writer - FPU Present"]
pub type SYSCTL_SYSPROP_FPU_W<'a> = crate::BitWriter<'a, u32, SYSPROP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - FPU Present"]
    #[inline(always)]
    pub fn sysctl_sysprop_fpu(&self) -> SYSCTL_SYSPROP_FPU_R {
        SYSCTL_SYSPROP_FPU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPU Present"]
    #[inline(always)]
    pub fn sysctl_sysprop_fpu(&mut self) -> SYSCTL_SYSPROP_FPU_W {
        SYSCTL_SYSPROP_FPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysprop](index.html) module"]
pub struct SYSPROP_SPEC;
impl crate::RegisterSpec for SYSPROP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysprop::R](R) reader structure"]
impl crate::Readable for SYSPROP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysprop::W](W) writer structure"]
impl crate::Writable for SYSPROP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPROP to value 0"]
impl crate::Resettable for SYSPROP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
