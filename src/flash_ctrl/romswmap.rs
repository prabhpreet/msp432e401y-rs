#[doc = "Register `ROMSWMAP` reader"]
pub struct R(crate::R<ROMSWMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROMSWMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROMSWMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROMSWMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROMSWMAP` writer"]
pub struct W(crate::W<ROMSWMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROMSWMAP_SPEC>;
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
impl From<crate::W<ROMSWMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROMSWMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ROM SW Region 0 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW0EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW0EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW0EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW0EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW0EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW0EN` reader - ROM SW Region 0 Availability"]
pub type FLASH_ROMSWMAP_SW0EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW0EN_A>;
impl FLASH_ROMSWMAP_SW0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW0EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW0EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw0en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW0EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw0en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW0EN` writer - ROM SW Region 0 Availability"]
pub type FLASH_ROMSWMAP_SW0EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW0EN_A, 2, 0>;
impl<'a> FLASH_ROMSWMAP_SW0EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW0EN_A::FLASH_ROMSWMAP_SW0EN_CORE)
    }
}
#[doc = "ROM SW Region 1 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW1EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW1EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW1EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW1EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW1EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW1EN` reader - ROM SW Region 1 Availability"]
pub type FLASH_ROMSWMAP_SW1EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW1EN_A>;
impl FLASH_ROMSWMAP_SW1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW1EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW1EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw1en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW1EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw1en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW1EN` writer - ROM SW Region 1 Availability"]
pub type FLASH_ROMSWMAP_SW1EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW1EN_A, 2, 2>;
impl<'a> FLASH_ROMSWMAP_SW1EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW1EN_A::FLASH_ROMSWMAP_SW1EN_CORE)
    }
}
#[doc = "ROM SW Region 2 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW2EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW2EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW2EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW2EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW2EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW2EN` reader - ROM SW Region 2 Availability"]
pub type FLASH_ROMSWMAP_SW2EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW2EN_A>;
impl FLASH_ROMSWMAP_SW2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW2EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW2EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw2en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW2EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw2en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW2EN` writer - ROM SW Region 2 Availability"]
pub type FLASH_ROMSWMAP_SW2EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW2EN_A, 2, 4>;
impl<'a> FLASH_ROMSWMAP_SW2EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW2EN_A::FLASH_ROMSWMAP_SW2EN_CORE)
    }
}
#[doc = "ROM SW Region 3 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW3EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW3EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW3EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW3EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW3EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW3EN` reader - ROM SW Region 3 Availability"]
pub type FLASH_ROMSWMAP_SW3EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW3EN_A>;
impl FLASH_ROMSWMAP_SW3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW3EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW3EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw3en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW3EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw3en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW3EN` writer - ROM SW Region 3 Availability"]
pub type FLASH_ROMSWMAP_SW3EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW3EN_A, 2, 6>;
impl<'a> FLASH_ROMSWMAP_SW3EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW3EN_A::FLASH_ROMSWMAP_SW3EN_CORE)
    }
}
#[doc = "ROM SW Region 4 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW4EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW4EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW4EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW4EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW4EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW4EN` reader - ROM SW Region 4 Availability"]
pub type FLASH_ROMSWMAP_SW4EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW4EN_A>;
impl FLASH_ROMSWMAP_SW4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW4EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW4EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw4en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW4EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw4en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW4EN` writer - ROM SW Region 4 Availability"]
pub type FLASH_ROMSWMAP_SW4EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW4EN_A, 2, 8>;
impl<'a> FLASH_ROMSWMAP_SW4EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW4EN_A::FLASH_ROMSWMAP_SW4EN_CORE)
    }
}
#[doc = "ROM SW Region 5 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW5EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW5EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW5EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW5EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW5EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW5EN` reader - ROM SW Region 5 Availability"]
pub type FLASH_ROMSWMAP_SW5EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW5EN_A>;
impl FLASH_ROMSWMAP_SW5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW5EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW5EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw5en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW5EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw5en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW5EN` writer - ROM SW Region 5 Availability"]
pub type FLASH_ROMSWMAP_SW5EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW5EN_A, 2, 10>;
impl<'a> FLASH_ROMSWMAP_SW5EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW5EN_A::FLASH_ROMSWMAP_SW5EN_CORE)
    }
}
#[doc = "ROM SW Region 6 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW6EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW6EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW6EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW6EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW6EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW6EN` reader - ROM SW Region 6 Availability"]
pub type FLASH_ROMSWMAP_SW6EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW6EN_A>;
impl FLASH_ROMSWMAP_SW6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW6EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW6EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw6en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW6EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw6en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW6EN` writer - ROM SW Region 6 Availability"]
pub type FLASH_ROMSWMAP_SW6EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW6EN_A, 2, 12>;
impl<'a> FLASH_ROMSWMAP_SW6EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW6EN_A::FLASH_ROMSWMAP_SW6EN_CORE)
    }
}
#[doc = "ROM SW Region 7 Availability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_ROMSWMAP_SW7EN_A {
    #[doc = "0: Software region not available to the core"]
    FLASH_ROMSWMAP_SW7EN_NOTVIS = 0,
    #[doc = "1: Region available to core"]
    FLASH_ROMSWMAP_SW7EN_CORE = 1,
}
impl From<FLASH_ROMSWMAP_SW7EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_ROMSWMAP_SW7EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW7EN` reader - ROM SW Region 7 Availability"]
pub type FLASH_ROMSWMAP_SW7EN_R = crate::FieldReader<u8, FLASH_ROMSWMAP_SW7EN_A>;
impl FLASH_ROMSWMAP_SW7EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_ROMSWMAP_SW7EN_A> {
        match self.bits {
            0 => Some(FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_NOTVIS),
            1 => Some(FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_CORE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW7EN_NOTVIS`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw7en_notvis(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_NOTVIS
    }
    #[doc = "Checks if the value of the field is `FLASH_ROMSWMAP_SW7EN_CORE`"]
    #[inline(always)]
    pub fn is_flash_romswmap_sw7en_core(&self) -> bool {
        *self == FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_CORE
    }
}
#[doc = "Field `FLASH_ROMSWMAP_SW7EN` writer - ROM SW Region 7 Availability"]
pub type FLASH_ROMSWMAP_SW7EN_W<'a> =
    crate::FieldWriter<'a, u32, ROMSWMAP_SPEC, u8, FLASH_ROMSWMAP_SW7EN_A, 2, 14>;
impl<'a> FLASH_ROMSWMAP_SW7EN_W<'a> {
    #[doc = "Software region not available to the core"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en_notvis(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_NOTVIS)
    }
    #[doc = "Region available to core"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en_core(self) -> &'a mut W {
        self.variant(FLASH_ROMSWMAP_SW7EN_A::FLASH_ROMSWMAP_SW7EN_CORE)
    }
}
impl R {
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en(&self) -> FLASH_ROMSWMAP_SW0EN_R {
        FLASH_ROMSWMAP_SW0EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en(&self) -> FLASH_ROMSWMAP_SW1EN_R {
        FLASH_ROMSWMAP_SW1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en(&self) -> FLASH_ROMSWMAP_SW2EN_R {
        FLASH_ROMSWMAP_SW2EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en(&self) -> FLASH_ROMSWMAP_SW3EN_R {
        FLASH_ROMSWMAP_SW3EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en(&self) -> FLASH_ROMSWMAP_SW4EN_R {
        FLASH_ROMSWMAP_SW4EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en(&self) -> FLASH_ROMSWMAP_SW5EN_R {
        FLASH_ROMSWMAP_SW5EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en(&self) -> FLASH_ROMSWMAP_SW6EN_R {
        FLASH_ROMSWMAP_SW6EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en(&self) -> FLASH_ROMSWMAP_SW7EN_R {
        FLASH_ROMSWMAP_SW7EN_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ROM SW Region 0 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw0en(&mut self) -> FLASH_ROMSWMAP_SW0EN_W {
        FLASH_ROMSWMAP_SW0EN_W::new(self)
    }
    #[doc = "Bits 2:3 - ROM SW Region 1 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw1en(&mut self) -> FLASH_ROMSWMAP_SW1EN_W {
        FLASH_ROMSWMAP_SW1EN_W::new(self)
    }
    #[doc = "Bits 4:5 - ROM SW Region 2 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw2en(&mut self) -> FLASH_ROMSWMAP_SW2EN_W {
        FLASH_ROMSWMAP_SW2EN_W::new(self)
    }
    #[doc = "Bits 6:7 - ROM SW Region 3 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw3en(&mut self) -> FLASH_ROMSWMAP_SW3EN_W {
        FLASH_ROMSWMAP_SW3EN_W::new(self)
    }
    #[doc = "Bits 8:9 - ROM SW Region 4 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw4en(&mut self) -> FLASH_ROMSWMAP_SW4EN_W {
        FLASH_ROMSWMAP_SW4EN_W::new(self)
    }
    #[doc = "Bits 10:11 - ROM SW Region 5 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw5en(&mut self) -> FLASH_ROMSWMAP_SW5EN_W {
        FLASH_ROMSWMAP_SW5EN_W::new(self)
    }
    #[doc = "Bits 12:13 - ROM SW Region 6 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw6en(&mut self) -> FLASH_ROMSWMAP_SW6EN_W {
        FLASH_ROMSWMAP_SW6EN_W::new(self)
    }
    #[doc = "Bits 14:15 - ROM SW Region 7 Availability"]
    #[inline(always)]
    pub fn flash_romswmap_sw7en(&mut self) -> FLASH_ROMSWMAP_SW7EN_W {
        FLASH_ROMSWMAP_SW7EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM Software Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [romswmap](index.html) module"]
pub struct ROMSWMAP_SPEC;
impl crate::RegisterSpec for ROMSWMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [romswmap::R](R) reader structure"]
impl crate::Readable for ROMSWMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [romswmap::W](W) writer structure"]
impl crate::Writable for ROMSWMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROMSWMAP to value 0"]
impl crate::Resettable for ROMSWMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
