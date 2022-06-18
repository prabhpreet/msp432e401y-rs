#[doc = "Register `_2_GENA` reader"]
pub struct R(crate::R<_2_GENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_GENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_GENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_GENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_2_GENA` writer"]
pub struct W(crate::W<_2_GENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_2_GENA_SPEC>;
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
impl From<crate::W<_2_GENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_2_GENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Action for Counter=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTZERO_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTZERO_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTZERO_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTZERO_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTZERO_ONE = 3,
}
impl From<PWM_2_GENA_ACTZERO_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTZERO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTZERO` reader - Action for Counter=0"]
pub type PWM_2_GENA_ACTZERO_R = crate::FieldReader<u8, PWM_2_GENA_ACTZERO_A>;
impl PWM_2_GENA_ACTZERO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTZERO_A {
        match self.bits {
            0 => PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_NONE,
            1 => PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_INV,
            2 => PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ZERO,
            3 => PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTZERO_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actzero_none(&self) -> bool {
        *self == PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTZERO_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actzero_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTZERO_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actzero_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTZERO_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actzero_one(&self) -> bool {
        *self == PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTZERO` writer - Action for Counter=0"]
pub type PWM_2_GENA_ACTZERO_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTZERO_A, 2, 0>;
impl<'a> PWM_2_GENA_ACTZERO_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTZERO_A::PWM_2_GENA_ACTZERO_ONE)
    }
}
#[doc = "Action for Counter=LOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTLOAD_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTLOAD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTLOAD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTLOAD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTLOAD_ONE = 3,
}
impl From<PWM_2_GENA_ACTLOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTLOAD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTLOAD` reader - Action for Counter=LOAD"]
pub type PWM_2_GENA_ACTLOAD_R = crate::FieldReader<u8, PWM_2_GENA_ACTLOAD_A>;
impl PWM_2_GENA_ACTLOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTLOAD_A {
        match self.bits {
            0 => PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_NONE,
            1 => PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_INV,
            2 => PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ZERO,
            3 => PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTLOAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actload_none(&self) -> bool {
        *self == PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTLOAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actload_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTLOAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actload_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTLOAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actload_one(&self) -> bool {
        *self == PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTLOAD` writer - Action for Counter=LOAD"]
pub type PWM_2_GENA_ACTLOAD_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTLOAD_A, 2, 2>;
impl<'a> PWM_2_GENA_ACTLOAD_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actload_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actload_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actload_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actload_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTLOAD_A::PWM_2_GENA_ACTLOAD_ONE)
    }
}
#[doc = "Action for Comparator A Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTCMPAU_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTCMPAU_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTCMPAU_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTCMPAU_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTCMPAU_ONE = 3,
}
impl From<PWM_2_GENA_ACTCMPAU_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTCMPAU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPAU` reader - Action for Comparator A Up"]
pub type PWM_2_GENA_ACTCMPAU_R = crate::FieldReader<u8, PWM_2_GENA_ACTCMPAU_A>;
impl PWM_2_GENA_ACTCMPAU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTCMPAU_A {
        match self.bits {
            0 => PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_NONE,
            1 => PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_INV,
            2 => PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ZERO,
            3 => PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpau_none(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAU_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpau_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpau_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpau_one(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPAU` writer - Action for Comparator A Up"]
pub type PWM_2_GENA_ACTCMPAU_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTCMPAU_A, 2, 4>;
impl<'a> PWM_2_GENA_ACTCMPAU_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAU_A::PWM_2_GENA_ACTCMPAU_ONE)
    }
}
#[doc = "Action for Comparator A Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTCMPAD_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTCMPAD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTCMPAD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTCMPAD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTCMPAD_ONE = 3,
}
impl From<PWM_2_GENA_ACTCMPAD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTCMPAD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPAD` reader - Action for Comparator A Down"]
pub type PWM_2_GENA_ACTCMPAD_R = crate::FieldReader<u8, PWM_2_GENA_ACTCMPAD_A>;
impl PWM_2_GENA_ACTCMPAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTCMPAD_A {
        match self.bits {
            0 => PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_NONE,
            1 => PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_INV,
            2 => PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ZERO,
            3 => PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpad_none(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpad_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpad_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpad_one(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPAD` writer - Action for Comparator A Down"]
pub type PWM_2_GENA_ACTCMPAD_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTCMPAD_A, 2, 6>;
impl<'a> PWM_2_GENA_ACTCMPAD_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPAD_A::PWM_2_GENA_ACTCMPAD_ONE)
    }
}
#[doc = "Action for Comparator B Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTCMPBU_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTCMPBU_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTCMPBU_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTCMPBU_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTCMPBU_ONE = 3,
}
impl From<PWM_2_GENA_ACTCMPBU_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTCMPBU_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPBU` reader - Action for Comparator B Up"]
pub type PWM_2_GENA_ACTCMPBU_R = crate::FieldReader<u8, PWM_2_GENA_ACTCMPBU_A>;
impl PWM_2_GENA_ACTCMPBU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTCMPBU_A {
        match self.bits {
            0 => PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_NONE,
            1 => PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_INV,
            2 => PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ZERO,
            3 => PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbu_none(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBU_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbu_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbu_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbu_one(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPBU` writer - Action for Comparator B Up"]
pub type PWM_2_GENA_ACTCMPBU_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTCMPBU_A, 2, 8>;
impl<'a> PWM_2_GENA_ACTCMPBU_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBU_A::PWM_2_GENA_ACTCMPBU_ONE)
    }
}
#[doc = "Action for Comparator B Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_GENA_ACTCMPBD_A {
    #[doc = "0: Do nothing"]
    PWM_2_GENA_ACTCMPBD_NONE = 0,
    #[doc = "1: Invert pwmA"]
    PWM_2_GENA_ACTCMPBD_INV = 1,
    #[doc = "2: Drive pwmA Low"]
    PWM_2_GENA_ACTCMPBD_ZERO = 2,
    #[doc = "3: Drive pwmA High"]
    PWM_2_GENA_ACTCMPBD_ONE = 3,
}
impl From<PWM_2_GENA_ACTCMPBD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_GENA_ACTCMPBD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPBD` reader - Action for Comparator B Down"]
pub type PWM_2_GENA_ACTCMPBD_R = crate::FieldReader<u8, PWM_2_GENA_ACTCMPBD_A>;
impl PWM_2_GENA_ACTCMPBD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM_2_GENA_ACTCMPBD_A {
        match self.bits {
            0 => PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_NONE,
            1 => PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_INV,
            2 => PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ZERO,
            3 => PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbd_none(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBD_INV`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbd_inv(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbd_zero(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_2_GENA_ACTCMPBD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_2_gena_actcmpbd_one(&self) -> bool {
        *self == PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ONE
    }
}
#[doc = "Field `PWM_2_GENA_ACTCMPBD` writer - Action for Comparator B Down"]
pub type PWM_2_GENA_ACTCMPBD_W<'a> =
    crate::FieldWriterSafe<'a, u32, _2_GENA_SPEC, u8, PWM_2_GENA_ACTCMPBD_A, 2, 10>;
impl<'a> PWM_2_GENA_ACTCMPBD_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd_none(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd_inv(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd_zero(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd_one(self) -> &'a mut W {
        self.variant(PWM_2_GENA_ACTCMPBD_A::PWM_2_GENA_ACTCMPBD_ONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero(&self) -> PWM_2_GENA_ACTZERO_R {
        PWM_2_GENA_ACTZERO_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_2_gena_actload(&self) -> PWM_2_GENA_ACTLOAD_R {
        PWM_2_GENA_ACTLOAD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau(&self) -> PWM_2_GENA_ACTCMPAU_R {
        PWM_2_GENA_ACTCMPAU_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad(&self) -> PWM_2_GENA_ACTCMPAD_R {
        PWM_2_GENA_ACTCMPAD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu(&self) -> PWM_2_GENA_ACTCMPBU_R {
        PWM_2_GENA_ACTCMPBU_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd(&self) -> PWM_2_GENA_ACTCMPBD_R {
        PWM_2_GENA_ACTCMPBD_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_2_gena_actzero(&mut self) -> PWM_2_GENA_ACTZERO_W {
        PWM_2_GENA_ACTZERO_W::new(self)
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_2_gena_actload(&mut self) -> PWM_2_GENA_ACTLOAD_W {
        PWM_2_GENA_ACTLOAD_W::new(self)
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpau(&mut self) -> PWM_2_GENA_ACTCMPAU_W {
        PWM_2_GENA_ACTCMPAU_W::new(self)
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpad(&mut self) -> PWM_2_GENA_ACTCMPAD_W {
        PWM_2_GENA_ACTCMPAD_W::new(self)
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbu(&mut self) -> PWM_2_GENA_ACTCMPBU_W {
        PWM_2_GENA_ACTCMPBU_W::new(self)
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_2_gena_actcmpbd(&mut self) -> PWM_2_GENA_ACTCMPBD_W {
        PWM_2_GENA_ACTCMPBD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM2 Generator A Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_gena](index.html) module"]
pub struct _2_GENA_SPEC;
impl crate::RegisterSpec for _2_GENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_gena::R](R) reader structure"]
impl crate::Readable for _2_GENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_2_gena::W](W) writer structure"]
impl crate::Writable for _2_GENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _2_GENA to value 0"]
impl crate::Resettable for _2_GENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
