#[doc = "Register `TSSEL` reader"]
pub struct R(crate::R<TSSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSSEL` writer"]
pub struct W(crate::W<TSSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSSEL_SPEC>;
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
impl From<crate::W<TSSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Generator 0 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS0_A {
    #[doc = "0: Use Generator 0 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS0_0 = 0,
}
impl From<ADC_TSSEL_PS0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_TSSEL_PS0` reader - Generator 0 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS0_R = crate::FieldReader<u8, ADC_TSSEL_PS0_A>;
impl ADC_TSSEL_PS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_TSSEL_PS0_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS0_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps0_0(&self) -> bool {
        *self == ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0
    }
}
#[doc = "Field `ADC_TSSEL_PS0` writer - Generator 0 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS0_W<'a> = crate::FieldWriter<'a, u32, TSSEL_SPEC, u8, ADC_TSSEL_PS0_A, 2, 4>;
impl<'a> ADC_TSSEL_PS0_W<'a> {
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps0_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS0_A::ADC_TSSEL_PS0_0)
    }
}
#[doc = "Generator 1 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS1_A {
    #[doc = "0: Use Generator 1 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS1_0 = 0,
}
impl From<ADC_TSSEL_PS1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_TSSEL_PS1` reader - Generator 1 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS1_R = crate::FieldReader<u8, ADC_TSSEL_PS1_A>;
impl ADC_TSSEL_PS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_TSSEL_PS1_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS1_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps1_0(&self) -> bool {
        *self == ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0
    }
}
#[doc = "Field `ADC_TSSEL_PS1` writer - Generator 1 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS1_W<'a> = crate::FieldWriter<'a, u32, TSSEL_SPEC, u8, ADC_TSSEL_PS1_A, 2, 12>;
impl<'a> ADC_TSSEL_PS1_W<'a> {
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps1_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS1_A::ADC_TSSEL_PS1_0)
    }
}
#[doc = "Generator 2 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS2_A {
    #[doc = "0: Use Generator 2 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS2_0 = 0,
}
impl From<ADC_TSSEL_PS2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_TSSEL_PS2` reader - Generator 2 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS2_R = crate::FieldReader<u8, ADC_TSSEL_PS2_A>;
impl ADC_TSSEL_PS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_TSSEL_PS2_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS2_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps2_0(&self) -> bool {
        *self == ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0
    }
}
#[doc = "Field `ADC_TSSEL_PS2` writer - Generator 2 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS2_W<'a> = crate::FieldWriter<'a, u32, TSSEL_SPEC, u8, ADC_TSSEL_PS2_A, 2, 20>;
impl<'a> ADC_TSSEL_PS2_W<'a> {
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps2_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS2_A::ADC_TSSEL_PS2_0)
    }
}
#[doc = "Generator 3 PWM Module Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_TSSEL_PS3_A {
    #[doc = "0: Use Generator 3 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS3_0 = 0,
}
impl From<ADC_TSSEL_PS3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_TSSEL_PS3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_TSSEL_PS3` reader - Generator 3 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS3_R = crate::FieldReader<u8, ADC_TSSEL_PS3_A>;
impl ADC_TSSEL_PS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_TSSEL_PS3_A> {
        match self.bits {
            0 => Some(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS3_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps3_0(&self) -> bool {
        *self == ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0
    }
}
#[doc = "Field `ADC_TSSEL_PS3` writer - Generator 3 PWM Module Trigger Select"]
pub type ADC_TSSEL_PS3_W<'a> = crate::FieldWriter<'a, u32, TSSEL_SPEC, u8, ADC_TSSEL_PS3_A, 2, 28>;
impl<'a> ADC_TSSEL_PS3_W<'a> {
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps3_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS3_A::ADC_TSSEL_PS3_0)
    }
}
impl R {
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps0(&self) -> ADC_TSSEL_PS0_R {
        ADC_TSSEL_PS0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps1(&self) -> ADC_TSSEL_PS1_R {
        ADC_TSSEL_PS1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps2(&self) -> ADC_TSSEL_PS2_R {
        ADC_TSSEL_PS2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps3(&self) -> ADC_TSSEL_PS3_R {
        ADC_TSSEL_PS3_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps0(&mut self) -> ADC_TSSEL_PS0_W {
        ADC_TSSEL_PS0_W::new(self)
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps1(&mut self) -> ADC_TSSEL_PS1_W {
        ADC_TSSEL_PS1_W::new(self)
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps2(&mut self) -> ADC_TSSEL_PS2_W {
        ADC_TSSEL_PS2_W::new(self)
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps3(&mut self) -> ADC_TSSEL_PS3_W {
        ADC_TSSEL_PS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trigger Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tssel](index.html) module"]
pub struct TSSEL_SPEC;
impl crate::RegisterSpec for TSSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tssel::R](R) reader structure"]
impl crate::Readable for TSSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tssel::W](W) writer structure"]
impl crate::Writable for TSSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSSEL to value 0"]
impl crate::Resettable for TSSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
