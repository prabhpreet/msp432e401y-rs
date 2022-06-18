#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_STATUS_FAULT0` reader - Generator 0 Fault Status"]
pub type PWM_STATUS_FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_STATUS_FAULT0` writer - Generator 0 Fault Status"]
pub type PWM_STATUS_FAULT0_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 0>;
#[doc = "Field `PWM_STATUS_FAULT1` reader - Generator 1 Fault Status"]
pub type PWM_STATUS_FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_STATUS_FAULT1` writer - Generator 1 Fault Status"]
pub type PWM_STATUS_FAULT1_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 1>;
#[doc = "Field `PWM_STATUS_FAULT2` reader - Generator 2 Fault Status"]
pub type PWM_STATUS_FAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_STATUS_FAULT2` writer - Generator 2 Fault Status"]
pub type PWM_STATUS_FAULT2_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 2>;
#[doc = "Field `PWM_STATUS_FAULT3` reader - Generator 3 Fault Status"]
pub type PWM_STATUS_FAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_STATUS_FAULT3` writer - Generator 3 Fault Status"]
pub type PWM_STATUS_FAULT3_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Generator 0 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault0(&self) -> PWM_STATUS_FAULT0_R {
        PWM_STATUS_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generator 1 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault1(&self) -> PWM_STATUS_FAULT1_R {
        PWM_STATUS_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generator 2 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault2(&self) -> PWM_STATUS_FAULT2_R {
        PWM_STATUS_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generator 3 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault3(&self) -> PWM_STATUS_FAULT3_R {
        PWM_STATUS_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generator 0 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault0(&mut self) -> PWM_STATUS_FAULT0_W {
        PWM_STATUS_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Generator 1 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault1(&mut self) -> PWM_STATUS_FAULT1_W {
        PWM_STATUS_FAULT1_W::new(self)
    }
    #[doc = "Bit 2 - Generator 2 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault2(&mut self) -> PWM_STATUS_FAULT2_W {
        PWM_STATUS_FAULT2_W::new(self)
    }
    #[doc = "Bit 3 - Generator 3 Fault Status"]
    #[inline(always)]
    pub fn pwm_status_fault3(&mut self) -> PWM_STATUS_FAULT3_W {
        PWM_STATUS_FAULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
