#[doc = "Register `_3_FLTSTAT1` reader"]
pub struct R(crate::R<_3_FLTSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_3_FLTSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_3_FLTSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_3_FLTSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PWM_3_FLTSTAT1_DCMP0` reader - Digital Comparator 0 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP1` reader - Digital Comparator 1 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP2` reader - Digital Comparator 2 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP3` reader - Digital Comparator 3 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP4` reader - Digital Comparator 4 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP4_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP5` reader - Digital Comparator 5 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP5_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP6` reader - Digital Comparator 6 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP6_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSTAT1_DCMP7` reader - Digital Comparator 7 Trigger"]
pub type PWM_3_FLTSTAT1_DCMP7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Digital Comparator 0 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp0(&self) -> PWM_3_FLTSTAT1_DCMP0_R {
        PWM_3_FLTSTAT1_DCMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Comparator 1 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp1(&self) -> PWM_3_FLTSTAT1_DCMP1_R {
        PWM_3_FLTSTAT1_DCMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Comparator 2 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp2(&self) -> PWM_3_FLTSTAT1_DCMP2_R {
        PWM_3_FLTSTAT1_DCMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Comparator 3 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp3(&self) -> PWM_3_FLTSTAT1_DCMP3_R {
        PWM_3_FLTSTAT1_DCMP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digital Comparator 4 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp4(&self) -> PWM_3_FLTSTAT1_DCMP4_R {
        PWM_3_FLTSTAT1_DCMP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Comparator 5 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp5(&self) -> PWM_3_FLTSTAT1_DCMP5_R {
        PWM_3_FLTSTAT1_DCMP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Digital Comparator 6 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp6(&self) -> PWM_3_FLTSTAT1_DCMP6_R {
        PWM_3_FLTSTAT1_DCMP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Comparator 7 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp7(&self) -> PWM_3_FLTSTAT1_DCMP7_R {
        PWM_3_FLTSTAT1_DCMP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWM3 Fault Status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_3_fltstat1](index.html) module"]
pub struct _3_FLTSTAT1_SPEC;
impl crate::RegisterSpec for _3_FLTSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_3_fltstat1::R](R) reader structure"]
impl crate::Readable for _3_FLTSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _3_FLTSTAT1 to value 0"]
impl crate::Resettable for _3_FLTSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
