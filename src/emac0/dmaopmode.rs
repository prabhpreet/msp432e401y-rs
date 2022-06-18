#[doc = "Register `DMAOPMODE` reader"]
pub struct R(crate::R<DMAOPMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAOPMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAOPMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAOPMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAOPMODE` writer"]
pub struct W(crate::W<DMAOPMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAOPMODE_SPEC>;
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
impl From<crate::W<DMAOPMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAOPMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_DMAOPMODE_SR` reader - Start or Stop Receive"]
pub type EMAC_DMAOPMODE_SR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_SR` writer - Start or Stop Receive"]
pub type EMAC_DMAOPMODE_SR_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 1>;
#[doc = "Field `EMAC_DMAOPMODE_OSF` reader - Operate on Second Frame"]
pub type EMAC_DMAOPMODE_OSF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_OSF` writer - Operate on Second Frame"]
pub type EMAC_DMAOPMODE_OSF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 2>;
#[doc = "Receive Threshold Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_DMAOPMODE_RTC_A {
    #[doc = "0: 64 bytes"]
    EMAC_DMAOPMODE_RTC_64 = 0,
    #[doc = "1: 32 bytes"]
    EMAC_DMAOPMODE_RTC_32 = 1,
    #[doc = "2: 96 bytes"]
    EMAC_DMAOPMODE_RTC_96 = 2,
    #[doc = "3: 128 bytes"]
    EMAC_DMAOPMODE_RTC_128 = 3,
}
impl From<EMAC_DMAOPMODE_RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_DMAOPMODE_RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_DMAOPMODE_RTC` reader - Receive Threshold Control"]
pub type EMAC_DMAOPMODE_RTC_R = crate::FieldReader<u8, EMAC_DMAOPMODE_RTC_A>;
impl EMAC_DMAOPMODE_RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_DMAOPMODE_RTC_A {
        match self.bits {
            0 => EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_64,
            1 => EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_32,
            2 => EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_96,
            3 => EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_64`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_64(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_64
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_32`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_32(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_32
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_96`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_96(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_96
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_RTC_128`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_rtc_128(&self) -> bool {
        *self == EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_128
    }
}
#[doc = "Field `EMAC_DMAOPMODE_RTC` writer - Receive Threshold Control"]
pub type EMAC_DMAOPMODE_RTC_W<'a> =
    crate::FieldWriterSafe<'a, u32, DMAOPMODE_SPEC, u8, EMAC_DMAOPMODE_RTC_A, 2, 3>;
impl<'a> EMAC_DMAOPMODE_RTC_W<'a> {
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_64(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_32(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_96(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc_128(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_RTC_A::EMAC_DMAOPMODE_RTC_128)
    }
}
#[doc = "Field `EMAC_DMAOPMODE_DGF` reader - Drop Giant Frame Enable"]
pub type EMAC_DMAOPMODE_DGF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_DGF` writer - Drop Giant Frame Enable"]
pub type EMAC_DMAOPMODE_DGF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 5>;
#[doc = "Field `EMAC_DMAOPMODE_FUF` reader - Forward Undersized Good Frames"]
pub type EMAC_DMAOPMODE_FUF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_FUF` writer - Forward Undersized Good Frames"]
pub type EMAC_DMAOPMODE_FUF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 6>;
#[doc = "Field `EMAC_DMAOPMODE_FEF` reader - Forward Error Frames"]
pub type EMAC_DMAOPMODE_FEF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_FEF` writer - Forward Error Frames"]
pub type EMAC_DMAOPMODE_FEF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 7>;
#[doc = "Field `EMAC_DMAOPMODE_ST` reader - Start or Stop Transmission Command"]
pub type EMAC_DMAOPMODE_ST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_ST` writer - Start or Stop Transmission Command"]
pub type EMAC_DMAOPMODE_ST_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 13>;
#[doc = "Transmit Threshold Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_DMAOPMODE_TTC_A {
    #[doc = "0: 64 bytes"]
    EMAC_DMAOPMODE_TTC_64 = 0,
    #[doc = "1: 128 bytes"]
    EMAC_DMAOPMODE_TTC_128 = 1,
    #[doc = "2: 192 bytes"]
    EMAC_DMAOPMODE_TTC_192 = 2,
    #[doc = "3: 256 bytes"]
    EMAC_DMAOPMODE_TTC_256 = 3,
    #[doc = "4: 40 bytes"]
    EMAC_DMAOPMODE_TTC_40 = 4,
    #[doc = "5: 32 bytes"]
    EMAC_DMAOPMODE_TTC_32 = 5,
    #[doc = "6: 24 bytes"]
    EMAC_DMAOPMODE_TTC_24 = 6,
    #[doc = "7: 16 bytes"]
    EMAC_DMAOPMODE_TTC_16 = 7,
}
impl From<EMAC_DMAOPMODE_TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_DMAOPMODE_TTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_DMAOPMODE_TTC` reader - Transmit Threshold Control"]
pub type EMAC_DMAOPMODE_TTC_R = crate::FieldReader<u8, EMAC_DMAOPMODE_TTC_A>;
impl EMAC_DMAOPMODE_TTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_DMAOPMODE_TTC_A {
        match self.bits {
            0 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_64,
            1 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_128,
            2 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_192,
            3 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_256,
            4 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_40,
            5 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_32,
            6 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_24,
            7 => EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_64`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_64(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_64
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_128`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_128(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_128
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_192`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_192(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_192
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_256`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_256(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_256
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_40`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_40(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_40
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_32`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_32(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_32
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_24`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_24(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_24
    }
    #[doc = "Checks if the value of the field is `EMAC_DMAOPMODE_TTC_16`"]
    #[inline(always)]
    pub fn is_emac_dmaopmode_ttc_16(&self) -> bool {
        *self == EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_16
    }
}
#[doc = "Field `EMAC_DMAOPMODE_TTC` writer - Transmit Threshold Control"]
pub type EMAC_DMAOPMODE_TTC_W<'a> =
    crate::FieldWriterSafe<'a, u32, DMAOPMODE_SPEC, u8, EMAC_DMAOPMODE_TTC_A, 3, 14>;
impl<'a> EMAC_DMAOPMODE_TTC_W<'a> {
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_64(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_128(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_192(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_256(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_40(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_32(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_24(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc_16(self) -> &'a mut W {
        self.variant(EMAC_DMAOPMODE_TTC_A::EMAC_DMAOPMODE_TTC_16)
    }
}
#[doc = "Field `EMAC_DMAOPMODE_FTF` reader - Flush Transmit FIFO"]
pub type EMAC_DMAOPMODE_FTF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_FTF` writer - Flush Transmit FIFO"]
pub type EMAC_DMAOPMODE_FTF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 20>;
#[doc = "Field `EMAC_DMAOPMODE_TSF` reader - Transmit Store and Forward"]
pub type EMAC_DMAOPMODE_TSF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_TSF` writer - Transmit Store and Forward"]
pub type EMAC_DMAOPMODE_TSF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 21>;
#[doc = "Field `EMAC_DMAOPMODE_DFF` reader - Disable Flushing of Received Frames"]
pub type EMAC_DMAOPMODE_DFF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_DFF` writer - Disable Flushing of Received Frames"]
pub type EMAC_DMAOPMODE_DFF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 24>;
#[doc = "Field `EMAC_DMAOPMODE_RSF` reader - Receive Store and Forward"]
pub type EMAC_DMAOPMODE_RSF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_RSF` writer - Receive Store and Forward"]
pub type EMAC_DMAOPMODE_RSF_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 25>;
#[doc = "Field `EMAC_DMAOPMODE_DT` reader - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type EMAC_DMAOPMODE_DT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAOPMODE_DT` writer - Disable Dropping of TCP/IP Checksum Error Frames"]
pub type EMAC_DMAOPMODE_DT_W<'a> = crate::BitWriter<'a, u32, DMAOPMODE_SPEC, bool, 26>;
impl R {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn emac_dmaopmode_sr(&self) -> EMAC_DMAOPMODE_SR_R {
        EMAC_DMAOPMODE_SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn emac_dmaopmode_osf(&self) -> EMAC_DMAOPMODE_OSF_R {
        EMAC_DMAOPMODE_OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc(&self) -> EMAC_DMAOPMODE_RTC_R {
        EMAC_DMAOPMODE_RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn emac_dmaopmode_dgf(&self) -> EMAC_DMAOPMODE_DGF_R {
        EMAC_DMAOPMODE_DGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fuf(&self) -> EMAC_DMAOPMODE_FUF_R {
        EMAC_DMAOPMODE_FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fef(&self) -> EMAC_DMAOPMODE_FEF_R {
        EMAC_DMAOPMODE_FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn emac_dmaopmode_st(&self) -> EMAC_DMAOPMODE_ST_R {
        EMAC_DMAOPMODE_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc(&self) -> EMAC_DMAOPMODE_TTC_R {
        EMAC_DMAOPMODE_TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn emac_dmaopmode_ftf(&self) -> EMAC_DMAOPMODE_FTF_R {
        EMAC_DMAOPMODE_FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_tsf(&self) -> EMAC_DMAOPMODE_TSF_R {
        EMAC_DMAOPMODE_TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dff(&self) -> EMAC_DMAOPMODE_DFF_R {
        EMAC_DMAOPMODE_DFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_rsf(&self) -> EMAC_DMAOPMODE_RSF_R {
        EMAC_DMAOPMODE_RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dt(&self) -> EMAC_DMAOPMODE_DT_R {
        EMAC_DMAOPMODE_DT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start or Stop Receive"]
    #[inline(always)]
    pub fn emac_dmaopmode_sr(&mut self) -> EMAC_DMAOPMODE_SR_W {
        EMAC_DMAOPMODE_SR_W::new(self)
    }
    #[doc = "Bit 2 - Operate on Second Frame"]
    #[inline(always)]
    pub fn emac_dmaopmode_osf(&mut self) -> EMAC_DMAOPMODE_OSF_W {
        EMAC_DMAOPMODE_OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_rtc(&mut self) -> EMAC_DMAOPMODE_RTC_W {
        EMAC_DMAOPMODE_RTC_W::new(self)
    }
    #[doc = "Bit 5 - Drop Giant Frame Enable"]
    #[inline(always)]
    pub fn emac_dmaopmode_dgf(&mut self) -> EMAC_DMAOPMODE_DGF_W {
        EMAC_DMAOPMODE_DGF_W::new(self)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fuf(&mut self) -> EMAC_DMAOPMODE_FUF_W {
        EMAC_DMAOPMODE_FUF_W::new(self)
    }
    #[doc = "Bit 7 - Forward Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_fef(&mut self) -> EMAC_DMAOPMODE_FEF_W {
        EMAC_DMAOPMODE_FEF_W::new(self)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn emac_dmaopmode_st(&mut self) -> EMAC_DMAOPMODE_ST_W {
        EMAC_DMAOPMODE_ST_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn emac_dmaopmode_ttc(&mut self) -> EMAC_DMAOPMODE_TTC_W {
        EMAC_DMAOPMODE_TTC_W::new(self)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO"]
    #[inline(always)]
    pub fn emac_dmaopmode_ftf(&mut self) -> EMAC_DMAOPMODE_FTF_W {
        EMAC_DMAOPMODE_FTF_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_tsf(&mut self) -> EMAC_DMAOPMODE_TSF_W {
        EMAC_DMAOPMODE_TSF_W::new(self)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dff(&mut self) -> EMAC_DMAOPMODE_DFF_W {
        EMAC_DMAOPMODE_DFF_W::new(self)
    }
    #[doc = "Bit 25 - Receive Store and Forward"]
    #[inline(always)]
    pub fn emac_dmaopmode_rsf(&mut self) -> EMAC_DMAOPMODE_RSF_W {
        EMAC_DMAOPMODE_RSF_W::new(self)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames"]
    #[inline(always)]
    pub fn emac_dmaopmode_dt(&mut self) -> EMAC_DMAOPMODE_DT_W {
        EMAC_DMAOPMODE_DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC DMA Operation Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaopmode](index.html) module"]
pub struct DMAOPMODE_SPEC;
impl crate::RegisterSpec for DMAOPMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaopmode::R](R) reader structure"]
impl crate::Readable for DMAOPMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaopmode::W](W) writer structure"]
impl crate::Writable for DMAOPMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAOPMODE to value 0"]
impl crate::Resettable for DMAOPMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
