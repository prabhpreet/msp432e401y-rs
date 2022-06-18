#[doc = "Register `SCGCPWM` reader"]
pub struct R(crate::R<SCGCPWM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCPWM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCPWM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCPWM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCPWM` writer"]
pub struct W(crate::W<SCGCPWM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCPWM_SPEC>;
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
impl From<crate::W<SCGCPWM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCPWM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCPWM_S0` reader - PWM Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCPWM_S0` writer - PWM Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCPWM_S0_W<'a> = crate::BitWriter<'a, u32, SCGCPWM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcpwm_s0(&self) -> SYSCTL_SCGCPWM_S0_R {
        SYSCTL_SCGCPWM_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcpwm_s0(&mut self) -> SYSCTL_SCGCPWM_S0_W {
        SYSCTL_SCGCPWM_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcpwm](index.html) module"]
pub struct SCGCPWM_SPEC;
impl crate::RegisterSpec for SCGCPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcpwm::R](R) reader structure"]
impl crate::Readable for SCGCPWM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcpwm::W](W) writer structure"]
impl crate::Writable for SCGCPWM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCPWM to value 0"]
impl crate::Resettable for SCGCPWM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
