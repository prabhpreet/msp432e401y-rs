#[doc = "Register `FAULTVAL` reader"]
pub struct R(crate::R<FAULTVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULTVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULTVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULTVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULTVAL` writer"]
pub struct W(crate::W<FAULTVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULTVAL_SPEC>;
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
impl From<crate::W<FAULTVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULTVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_FAULTVAL_PWM0` reader - MnPWM0 Fault Value"]
pub type PWM_FAULTVAL_PWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM0` writer - MnPWM0 Fault Value"]
pub type PWM_FAULTVAL_PWM0_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 0>;
#[doc = "Field `PWM_FAULTVAL_PWM1` reader - MnPWM1 Fault Value"]
pub type PWM_FAULTVAL_PWM1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM1` writer - MnPWM1 Fault Value"]
pub type PWM_FAULTVAL_PWM1_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 1>;
#[doc = "Field `PWM_FAULTVAL_PWM2` reader - MnPWM2 Fault Value"]
pub type PWM_FAULTVAL_PWM2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM2` writer - MnPWM2 Fault Value"]
pub type PWM_FAULTVAL_PWM2_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 2>;
#[doc = "Field `PWM_FAULTVAL_PWM3` reader - MnPWM3 Fault Value"]
pub type PWM_FAULTVAL_PWM3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM3` writer - MnPWM3 Fault Value"]
pub type PWM_FAULTVAL_PWM3_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 3>;
#[doc = "Field `PWM_FAULTVAL_PWM4` reader - MnPWM4 Fault Value"]
pub type PWM_FAULTVAL_PWM4_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM4` writer - MnPWM4 Fault Value"]
pub type PWM_FAULTVAL_PWM4_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 4>;
#[doc = "Field `PWM_FAULTVAL_PWM5` reader - MnPWM5 Fault Value"]
pub type PWM_FAULTVAL_PWM5_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM5` writer - MnPWM5 Fault Value"]
pub type PWM_FAULTVAL_PWM5_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 5>;
#[doc = "Field `PWM_FAULTVAL_PWM6` reader - MnPWM6 Fault Value"]
pub type PWM_FAULTVAL_PWM6_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM6` writer - MnPWM6 Fault Value"]
pub type PWM_FAULTVAL_PWM6_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 6>;
#[doc = "Field `PWM_FAULTVAL_PWM7` reader - MnPWM7 Fault Value"]
pub type PWM_FAULTVAL_PWM7_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULTVAL_PWM7` writer - MnPWM7 Fault Value"]
pub type PWM_FAULTVAL_PWM7_W<'a> = crate::BitWriter<'a, u32, FAULTVAL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm0(&self) -> PWM_FAULTVAL_PWM0_R {
        PWM_FAULTVAL_PWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm1(&self) -> PWM_FAULTVAL_PWM1_R {
        PWM_FAULTVAL_PWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm2(&self) -> PWM_FAULTVAL_PWM2_R {
        PWM_FAULTVAL_PWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm3(&self) -> PWM_FAULTVAL_PWM3_R {
        PWM_FAULTVAL_PWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm4(&self) -> PWM_FAULTVAL_PWM4_R {
        PWM_FAULTVAL_PWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm5(&self) -> PWM_FAULTVAL_PWM5_R {
        PWM_FAULTVAL_PWM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm6(&self) -> PWM_FAULTVAL_PWM6_R {
        PWM_FAULTVAL_PWM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm7(&self) -> PWM_FAULTVAL_PWM7_R {
        PWM_FAULTVAL_PWM7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm0(&mut self) -> PWM_FAULTVAL_PWM0_W {
        PWM_FAULTVAL_PWM0_W::new(self)
    }
    #[doc = "Bit 1 - MnPWM1 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm1(&mut self) -> PWM_FAULTVAL_PWM1_W {
        PWM_FAULTVAL_PWM1_W::new(self)
    }
    #[doc = "Bit 2 - MnPWM2 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm2(&mut self) -> PWM_FAULTVAL_PWM2_W {
        PWM_FAULTVAL_PWM2_W::new(self)
    }
    #[doc = "Bit 3 - MnPWM3 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm3(&mut self) -> PWM_FAULTVAL_PWM3_W {
        PWM_FAULTVAL_PWM3_W::new(self)
    }
    #[doc = "Bit 4 - MnPWM4 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm4(&mut self) -> PWM_FAULTVAL_PWM4_W {
        PWM_FAULTVAL_PWM4_W::new(self)
    }
    #[doc = "Bit 5 - MnPWM5 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm5(&mut self) -> PWM_FAULTVAL_PWM5_W {
        PWM_FAULTVAL_PWM5_W::new(self)
    }
    #[doc = "Bit 6 - MnPWM6 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm6(&mut self) -> PWM_FAULTVAL_PWM6_W {
        PWM_FAULTVAL_PWM6_W::new(self)
    }
    #[doc = "Bit 7 - MnPWM7 Fault Value"]
    #[inline(always)]
    pub fn pwm_faultval_pwm7(&mut self) -> PWM_FAULTVAL_PWM7_W {
        PWM_FAULTVAL_PWM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Fault Condition Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultval](index.html) module"]
pub struct FAULTVAL_SPEC;
impl crate::RegisterSpec for FAULTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faultval::R](R) reader structure"]
impl crate::Readable for FAULTVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faultval::W](W) writer structure"]
impl crate::Writable for FAULTVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAULTVAL to value 0"]
impl crate::Resettable for FAULTVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
