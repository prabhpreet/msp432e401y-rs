#[doc = "Register `HB8CFG2` reader"]
pub struct R(crate::R<EPI_ALT8_HB8CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPI_ALT8_HB8CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPI_ALT8_HB8CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPI_ALT8_HB8CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB8CFG2` writer"]
pub struct W(crate::W<EPI_ALT8_HB8CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPI_ALT8_HB8CFG2_SPEC>;
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
impl From<crate::W<EPI_ALT8_HB8CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPI_ALT8_HB8CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CS1n Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG2_MODE_A {
    #[doc = "0: ADMUX - AD\\[7:0\\]"]
    EPI_HB8CFG2_MODE_ADMUX = 0,
    #[doc = "1: ADNONMUX - D\\[7:0\\]"]
    EPI_HB8CFG2_MODE_AD = 1,
}
impl From<EPI_HB8CFG2_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG2_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG2_MODE` reader - CS1n Host Bus Sub-Mode"]
pub type EPI_HB8CFG2_MODE_R = crate::FieldReader<u8, EPI_HB8CFG2_MODE_A>;
impl EPI_HB8CFG2_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_HB8CFG2_MODE_A> {
        match self.bits {
            0 => Some(EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_ADMUX),
            1 => Some(EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_AD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_mode_admux(&self) -> bool {
        *self == EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_MODE_AD`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_mode_ad(&self) -> bool {
        *self == EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_AD
    }
}
#[doc = "Field `EPI_HB8CFG2_MODE` writer - CS1n Host Bus Sub-Mode"]
pub type EPI_HB8CFG2_MODE_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, u8, EPI_HB8CFG2_MODE_A, 2, 0>;
impl<'a> EPI_HB8CFG2_MODE_W<'a> {
    #[doc = "ADMUX - AD\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg2_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg2_mode_ad(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_MODE_A::EPI_HB8CFG2_MODE_AD)
    }
}
#[doc = "CS1n Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG2_RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    EPI_HB8CFG2_RDWS_2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    EPI_HB8CFG2_RDWS_4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    EPI_HB8CFG2_RDWS_6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    EPI_HB8CFG2_RDWS_8 = 3,
}
impl From<EPI_HB8CFG2_RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG2_RDWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG2_RDWS` reader - CS1n Read Wait States"]
pub type EPI_HB8CFG2_RDWS_R = crate::FieldReader<u8, EPI_HB8CFG2_RDWS_A>;
impl EPI_HB8CFG2_RDWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG2_RDWS_A {
        match self.bits {
            0 => EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_2,
            1 => EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_4,
            2 => EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_6,
            3 => EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_rdws_2(&self) -> bool {
        *self == EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_rdws_4(&self) -> bool {
        *self == EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_rdws_6(&self) -> bool {
        *self == EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_rdws_8(&self) -> bool {
        *self == EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_8
    }
}
#[doc = "Field `EPI_HB8CFG2_RDWS` writer - CS1n Read Wait States"]
pub type EPI_HB8CFG2_RDWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG2_SPEC, u8, EPI_HB8CFG2_RDWS_A, 2, 4>;
impl<'a> EPI_HB8CFG2_RDWS_W<'a> {
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_RDWS_A::EPI_HB8CFG2_RDWS_8)
    }
}
#[doc = "CS1n Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG2_WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    EPI_HB8CFG2_WRWS_2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    EPI_HB8CFG2_WRWS_4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    EPI_HB8CFG2_WRWS_6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    EPI_HB8CFG2_WRWS_8 = 3,
}
impl From<EPI_HB8CFG2_WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG2_WRWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG2_WRWS` reader - CS1n Write Wait States"]
pub type EPI_HB8CFG2_WRWS_R = crate::FieldReader<u8, EPI_HB8CFG2_WRWS_A>;
impl EPI_HB8CFG2_WRWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG2_WRWS_A {
        match self.bits {
            0 => EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_2,
            1 => EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_4,
            2 => EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_6,
            3 => EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_wrws_2(&self) -> bool {
        *self == EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_wrws_4(&self) -> bool {
        *self == EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_wrws_6(&self) -> bool {
        *self == EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_wrws_8(&self) -> bool {
        *self == EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_8
    }
}
#[doc = "Field `EPI_HB8CFG2_WRWS` writer - CS1n Write Wait States"]
pub type EPI_HB8CFG2_WRWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG2_SPEC, u8, EPI_HB8CFG2_WRWS_A, 2, 6>;
impl<'a> EPI_HB8CFG2_WRWS_W<'a> {
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_WRWS_A::EPI_HB8CFG2_WRWS_8)
    }
}
#[doc = "Field `EPI_HB8CFG2_ALEHIGH` reader - CS1n ALE Strobe Polarity"]
pub type EPI_HB8CFG2_ALEHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG2_ALEHIGH` writer - CS1n ALE Strobe Polarity"]
pub type EPI_HB8CFG2_ALEHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, bool, 19>;
#[doc = "Field `EPI_HB8CFG2_RDHIGH` reader - CS1n READ Strobe Polarity"]
pub type EPI_HB8CFG2_RDHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG2_RDHIGH` writer - CS1n READ Strobe Polarity"]
pub type EPI_HB8CFG2_RDHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, bool, 20>;
#[doc = "Field `EPI_HB8CFG2_WRHIGH` reader - CS1n WRITE Strobe Polarity"]
pub type EPI_HB8CFG2_WRHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG2_WRHIGH` writer - CS1n WRITE Strobe Polarity"]
pub type EPI_HB8CFG2_WRHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, bool, 21>;
#[doc = "Chip Select Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG2_CSCFG_A {
    #[doc = "0: ALE Configuration"]
    EPI_HB8CFG2_CSCFG_ALE = 0,
    #[doc = "1: CSn Configuration"]
    EPI_HB8CFG2_CSCFG_CS = 1,
    #[doc = "2: Dual CSn Configuration"]
    EPI_HB8CFG2_CSCFG_DCS = 2,
    #[doc = "3: ALE with Dual CSn Configuration"]
    EPI_HB8CFG2_CSCFG_ADCS = 3,
}
impl From<EPI_HB8CFG2_CSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG2_CSCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG2_CSCFG` reader - Chip Select Configuration"]
pub type EPI_HB8CFG2_CSCFG_R = crate::FieldReader<u8, EPI_HB8CFG2_CSCFG_A>;
impl EPI_HB8CFG2_CSCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG2_CSCFG_A {
        match self.bits {
            0 => EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ALE,
            1 => EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_CS,
            2 => EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_DCS,
            3 => EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ADCS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_CSCFG_ALE`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_cscfg_ale(&self) -> bool {
        *self == EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ALE
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_CSCFG_CS`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_cscfg_cs(&self) -> bool {
        *self == EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_CS
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_CSCFG_DCS`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_cscfg_dcs(&self) -> bool {
        *self == EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_DCS
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG2_CSCFG_ADCS`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg2_cscfg_adcs(&self) -> bool {
        *self == EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ADCS
    }
}
#[doc = "Field `EPI_HB8CFG2_CSCFG` writer - Chip Select Configuration"]
pub type EPI_HB8CFG2_CSCFG_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG2_SPEC, u8, EPI_HB8CFG2_CSCFG_A, 2, 24>;
impl<'a> EPI_HB8CFG2_CSCFG_W<'a> {
    #[doc = "ALE Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg_ale(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ALE)
    }
    #[doc = "CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg_cs(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_CS)
    }
    #[doc = "Dual CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg_dcs(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_DCS)
    }
    #[doc = "ALE with Dual CSn Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg_adcs(self) -> &'a mut W {
        self.variant(EPI_HB8CFG2_CSCFG_A::EPI_HB8CFG2_CSCFG_ADCS)
    }
}
#[doc = "Field `EPI_HB8CFG2_CSBAUD` reader - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
pub type EPI_HB8CFG2_CSBAUD_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG2_CSBAUD` writer - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
pub type EPI_HB8CFG2_CSBAUD_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, bool, 26>;
#[doc = "Field `EPI_HB8CFG2_CSCFGEXT` reader - Chip Select Extended Configuration"]
pub type EPI_HB8CFG2_CSCFGEXT_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG2_CSCFGEXT` writer - Chip Select Extended Configuration"]
pub type EPI_HB8CFG2_CSCFGEXT_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG2_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg2_mode(&self) -> EPI_HB8CFG2_MODE_R {
        EPI_HB8CFG2_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws(&self) -> EPI_HB8CFG2_RDWS_R {
        EPI_HB8CFG2_RDWS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws(&self) -> EPI_HB8CFG2_WRWS_R {
        EPI_HB8CFG2_WRWS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_alehigh(&self) -> EPI_HB8CFG2_ALEHIGH_R {
        EPI_HB8CFG2_ALEHIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdhigh(&self) -> EPI_HB8CFG2_RDHIGH_R {
        EPI_HB8CFG2_RDHIGH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrhigh(&self) -> EPI_HB8CFG2_WRHIGH_R {
        EPI_HB8CFG2_WRHIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg(&self) -> EPI_HB8CFG2_CSCFG_R {
        EPI_HB8CFG2_CSCFG_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn epi_hb8cfg2_csbaud(&self) -> EPI_HB8CFG2_CSBAUD_R {
        EPI_HB8CFG2_CSBAUD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfgext(&self) -> EPI_HB8CFG2_CSCFGEXT_R {
        EPI_HB8CFG2_CSCFGEXT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg2_mode(&mut self) -> EPI_HB8CFG2_MODE_W {
        EPI_HB8CFG2_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdws(&mut self) -> EPI_HB8CFG2_RDWS_W {
        EPI_HB8CFG2_RDWS_W::new(self)
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrws(&mut self) -> EPI_HB8CFG2_WRWS_W {
        EPI_HB8CFG2_WRWS_W::new(self)
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_alehigh(&mut self) -> EPI_HB8CFG2_ALEHIGH_W {
        EPI_HB8CFG2_ALEHIGH_W::new(self)
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_rdhigh(&mut self) -> EPI_HB8CFG2_RDHIGH_W {
        EPI_HB8CFG2_RDHIGH_W::new(self)
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg2_wrhigh(&mut self) -> EPI_HB8CFG2_WRHIGH_W {
        EPI_HB8CFG2_WRHIGH_W::new(self)
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfg(&mut self) -> EPI_HB8CFG2_CSCFG_W {
        EPI_HB8CFG2_CSCFG_W::new(self)
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn epi_hb8cfg2_csbaud(&mut self) -> EPI_HB8CFG2_CSBAUD_W {
        EPI_HB8CFG2_CSBAUD_W::new(self)
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn epi_hb8cfg2_cscfgext(&mut self) -> EPI_HB8CFG2_CSCFGEXT_W {
        EPI_HB8CFG2_CSCFGEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 8 Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epi_alt8_hb8cfg2](index.html) module"]
pub struct EPI_ALT8_HB8CFG2_SPEC;
impl crate::RegisterSpec for EPI_ALT8_HB8CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epi_alt8_hb8cfg2::R](R) reader structure"]
impl crate::Readable for EPI_ALT8_HB8CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epi_alt8_hb8cfg2::W](W) writer structure"]
impl crate::Writable for EPI_ALT8_HB8CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB8CFG2 to value 0"]
impl crate::Resettable for EPI_ALT8_HB8CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
