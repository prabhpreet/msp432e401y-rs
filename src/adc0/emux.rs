#[doc = "Register `EMUX` reader"]
pub struct R(crate::R<EMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMUX` writer"]
pub struct W(crate::W<EMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMUX_SPEC>;
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
impl From<crate::W<EMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SS0 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM0_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM0_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM0_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM0_COMP1 = 2,
    #[doc = "3: Analog Comparator 2"]
    ADC_EMUX_EM0_COMP2 = 3,
    #[doc = "4: External (GPIO Pins)"]
    ADC_EMUX_EM0_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM0_TIMER = 5,
    #[doc = "6: PWM generator 0"]
    ADC_EMUX_EM0_PWM0 = 6,
    #[doc = "7: PWM generator 1"]
    ADC_EMUX_EM0_PWM1 = 7,
    #[doc = "8: PWM generator 2"]
    ADC_EMUX_EM0_PWM2 = 8,
    #[doc = "9: PWM generator 3"]
    ADC_EMUX_EM0_PWM3 = 9,
    #[doc = "14: Never Trigger"]
    ADC_EMUX_EM0_NEVER = 14,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM0_ALWAYS = 15,
}
impl From<ADC_EMUX_EM0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_EMUX_EM0` reader - SS0 Trigger Select"]
pub type ADC_EMUX_EM0_R = crate::FieldReader<u8, ADC_EMUX_EM0_A>;
impl ADC_EMUX_EM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_EMUX_EM0_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR),
            1 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0),
            2 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1),
            3 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP2),
            4 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL),
            5 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER),
            6 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0),
            7 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1),
            8 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2),
            9 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3),
            14 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_NEVER),
            15 => Some(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_processor(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp2(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_external(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_timer(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_never(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_always(&self) -> bool {
        *self == ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM0` writer - SS0 Trigger Select"]
pub type ADC_EMUX_EM0_W<'a> = crate::FieldWriter<'a, u32, EMUX_SPEC, u8, ADC_EMUX_EM0_A, 4, 0>;
impl<'a> ADC_EMUX_EM0_W<'a> {
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em0_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em0_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em0_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em0_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em0_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em0_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0_A::ADC_EMUX_EM0_ALWAYS)
    }
}
#[doc = "SS1 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM1_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM1_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM1_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM1_COMP1 = 2,
    #[doc = "3: Analog Comparator 2"]
    ADC_EMUX_EM1_COMP2 = 3,
    #[doc = "4: External (GPIO Pins)"]
    ADC_EMUX_EM1_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM1_TIMER = 5,
    #[doc = "6: PWM generator 0"]
    ADC_EMUX_EM1_PWM0 = 6,
    #[doc = "7: PWM generator 1"]
    ADC_EMUX_EM1_PWM1 = 7,
    #[doc = "8: PWM generator 2"]
    ADC_EMUX_EM1_PWM2 = 8,
    #[doc = "9: PWM generator 3"]
    ADC_EMUX_EM1_PWM3 = 9,
    #[doc = "14: Never Trigger"]
    ADC_EMUX_EM1_NEVER = 14,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM1_ALWAYS = 15,
}
impl From<ADC_EMUX_EM1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_EMUX_EM1` reader - SS1 Trigger Select"]
pub type ADC_EMUX_EM1_R = crate::FieldReader<u8, ADC_EMUX_EM1_A>;
impl ADC_EMUX_EM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_EMUX_EM1_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR),
            1 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0),
            2 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1),
            3 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP2),
            4 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL),
            5 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER),
            6 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0),
            7 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1),
            8 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2),
            9 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3),
            14 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_NEVER),
            15 => Some(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_processor(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp2(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_external(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_timer(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_never(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_always(&self) -> bool {
        *self == ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM1` writer - SS1 Trigger Select"]
pub type ADC_EMUX_EM1_W<'a> = crate::FieldWriter<'a, u32, EMUX_SPEC, u8, ADC_EMUX_EM1_A, 4, 4>;
impl<'a> ADC_EMUX_EM1_W<'a> {
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em1_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em1_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em1_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em1_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em1_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em1_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1_A::ADC_EMUX_EM1_ALWAYS)
    }
}
#[doc = "SS2 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM2_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM2_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM2_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM2_COMP1 = 2,
    #[doc = "3: Analog Comparator 2"]
    ADC_EMUX_EM2_COMP2 = 3,
    #[doc = "4: External (GPIO Pins)"]
    ADC_EMUX_EM2_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM2_TIMER = 5,
    #[doc = "6: PWM generator 0"]
    ADC_EMUX_EM2_PWM0 = 6,
    #[doc = "7: PWM generator 1"]
    ADC_EMUX_EM2_PWM1 = 7,
    #[doc = "8: PWM generator 2"]
    ADC_EMUX_EM2_PWM2 = 8,
    #[doc = "9: PWM generator 3"]
    ADC_EMUX_EM2_PWM3 = 9,
    #[doc = "14: Never Trigger"]
    ADC_EMUX_EM2_NEVER = 14,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM2_ALWAYS = 15,
}
impl From<ADC_EMUX_EM2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_EMUX_EM2` reader - SS2 Trigger Select"]
pub type ADC_EMUX_EM2_R = crate::FieldReader<u8, ADC_EMUX_EM2_A>;
impl ADC_EMUX_EM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_EMUX_EM2_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR),
            1 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0),
            2 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1),
            3 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP2),
            4 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL),
            5 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER),
            6 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0),
            7 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1),
            8 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2),
            9 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3),
            14 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_NEVER),
            15 => Some(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_processor(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp2(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_external(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_timer(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_never(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_always(&self) -> bool {
        *self == ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM2` writer - SS2 Trigger Select"]
pub type ADC_EMUX_EM2_W<'a> = crate::FieldWriter<'a, u32, EMUX_SPEC, u8, ADC_EMUX_EM2_A, 4, 8>;
impl<'a> ADC_EMUX_EM2_W<'a> {
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em2_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em2_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em2_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em2_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em2_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em2_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2_A::ADC_EMUX_EM2_ALWAYS)
    }
}
#[doc = "SS3 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_EMUX_EM3_A {
    #[doc = "0: Processor (default)"]
    ADC_EMUX_EM3_PROCESSOR = 0,
    #[doc = "1: Analog Comparator 0"]
    ADC_EMUX_EM3_COMP0 = 1,
    #[doc = "2: Analog Comparator 1"]
    ADC_EMUX_EM3_COMP1 = 2,
    #[doc = "3: Analog Comparator 2"]
    ADC_EMUX_EM3_COMP2 = 3,
    #[doc = "4: External (GPIO Pins)"]
    ADC_EMUX_EM3_EXTERNAL = 4,
    #[doc = "5: Timer"]
    ADC_EMUX_EM3_TIMER = 5,
    #[doc = "6: PWM generator 0"]
    ADC_EMUX_EM3_PWM0 = 6,
    #[doc = "7: PWM generator 1"]
    ADC_EMUX_EM3_PWM1 = 7,
    #[doc = "8: PWM generator 2"]
    ADC_EMUX_EM3_PWM2 = 8,
    #[doc = "9: PWM generator 3"]
    ADC_EMUX_EM3_PWM3 = 9,
    #[doc = "14: Never Trigger"]
    ADC_EMUX_EM3_NEVER = 14,
    #[doc = "15: Always (continuously sample)"]
    ADC_EMUX_EM3_ALWAYS = 15,
}
impl From<ADC_EMUX_EM3_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_EMUX_EM3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_EMUX_EM3` reader - SS3 Trigger Select"]
pub type ADC_EMUX_EM3_R = crate::FieldReader<u8, ADC_EMUX_EM3_A>;
impl ADC_EMUX_EM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_EMUX_EM3_A> {
        match self.bits {
            0 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR),
            1 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0),
            2 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1),
            3 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP2),
            4 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL),
            5 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER),
            6 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0),
            7 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1),
            8 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2),
            9 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3),
            14 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_NEVER),
            15 => Some(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_processor(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp2(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_external(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_timer(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_never(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_always(&self) -> bool {
        *self == ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS
    }
}
#[doc = "Field `ADC_EMUX_EM3` writer - SS3 Trigger Select"]
pub type ADC_EMUX_EM3_W<'a> = crate::FieldWriter<'a, u32, EMUX_SPEC, u8, ADC_EMUX_EM3_A, 4, 12>;
impl<'a> ADC_EMUX_EM3_W<'a> {
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em3_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em3_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em3_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em3_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em3_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em3_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3_A::ADC_EMUX_EM3_ALWAYS)
    }
}
impl R {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&self) -> ADC_EMUX_EM0_R {
        ADC_EMUX_EM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&self) -> ADC_EMUX_EM1_R {
        ADC_EMUX_EM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&self) -> ADC_EMUX_EM2_R {
        ADC_EMUX_EM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&self) -> ADC_EMUX_EM3_R {
        ADC_EMUX_EM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&mut self) -> ADC_EMUX_EM0_W {
        ADC_EMUX_EM0_W::new(self)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&mut self) -> ADC_EMUX_EM1_W {
        ADC_EMUX_EM1_W::new(self)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&mut self) -> ADC_EMUX_EM2_W {
        ADC_EMUX_EM2_W::new(self)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&mut self) -> ADC_EMUX_EM3_W {
        ADC_EMUX_EM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Event Multiplexer Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emux](index.html) module"]
pub struct EMUX_SPEC;
impl crate::RegisterSpec for EMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emux::R](R) reader structure"]
impl crate::Readable for EMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emux::W](W) writer structure"]
impl crate::Writable for EMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMUX to value 0"]
impl crate::Resettable for EMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
