#[doc = "Register `_3_FLTSRC0` reader"]
pub struct R(crate::R<_3_FLTSRC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_3_FLTSRC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_3_FLTSRC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_3_FLTSRC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_3_FLTSRC0` writer"]
pub struct W(crate::W<_3_FLTSRC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_3_FLTSRC0_SPEC>;
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
impl From<crate::W<_3_FLTSRC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_3_FLTSRC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_3_FLTSRC0_FAULT0` reader - Fault0 Input"]
pub type PWM_3_FLTSRC0_FAULT0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT0` writer - Fault0 Input"]
pub type PWM_3_FLTSRC0_FAULT0_W<'a> = crate::BitWriter<'a, u32, _3_FLTSRC0_SPEC, bool, 0>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT1` reader - Fault1 Input"]
pub type PWM_3_FLTSRC0_FAULT1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT1` writer - Fault1 Input"]
pub type PWM_3_FLTSRC0_FAULT1_W<'a> = crate::BitWriter<'a, u32, _3_FLTSRC0_SPEC, bool, 1>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT2` reader - Fault2 Input"]
pub type PWM_3_FLTSRC0_FAULT2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT2` writer - Fault2 Input"]
pub type PWM_3_FLTSRC0_FAULT2_W<'a> = crate::BitWriter<'a, u32, _3_FLTSRC0_SPEC, bool, 2>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT3` reader - Fault3 Input"]
pub type PWM_3_FLTSRC0_FAULT3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_FLTSRC0_FAULT3` writer - Fault3 Input"]
pub type PWM_3_FLTSRC0_FAULT3_W<'a> = crate::BitWriter<'a, u32, _3_FLTSRC0_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Fault0 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault0(&self) -> PWM_3_FLTSRC0_FAULT0_R {
        PWM_3_FLTSRC0_FAULT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault1 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault1(&self) -> PWM_3_FLTSRC0_FAULT1_R {
        PWM_3_FLTSRC0_FAULT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault2 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault2(&self) -> PWM_3_FLTSRC0_FAULT2_R {
        PWM_3_FLTSRC0_FAULT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault3 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault3(&self) -> PWM_3_FLTSRC0_FAULT3_R {
        PWM_3_FLTSRC0_FAULT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault0 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault0(&mut self) -> PWM_3_FLTSRC0_FAULT0_W {
        PWM_3_FLTSRC0_FAULT0_W::new(self)
    }
    #[doc = "Bit 1 - Fault1 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault1(&mut self) -> PWM_3_FLTSRC0_FAULT1_W {
        PWM_3_FLTSRC0_FAULT1_W::new(self)
    }
    #[doc = "Bit 2 - Fault2 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault2(&mut self) -> PWM_3_FLTSRC0_FAULT2_W {
        PWM_3_FLTSRC0_FAULT2_W::new(self)
    }
    #[doc = "Bit 3 - Fault3 Input"]
    #[inline(always)]
    pub fn pwm_3_fltsrc0_fault3(&mut self) -> PWM_3_FLTSRC0_FAULT3_W {
        PWM_3_FLTSRC0_FAULT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM3 Fault Source 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_3_fltsrc0](index.html) module"]
pub struct _3_FLTSRC0_SPEC;
impl crate::RegisterSpec for _3_FLTSRC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_3_fltsrc0::R](R) reader structure"]
impl crate::Readable for _3_FLTSRC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_3_fltsrc0::W](W) writer structure"]
impl crate::Writable for _3_FLTSRC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _3_FLTSRC0 to value 0"]
impl crate::Resettable for _3_FLTSRC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
