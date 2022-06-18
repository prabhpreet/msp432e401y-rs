#[doc = "Register `FAULT` reader"]
pub struct R(crate::R<FAULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULT` writer"]
pub struct W(crate::W<FAULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULT_SPEC>;
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
impl From<crate::W<FAULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_FAULT_FAULT0` reader - MnPWM0 Fault"]
pub type PWM_FAULT_FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT0` writer - MnPWM0 Fault"]
pub type PWM_FAULT_FAULT0_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 0>;
#[doc = "Field `PWM_FAULT_FAULT1` reader - MnPWM1 Fault"]
pub type PWM_FAULT_FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT1` writer - MnPWM1 Fault"]
pub type PWM_FAULT_FAULT1_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 1>;
#[doc = "Field `PWM_FAULT_FAULT2` reader - MnPWM2 Fault"]
pub type PWM_FAULT_FAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT2` writer - MnPWM2 Fault"]
pub type PWM_FAULT_FAULT2_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 2>;
#[doc = "Field `PWM_FAULT_FAULT3` reader - MnPWM3 Fault"]
pub type PWM_FAULT_FAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT3` writer - MnPWM3 Fault"]
pub type PWM_FAULT_FAULT3_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 3>;
#[doc = "Field `PWM_FAULT_FAULT4` reader - MnPWM4 Fault"]
pub type PWM_FAULT_FAULT4_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT4` writer - MnPWM4 Fault"]
pub type PWM_FAULT_FAULT4_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 4>;
#[doc = "Field `PWM_FAULT_FAULT5` reader - MnPWM5 Fault"]
pub type PWM_FAULT_FAULT5_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT5` writer - MnPWM5 Fault"]
pub type PWM_FAULT_FAULT5_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 5>;
#[doc = "Field `PWM_FAULT_FAULT6` reader - MnPWM6 Fault"]
pub type PWM_FAULT_FAULT6_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT6` writer - MnPWM6 Fault"]
pub type PWM_FAULT_FAULT6_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 6>;
#[doc = "Field `PWM_FAULT_FAULT7` reader - MnPWM7 Fault"]
pub type PWM_FAULT_FAULT7_R = crate::BitReader<bool>;
#[doc = "Field `PWM_FAULT_FAULT7` writer - MnPWM7 Fault"]
pub type PWM_FAULT_FAULT7_W<'a> = crate::BitWriter<'a, u32, FAULT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - MnPWM0 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault0(&self) -> PWM_FAULT_FAULT0_R {
        PWM_FAULT_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MnPWM1 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault1(&self) -> PWM_FAULT_FAULT1_R {
        PWM_FAULT_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MnPWM2 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault2(&self) -> PWM_FAULT_FAULT2_R {
        PWM_FAULT_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MnPWM3 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault3(&self) -> PWM_FAULT_FAULT3_R {
        PWM_FAULT_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MnPWM4 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault4(&self) -> PWM_FAULT_FAULT4_R {
        PWM_FAULT_FAULT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MnPWM5 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault5(&self) -> PWM_FAULT_FAULT5_R {
        PWM_FAULT_FAULT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MnPWM6 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault6(&self) -> PWM_FAULT_FAULT6_R {
        PWM_FAULT_FAULT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MnPWM7 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault7(&self) -> PWM_FAULT_FAULT7_R {
        PWM_FAULT_FAULT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MnPWM0 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault0(&mut self) -> PWM_FAULT_FAULT0_W {
        PWM_FAULT_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - MnPWM1 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault1(&mut self) -> PWM_FAULT_FAULT1_W {
        PWM_FAULT_FAULT1_W::new(self)
    }
    #[doc = "Bit 2 - MnPWM2 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault2(&mut self) -> PWM_FAULT_FAULT2_W {
        PWM_FAULT_FAULT2_W::new(self)
    }
    #[doc = "Bit 3 - MnPWM3 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault3(&mut self) -> PWM_FAULT_FAULT3_W {
        PWM_FAULT_FAULT3_W::new(self)
    }
    #[doc = "Bit 4 - MnPWM4 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault4(&mut self) -> PWM_FAULT_FAULT4_W {
        PWM_FAULT_FAULT4_W::new(self)
    }
    #[doc = "Bit 5 - MnPWM5 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault5(&mut self) -> PWM_FAULT_FAULT5_W {
        PWM_FAULT_FAULT5_W::new(self)
    }
    #[doc = "Bit 6 - MnPWM6 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault6(&mut self) -> PWM_FAULT_FAULT6_W {
        PWM_FAULT_FAULT6_W::new(self)
    }
    #[doc = "Bit 7 - MnPWM7 Fault"]
    #[inline(always)]
    pub fn pwm_fault_fault7(&mut self) -> PWM_FAULT_FAULT7_W {
        PWM_FAULT_FAULT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Fault\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault](index.html) module"]
pub struct FAULT_SPEC;
impl crate::RegisterSpec for FAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fault::R](R) reader structure"]
impl crate::Readable for FAULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fault::W](W) writer structure"]
impl crate::Writable for FAULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAULT to value 0"]
impl crate::Resettable for FAULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
