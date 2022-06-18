#[doc = "Register `_1_FLTSEN` reader"]
pub struct R(crate::R<_1_FLTSEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_FLTSEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_FLTSEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_FLTSEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1_FLTSEN` writer"]
pub struct W(crate::W<_1_FLTSEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1_FLTSEN_SPEC>;
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
impl From<crate::W<_1_FLTSEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1_FLTSEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_1_FLTSEN_FAULT0` reader - Fault0 Sense"]
pub type PWM_1_FLTSEN_FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_1_FLTSEN_FAULT0` writer - Fault0 Sense"]
pub type PWM_1_FLTSEN_FAULT0_W<'a> = crate::BitWriter<'a, u32, _1_FLTSEN_SPEC, bool, 0>;
#[doc = "Field `PWM_1_FLTSEN_FAULT1` reader - Fault1 Sense"]
pub type PWM_1_FLTSEN_FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_1_FLTSEN_FAULT1` writer - Fault1 Sense"]
pub type PWM_1_FLTSEN_FAULT1_W<'a> = crate::BitWriter<'a, u32, _1_FLTSEN_SPEC, bool, 1>;
#[doc = "Field `PWM_1_FLTSEN_FAULT2` reader - Fault2 Sense"]
pub type PWM_1_FLTSEN_FAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_1_FLTSEN_FAULT2` writer - Fault2 Sense"]
pub type PWM_1_FLTSEN_FAULT2_W<'a> = crate::BitWriter<'a, u32, _1_FLTSEN_SPEC, bool, 2>;
#[doc = "Field `PWM_1_FLTSEN_FAULT3` reader - Fault3 Sense"]
pub type PWM_1_FLTSEN_FAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_1_FLTSEN_FAULT3` writer - Fault3 Sense"]
pub type PWM_1_FLTSEN_FAULT3_W<'a> = crate::BitWriter<'a, u32, _1_FLTSEN_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault0(&self) -> PWM_1_FLTSEN_FAULT0_R {
        PWM_1_FLTSEN_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault1(&self) -> PWM_1_FLTSEN_FAULT1_R {
        PWM_1_FLTSEN_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault2 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault2(&self) -> PWM_1_FLTSEN_FAULT2_R {
        PWM_1_FLTSEN_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault3 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault3(&self) -> PWM_1_FLTSEN_FAULT3_R {
        PWM_1_FLTSEN_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault0 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault0(&mut self) -> PWM_1_FLTSEN_FAULT0_W {
        PWM_1_FLTSEN_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Fault1 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault1(&mut self) -> PWM_1_FLTSEN_FAULT1_W {
        PWM_1_FLTSEN_FAULT1_W::new(self)
    }
    #[doc = "Bit 2 - Fault2 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault2(&mut self) -> PWM_1_FLTSEN_FAULT2_W {
        PWM_1_FLTSEN_FAULT2_W::new(self)
    }
    #[doc = "Bit 3 - Fault3 Sense"]
    #[inline(always)]
    pub fn pwm_1_fltsen_fault3(&mut self) -> PWM_1_FLTSEN_FAULT3_W {
        PWM_1_FLTSEN_FAULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM1 Fault Pin Logic Sense\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_fltsen](index.html) module"]
pub struct _1_FLTSEN_SPEC;
impl crate::RegisterSpec for _1_FLTSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_fltsen::R](R) reader structure"]
impl crate::Readable for _1_FLTSEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1_fltsen::W](W) writer structure"]
impl crate::Writable for _1_FLTSEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1_FLTSEN to value 0"]
impl crate::Resettable for _1_FLTSEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
