#[doc = "Register `DCGCQEI` reader"]
pub struct R(crate::R<DCGCQEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCQEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCQEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCQEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCQEI` writer"]
pub struct W(crate::W<DCGCQEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCQEI_SPEC>;
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
impl From<crate::W<DCGCQEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCQEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCQEI_D0` reader - QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCQEI_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCQEI_D0` writer - QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCQEI_D0_W<'a> = crate::BitWriter<'a, u32, DCGCQEI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcqei_d0(&self) -> SYSCTL_DCGCQEI_D0_R {
        SYSCTL_DCGCQEI_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcqei_d0(&mut self) -> SYSCTL_DCGCQEI_D0_W {
        SYSCTL_DCGCQEI_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcqei](index.html) module"]
pub struct DCGCQEI_SPEC;
impl crate::RegisterSpec for DCGCQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcqei::R](R) reader structure"]
impl crate::Readable for DCGCQEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcqei::W](W) writer structure"]
impl crate::Writable for DCGCQEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCQEI to value 0"]
impl crate::Resettable for DCGCQEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
