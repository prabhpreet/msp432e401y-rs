#[doc = "Register `MEMTIM0` reader"]
pub struct R(crate::R<MEMTIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMTIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMTIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMTIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMTIM0` writer"]
pub struct W(crate::W<MEMTIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMTIM0_SPEC>;
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
impl From<crate::W<MEMTIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMTIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_FWS` reader - Flash Wait State"]
pub type SYSCTL_MEMTIM0_FWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_MEMTIM0_FWS` writer - Flash Wait State"]
pub type SYSCTL_MEMTIM0_FWS_W<'a> = crate::FieldWriter<'a, u32, MEMTIM0_SPEC, u8, u8, 4, 0>;
#[doc = "Field `RESERVED0` reader - Value of this reserved bit must be read as 1"]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - Value of this reserved bit must be read as 1"]
pub type RESERVED0_W<'a> = crate::BitWriter<'a, u32, MEMTIM0_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_MEMTIM0_FBCE` reader - Flash Bank Clock Edge"]
pub type SYSCTL_MEMTIM0_FBCE_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MEMTIM0_FBCE` writer - Flash Bank Clock Edge"]
pub type SYSCTL_MEMTIM0_FBCE_W<'a> = crate::BitWriter<'a, u32, MEMTIM0_SPEC, bool, 5>;
#[doc = "Flash Bank Clock High Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_MEMTIM0_FBCHT_A {
    #[doc = "0: 1/2 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_0_5 = 0,
    #[doc = "1: 1 system clock period"]
    SYSCTL_MEMTIM0_FBCHT_1 = 1,
    #[doc = "2: 1.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_1_5 = 2,
    #[doc = "3: 2 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2 = 3,
    #[doc = "4: 2.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_2_5 = 4,
    #[doc = "5: 3 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3 = 5,
    #[doc = "6: 3.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_3_5 = 6,
    #[doc = "7: 4 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4 = 7,
    #[doc = "8: 4.5 system clock periods"]
    SYSCTL_MEMTIM0_FBCHT_4_5 = 8,
}
impl From<SYSCTL_MEMTIM0_FBCHT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_MEMTIM0_FBCHT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_FBCHT` reader - Flash Bank Clock High Time"]
pub type SYSCTL_MEMTIM0_FBCHT_R = crate::FieldReader<u8, SYSCTL_MEMTIM0_FBCHT_A>;
impl SYSCTL_MEMTIM0_FBCHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_MEMTIM0_FBCHT_A> {
        match self.bits {
            0 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_0_5),
            1 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1),
            2 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1_5),
            3 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2),
            4 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2_5),
            5 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3),
            6 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3_5),
            7 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4),
            8 => Some(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_0_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_0_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_0_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_1`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_1(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_1_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_1_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_2`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_2(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_2_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_2_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_3`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_3(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_3_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_3_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_4`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_4(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_FBCHT_4_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_fbcht_4_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4_5
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_FBCHT` writer - Flash Bank Clock High Time"]
pub type SYSCTL_MEMTIM0_FBCHT_W<'a> =
    crate::FieldWriter<'a, u32, MEMTIM0_SPEC, u8, SYSCTL_MEMTIM0_FBCHT_A, 4, 6>;
impl<'a> SYSCTL_MEMTIM0_FBCHT_W<'a> {
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_0_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_1(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_1_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_2(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_2_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_3(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_3_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_4(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht_4_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_FBCHT_A::SYSCTL_MEMTIM0_FBCHT_4_5)
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_EWS` reader - EEPROM Wait States"]
pub type SYSCTL_MEMTIM0_EWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_MEMTIM0_EWS` writer - EEPROM Wait States"]
pub type SYSCTL_MEMTIM0_EWS_W<'a> = crate::FieldWriter<'a, u32, MEMTIM0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `RESERVED1` reader - Value of this reserved bit must be read as 1"]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - Value of this reserved bit must be read as 1"]
pub type RESERVED1_W<'a> = crate::BitWriter<'a, u32, MEMTIM0_SPEC, bool, 20>;
#[doc = "Field `SYSCTL_MEMTIM0_EBCE` reader - EEPROM Bank Clock Edge"]
pub type SYSCTL_MEMTIM0_EBCE_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MEMTIM0_EBCE` writer - EEPROM Bank Clock Edge"]
pub type SYSCTL_MEMTIM0_EBCE_W<'a> = crate::BitWriter<'a, u32, MEMTIM0_SPEC, bool, 21>;
#[doc = "EEPROM Clock High Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_MEMTIM0_EBCHT_A {
    #[doc = "0: 1/2 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_0_5 = 0,
    #[doc = "1: 1 system clock period"]
    SYSCTL_MEMTIM0_EBCHT_1 = 1,
    #[doc = "2: 1.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_1_5 = 2,
    #[doc = "3: 2 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2 = 3,
    #[doc = "4: 2.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_2_5 = 4,
    #[doc = "5: 3 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3 = 5,
    #[doc = "6: 3.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_3_5 = 6,
    #[doc = "7: 4 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4 = 7,
    #[doc = "8: 4.5 system clock periods"]
    SYSCTL_MEMTIM0_EBCHT_4_5 = 8,
}
impl From<SYSCTL_MEMTIM0_EBCHT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_MEMTIM0_EBCHT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_EBCHT` reader - EEPROM Clock High Time"]
pub type SYSCTL_MEMTIM0_EBCHT_R = crate::FieldReader<u8, SYSCTL_MEMTIM0_EBCHT_A>;
impl SYSCTL_MEMTIM0_EBCHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_MEMTIM0_EBCHT_A> {
        match self.bits {
            0 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_0_5),
            1 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1),
            2 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1_5),
            3 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2),
            4 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2_5),
            5 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3),
            6 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3_5),
            7 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4),
            8 => Some(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_0_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_0_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_0_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_1`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_1(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_1_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_1_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_2`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_2(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_2_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_2_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_3`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_3(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_3_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_3_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3_5
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_4`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_4(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4
    }
    #[doc = "Checks if the value of the field is `SYSCTL_MEMTIM0_EBCHT_4_5`"]
    #[inline(always)]
    pub fn is_sysctl_memtim0_ebcht_4_5(&self) -> bool {
        *self == SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4_5
    }
}
#[doc = "Field `SYSCTL_MEMTIM0_EBCHT` writer - EEPROM Clock High Time"]
pub type SYSCTL_MEMTIM0_EBCHT_W<'a> =
    crate::FieldWriter<'a, u32, MEMTIM0_SPEC, u8, SYSCTL_MEMTIM0_EBCHT_A, 4, 22>;
impl<'a> SYSCTL_MEMTIM0_EBCHT_W<'a> {
    #[doc = "1/2 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_0_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_1(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_1_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_2(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_2_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_3(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_3_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_4(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht_4_5(self) -> &'a mut W {
        self.variant(SYSCTL_MEMTIM0_EBCHT_A::SYSCTL_MEMTIM0_EBCHT_4_5)
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn sysctl_memtim0_fws(&self) -> SYSCTL_MEMTIM0_FWS_R {
        SYSCTL_MEMTIM0_FWS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbce(&self) -> SYSCTL_MEMTIM0_FBCE_R {
        SYSCTL_MEMTIM0_FBCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht(&self) -> SYSCTL_MEMTIM0_FBCHT_R {
        SYSCTL_MEMTIM0_FBCHT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn sysctl_memtim0_ews(&self) -> SYSCTL_MEMTIM0_EWS_R {
        SYSCTL_MEMTIM0_EWS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebce(&self) -> SYSCTL_MEMTIM0_EBCE_R {
        SYSCTL_MEMTIM0_EBCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht(&self) -> SYSCTL_MEMTIM0_EBCHT_R {
        SYSCTL_MEMTIM0_EBCHT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline(always)]
    pub fn sysctl_memtim0_fws(&mut self) -> SYSCTL_MEMTIM0_FWS_W {
        SYSCTL_MEMTIM0_FWS_W::new(self)
    }
    #[doc = "Bit 4 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbce(&mut self) -> SYSCTL_MEMTIM0_FBCE_W {
        SYSCTL_MEMTIM0_FBCE_W::new(self)
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_fbcht(&mut self) -> SYSCTL_MEMTIM0_FBCHT_W {
        SYSCTL_MEMTIM0_FBCHT_W::new(self)
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline(always)]
    pub fn sysctl_memtim0_ews(&mut self) -> SYSCTL_MEMTIM0_EWS_W {
        SYSCTL_MEMTIM0_EWS_W::new(self)
    }
    #[doc = "Bit 20 - Value of this reserved bit must be read as 1"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebce(&mut self) -> SYSCTL_MEMTIM0_EBCE_W {
        SYSCTL_MEMTIM0_EBCE_W::new(self)
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline(always)]
    pub fn sysctl_memtim0_ebcht(&mut self) -> SYSCTL_MEMTIM0_EBCHT_W {
        SYSCTL_MEMTIM0_EBCHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memtim0](index.html) module"]
pub struct MEMTIM0_SPEC;
impl crate::RegisterSpec for MEMTIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memtim0::R](R) reader structure"]
impl crate::Readable for MEMTIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memtim0::W](W) writer structure"]
impl crate::Writable for MEMTIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMTIM0 to value 0"]
impl crate::Resettable for MEMTIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
