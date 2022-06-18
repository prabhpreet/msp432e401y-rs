#[doc = "Register `DCGCACMP` reader"]
pub struct R(crate::R<DCGCACMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCACMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCACMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCACMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCACMP` writer"]
pub struct W(crate::W<DCGCACMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCACMP_SPEC>;
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
impl From<crate::W<DCGCACMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCACMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCACMP_D0` reader - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCACMP_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCACMP_D0` writer - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCACMP_D0_W<'a> = crate::BitWriter<'a, u32, DCGCACMP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcacmp_d0(&self) -> SYSCTL_DCGCACMP_D0_R {
        SYSCTL_DCGCACMP_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcacmp_d0(&mut self) -> SYSCTL_DCGCACMP_D0_W {
        SYSCTL_DCGCACMP_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcacmp](index.html) module"]
pub struct DCGCACMP_SPEC;
impl crate::RegisterSpec for DCGCACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcacmp::R](R) reader structure"]
impl crate::Readable for DCGCACMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcacmp::W](W) writer structure"]
impl crate::Writable for DCGCACMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCACMP to value 0"]
impl crate::Resettable for DCGCACMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
