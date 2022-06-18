#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_RIS_INTPWM0` reader - PWM0 Interrupt Asserted"]
pub type PWM_RIS_INTPWM0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTPWM0` writer - PWM0 Interrupt Asserted"]
pub type PWM_RIS_INTPWM0_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `PWM_RIS_INTPWM1` reader - PWM1 Interrupt Asserted"]
pub type PWM_RIS_INTPWM1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTPWM1` writer - PWM1 Interrupt Asserted"]
pub type PWM_RIS_INTPWM1_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `PWM_RIS_INTPWM2` reader - PWM2 Interrupt Asserted"]
pub type PWM_RIS_INTPWM2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTPWM2` writer - PWM2 Interrupt Asserted"]
pub type PWM_RIS_INTPWM2_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `PWM_RIS_INTPWM3` reader - PWM3 Interrupt Asserted"]
pub type PWM_RIS_INTPWM3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTPWM3` writer - PWM3 Interrupt Asserted"]
pub type PWM_RIS_INTPWM3_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `PWM_RIS_INTFAULT0` reader - Interrupt Fault PWM 0"]
pub type PWM_RIS_INTFAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTFAULT0` writer - Interrupt Fault PWM 0"]
pub type PWM_RIS_INTFAULT0_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 16>;
#[doc = "Field `PWM_RIS_INTFAULT1` reader - Interrupt Fault PWM 1"]
pub type PWM_RIS_INTFAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTFAULT1` writer - Interrupt Fault PWM 1"]
pub type PWM_RIS_INTFAULT1_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 17>;
#[doc = "Field `PWM_RIS_INTFAULT2` reader - Interrupt Fault PWM 2"]
pub type PWM_RIS_INTFAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTFAULT2` writer - Interrupt Fault PWM 2"]
pub type PWM_RIS_INTFAULT2_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 18>;
#[doc = "Field `PWM_RIS_INTFAULT3` reader - Interrupt Fault PWM 3"]
pub type PWM_RIS_INTFAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_RIS_INTFAULT3` writer - Interrupt Fault PWM 3"]
pub type PWM_RIS_INTFAULT3_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - PWM0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm0(&self) -> PWM_RIS_INTPWM0_R {
        PWM_RIS_INTPWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm1(&self) -> PWM_RIS_INTPWM1_R {
        PWM_RIS_INTPWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm2(&self) -> PWM_RIS_INTPWM2_R {
        PWM_RIS_INTPWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm3(&self) -> PWM_RIS_INTPWM3_R {
        PWM_RIS_INTPWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Fault PWM 0"]
    #[inline(always)]
    pub fn pwm_ris_intfault0(&self) -> PWM_RIS_INTFAULT0_R {
        PWM_RIS_INTFAULT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Fault PWM 1"]
    #[inline(always)]
    pub fn pwm_ris_intfault1(&self) -> PWM_RIS_INTFAULT1_R {
        PWM_RIS_INTFAULT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Fault PWM 2"]
    #[inline(always)]
    pub fn pwm_ris_intfault2(&self) -> PWM_RIS_INTFAULT2_R {
        PWM_RIS_INTFAULT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Fault PWM 3"]
    #[inline(always)]
    pub fn pwm_ris_intfault3(&self) -> PWM_RIS_INTFAULT3_R {
        PWM_RIS_INTFAULT3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm0(&mut self) -> PWM_RIS_INTPWM0_W {
        PWM_RIS_INTPWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm1(&mut self) -> PWM_RIS_INTPWM1_W {
        PWM_RIS_INTPWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm2(&mut self) -> PWM_RIS_INTPWM2_W {
        PWM_RIS_INTPWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Interrupt Asserted"]
    #[inline(always)]
    pub fn pwm_ris_intpwm3(&mut self) -> PWM_RIS_INTPWM3_W {
        PWM_RIS_INTPWM3_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Fault PWM 0"]
    #[inline(always)]
    pub fn pwm_ris_intfault0(&mut self) -> PWM_RIS_INTFAULT0_W {
        PWM_RIS_INTFAULT0_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Fault PWM 1"]
    #[inline(always)]
    pub fn pwm_ris_intfault1(&mut self) -> PWM_RIS_INTFAULT1_W {
        PWM_RIS_INTFAULT1_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Fault PWM 2"]
    #[inline(always)]
    pub fn pwm_ris_intfault2(&mut self) -> PWM_RIS_INTFAULT2_W {
        PWM_RIS_INTFAULT2_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Fault PWM 3"]
    #[inline(always)]
    pub fn pwm_ris_intfault3(&mut self) -> PWM_RIS_INTFAULT3_W {
        PWM_RIS_INTFAULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
