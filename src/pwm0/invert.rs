#[doc = "Register `INVERT` reader"]
pub struct R(crate::R<INVERT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INVERT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INVERT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INVERT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INVERT` writer"]
pub struct W(crate::W<INVERT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INVERT_SPEC>;
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
impl From<crate::W<INVERT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INVERT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_INVERT_PWM0INV` reader - Invert MnPWM0 Signal"]
pub type PWM_INVERT_PWM0INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM0INV` writer - Invert MnPWM0 Signal"]
pub type PWM_INVERT_PWM0INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 0>;
#[doc = "Field `PWM_INVERT_PWM1INV` reader - Invert MnPWM1 Signal"]
pub type PWM_INVERT_PWM1INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM1INV` writer - Invert MnPWM1 Signal"]
pub type PWM_INVERT_PWM1INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 1>;
#[doc = "Field `PWM_INVERT_PWM2INV` reader - Invert MnPWM2 Signal"]
pub type PWM_INVERT_PWM2INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM2INV` writer - Invert MnPWM2 Signal"]
pub type PWM_INVERT_PWM2INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 2>;
#[doc = "Field `PWM_INVERT_PWM3INV` reader - Invert MnPWM3 Signal"]
pub type PWM_INVERT_PWM3INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM3INV` writer - Invert MnPWM3 Signal"]
pub type PWM_INVERT_PWM3INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 3>;
#[doc = "Field `PWM_INVERT_PWM4INV` reader - Invert MnPWM4 Signal"]
pub type PWM_INVERT_PWM4INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM4INV` writer - Invert MnPWM4 Signal"]
pub type PWM_INVERT_PWM4INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 4>;
#[doc = "Field `PWM_INVERT_PWM5INV` reader - Invert MnPWM5 Signal"]
pub type PWM_INVERT_PWM5INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM5INV` writer - Invert MnPWM5 Signal"]
pub type PWM_INVERT_PWM5INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 5>;
#[doc = "Field `PWM_INVERT_PWM6INV` reader - Invert MnPWM6 Signal"]
pub type PWM_INVERT_PWM6INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM6INV` writer - Invert MnPWM6 Signal"]
pub type PWM_INVERT_PWM6INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 6>;
#[doc = "Field `PWM_INVERT_PWM7INV` reader - Invert MnPWM7 Signal"]
pub type PWM_INVERT_PWM7INV_R = crate::BitReader<bool>;
#[doc = "Field `PWM_INVERT_PWM7INV` writer - Invert MnPWM7 Signal"]
pub type PWM_INVERT_PWM7INV_W<'a> = crate::BitWriter<'a, u32, INVERT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&self) -> PWM_INVERT_PWM0INV_R {
        PWM_INVERT_PWM0INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&self) -> PWM_INVERT_PWM1INV_R {
        PWM_INVERT_PWM1INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&self) -> PWM_INVERT_PWM2INV_R {
        PWM_INVERT_PWM2INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&self) -> PWM_INVERT_PWM3INV_R {
        PWM_INVERT_PWM3INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&self) -> PWM_INVERT_PWM4INV_R {
        PWM_INVERT_PWM4INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&self) -> PWM_INVERT_PWM5INV_R {
        PWM_INVERT_PWM5INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm6inv(&self) -> PWM_INVERT_PWM6INV_R {
        PWM_INVERT_PWM6INV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm7inv(&self) -> PWM_INVERT_PWM7INV_R {
        PWM_INVERT_PWM7INV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert MnPWM0 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm0inv(&mut self) -> PWM_INVERT_PWM0INV_W {
        PWM_INVERT_PWM0INV_W::new(self)
    }
    #[doc = "Bit 1 - Invert MnPWM1 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm1inv(&mut self) -> PWM_INVERT_PWM1INV_W {
        PWM_INVERT_PWM1INV_W::new(self)
    }
    #[doc = "Bit 2 - Invert MnPWM2 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm2inv(&mut self) -> PWM_INVERT_PWM2INV_W {
        PWM_INVERT_PWM2INV_W::new(self)
    }
    #[doc = "Bit 3 - Invert MnPWM3 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm3inv(&mut self) -> PWM_INVERT_PWM3INV_W {
        PWM_INVERT_PWM3INV_W::new(self)
    }
    #[doc = "Bit 4 - Invert MnPWM4 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm4inv(&mut self) -> PWM_INVERT_PWM4INV_W {
        PWM_INVERT_PWM4INV_W::new(self)
    }
    #[doc = "Bit 5 - Invert MnPWM5 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm5inv(&mut self) -> PWM_INVERT_PWM5INV_W {
        PWM_INVERT_PWM5INV_W::new(self)
    }
    #[doc = "Bit 6 - Invert MnPWM6 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm6inv(&mut self) -> PWM_INVERT_PWM6INV_W {
        PWM_INVERT_PWM6INV_W::new(self)
    }
    #[doc = "Bit 7 - Invert MnPWM7 Signal"]
    #[inline(always)]
    pub fn pwm_invert_pwm7inv(&mut self) -> PWM_INVERT_PWM7INV_W {
        PWM_INVERT_PWM7INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Inversion\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invert](index.html) module"]
pub struct INVERT_SPEC;
impl crate::RegisterSpec for INVERT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [invert::R](R) reader structure"]
impl crate::Readable for INVERT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [invert::W](W) writer structure"]
impl crate::Writable for INVERT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INVERT to value 0"]
impl crate::Resettable for INVERT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
