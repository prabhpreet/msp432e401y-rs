#[doc = "Register `ENUPD` reader"]
pub struct R(crate::R<ENUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENUPD` writer"]
pub struct W(crate::W<ENUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENUPD_SPEC>;
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
impl From<crate::W<ENUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MnPWM0 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD0_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD0_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD0_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD0_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD0` reader - MnPWM0 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD0_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD0_A>;
impl PWM_ENUPD_ENUPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD0_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM),
            2 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD0` writer - MnPWM0 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD0_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD0_A, 2, 0>;
impl<'a> PWM_ENUPD_ENUPD0_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0_A::PWM_ENUPD_ENUPD0_GSYNC)
    }
}
#[doc = "MnPWM1 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD1_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD1_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD1_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD1_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD1_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD1` reader - MnPWM1 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD1_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD1_A>;
impl PWM_ENUPD_ENUPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD1_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM),
            2 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD1` writer - MnPWM1 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD1_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD1_A, 2, 2>;
impl<'a> PWM_ENUPD_ENUPD1_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1_A::PWM_ENUPD_ENUPD1_GSYNC)
    }
}
#[doc = "MnPWM2 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD2_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD2_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD2_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD2_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD2_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD2` reader - MnPWM2 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD2_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD2_A>;
impl PWM_ENUPD_ENUPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD2_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM),
            2 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD2` writer - MnPWM2 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD2_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD2_A, 2, 4>;
impl<'a> PWM_ENUPD_ENUPD2_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2_A::PWM_ENUPD_ENUPD2_GSYNC)
    }
}
#[doc = "MnPWM3 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD3_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD3_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD3_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD3_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD3_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD3` reader - MnPWM3 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD3_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD3_A>;
impl PWM_ENUPD_ENUPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD3_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM),
            2 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD3` writer - MnPWM3 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD3_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD3_A, 2, 6>;
impl<'a> PWM_ENUPD_ENUPD3_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3_A::PWM_ENUPD_ENUPD3_GSYNC)
    }
}
#[doc = "MnPWM4 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD4_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD4_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD4_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD4_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD4_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD4` reader - MnPWM4 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD4_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD4_A>;
impl PWM_ENUPD_ENUPD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD4_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM),
            2 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD4` writer - MnPWM4 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD4_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD4_A, 2, 8>;
impl<'a> PWM_ENUPD_ENUPD4_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4_A::PWM_ENUPD_ENUPD4_GSYNC)
    }
}
#[doc = "MnPWM5 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD5_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD5_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD5_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD5_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD5_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD5` reader - MnPWM5 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD5_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD5_A>;
impl PWM_ENUPD_ENUPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD5_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM),
            2 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD5` writer - MnPWM5 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD5_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD5_A, 2, 10>;
impl<'a> PWM_ENUPD_ENUPD5_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5_A::PWM_ENUPD_ENUPD5_GSYNC)
    }
}
#[doc = "MnPWM6 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD6_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD6_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD6_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD6_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD6_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD6` reader - MnPWM6 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD6_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD6_A>;
impl PWM_ENUPD_ENUPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD6_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM),
            2 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD6` writer - MnPWM6 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD6_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD6_A, 2, 12>;
impl<'a> PWM_ENUPD_ENUPD6_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6_A::PWM_ENUPD_ENUPD6_GSYNC)
    }
}
#[doc = "MnPWM7 Enable Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_ENUPD_ENUPD7_A {
    #[doc = "0: Immediate"]
    PWM_ENUPD_ENUPD7_IMM = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_ENUPD_ENUPD7_LSYNC = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_ENUPD_ENUPD7_GSYNC = 3,
}
impl From<PWM_ENUPD_ENUPD7_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_ENUPD_ENUPD7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD7` reader - MnPWM7 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD7_R = crate::FieldReader<u8, PWM_ENUPD_ENUPD7_A>;
impl PWM_ENUPD_ENUPD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_ENUPD_ENUPD7_A> {
        match self.bits {
            0 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM),
            2 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC),
            3 => Some(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC
    }
}
#[doc = "Field `PWM_ENUPD_ENUPD7` writer - MnPWM7 Enable Update Mode"]
pub type PWM_ENUPD_ENUPD7_W<'a> =
    crate::FieldWriter<'a, u32, ENUPD_SPEC, u8, PWM_ENUPD_ENUPD7_A, 2, 14>;
impl<'a> PWM_ENUPD_ENUPD7_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7_A::PWM_ENUPD_ENUPD7_GSYNC)
    }
}
impl R {
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0(&self) -> PWM_ENUPD_ENUPD0_R {
        PWM_ENUPD_ENUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1(&self) -> PWM_ENUPD_ENUPD1_R {
        PWM_ENUPD_ENUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2(&self) -> PWM_ENUPD_ENUPD2_R {
        PWM_ENUPD_ENUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3(&self) -> PWM_ENUPD_ENUPD3_R {
        PWM_ENUPD_ENUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4(&self) -> PWM_ENUPD_ENUPD4_R {
        PWM_ENUPD_ENUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5(&self) -> PWM_ENUPD_ENUPD5_R {
        PWM_ENUPD_ENUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6(&self) -> PWM_ENUPD_ENUPD6_R {
        PWM_ENUPD_ENUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7(&self) -> PWM_ENUPD_ENUPD7_R {
        PWM_ENUPD_ENUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0(&mut self) -> PWM_ENUPD_ENUPD0_W {
        PWM_ENUPD_ENUPD0_W::new(self)
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1(&mut self) -> PWM_ENUPD_ENUPD1_W {
        PWM_ENUPD_ENUPD1_W::new(self)
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2(&mut self) -> PWM_ENUPD_ENUPD2_W {
        PWM_ENUPD_ENUPD2_W::new(self)
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3(&mut self) -> PWM_ENUPD_ENUPD3_W {
        PWM_ENUPD_ENUPD3_W::new(self)
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4(&mut self) -> PWM_ENUPD_ENUPD4_W {
        PWM_ENUPD_ENUPD4_W::new(self)
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5(&mut self) -> PWM_ENUPD_ENUPD5_W {
        PWM_ENUPD_ENUPD5_W::new(self)
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6(&mut self) -> PWM_ENUPD_ENUPD6_W {
        PWM_ENUPD_ENUPD6_W::new(self)
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7(&mut self) -> PWM_ENUPD_ENUPD7_W {
        PWM_ENUPD_ENUPD7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Enable Update\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enupd](index.html) module"]
pub struct ENUPD_SPEC;
impl crate::RegisterSpec for ENUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enupd::R](R) reader structure"]
impl crate::Readable for ENUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enupd::W](W) writer structure"]
impl crate::Writable for ENUPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENUPD to value 0"]
impl crate::Resettable for ENUPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
