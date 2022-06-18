#[doc = "Register `HB8CFG` reader"]
pub struct R(crate::R<EPI_ALT8_HB8CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPI_ALT8_HB8CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPI_ALT8_HB8CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPI_ALT8_HB8CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB8CFG` writer"]
pub struct W(crate::W<EPI_ALT8_HB8CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPI_ALT8_HB8CFG_SPEC>;
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
impl From<crate::W<EPI_ALT8_HB8CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPI_ALT8_HB8CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG_MODE_A {
    #[doc = "0: ADMUX - AD\\[7:0\\]"]
    EPI_HB8CFG_MODE_MUX = 0,
    #[doc = "1: ADNONMUX - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_NMUX = 1,
    #[doc = "2: Continuous Read - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_SRAM = 2,
    #[doc = "3: XFIFO - D\\[7:0\\]"]
    EPI_HB8CFG_MODE_FIFO = 3,
}
impl From<EPI_HB8CFG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG_MODE` reader - Host Bus Sub-Mode"]
pub type EPI_HB8CFG_MODE_R = crate::FieldReader<u8, EPI_HB8CFG_MODE_A>;
impl EPI_HB8CFG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG_MODE_A {
        match self.bits {
            0 => EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_MUX,
            1 => EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_NMUX,
            2 => EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_SRAM,
            3 => EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_FIFO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_MUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_mux(&self) -> bool {
        *self == EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_MUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_NMUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_nmux(&self) -> bool {
        *self == EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_NMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_SRAM`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_sram(&self) -> bool {
        *self == EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_SRAM
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_MODE_FIFO`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_mode_fifo(&self) -> bool {
        *self == EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_FIFO
    }
}
#[doc = "Field `EPI_HB8CFG_MODE` writer - Host Bus Sub-Mode"]
pub type EPI_HB8CFG_MODE_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG_SPEC, u8, EPI_HB8CFG_MODE_A, 2, 0>;
impl<'a> EPI_HB8CFG_MODE_W<'a> {
    #[doc = "ADMUX - AD\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_mux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_MUX)
    }
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_nmux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_NMUX)
    }
    #[doc = "Continuous Read - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_sram(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_SRAM)
    }
    #[doc = "XFIFO - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode_fifo(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_MODE_A::EPI_HB8CFG_MODE_FIFO)
    }
}
#[doc = "Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG_RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    EPI_HB8CFG_RDWS_2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    EPI_HB8CFG_RDWS_4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    EPI_HB8CFG_RDWS_6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    EPI_HB8CFG_RDWS_8 = 3,
}
impl From<EPI_HB8CFG_RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG_RDWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG_RDWS` reader - Read Wait States"]
pub type EPI_HB8CFG_RDWS_R = crate::FieldReader<u8, EPI_HB8CFG_RDWS_A>;
impl EPI_HB8CFG_RDWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG_RDWS_A {
        match self.bits {
            0 => EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_2,
            1 => EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_4,
            2 => EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_6,
            3 => EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_2(&self) -> bool {
        *self == EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_4(&self) -> bool {
        *self == EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_6(&self) -> bool {
        *self == EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_rdws_8(&self) -> bool {
        *self == EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_8
    }
}
#[doc = "Field `EPI_HB8CFG_RDWS` writer - Read Wait States"]
pub type EPI_HB8CFG_RDWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG_SPEC, u8, EPI_HB8CFG_RDWS_A, 2, 4>;
impl<'a> EPI_HB8CFG_RDWS_W<'a> {
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_RDWS_A::EPI_HB8CFG_RDWS_8)
    }
}
#[doc = "Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG_WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    EPI_HB8CFG_WRWS_2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    EPI_HB8CFG_WRWS_4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    EPI_HB8CFG_WRWS_6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    EPI_HB8CFG_WRWS_8 = 3,
}
impl From<EPI_HB8CFG_WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG_WRWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG_WRWS` reader - Write Wait States"]
pub type EPI_HB8CFG_WRWS_R = crate::FieldReader<u8, EPI_HB8CFG_WRWS_A>;
impl EPI_HB8CFG_WRWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG_WRWS_A {
        match self.bits {
            0 => EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_2,
            1 => EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_4,
            2 => EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_6,
            3 => EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_2(&self) -> bool {
        *self == EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_4(&self) -> bool {
        *self == EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_6(&self) -> bool {
        *self == EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg_wrws_8(&self) -> bool {
        *self == EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_8
    }
}
#[doc = "Field `EPI_HB8CFG_WRWS` writer - Write Wait States"]
pub type EPI_HB8CFG_WRWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT8_HB8CFG_SPEC, u8, EPI_HB8CFG_WRWS_A, 2, 6>;
impl<'a> EPI_HB8CFG_WRWS_W<'a> {
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG_WRWS_A::EPI_HB8CFG_WRWS_8)
    }
}
#[doc = "Field `EPI_HB8CFG_MAXWAIT` reader - Maximum Wait"]
pub type EPI_HB8CFG_MAXWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_HB8CFG_MAXWAIT` writer - Maximum Wait"]
pub type EPI_HB8CFG_MAXWAIT_W<'a> = crate::FieldWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, u8, u8, 8, 8>;
#[doc = "Field `EPI_HB8CFG_ALEHIGH` reader - ALE Strobe Polarity"]
pub type EPI_HB8CFG_ALEHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_ALEHIGH` writer - ALE Strobe Polarity"]
pub type EPI_HB8CFG_ALEHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 19>;
#[doc = "Field `EPI_HB8CFG_RDHIGH` reader - READ Strobe Polarity"]
pub type EPI_HB8CFG_RDHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_RDHIGH` writer - READ Strobe Polarity"]
pub type EPI_HB8CFG_RDHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 20>;
#[doc = "Field `EPI_HB8CFG_WRHIGH` reader - WRITE Strobe Polarity"]
pub type EPI_HB8CFG_WRHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_WRHIGH` writer - WRITE Strobe Polarity"]
pub type EPI_HB8CFG_WRHIGH_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 21>;
#[doc = "Field `EPI_HB8CFG_XFEEN` reader - External FIFO EMPTY Enable"]
pub type EPI_HB8CFG_XFEEN_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_XFEEN` writer - External FIFO EMPTY Enable"]
pub type EPI_HB8CFG_XFEEN_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 22>;
#[doc = "Field `EPI_HB8CFG_XFFEN` reader - External FIFO FULL Enable"]
pub type EPI_HB8CFG_XFFEN_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_XFFEN` writer - External FIFO FULL Enable"]
pub type EPI_HB8CFG_XFFEN_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 23>;
#[doc = "Field `EPI_HB8CFG_IRDYINV` reader - Input Ready Invert"]
pub type EPI_HB8CFG_IRDYINV_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_IRDYINV` writer - Input Ready Invert"]
pub type EPI_HB8CFG_IRDYINV_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 27>;
#[doc = "Field `EPI_HB8CFG_RDYEN` reader - Input Ready Enable"]
pub type EPI_HB8CFG_RDYEN_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_RDYEN` writer - Input Ready Enable"]
pub type EPI_HB8CFG_RDYEN_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 28>;
#[doc = "Field `EPI_HB8CFG_CLKINV` reader - Invert Output Clock Enable"]
pub type EPI_HB8CFG_CLKINV_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_CLKINV` writer - Invert Output Clock Enable"]
pub type EPI_HB8CFG_CLKINV_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 29>;
#[doc = "Field `EPI_HB8CFG_CLKGATEI` reader - Clock Gated when Idle"]
pub type EPI_HB8CFG_CLKGATEI_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_CLKGATEI` writer - Clock Gated when Idle"]
pub type EPI_HB8CFG_CLKGATEI_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 30>;
#[doc = "Field `EPI_HB8CFG_CLKGATE` reader - Clock Gated"]
pub type EPI_HB8CFG_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG_CLKGATE` writer - Clock Gated"]
pub type EPI_HB8CFG_CLKGATE_W<'a> = crate::BitWriter<'a, u32, EPI_ALT8_HB8CFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode(&self) -> EPI_HB8CFG_MODE_R {
        EPI_HB8CFG_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws(&self) -> EPI_HB8CFG_RDWS_R {
        EPI_HB8CFG_RDWS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws(&self) -> EPI_HB8CFG_WRWS_R {
        EPI_HB8CFG_WRWS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb8cfg_maxwait(&self) -> EPI_HB8CFG_MAXWAIT_R {
        EPI_HB8CFG_MAXWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_alehigh(&self) -> EPI_HB8CFG_ALEHIGH_R {
        EPI_HB8CFG_ALEHIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdhigh(&self) -> EPI_HB8CFG_RDHIGH_R {
        EPI_HB8CFG_RDHIGH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrhigh(&self) -> EPI_HB8CFG_WRHIGH_R {
        EPI_HB8CFG_WRHIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xfeen(&self) -> EPI_HB8CFG_XFEEN_R {
        EPI_HB8CFG_XFEEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xffen(&self) -> EPI_HB8CFG_XFFEN_R {
        EPI_HB8CFG_XFFEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb8cfg_irdyinv(&self) -> EPI_HB8CFG_IRDYINV_R {
        EPI_HB8CFG_IRDYINV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdyen(&self) -> EPI_HB8CFG_RDYEN_R {
        EPI_HB8CFG_RDYEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkinv(&self) -> EPI_HB8CFG_CLKINV_R {
        EPI_HB8CFG_CLKINV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgatei(&self) -> EPI_HB8CFG_CLKGATEI_R {
        EPI_HB8CFG_CLKGATEI_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgate(&self) -> EPI_HB8CFG_CLKGATE_R {
        EPI_HB8CFG_CLKGATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg_mode(&mut self) -> EPI_HB8CFG_MODE_W {
        EPI_HB8CFG_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdws(&mut self) -> EPI_HB8CFG_RDWS_W {
        EPI_HB8CFG_RDWS_W::new(self)
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrws(&mut self) -> EPI_HB8CFG_WRWS_W {
        EPI_HB8CFG_WRWS_W::new(self)
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline(always)]
    pub fn epi_hb8cfg_maxwait(&mut self) -> EPI_HB8CFG_MAXWAIT_W {
        EPI_HB8CFG_MAXWAIT_W::new(self)
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_alehigh(&mut self) -> EPI_HB8CFG_ALEHIGH_W {
        EPI_HB8CFG_ALEHIGH_W::new(self)
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdhigh(&mut self) -> EPI_HB8CFG_RDHIGH_W {
        EPI_HB8CFG_RDHIGH_W::new(self)
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg_wrhigh(&mut self) -> EPI_HB8CFG_WRHIGH_W {
        EPI_HB8CFG_WRHIGH_W::new(self)
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xfeen(&mut self) -> EPI_HB8CFG_XFEEN_W {
        EPI_HB8CFG_XFEEN_W::new(self)
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_xffen(&mut self) -> EPI_HB8CFG_XFFEN_W {
        EPI_HB8CFG_XFFEN_W::new(self)
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline(always)]
    pub fn epi_hb8cfg_irdyinv(&mut self) -> EPI_HB8CFG_IRDYINV_W {
        EPI_HB8CFG_IRDYINV_W::new(self)
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_rdyen(&mut self) -> EPI_HB8CFG_RDYEN_W {
        EPI_HB8CFG_RDYEN_W::new(self)
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkinv(&mut self) -> EPI_HB8CFG_CLKINV_W {
        EPI_HB8CFG_CLKINV_W::new(self)
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgatei(&mut self) -> EPI_HB8CFG_CLKGATEI_W {
        EPI_HB8CFG_CLKGATEI_W::new(self)
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline(always)]
    pub fn epi_hb8cfg_clkgate(&mut self) -> EPI_HB8CFG_CLKGATE_W {
        EPI_HB8CFG_CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 8 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epi_alt8_hb8cfg](index.html) module"]
pub struct EPI_ALT8_HB8CFG_SPEC;
impl crate::RegisterSpec for EPI_ALT8_HB8CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epi_alt8_hb8cfg::R](R) reader structure"]
impl crate::Readable for EPI_ALT8_HB8CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epi_alt8_hb8cfg::W](W) writer structure"]
impl crate::Writable for EPI_ALT8_HB8CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB8CFG to value 0"]
impl crate::Resettable for EPI_ALT8_HB8CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
