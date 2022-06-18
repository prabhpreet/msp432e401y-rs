#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Preamble Length for Transmit Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_CFG_PRELEN_A {
    #[doc = "0: 7 bytes of preamble"]
    EMAC_CFG_PRELEN_7 = 0,
    #[doc = "1: 5 bytes of preamble"]
    EMAC_CFG_PRELEN_5 = 1,
    #[doc = "2: 3 bytes of preamble"]
    EMAC_CFG_PRELEN_3 = 2,
}
impl From<EMAC_CFG_PRELEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_CFG_PRELEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_CFG_PRELEN` reader - Preamble Length for Transmit Frames"]
pub type EMAC_CFG_PRELEN_R = crate::FieldReader<u8, EMAC_CFG_PRELEN_A>;
impl EMAC_CFG_PRELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_CFG_PRELEN_A> {
        match self.bits {
            0 => Some(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_7),
            1 => Some(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_5),
            2 => Some(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_7`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_7(&self) -> bool {
        *self == EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_7
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_5`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_5(&self) -> bool {
        *self == EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_5
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_3`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_3(&self) -> bool {
        *self == EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_3
    }
}
#[doc = "Field `EMAC_CFG_PRELEN` writer - Preamble Length for Transmit Frames"]
pub type EMAC_CFG_PRELEN_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, EMAC_CFG_PRELEN_A, 2, 0>;
impl<'a> EMAC_CFG_PRELEN_W<'a> {
    #[doc = "7 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_7(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_7)
    }
    #[doc = "5 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_5(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_5)
    }
    #[doc = "3 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_3(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELEN_A::EMAC_CFG_PRELEN_3)
    }
}
#[doc = "Field `EMAC_CFG_RE` reader - Receiver Enable"]
pub type EMAC_CFG_RE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_RE` writer - Receiver Enable"]
pub type EMAC_CFG_RE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 2>;
#[doc = "Field `EMAC_CFG_TE` reader - Transmitter Enable"]
pub type EMAC_CFG_TE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_TE` writer - Transmitter Enable"]
pub type EMAC_CFG_TE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 3>;
#[doc = "Field `EMAC_CFG_DC` reader - Deferral Check"]
pub type EMAC_CFG_DC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_DC` writer - Deferral Check"]
pub type EMAC_CFG_DC_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 4>;
#[doc = "Back-Off Limit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_CFG_BL_A {
    #[doc = "0: k = min (n,10)"]
    EMAC_CFG_BL_1024 = 0,
    #[doc = "1: k = min (n,8)"]
    EMAC_CFG_BL_256 = 1,
    #[doc = "2: k = min (n,4)"]
    EMAC_CFG_BL_8 = 2,
    #[doc = "3: k = min (n,1)"]
    EMAC_CFG_BL_2 = 3,
}
impl From<EMAC_CFG_BL_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_CFG_BL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_CFG_BL` reader - Back-Off Limit"]
pub type EMAC_CFG_BL_R = crate::FieldReader<u8, EMAC_CFG_BL_A>;
impl EMAC_CFG_BL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_CFG_BL_A {
        match self.bits {
            0 => EMAC_CFG_BL_A::EMAC_CFG_BL_1024,
            1 => EMAC_CFG_BL_A::EMAC_CFG_BL_256,
            2 => EMAC_CFG_BL_A::EMAC_CFG_BL_8,
            3 => EMAC_CFG_BL_A::EMAC_CFG_BL_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_1024`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_1024(&self) -> bool {
        *self == EMAC_CFG_BL_A::EMAC_CFG_BL_1024
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_256`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_256(&self) -> bool {
        *self == EMAC_CFG_BL_A::EMAC_CFG_BL_256
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_8`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_8(&self) -> bool {
        *self == EMAC_CFG_BL_A::EMAC_CFG_BL_8
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_2`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_2(&self) -> bool {
        *self == EMAC_CFG_BL_A::EMAC_CFG_BL_2
    }
}
#[doc = "Field `EMAC_CFG_BL` writer - Back-Off Limit"]
pub type EMAC_CFG_BL_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, EMAC_CFG_BL_A, 2, 5>;
impl<'a> EMAC_CFG_BL_W<'a> {
    #[doc = "k = min (n,10)"]
    #[inline(always)]
    pub fn emac_cfg_bl_1024(self) -> &'a mut W {
        self.variant(EMAC_CFG_BL_A::EMAC_CFG_BL_1024)
    }
    #[doc = "k = min (n,8)"]
    #[inline(always)]
    pub fn emac_cfg_bl_256(self) -> &'a mut W {
        self.variant(EMAC_CFG_BL_A::EMAC_CFG_BL_256)
    }
    #[doc = "k = min (n,4)"]
    #[inline(always)]
    pub fn emac_cfg_bl_8(self) -> &'a mut W {
        self.variant(EMAC_CFG_BL_A::EMAC_CFG_BL_8)
    }
    #[doc = "k = min (n,1)"]
    #[inline(always)]
    pub fn emac_cfg_bl_2(self) -> &'a mut W {
        self.variant(EMAC_CFG_BL_A::EMAC_CFG_BL_2)
    }
}
#[doc = "Field `EMAC_CFG_ACS` reader - Automatic Pad or CRC Stripping"]
pub type EMAC_CFG_ACS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_ACS` writer - Automatic Pad or CRC Stripping"]
pub type EMAC_CFG_ACS_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 7>;
#[doc = "Field `EMAC_CFG_DR` reader - Disable Retry"]
pub type EMAC_CFG_DR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_DR` writer - Disable Retry"]
pub type EMAC_CFG_DR_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 9>;
#[doc = "Field `EMAC_CFG_IPC` reader - Checksum Offload"]
pub type EMAC_CFG_IPC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_IPC` writer - Checksum Offload"]
pub type EMAC_CFG_IPC_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 10>;
#[doc = "Field `EMAC_CFG_DUPM` reader - Duplex Mode"]
pub type EMAC_CFG_DUPM_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_DUPM` writer - Duplex Mode"]
pub type EMAC_CFG_DUPM_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 11>;
#[doc = "Field `EMAC_CFG_LOOPBM` reader - Loopback Mode"]
pub type EMAC_CFG_LOOPBM_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_LOOPBM` writer - Loopback Mode"]
pub type EMAC_CFG_LOOPBM_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 12>;
#[doc = "Field `EMAC_CFG_DRO` reader - Disable Receive Own"]
pub type EMAC_CFG_DRO_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_DRO` writer - Disable Receive Own"]
pub type EMAC_CFG_DRO_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 13>;
#[doc = "Field `EMAC_CFG_FES` reader - Speed"]
pub type EMAC_CFG_FES_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_FES` writer - Speed"]
pub type EMAC_CFG_FES_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 14>;
#[doc = "Field `EMAC_CFG_PS` reader - Port Select"]
pub type EMAC_CFG_PS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_PS` writer - Port Select"]
pub type EMAC_CFG_PS_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 15>;
#[doc = "Field `EMAC_CFG_DISCRS` reader - Disable Carrier Sense During Transmission"]
pub type EMAC_CFG_DISCRS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_DISCRS` writer - Disable Carrier Sense During Transmission"]
pub type EMAC_CFG_DISCRS_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 16>;
#[doc = "Inter-Frame Gap (IFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_CFG_IFG_A {
    #[doc = "0: 96 bit times"]
    EMAC_CFG_IFG_96 = 0,
    #[doc = "1: 88 bit times"]
    EMAC_CFG_IFG_88 = 1,
    #[doc = "2: 80 bit times"]
    EMAC_CFG_IFG_80 = 2,
    #[doc = "3: 72 bit times"]
    EMAC_CFG_IFG_72 = 3,
    #[doc = "4: 64 bit times"]
    EMAC_CFG_IFG_64 = 4,
    #[doc = "5: 56 bit times"]
    EMAC_CFG_IFG_56 = 5,
    #[doc = "6: 48 bit times"]
    EMAC_CFG_IFG_48 = 6,
    #[doc = "7: 40 bit times"]
    EMAC_CFG_IFG_40 = 7,
}
impl From<EMAC_CFG_IFG_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_CFG_IFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_CFG_IFG` reader - Inter-Frame Gap (IFG)"]
pub type EMAC_CFG_IFG_R = crate::FieldReader<u8, EMAC_CFG_IFG_A>;
impl EMAC_CFG_IFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_CFG_IFG_A {
        match self.bits {
            0 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_96,
            1 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_88,
            2 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_80,
            3 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_72,
            4 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_64,
            5 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_56,
            6 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_48,
            7 => EMAC_CFG_IFG_A::EMAC_CFG_IFG_40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_96`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_96(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_96
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_88`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_88(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_88
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_80`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_80(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_80
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_72`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_72(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_72
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_64`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_64(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_64
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_56`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_56(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_56
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_48`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_48(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_48
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_40`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_40(&self) -> bool {
        *self == EMAC_CFG_IFG_A::EMAC_CFG_IFG_40
    }
}
#[doc = "Field `EMAC_CFG_IFG` writer - Inter-Frame Gap (IFG)"]
pub type EMAC_CFG_IFG_W<'a> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, EMAC_CFG_IFG_A, 3, 17>;
impl<'a> EMAC_CFG_IFG_W<'a> {
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_96(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_96)
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_88(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_88)
    }
    #[doc = "80 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_80(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_80)
    }
    #[doc = "72 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_72(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_72)
    }
    #[doc = "64 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_64(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_64)
    }
    #[doc = "56 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_56(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_56)
    }
    #[doc = "48 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_48(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_48)
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_40(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFG_A::EMAC_CFG_IFG_40)
    }
}
#[doc = "Field `EMAC_CFG_JFEN` reader - Jumbo Frame Enable"]
pub type EMAC_CFG_JFEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_JFEN` writer - Jumbo Frame Enable"]
pub type EMAC_CFG_JFEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 20>;
#[doc = "Field `EMAC_CFG_JD` reader - Jabber Disable"]
pub type EMAC_CFG_JD_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_JD` writer - Jabber Disable"]
pub type EMAC_CFG_JD_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 22>;
#[doc = "Field `EMAC_CFG_WDDIS` reader - Watchdog Disable"]
pub type EMAC_CFG_WDDIS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_WDDIS` writer - Watchdog Disable"]
pub type EMAC_CFG_WDDIS_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 23>;
#[doc = "Field `EMAC_CFG_CST` reader - CRC Stripping for Type Frames"]
pub type EMAC_CFG_CST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_CST` writer - CRC Stripping for Type Frames"]
pub type EMAC_CFG_CST_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 25>;
#[doc = "Field `EMAC_CFG_TWOKPEN` reader - IEEE 802"]
pub type EMAC_CFG_TWOKPEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_CFG_TWOKPEN` writer - IEEE 802"]
pub type EMAC_CFG_TWOKPEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 27>;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn emac_cfg_prelen(&self) -> EMAC_CFG_PRELEN_R {
        EMAC_CFG_PRELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn emac_cfg_re(&self) -> EMAC_CFG_RE_R {
        EMAC_CFG_RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn emac_cfg_te(&self) -> EMAC_CFG_TE_R {
        EMAC_CFG_TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn emac_cfg_dc(&self) -> EMAC_CFG_DC_R {
        EMAC_CFG_DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn emac_cfg_bl(&self) -> EMAC_CFG_BL_R {
        EMAC_CFG_BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn emac_cfg_acs(&self) -> EMAC_CFG_ACS_R {
        EMAC_CFG_ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn emac_cfg_dr(&self) -> EMAC_CFG_DR_R {
        EMAC_CFG_DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn emac_cfg_ipc(&self) -> EMAC_CFG_IPC_R {
        EMAC_CFG_IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn emac_cfg_dupm(&self) -> EMAC_CFG_DUPM_R {
        EMAC_CFG_DUPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn emac_cfg_loopbm(&self) -> EMAC_CFG_LOOPBM_R {
        EMAC_CFG_LOOPBM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn emac_cfg_dro(&self) -> EMAC_CFG_DRO_R {
        EMAC_CFG_DRO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn emac_cfg_fes(&self) -> EMAC_CFG_FES_R {
        EMAC_CFG_FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn emac_cfg_ps(&self) -> EMAC_CFG_PS_R {
        EMAC_CFG_PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn emac_cfg_discrs(&self) -> EMAC_CFG_DISCRS_R {
        EMAC_CFG_DISCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn emac_cfg_ifg(&self) -> EMAC_CFG_IFG_R {
        EMAC_CFG_IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn emac_cfg_jfen(&self) -> EMAC_CFG_JFEN_R {
        EMAC_CFG_JFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn emac_cfg_jd(&self) -> EMAC_CFG_JD_R {
        EMAC_CFG_JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn emac_cfg_wddis(&self) -> EMAC_CFG_WDDIS_R {
        EMAC_CFG_WDDIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn emac_cfg_cst(&self) -> EMAC_CFG_CST_R {
        EMAC_CFG_CST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn emac_cfg_twokpen(&self) -> EMAC_CFG_TWOKPEN_R {
        EMAC_CFG_TWOKPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn emac_cfg_prelen(&mut self) -> EMAC_CFG_PRELEN_W {
        EMAC_CFG_PRELEN_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn emac_cfg_re(&mut self) -> EMAC_CFG_RE_W {
        EMAC_CFG_RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn emac_cfg_te(&mut self) -> EMAC_CFG_TE_W {
        EMAC_CFG_TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn emac_cfg_dc(&mut self) -> EMAC_CFG_DC_W {
        EMAC_CFG_DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn emac_cfg_bl(&mut self) -> EMAC_CFG_BL_W {
        EMAC_CFG_BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn emac_cfg_acs(&mut self) -> EMAC_CFG_ACS_W {
        EMAC_CFG_ACS_W::new(self)
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn emac_cfg_dr(&mut self) -> EMAC_CFG_DR_W {
        EMAC_CFG_DR_W::new(self)
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn emac_cfg_ipc(&mut self) -> EMAC_CFG_IPC_W {
        EMAC_CFG_IPC_W::new(self)
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn emac_cfg_dupm(&mut self) -> EMAC_CFG_DUPM_W {
        EMAC_CFG_DUPM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn emac_cfg_loopbm(&mut self) -> EMAC_CFG_LOOPBM_W {
        EMAC_CFG_LOOPBM_W::new(self)
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn emac_cfg_dro(&mut self) -> EMAC_CFG_DRO_W {
        EMAC_CFG_DRO_W::new(self)
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn emac_cfg_fes(&mut self) -> EMAC_CFG_FES_W {
        EMAC_CFG_FES_W::new(self)
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn emac_cfg_ps(&mut self) -> EMAC_CFG_PS_W {
        EMAC_CFG_PS_W::new(self)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn emac_cfg_discrs(&mut self) -> EMAC_CFG_DISCRS_W {
        EMAC_CFG_DISCRS_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn emac_cfg_ifg(&mut self) -> EMAC_CFG_IFG_W {
        EMAC_CFG_IFG_W::new(self)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn emac_cfg_jfen(&mut self) -> EMAC_CFG_JFEN_W {
        EMAC_CFG_JFEN_W::new(self)
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn emac_cfg_jd(&mut self) -> EMAC_CFG_JD_W {
        EMAC_CFG_JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn emac_cfg_wddis(&mut self) -> EMAC_CFG_WDDIS_W {
        EMAC_CFG_WDDIS_W::new(self)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn emac_cfg_cst(&mut self) -> EMAC_CFG_CST_W {
        EMAC_CFG_CST_W::new(self)
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn emac_cfg_twokpen(&mut self) -> EMAC_CFG_TWOKPEN_W {
        EMAC_CFG_TWOKPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
