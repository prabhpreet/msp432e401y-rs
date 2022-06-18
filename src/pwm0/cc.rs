#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
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
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_CC_PWMDIV_A {
    #[doc = "0: /2"]
    PWM_CC_PWMDIV_2 = 0,
    #[doc = "1: /4"]
    PWM_CC_PWMDIV_4 = 1,
    #[doc = "2: /8"]
    PWM_CC_PWMDIV_8 = 2,
    #[doc = "3: /16"]
    PWM_CC_PWMDIV_16 = 3,
    #[doc = "4: /32"]
    PWM_CC_PWMDIV_32 = 4,
    #[doc = "5: /64"]
    PWM_CC_PWMDIV_64 = 5,
}
impl From<PWM_CC_PWMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_CC_PWMDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_CC_PWMDIV` reader - PWM Clock Divider"]
pub type PWM_CC_PWMDIV_R = crate::FieldReader<u8, PWM_CC_PWMDIV_A>;
impl PWM_CC_PWMDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_CC_PWMDIV_A> {
        match self.bits {
            0 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_2),
            1 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_4),
            2 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_8),
            3 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_16),
            4 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_32),
            5 => Some(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_2`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_2(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_2
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_4`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_4(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_4
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_8`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_8(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_8
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_16`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_16(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_16
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_32`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_32(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_32
    }
    #[doc = "Checks if the value of the field is `PWM_CC_PWMDIV_64`"]
    #[inline(always)]
    pub fn is_pwm_cc_pwmdiv_64(&self) -> bool {
        *self == PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_64
    }
}
#[doc = "Field `PWM_CC_PWMDIV` writer - PWM Clock Divider"]
pub type PWM_CC_PWMDIV_W<'a> = crate::FieldWriter<'a, u32, CC_SPEC, u8, PWM_CC_PWMDIV_A, 3, 0>;
impl<'a> PWM_CC_PWMDIV_W<'a> {
    #[doc = "/2"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_2(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_4(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_8(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_16(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_32(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv_64(self) -> &'a mut W {
        self.variant(PWM_CC_PWMDIV_A::PWM_CC_PWMDIV_64)
    }
}
#[doc = "Field `PWM_CC_USEPWM` reader - Use PWM Clock Divisor"]
pub type PWM_CC_USEPWM_R = crate::BitReader<bool>;
#[doc = "Field `PWM_CC_USEPWM` writer - Use PWM Clock Divisor"]
pub type PWM_CC_USEPWM_W<'a> = crate::BitWriter<'a, u32, CC_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv(&self) -> PWM_CC_PWMDIV_R {
        PWM_CC_PWMDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn pwm_cc_usepwm(&self) -> PWM_CC_USEPWM_R {
        PWM_CC_USEPWM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PWM Clock Divider"]
    #[inline(always)]
    pub fn pwm_cc_pwmdiv(&mut self) -> PWM_CC_PWMDIV_W {
        PWM_CC_PWMDIV_W::new(self)
    }
    #[doc = "Bit 8 - Use PWM Clock Divisor"]
    #[inline(always)]
    pub fn pwm_cc_usepwm(&mut self) -> PWM_CC_USEPWM_W {
        PWM_CC_USEPWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
