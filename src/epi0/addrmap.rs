#[doc = "Register `ADDRMAP` reader"]
pub struct R(crate::R<ADDRMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRMAP` writer"]
pub struct W(crate::W<ADDRMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRMAP_SPEC>;
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
impl From<crate::W<ADDRMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRMAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External RAM Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_ERADR_A {
    #[doc = "0: Not mapped"]
    EPI_ADDRMAP_ERADR_NONE = 0,
    #[doc = "1: At 0x6000.0000"]
    EPI_ADDRMAP_ERADR_6000 = 1,
    #[doc = "2: At 0x8000.0000"]
    EPI_ADDRMAP_ERADR_8000 = 2,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    EPI_ADDRMAP_ERADR_HBQS = 3,
}
impl From<EPI_ADDRMAP_ERADR_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_ERADR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_ERADR` reader - External RAM Address"]
pub type EPI_ADDRMAP_ERADR_R = crate::FieldReader<u8, EPI_ADDRMAP_ERADR_A>;
impl EPI_ADDRMAP_ERADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_ADDRMAP_ERADR_A {
        match self.bits {
            0 => EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_NONE,
            1 => EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_6000,
            2 => EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_8000,
            3 => EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_none(&self) -> bool {
        *self == EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_6000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_6000(&self) -> bool {
        *self == EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_6000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_8000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_8000(&self) -> bool {
        *self == EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_8000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_HBQS`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_hbqs(&self) -> bool {
        *self == EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_HBQS
    }
}
#[doc = "Field `EPI_ADDRMAP_ERADR` writer - External RAM Address"]
pub type EPI_ADDRMAP_ERADR_W<'a> =
    crate::FieldWriterSafe<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_ERADR_A, 2, 0>;
impl<'a> EPI_ADDRMAP_ERADR_W<'a> {
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_NONE)
    }
    #[doc = "At 0x6000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_6000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_6000)
    }
    #[doc = "At 0x8000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_8000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_8000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_hbqs(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADR_A::EPI_ADDRMAP_ERADR_HBQS)
    }
}
#[doc = "External RAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_ERSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ERSZ_256B = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ERSZ_64KB = 1,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ERSZ_16MB = 2,
    #[doc = "3: 256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_ERSZ_256MB = 3,
}
impl From<EPI_ADDRMAP_ERSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_ERSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_ERSZ` reader - External RAM Size"]
pub type EPI_ADDRMAP_ERSZ_R = crate::FieldReader<u8, EPI_ADDRMAP_ERSZ_A>;
impl EPI_ADDRMAP_ERSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_ADDRMAP_ERSZ_A {
        match self.bits {
            0 => EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256B,
            1 => EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_64KB,
            2 => EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_16MB,
            3 => EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256MB
    }
}
#[doc = "Field `EPI_ADDRMAP_ERSZ` writer - External RAM Size"]
pub type EPI_ADDRMAP_ERSZ_W<'a> =
    crate::FieldWriterSafe<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_ERSZ_A, 2, 2>;
impl<'a> EPI_ADDRMAP_ERSZ_W<'a> {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZ_A::EPI_ADDRMAP_ERSZ_256MB)
    }
}
#[doc = "External Peripheral Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_EPADR_A {
    #[doc = "0: Not mapped"]
    EPI_ADDRMAP_EPADR_NONE = 0,
    #[doc = "1: At 0xA000.0000"]
    EPI_ADDRMAP_EPADR_A000 = 1,
    #[doc = "2: At 0xC000.0000"]
    EPI_ADDRMAP_EPADR_C000 = 2,
    #[doc = "3: Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    EPI_ADDRMAP_EPADR_HBQS = 3,
}
impl From<EPI_ADDRMAP_EPADR_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_EPADR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_EPADR` reader - External Peripheral Address"]
pub type EPI_ADDRMAP_EPADR_R = crate::FieldReader<u8, EPI_ADDRMAP_EPADR_A>;
impl EPI_ADDRMAP_EPADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_ADDRMAP_EPADR_A {
        match self.bits {
            0 => EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_NONE,
            1 => EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_A000,
            2 => EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_C000,
            3 => EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_none(&self) -> bool {
        *self == EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_A000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_a000(&self) -> bool {
        *self == EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_A000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_C000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_c000(&self) -> bool {
        *self == EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_C000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_HBQS`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_hbqs(&self) -> bool {
        *self == EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_HBQS
    }
}
#[doc = "Field `EPI_ADDRMAP_EPADR` writer - External Peripheral Address"]
pub type EPI_ADDRMAP_EPADR_W<'a> =
    crate::FieldWriterSafe<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_EPADR_A, 2, 4>;
impl<'a> EPI_ADDRMAP_EPADR_W<'a> {
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_NONE)
    }
    #[doc = "At 0xA000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_a000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_A000)
    }
    #[doc = "At 0xC000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_c000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_C000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_hbqs(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADR_A::EPI_ADDRMAP_EPADR_HBQS)
    }
}
#[doc = "External Peripheral Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_EPSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_EPSZ_256B = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_EPSZ_64KB = 1,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_EPSZ_16MB = 2,
    #[doc = "3: 256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_EPSZ_256MB = 3,
}
impl From<EPI_ADDRMAP_EPSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_EPSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_EPSZ` reader - External Peripheral Size"]
pub type EPI_ADDRMAP_EPSZ_R = crate::FieldReader<u8, EPI_ADDRMAP_EPSZ_A>;
impl EPI_ADDRMAP_EPSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_ADDRMAP_EPSZ_A {
        match self.bits {
            0 => EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256B,
            1 => EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_64KB,
            2 => EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_16MB,
            3 => EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256MB
    }
}
#[doc = "Field `EPI_ADDRMAP_EPSZ` writer - External Peripheral Size"]
pub type EPI_ADDRMAP_EPSZ_W<'a> =
    crate::FieldWriterSafe<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_EPSZ_A, 2, 6>;
impl<'a> EPI_ADDRMAP_EPSZ_W<'a> {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZ_A::EPI_ADDRMAP_EPSZ_256MB)
    }
}
#[doc = "External Code Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_ECADR_A {
    #[doc = "0: Not mapped"]
    EPI_ADDRMAP_ECADR_NONE = 0,
    #[doc = "1: At 0x1000.0000"]
    EPI_ADDRMAP_ECADR_1000 = 1,
}
impl From<EPI_ADDRMAP_ECADR_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_ECADR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_ECADR` reader - External Code Address"]
pub type EPI_ADDRMAP_ECADR_R = crate::FieldReader<u8, EPI_ADDRMAP_ECADR_A>;
impl EPI_ADDRMAP_ECADR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_ADDRMAP_ECADR_A> {
        match self.bits {
            0 => Some(EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_NONE),
            1 => Some(EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_1000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecadr_none(&self) -> bool {
        *self == EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECADR_1000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecadr_1000(&self) -> bool {
        *self == EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_1000
    }
}
#[doc = "Field `EPI_ADDRMAP_ECADR` writer - External Code Address"]
pub type EPI_ADDRMAP_ECADR_W<'a> =
    crate::FieldWriter<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_ECADR_A, 2, 8>;
impl<'a> EPI_ADDRMAP_ECADR_W<'a> {
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_NONE)
    }
    #[doc = "At 0x1000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr_1000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECADR_A::EPI_ADDRMAP_ECADR_1000)
    }
}
#[doc = "External Code Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_ADDRMAP_ECSZ_A {
    #[doc = "0: 256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ECSZ_256B = 0,
    #[doc = "1: 64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ECSZ_64KB = 1,
    #[doc = "2: 16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ECSZ_16MB = 2,
    #[doc = "3: 256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    EPI_ADDRMAP_ECSZ_256MB = 3,
}
impl From<EPI_ADDRMAP_ECSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_ADDRMAP_ECSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_ADDRMAP_ECSZ` reader - External Code Size"]
pub type EPI_ADDRMAP_ECSZ_R = crate::FieldReader<u8, EPI_ADDRMAP_ECSZ_A>;
impl EPI_ADDRMAP_ECSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_ADDRMAP_ECSZ_A {
        match self.bits {
            0 => EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256B,
            1 => EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_64KB,
            2 => EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_16MB,
            3 => EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256MB
    }
}
#[doc = "Field `EPI_ADDRMAP_ECSZ` writer - External Code Size"]
pub type EPI_ADDRMAP_ECSZ_W<'a> =
    crate::FieldWriterSafe<'a, u32, ADDRMAP_SPEC, u8, EPI_ADDRMAP_ECSZ_A, 2, 10>;
impl<'a> EPI_ADDRMAP_ECSZ_W<'a> {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_16MB)
    }
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZ_A::EPI_ADDRMAP_ECSZ_256MB)
    }
}
impl R {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn epi_addrmap_eradr(&self) -> EPI_ADDRMAP_ERADR_R {
        EPI_ADDRMAP_ERADR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn epi_addrmap_ersz(&self) -> EPI_ADDRMAP_ERSZ_R {
        EPI_ADDRMAP_ERSZ_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epi_addrmap_epadr(&self) -> EPI_ADDRMAP_EPADR_R {
        EPI_ADDRMAP_EPADR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epi_addrmap_epsz(&self) -> EPI_ADDRMAP_EPSZ_R {
        EPI_ADDRMAP_EPSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr(&self) -> EPI_ADDRMAP_ECADR_R {
        EPI_ADDRMAP_ECADR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz(&self) -> EPI_ADDRMAP_ECSZ_R {
        EPI_ADDRMAP_ECSZ_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn epi_addrmap_eradr(&mut self) -> EPI_ADDRMAP_ERADR_W {
        EPI_ADDRMAP_ERADR_W::new(self)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn epi_addrmap_ersz(&mut self) -> EPI_ADDRMAP_ERSZ_W {
        EPI_ADDRMAP_ERSZ_W::new(self)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epi_addrmap_epadr(&mut self) -> EPI_ADDRMAP_EPADR_W {
        EPI_ADDRMAP_EPADR_W::new(self)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epi_addrmap_epsz(&mut self) -> EPI_ADDRMAP_EPSZ_W {
        EPI_ADDRMAP_EPSZ_W::new(self)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr(&mut self) -> EPI_ADDRMAP_ECADR_W {
        EPI_ADDRMAP_ECADR_W::new(self)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz(&mut self) -> EPI_ADDRMAP_ECSZ_W {
        EPI_ADDRMAP_ECSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Address Map\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrmap](index.html) module"]
pub struct ADDRMAP_SPEC;
impl crate::RegisterSpec for ADDRMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrmap::R](R) reader structure"]
impl crate::Readable for ADDRMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrmap::W](W) writer structure"]
impl crate::Writable for ADDRMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRMAP to value 0"]
impl crate::Resettable for ADDRMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
