#[doc = "Register `SYNC` reader"]
pub struct R(crate::R<SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC` writer"]
pub struct W(crate::W<SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_SPEC>;
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
impl From<crate::W<SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Synchronize GPTM Timer 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT0_A {
    #[doc = "0: GPTM0 is not affected"]
    TIMER_SYNC_SYNCT0_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    TIMER_SYNC_SYNCT0_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT0_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT0` reader - Synchronize GPTM Timer 0"]
pub type TIMER_SYNC_SYNCT0_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT0_A>;
impl TIMER_SYNC_SYNCT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT0_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE,
            1 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA,
            2 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB,
            3 => TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT0_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct0_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT0` writer - Synchronize GPTM Timer 0"]
pub type TIMER_SYNC_SYNCT0_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT0_A, 2, 0>;
impl<'a> TIMER_SYNC_SYNCT0_W<'a> {
    #[doc = "GPTM0 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct0_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM0 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct0_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT0_A::TIMER_SYNC_SYNCT0_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT1_A {
    #[doc = "0: GPTM1 is not affected"]
    TIMER_SYNC_SYNCT1_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    TIMER_SYNC_SYNCT1_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT1_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT1` reader - Synchronize GPTM Timer 1"]
pub type TIMER_SYNC_SYNCT1_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT1_A>;
impl TIMER_SYNC_SYNCT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT1_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE,
            1 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA,
            2 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB,
            3 => TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT1_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct1_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT1` writer - Synchronize GPTM Timer 1"]
pub type TIMER_SYNC_SYNCT1_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT1_A, 2, 2>;
impl<'a> TIMER_SYNC_SYNCT1_W<'a> {
    #[doc = "GPTM1 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct1_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM1 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct1_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT1_A::TIMER_SYNC_SYNCT1_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT2_A {
    #[doc = "0: GPTM2 is not affected"]
    TIMER_SYNC_SYNCT2_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    TIMER_SYNC_SYNCT2_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT2_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT2` reader - Synchronize GPTM Timer 2"]
pub type TIMER_SYNC_SYNCT2_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT2_A>;
impl TIMER_SYNC_SYNCT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT2_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE,
            1 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA,
            2 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB,
            3 => TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT2_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct2_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT2` writer - Synchronize GPTM Timer 2"]
pub type TIMER_SYNC_SYNCT2_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT2_A, 2, 4>;
impl<'a> TIMER_SYNC_SYNCT2_W<'a> {
    #[doc = "GPTM2 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct2_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM2 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct2_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT2_A::TIMER_SYNC_SYNCT2_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT3_A {
    #[doc = "0: GPTM3 is not affected"]
    TIMER_SYNC_SYNCT3_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    TIMER_SYNC_SYNCT3_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT3_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT3` reader - Synchronize GPTM Timer 3"]
pub type TIMER_SYNC_SYNCT3_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT3_A>;
impl TIMER_SYNC_SYNCT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT3_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE,
            1 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA,
            2 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB,
            3 => TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT3_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct3_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT3` writer - Synchronize GPTM Timer 3"]
pub type TIMER_SYNC_SYNCT3_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT3_A, 2, 6>;
impl<'a> TIMER_SYNC_SYNCT3_W<'a> {
    #[doc = "GPTM3 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct3_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM3 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct3_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT3_A::TIMER_SYNC_SYNCT3_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT4_A {
    #[doc = "0: GPTM4 is not affected"]
    TIMER_SYNC_SYNCT4_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    TIMER_SYNC_SYNCT4_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT4_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT4` reader - Synchronize GPTM Timer 4"]
pub type TIMER_SYNC_SYNCT4_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT4_A>;
impl TIMER_SYNC_SYNCT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT4_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE,
            1 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA,
            2 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB,
            3 => TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT4_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct4_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT4` writer - Synchronize GPTM Timer 4"]
pub type TIMER_SYNC_SYNCT4_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT4_A, 2, 8>;
impl<'a> TIMER_SYNC_SYNCT4_W<'a> {
    #[doc = "GPTM4 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct4_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM4 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct4_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT4_A::TIMER_SYNC_SYNCT4_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT5_A {
    #[doc = "0: GPTM5 is not affected"]
    TIMER_SYNC_SYNCT5_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    TIMER_SYNC_SYNCT5_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT5_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT5` reader - Synchronize GPTM Timer 5"]
pub type TIMER_SYNC_SYNCT5_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT5_A>;
impl TIMER_SYNC_SYNCT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT5_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE,
            1 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA,
            2 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB,
            3 => TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT5_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct5_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT5` writer - Synchronize GPTM Timer 5"]
pub type TIMER_SYNC_SYNCT5_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT5_A, 2, 10>;
impl<'a> TIMER_SYNC_SYNCT5_W<'a> {
    #[doc = "GPTM5 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct5_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM5 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct5_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT5_A::TIMER_SYNC_SYNCT5_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT6_A {
    #[doc = "0: GPTM6 is not affected"]
    TIMER_SYNC_SYNCT6_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM6 is triggered"]
    TIMER_SYNC_SYNCT6_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT6_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT6` reader - Synchronize GPTM Timer 6"]
pub type TIMER_SYNC_SYNCT6_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT6_A>;
impl TIMER_SYNC_SYNCT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT6_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_NONE,
            1 => TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TA,
            2 => TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TB,
            3 => TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT6_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct6_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT6` writer - Synchronize GPTM Timer 6"]
pub type TIMER_SYNC_SYNCT6_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT6_A, 2, 12>;
impl<'a> TIMER_SYNC_SYNCT6_W<'a> {
    #[doc = "GPTM6 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct6_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM6 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct6_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT6_A::TIMER_SYNC_SYNCT6_TATB)
    }
}
#[doc = "Synchronize GPTM Timer 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_SYNC_SYNCT7_A {
    #[doc = "0: GPT7 is not affected"]
    TIMER_SYNC_SYNCT7_NONE = 0,
    #[doc = "1: A timeout event for Timer A of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TA = 1,
    #[doc = "2: A timeout event for Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TB = 2,
    #[doc = "3: A timeout event for both Timer A and Timer B of GPTM7 is triggered"]
    TIMER_SYNC_SYNCT7_TATB = 3,
}
impl From<TIMER_SYNC_SYNCT7_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_SYNC_SYNCT7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT7` reader - Synchronize GPTM Timer 7"]
pub type TIMER_SYNC_SYNCT7_R = crate::FieldReader<u8, TIMER_SYNC_SYNCT7_A>;
impl TIMER_SYNC_SYNCT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_SYNC_SYNCT7_A {
        match self.bits {
            0 => TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_NONE,
            1 => TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TA,
            2 => TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TB,
            3 => TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TATB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_NONE`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_none(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TA`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_ta(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TA
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_tb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TB
    }
    #[doc = "Checks if the value of the field is `TIMER_SYNC_SYNCT7_TATB`"]
    #[inline(always)]
    pub fn is_timer_sync_synct7_tatb(&self) -> bool {
        *self == TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TATB
    }
}
#[doc = "Field `TIMER_SYNC_SYNCT7` writer - Synchronize GPTM Timer 7"]
pub type TIMER_SYNC_SYNCT7_W<'a> =
    crate::FieldWriterSafe<'a, u32, SYNC_SPEC, u8, TIMER_SYNC_SYNCT7_A, 2, 14>;
impl<'a> TIMER_SYNC_SYNCT7_W<'a> {
    #[doc = "GPT7 is not affected"]
    #[inline(always)]
    pub fn timer_sync_synct7_none(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_NONE)
    }
    #[doc = "A timeout event for Timer A of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_ta(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TA)
    }
    #[doc = "A timeout event for Timer B of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_tb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TB)
    }
    #[doc = "A timeout event for both Timer A and Timer B of GPTM7 is triggered"]
    #[inline(always)]
    pub fn timer_sync_synct7_tatb(self) -> &'a mut W {
        self.variant(TIMER_SYNC_SYNCT7_A::TIMER_SYNC_SYNCT7_TATB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn timer_sync_synct0(&self) -> TIMER_SYNC_SYNCT0_R {
        TIMER_SYNC_SYNCT0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn timer_sync_synct1(&self) -> TIMER_SYNC_SYNCT1_R {
        TIMER_SYNC_SYNCT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn timer_sync_synct2(&self) -> TIMER_SYNC_SYNCT2_R {
        TIMER_SYNC_SYNCT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn timer_sync_synct3(&self) -> TIMER_SYNC_SYNCT3_R {
        TIMER_SYNC_SYNCT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn timer_sync_synct4(&self) -> TIMER_SYNC_SYNCT4_R {
        TIMER_SYNC_SYNCT4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn timer_sync_synct5(&self) -> TIMER_SYNC_SYNCT5_R {
        TIMER_SYNC_SYNCT5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn timer_sync_synct6(&self) -> TIMER_SYNC_SYNCT6_R {
        TIMER_SYNC_SYNCT6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn timer_sync_synct7(&self) -> TIMER_SYNC_SYNCT7_R {
        TIMER_SYNC_SYNCT7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronize GPTM Timer 0"]
    #[inline(always)]
    pub fn timer_sync_synct0(&mut self) -> TIMER_SYNC_SYNCT0_W {
        TIMER_SYNC_SYNCT0_W::new(self)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM Timer 1"]
    #[inline(always)]
    pub fn timer_sync_synct1(&mut self) -> TIMER_SYNC_SYNCT1_W {
        TIMER_SYNC_SYNCT1_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM Timer 2"]
    #[inline(always)]
    pub fn timer_sync_synct2(&mut self) -> TIMER_SYNC_SYNCT2_W {
        TIMER_SYNC_SYNCT2_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM Timer 3"]
    #[inline(always)]
    pub fn timer_sync_synct3(&mut self) -> TIMER_SYNC_SYNCT3_W {
        TIMER_SYNC_SYNCT3_W::new(self)
    }
    #[doc = "Bits 8:9 - Synchronize GPTM Timer 4"]
    #[inline(always)]
    pub fn timer_sync_synct4(&mut self) -> TIMER_SYNC_SYNCT4_W {
        TIMER_SYNC_SYNCT4_W::new(self)
    }
    #[doc = "Bits 10:11 - Synchronize GPTM Timer 5"]
    #[inline(always)]
    pub fn timer_sync_synct5(&mut self) -> TIMER_SYNC_SYNCT5_W {
        TIMER_SYNC_SYNCT5_W::new(self)
    }
    #[doc = "Bits 12:13 - Synchronize GPTM Timer 6"]
    #[inline(always)]
    pub fn timer_sync_synct6(&mut self) -> TIMER_SYNC_SYNCT6_W {
        TIMER_SYNC_SYNCT6_W::new(self)
    }
    #[doc = "Bits 14:15 - Synchronize GPTM Timer 7"]
    #[inline(always)]
    pub fn timer_sync_synct7(&mut self) -> TIMER_SYNC_SYNCT7_W {
        TIMER_SYNC_SYNCT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Synchronize\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](index.html) module"]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync::R](R) reader structure"]
impl crate::Readable for SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync::W](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
