#[doc = "Register `RCGCACMP` reader"]
pub struct R(crate::R<RCGCACMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCACMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCACMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCACMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCACMP` writer"]
pub struct W(crate::W<RCGCACMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCACMP_SPEC>;
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
impl From<crate::W<RCGCACMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCACMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCACMP_R0` reader - Analog Comparator Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCACMP_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCACMP_R0` writer - Analog Comparator Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCACMP_R0_W<'a> = crate::BitWriter<'a, u32, RCGCACMP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcacmp_r0(&self) -> SYSCTL_RCGCACMP_R0_R {
        SYSCTL_RCGCACMP_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcacmp_r0(&mut self) -> SYSCTL_RCGCACMP_R0_W {
        SYSCTL_RCGCACMP_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcacmp](index.html) module"]
pub struct RCGCACMP_SPEC;
impl crate::RegisterSpec for RCGCACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcacmp::R](R) reader structure"]
impl crate::Readable for RCGCACMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcacmp::W](W) writer structure"]
impl crate::Writable for RCGCACMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCACMP to value 0"]
impl crate::Resettable for RCGCACMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
