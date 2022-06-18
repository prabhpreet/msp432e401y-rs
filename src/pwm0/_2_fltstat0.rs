#[doc = "Register `_2_FLTSTAT0` reader"]
pub struct R(crate::R<_2_FLTSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_FLTSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_FLTSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_FLTSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PWM_2_FLTSTAT0_FAULT0` reader - Fault Input 0"]
pub type PWM_2_FLTSTAT0_FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_FLTSTAT0_FAULT1` reader - Fault Input 1"]
pub type PWM_2_FLTSTAT0_FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_FLTSTAT0_FAULT2` reader - Fault Input 2"]
pub type PWM_2_FLTSTAT0_FAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_FLTSTAT0_FAULT3` reader - Fault Input 3"]
pub type PWM_2_FLTSTAT0_FAULT3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Fault Input 0"]
    #[inline(always)]
    pub fn pwm_2_fltstat0_fault0(&self) -> PWM_2_FLTSTAT0_FAULT0_R {
        PWM_2_FLTSTAT0_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1"]
    #[inline(always)]
    pub fn pwm_2_fltstat0_fault1(&self) -> PWM_2_FLTSTAT0_FAULT1_R {
        PWM_2_FLTSTAT0_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2"]
    #[inline(always)]
    pub fn pwm_2_fltstat0_fault2(&self) -> PWM_2_FLTSTAT0_FAULT2_R {
        PWM_2_FLTSTAT0_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3"]
    #[inline(always)]
    pub fn pwm_2_fltstat0_fault3(&self) -> PWM_2_FLTSTAT0_FAULT3_R {
        PWM_2_FLTSTAT0_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PWM2 Fault Status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_fltstat0](index.html) module"]
pub struct _2_FLTSTAT0_SPEC;
impl crate::RegisterSpec for _2_FLTSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_fltstat0::R](R) reader structure"]
impl crate::Readable for _2_FLTSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _2_FLTSTAT0 to value 0"]
impl crate::Resettable for _2_FLTSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
