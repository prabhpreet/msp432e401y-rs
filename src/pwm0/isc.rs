#[doc = "Register `ISC` reader"]
pub struct R(crate::R<ISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISC` writer"]
pub struct W(crate::W<ISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISC_SPEC>;
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
impl From<crate::W<ISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_ISC_INTPWM0` reader - PWM0 Interrupt Status"]
pub type PWM_ISC_INTPWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTPWM0` writer - PWM0 Interrupt Status"]
pub type PWM_ISC_INTPWM0_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 0>;
#[doc = "Field `PWM_ISC_INTPWM1` reader - PWM1 Interrupt Status"]
pub type PWM_ISC_INTPWM1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTPWM1` writer - PWM1 Interrupt Status"]
pub type PWM_ISC_INTPWM1_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 1>;
#[doc = "Field `PWM_ISC_INTPWM2` reader - PWM2 Interrupt Status"]
pub type PWM_ISC_INTPWM2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTPWM2` writer - PWM2 Interrupt Status"]
pub type PWM_ISC_INTPWM2_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 2>;
#[doc = "Field `PWM_ISC_INTPWM3` reader - PWM3 Interrupt Status"]
pub type PWM_ISC_INTPWM3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTPWM3` writer - PWM3 Interrupt Status"]
pub type PWM_ISC_INTPWM3_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 3>;
#[doc = "Field `PWM_ISC_INTFAULT0` reader - FAULT0 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTFAULT0` writer - FAULT0 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT0_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 16>;
#[doc = "Field `PWM_ISC_INTFAULT1` reader - FAULT1 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTFAULT1` writer - FAULT1 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT1_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 17>;
#[doc = "Field `PWM_ISC_INTFAULT2` reader - FAULT2 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTFAULT2` writer - FAULT2 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT2_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 18>;
#[doc = "Field `PWM_ISC_INTFAULT3` reader - FAULT3 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_ISC_INTFAULT3` writer - FAULT3 Interrupt Asserted"]
pub type PWM_ISC_INTFAULT3_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm0(&self) -> PWM_ISC_INTPWM0_R {
        PWM_ISC_INTPWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm1(&self) -> PWM_ISC_INTPWM1_R {
        PWM_ISC_INTPWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm2(&self) -> PWM_ISC_INTPWM2_R {
        PWM_ISC_INTPWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm3(&self) -> PWM_ISC_INTPWM3_R {
        PWM_ISC_INTPWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - FAULT0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault0(&self) -> PWM_ISC_INTFAULT0_R {
        PWM_ISC_INTFAULT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FAULT1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault1(&self) -> PWM_ISC_INTFAULT1_R {
        PWM_ISC_INTFAULT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FAULT2 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault2(&self) -> PWM_ISC_INTFAULT2_R {
        PWM_ISC_INTFAULT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FAULT3 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault3(&self) -> PWM_ISC_INTFAULT3_R {
        PWM_ISC_INTFAULT3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm0(&mut self) -> PWM_ISC_INTPWM0_W {
        PWM_ISC_INTPWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm1(&mut self) -> PWM_ISC_INTPWM1_W {
        PWM_ISC_INTPWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm2(&mut self) -> PWM_ISC_INTPWM2_W {
        PWM_ISC_INTPWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_isc_intpwm3(&mut self) -> PWM_ISC_INTPWM3_W {
        PWM_ISC_INTPWM3_W::new(self)
    }
    #[doc = "Bit 16 - FAULT0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault0(&mut self) -> PWM_ISC_INTFAULT0_W {
        PWM_ISC_INTFAULT0_W::new(self)
    }
    #[doc = "Bit 17 - FAULT1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault1(&mut self) -> PWM_ISC_INTFAULT1_W {
        PWM_ISC_INTFAULT1_W::new(self)
    }
    #[doc = "Bit 18 - FAULT2 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault2(&mut self) -> PWM_ISC_INTFAULT2_W {
        PWM_ISC_INTFAULT2_W::new(self)
    }
    #[doc = "Bit 19 - FAULT3 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_isc_intfault3(&mut self) -> PWM_ISC_INTFAULT3_W {
        PWM_ISC_INTFAULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](index.html) module"]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isc::R](R) reader structure"]
impl crate::Readable for ISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isc::W](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
