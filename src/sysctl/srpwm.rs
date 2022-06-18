#[doc = "Register `SRPWM` reader"]
pub struct R(crate::R<SRPWM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRPWM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRPWM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRPWM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRPWM` writer"]
pub struct W(crate::W<SRPWM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRPWM_SPEC>;
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
impl From<crate::W<SRPWM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRPWM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRPWM_R0` reader - PWM Module 0 Software Reset"]
pub type SYSCTL_SRPWM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRPWM_R0` writer - PWM Module 0 Software Reset"]
pub type SYSCTL_SRPWM_R0_W<'a> = crate::BitWriter<'a, u32, SRPWM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - PWM Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srpwm_r0(&self) -> SYSCTL_SRPWM_R0_R {
        SYSCTL_SRPWM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srpwm_r0(&mut self) -> SYSCTL_SRPWM_R0_W {
        SYSCTL_SRPWM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Width Modulator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srpwm](index.html) module"]
pub struct SRPWM_SPEC;
impl crate::RegisterSpec for SRPWM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srpwm::R](R) reader structure"]
impl crate::Readable for SRPWM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srpwm::W](W) writer structure"]
impl crate::Writable for SRPWM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRPWM to value 0"]
impl crate::Resettable for SRPWM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
