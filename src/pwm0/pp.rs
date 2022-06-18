#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_PP_GCNT` reader - Generators"]
pub type PWM_PP_GCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWM_PP_GCNT` writer - Generators"]
pub type PWM_PP_GCNT_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 4, 0>;
#[doc = "Field `PWM_PP_FCNT` reader - Fault Inputs (per PWM unit)"]
pub type PWM_PP_FCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWM_PP_FCNT` writer - Fault Inputs (per PWM unit)"]
pub type PWM_PP_FCNT_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 4, 4>;
#[doc = "Field `PWM_PP_ESYNC` reader - Extended Synchronization"]
pub type PWM_PP_ESYNC_R = crate::BitReader<bool>;
#[doc = "Field `PWM_PP_ESYNC` writer - Extended Synchronization"]
pub type PWM_PP_ESYNC_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 8>;
#[doc = "Field `PWM_PP_EFAULT` reader - Extended Fault"]
pub type PWM_PP_EFAULT_R = crate::BitReader<bool>;
#[doc = "Field `PWM_PP_EFAULT` writer - Extended Fault"]
pub type PWM_PP_EFAULT_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 9>;
#[doc = "Field `PWM_PP_ONE` reader - One-Shot Mode"]
pub type PWM_PP_ONE_R = crate::BitReader<bool>;
#[doc = "Field `PWM_PP_ONE` writer - One-Shot Mode"]
pub type PWM_PP_ONE_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 10>;
impl R {
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn pwm_pp_gcnt(&self) -> PWM_PP_GCNT_R {
        PWM_PP_GCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Fault Inputs (per PWM unit)"]
    #[inline(always)]
    pub fn pwm_pp_fcnt(&self) -> PWM_PP_FCNT_R {
        PWM_PP_FCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn pwm_pp_esync(&self) -> PWM_PP_ESYNC_R {
        PWM_PP_ESYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn pwm_pp_efault(&self) -> PWM_PP_EFAULT_R {
        PWM_PP_EFAULT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn pwm_pp_one(&self) -> PWM_PP_ONE_R {
        PWM_PP_ONE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generators"]
    #[inline(always)]
    pub fn pwm_pp_gcnt(&mut self) -> PWM_PP_GCNT_W {
        PWM_PP_GCNT_W::new(self)
    }
    #[doc = "Bits 4:7 - Fault Inputs (per PWM unit)"]
    #[inline(always)]
    pub fn pwm_pp_fcnt(&mut self) -> PWM_PP_FCNT_W {
        PWM_PP_FCNT_W::new(self)
    }
    #[doc = "Bit 8 - Extended Synchronization"]
    #[inline(always)]
    pub fn pwm_pp_esync(&mut self) -> PWM_PP_ESYNC_W {
        PWM_PP_ESYNC_W::new(self)
    }
    #[doc = "Bit 9 - Extended Fault"]
    #[inline(always)]
    pub fn pwm_pp_efault(&mut self) -> PWM_PP_EFAULT_W {
        PWM_PP_EFAULT_W::new(self)
    }
    #[doc = "Bit 10 - One-Shot Mode"]
    #[inline(always)]
    pub fn pwm_pp_one(&mut self) -> PWM_PP_ONE_W {
        PWM_PP_ONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
