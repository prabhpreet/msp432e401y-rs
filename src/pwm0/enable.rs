#[doc = "Register `ENABLE` reader"]
pub struct R(crate::R<ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE` writer"]
pub struct W(crate::W<ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_SPEC>;
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
impl From<crate::W<ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_ENABLE_PWM0EN` reader - MnPWM0 Output Enable"]
pub type PWM_ENABLE_PWM0EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM0EN` writer - MnPWM0 Output Enable"]
pub type PWM_ENABLE_PWM0EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 0>;
#[doc = "Field `PWM_ENABLE_PWM1EN` reader - MnPWM1 Output Enable"]
pub type PWM_ENABLE_PWM1EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM1EN` writer - MnPWM1 Output Enable"]
pub type PWM_ENABLE_PWM1EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 1>;
#[doc = "Field `PWM_ENABLE_PWM2EN` reader - MnPWM2 Output Enable"]
pub type PWM_ENABLE_PWM2EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM2EN` writer - MnPWM2 Output Enable"]
pub type PWM_ENABLE_PWM2EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 2>;
#[doc = "Field `PWM_ENABLE_PWM3EN` reader - MnPWM3 Output Enable"]
pub type PWM_ENABLE_PWM3EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM3EN` writer - MnPWM3 Output Enable"]
pub type PWM_ENABLE_PWM3EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 3>;
#[doc = "Field `PWM_ENABLE_PWM4EN` reader - MnPWM4 Output Enable"]
pub type PWM_ENABLE_PWM4EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM4EN` writer - MnPWM4 Output Enable"]
pub type PWM_ENABLE_PWM4EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 4>;
#[doc = "Field `PWM_ENABLE_PWM5EN` reader - MnPWM5 Output Enable"]
pub type PWM_ENABLE_PWM5EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM5EN` writer - MnPWM5 Output Enable"]
pub type PWM_ENABLE_PWM5EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 5>;
#[doc = "Field `PWM_ENABLE_PWM6EN` reader - MnPWM6 Output Enable"]
pub type PWM_ENABLE_PWM6EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM6EN` writer - MnPWM6 Output Enable"]
pub type PWM_ENABLE_PWM6EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 6>;
#[doc = "Field `PWM_ENABLE_PWM7EN` reader - MnPWM7 Output Enable"]
pub type PWM_ENABLE_PWM7EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ENABLE_PWM7EN` writer - MnPWM7 Output Enable"]
pub type PWM_ENABLE_PWM7EN_W<'a> = crate::BitWriter<'a, u32, ENABLE_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&self) -> PWM_ENABLE_PWM0EN_R {
        PWM_ENABLE_PWM0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&self) -> PWM_ENABLE_PWM1EN_R {
        PWM_ENABLE_PWM1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&self) -> PWM_ENABLE_PWM2EN_R {
        PWM_ENABLE_PWM2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&self) -> PWM_ENABLE_PWM3EN_R {
        PWM_ENABLE_PWM3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&self) -> PWM_ENABLE_PWM4EN_R {
        PWM_ENABLE_PWM4EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&self) -> PWM_ENABLE_PWM5EN_R {
        PWM_ENABLE_PWM5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm6en(&self) -> PWM_ENABLE_PWM6EN_R {
        PWM_ENABLE_PWM6EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm7en(&self) -> PWM_ENABLE_PWM7EN_R {
        PWM_ENABLE_PWM7EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm0en(&mut self) -> PWM_ENABLE_PWM0EN_W {
        PWM_ENABLE_PWM0EN_W::new(self)
    }
    #[doc = "Bit 1 - MnPWM1 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm1en(&mut self) -> PWM_ENABLE_PWM1EN_W {
        PWM_ENABLE_PWM1EN_W::new(self)
    }
    #[doc = "Bit 2 - MnPWM2 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm2en(&mut self) -> PWM_ENABLE_PWM2EN_W {
        PWM_ENABLE_PWM2EN_W::new(self)
    }
    #[doc = "Bit 3 - MnPWM3 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm3en(&mut self) -> PWM_ENABLE_PWM3EN_W {
        PWM_ENABLE_PWM3EN_W::new(self)
    }
    #[doc = "Bit 4 - MnPWM4 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm4en(&mut self) -> PWM_ENABLE_PWM4EN_W {
        PWM_ENABLE_PWM4EN_W::new(self)
    }
    #[doc = "Bit 5 - MnPWM5 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm5en(&mut self) -> PWM_ENABLE_PWM5EN_W {
        PWM_ENABLE_PWM5EN_W::new(self)
    }
    #[doc = "Bit 6 - MnPWM6 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm6en(&mut self) -> PWM_ENABLE_PWM6EN_W {
        PWM_ENABLE_PWM6EN_W::new(self)
    }
    #[doc = "Bit 7 - MnPWM7 Output Enable"]
    #[inline(always)]
    pub fn pwm_enable_pwm7en(&mut self) -> PWM_ENABLE_PWM7EN_W {
        PWM_ENABLE_PWM7EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](index.html) module"]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable::R](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable::W](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
