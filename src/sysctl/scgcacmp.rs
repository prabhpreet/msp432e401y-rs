#[doc = "Register `SCGCACMP` reader"]
pub struct R(crate::R<SCGCACMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCACMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCACMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCACMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCACMP` writer"]
pub struct W(crate::W<SCGCACMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCACMP_SPEC>;
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
impl From<crate::W<SCGCACMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCACMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCACMP_S0` reader - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCACMP_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCACMP_S0` writer - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCACMP_S0_W<'a> = crate::BitWriter<'a, u32, SCGCACMP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcacmp_s0(&self) -> SYSCTL_SCGCACMP_S0_R {
        SYSCTL_SCGCACMP_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcacmp_s0(&mut self) -> SYSCTL_SCGCACMP_S0_W {
        SYSCTL_SCGCACMP_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcacmp](index.html) module"]
pub struct SCGCACMP_SPEC;
impl crate::RegisterSpec for SCGCACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcacmp::R](R) reader structure"]
impl crate::Readable for SCGCACMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcacmp::W](W) writer structure"]
impl crate::Writable for SCGCACMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCACMP to value 0"]
impl crate::Resettable for SCGCACMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
