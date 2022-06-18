#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_PC_PHYHOLD` reader - Ethernet PHY Hold"]
pub type EMAC_PC_PHYHOLD_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_PHYHOLD` writer - Ethernet PHY Hold"]
pub type EMAC_PC_PHYHOLD_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 0>;
#[doc = "Auto Negotiation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_PC_ANMODE_A {
    #[doc = "0: When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    EMAC_PC_ANMODE_10HD = 0,
    #[doc = "1: When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    EMAC_PC_ANMODE_10FD = 1,
    #[doc = "2: When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    EMAC_PC_ANMODE_100HD = 2,
    #[doc = "3: When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    EMAC_PC_ANMODE_100FD = 3,
}
impl From<EMAC_PC_ANMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_PC_ANMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_PC_ANMODE` reader - Auto Negotiation Mode"]
pub type EMAC_PC_ANMODE_R = crate::FieldReader<u8, EMAC_PC_ANMODE_A>;
impl EMAC_PC_ANMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_PC_ANMODE_A {
        match self.bits {
            0 => EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10HD,
            1 => EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10FD,
            2 => EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100HD,
            3 => EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100FD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_10HD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_10hd(&self) -> bool {
        *self == EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10HD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_10FD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_10fd(&self) -> bool {
        *self == EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10FD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_100HD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_100hd(&self) -> bool {
        *self == EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100HD
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_ANMODE_100FD`"]
    #[inline(always)]
    pub fn is_emac_pc_anmode_100fd(&self) -> bool {
        *self == EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100FD
    }
}
#[doc = "Field `EMAC_PC_ANMODE` writer - Auto Negotiation Mode"]
pub type EMAC_PC_ANMODE_W<'a> =
    crate::FieldWriterSafe<'a, u32, PC_SPEC, u8, EMAC_PC_ANMODE_A, 2, 1>;
impl<'a> EMAC_PC_ANMODE_W<'a> {
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Half-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_10hd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 10Base-T, Full-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_10fd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_10FD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Half-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_100hd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100HD)
    }
    #[doc = "When ANEN = 0x0, the mode is 100Base-TX, Full-Duplex"]
    #[inline(always)]
    pub fn emac_pc_anmode_100fd(self) -> &'a mut W {
        self.variant(EMAC_PC_ANMODE_A::EMAC_PC_ANMODE_100FD)
    }
}
#[doc = "Field `EMAC_PC_ANEN` reader - Auto Negotiation Enable"]
pub type EMAC_PC_ANEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_ANEN` writer - Auto Negotiation Enable"]
pub type EMAC_PC_ANEN_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 3>;
#[doc = "Field `EMAC_PC_FASTANSEL` reader - Fast Auto Negotiation Select"]
pub type EMAC_PC_FASTANSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_PC_FASTANSEL` writer - Fast Auto Negotiation Select"]
pub type EMAC_PC_FASTANSEL_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 2, 4>;
#[doc = "Field `EMAC_PC_FASTANEN` reader - Fast Auto Negotiation Enable"]
pub type EMAC_PC_FASTANEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_FASTANEN` writer - Fast Auto Negotiation Enable"]
pub type EMAC_PC_FASTANEN_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 6>;
#[doc = "Field `EMAC_PC_EXTFD` reader - Extended Full Duplex Ability"]
pub type EMAC_PC_EXTFD_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_EXTFD` writer - Extended Full Duplex Ability"]
pub type EMAC_PC_EXTFD_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 7>;
#[doc = "Field `EMAC_PC_FASTLUPD` reader - FAST Link-Up in Parallel Detect"]
pub type EMAC_PC_FASTLUPD_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_FASTLUPD` writer - FAST Link-Up in Parallel Detect"]
pub type EMAC_PC_FASTLUPD_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 8>;
#[doc = "Field `EMAC_PC_FASTRXDV` reader - Fast RXDV Detection"]
pub type EMAC_PC_FASTRXDV_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_FASTRXDV` writer - Fast RXDV Detection"]
pub type EMAC_PC_FASTRXDV_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 9>;
#[doc = "Field `EMAC_PC_MDIXEN` reader - MDIX Enable"]
pub type EMAC_PC_MDIXEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_MDIXEN` writer - MDIX Enable"]
pub type EMAC_PC_MDIXEN_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 10>;
#[doc = "Field `EMAC_PC_FASTMDIX` reader - Fast Auto MDI-X"]
pub type EMAC_PC_FASTMDIX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_FASTMDIX` writer - Fast Auto MDI-X"]
pub type EMAC_PC_FASTMDIX_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 11>;
#[doc = "Field `EMAC_PC_RBSTMDIX` reader - Robust Auto MDI-X"]
pub type EMAC_PC_RBSTMDIX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_RBSTMDIX` writer - Robust Auto MDI-X"]
pub type EMAC_PC_RBSTMDIX_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 12>;
#[doc = "Field `EMAC_PC_MDISWAP` reader - MDI Swap"]
pub type EMAC_PC_MDISWAP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_MDISWAP` writer - MDI Swap"]
pub type EMAC_PC_MDISWAP_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 13>;
#[doc = "Field `EMAC_PC_POLSWAP` reader - Polarity Swap"]
pub type EMAC_PC_POLSWAP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_POLSWAP` writer - Polarity Swap"]
pub type EMAC_PC_POLSWAP_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 14>;
#[doc = "Field `EMAC_PC_FASTLDMODE` reader - Fast Link Down Mode"]
pub type EMAC_PC_FASTLDMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_PC_FASTLDMODE` writer - Fast Link Down Mode"]
pub type EMAC_PC_FASTLDMODE_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, u8, 5, 15>;
#[doc = "Field `EMAC_PC_TDRRUN` reader - TDR Auto Run"]
pub type EMAC_PC_TDRRUN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_TDRRUN` writer - TDR Auto Run"]
pub type EMAC_PC_TDRRUN_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 20>;
#[doc = "Field `EMAC_PC_LRR` reader - Link Loss Recovery"]
pub type EMAC_PC_LRR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_LRR` writer - Link Loss Recovery"]
pub type EMAC_PC_LRR_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 21>;
#[doc = "Field `EMAC_PC_ISOMIILL` reader - Isolate MII in Link Loss"]
pub type EMAC_PC_ISOMIILL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_ISOMIILL` writer - Isolate MII in Link Loss"]
pub type EMAC_PC_ISOMIILL_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 22>;
#[doc = "Field `EMAC_PC_RXERIDLE` reader - RXER Detection During Idle"]
pub type EMAC_PC_RXERIDLE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_RXERIDLE` writer - RXER Detection During Idle"]
pub type EMAC_PC_RXERIDLE_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 23>;
#[doc = "Field `EMAC_PC_NIBDETDIS` reader - Odd Nibble TXER Detection Disable"]
pub type EMAC_PC_NIBDETDIS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_NIBDETDIS` writer - Odd Nibble TXER Detection Disable"]
pub type EMAC_PC_NIBDETDIS_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 24>;
#[doc = "Field `EMAC_PC_DIGRESTART` reader - PHY Soft Restart"]
pub type EMAC_PC_DIGRESTART_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_DIGRESTART` writer - PHY Soft Restart"]
pub type EMAC_PC_DIGRESTART_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 25>;
#[doc = "Ethernet Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_PC_PINTFS_A {
    #[doc = "0: MII (default) Used for internal PHY or external PHY connected via MII"]
    EMAC_PC_PINTFS_IMII = 0,
    #[doc = "4: RMII: Used for external PHY connected via RMII"]
    EMAC_PC_PINTFS_RMII = 4,
}
impl From<EMAC_PC_PINTFS_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_PC_PINTFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_PC_PINTFS` reader - Ethernet Interface Select"]
pub type EMAC_PC_PINTFS_R = crate::FieldReader<u8, EMAC_PC_PINTFS_A>;
impl EMAC_PC_PINTFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_PC_PINTFS_A> {
        match self.bits {
            0 => Some(EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_IMII),
            4 => Some(EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_RMII),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_PINTFS_IMII`"]
    #[inline(always)]
    pub fn is_emac_pc_pintfs_imii(&self) -> bool {
        *self == EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_IMII
    }
    #[doc = "Checks if the value of the field is `EMAC_PC_PINTFS_RMII`"]
    #[inline(always)]
    pub fn is_emac_pc_pintfs_rmii(&self) -> bool {
        *self == EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_RMII
    }
}
#[doc = "Field `EMAC_PC_PINTFS` writer - Ethernet Interface Select"]
pub type EMAC_PC_PINTFS_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, EMAC_PC_PINTFS_A, 3, 28>;
impl<'a> EMAC_PC_PINTFS_W<'a> {
    #[doc = "MII (default) Used for internal PHY or external PHY connected via MII"]
    #[inline(always)]
    pub fn emac_pc_pintfs_imii(self) -> &'a mut W {
        self.variant(EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_IMII)
    }
    #[doc = "RMII: Used for external PHY connected via RMII"]
    #[inline(always)]
    pub fn emac_pc_pintfs_rmii(self) -> &'a mut W {
        self.variant(EMAC_PC_PINTFS_A::EMAC_PC_PINTFS_RMII)
    }
}
#[doc = "Field `EMAC_PC_PHYEXT` reader - PHY Select"]
pub type EMAC_PC_PHYEXT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PC_PHYEXT` writer - PHY Select"]
pub type EMAC_PC_PHYEXT_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn emac_pc_phyhold(&self) -> EMAC_PC_PHYHOLD_R {
        EMAC_PC_PHYHOLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn emac_pc_anmode(&self) -> EMAC_PC_ANMODE_R {
        EMAC_PC_ANMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_anen(&self) -> EMAC_PC_ANEN_R {
        EMAC_PC_ANEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn emac_pc_fastansel(&self) -> EMAC_PC_FASTANSEL_R {
        EMAC_PC_FASTANSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_fastanen(&self) -> EMAC_PC_FASTANEN_R {
        EMAC_PC_FASTANEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn emac_pc_extfd(&self) -> EMAC_PC_EXTFD_R {
        EMAC_PC_EXTFD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn emac_pc_fastlupd(&self) -> EMAC_PC_FASTLUPD_R {
        EMAC_PC_FASTLUPD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn emac_pc_fastrxdv(&self) -> EMAC_PC_FASTRXDV_R {
        EMAC_PC_FASTRXDV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn emac_pc_mdixen(&self) -> EMAC_PC_MDIXEN_R {
        EMAC_PC_MDIXEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_fastmdix(&self) -> EMAC_PC_FASTMDIX_R {
        EMAC_PC_FASTMDIX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_rbstmdix(&self) -> EMAC_PC_RBSTMDIX_R {
        EMAC_PC_RBSTMDIX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn emac_pc_mdiswap(&self) -> EMAC_PC_MDISWAP_R {
        EMAC_PC_MDISWAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn emac_pc_polswap(&self) -> EMAC_PC_POLSWAP_R {
        EMAC_PC_POLSWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn emac_pc_fastldmode(&self) -> EMAC_PC_FASTLDMODE_R {
        EMAC_PC_FASTLDMODE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn emac_pc_tdrrun(&self) -> EMAC_PC_TDRRUN_R {
        EMAC_PC_TDRRUN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn emac_pc_lrr(&self) -> EMAC_PC_LRR_R {
        EMAC_PC_LRR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn emac_pc_isomiill(&self) -> EMAC_PC_ISOMIILL_R {
        EMAC_PC_ISOMIILL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn emac_pc_rxeridle(&self) -> EMAC_PC_RXERIDLE_R {
        EMAC_PC_RXERIDLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn emac_pc_nibdetdis(&self) -> EMAC_PC_NIBDETDIS_R {
        EMAC_PC_NIBDETDIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn emac_pc_digrestart(&self) -> EMAC_PC_DIGRESTART_R {
        EMAC_PC_DIGRESTART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn emac_pc_pintfs(&self) -> EMAC_PC_PINTFS_R {
        EMAC_PC_PINTFS_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn emac_pc_phyext(&self) -> EMAC_PC_PHYEXT_R {
        EMAC_PC_PHYEXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Hold"]
    #[inline(always)]
    pub fn emac_pc_phyhold(&mut self) -> EMAC_PC_PHYHOLD_W {
        EMAC_PC_PHYHOLD_W::new(self)
    }
    #[doc = "Bits 1:2 - Auto Negotiation Mode"]
    #[inline(always)]
    pub fn emac_pc_anmode(&mut self) -> EMAC_PC_ANMODE_W {
        EMAC_PC_ANMODE_W::new(self)
    }
    #[doc = "Bit 3 - Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_anen(&mut self) -> EMAC_PC_ANEN_W {
        EMAC_PC_ANEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Fast Auto Negotiation Select"]
    #[inline(always)]
    pub fn emac_pc_fastansel(&mut self) -> EMAC_PC_FASTANSEL_W {
        EMAC_PC_FASTANSEL_W::new(self)
    }
    #[doc = "Bit 6 - Fast Auto Negotiation Enable"]
    #[inline(always)]
    pub fn emac_pc_fastanen(&mut self) -> EMAC_PC_FASTANEN_W {
        EMAC_PC_FASTANEN_W::new(self)
    }
    #[doc = "Bit 7 - Extended Full Duplex Ability"]
    #[inline(always)]
    pub fn emac_pc_extfd(&mut self) -> EMAC_PC_EXTFD_W {
        EMAC_PC_EXTFD_W::new(self)
    }
    #[doc = "Bit 8 - FAST Link-Up in Parallel Detect"]
    #[inline(always)]
    pub fn emac_pc_fastlupd(&mut self) -> EMAC_PC_FASTLUPD_W {
        EMAC_PC_FASTLUPD_W::new(self)
    }
    #[doc = "Bit 9 - Fast RXDV Detection"]
    #[inline(always)]
    pub fn emac_pc_fastrxdv(&mut self) -> EMAC_PC_FASTRXDV_W {
        EMAC_PC_FASTRXDV_W::new(self)
    }
    #[doc = "Bit 10 - MDIX Enable"]
    #[inline(always)]
    pub fn emac_pc_mdixen(&mut self) -> EMAC_PC_MDIXEN_W {
        EMAC_PC_MDIXEN_W::new(self)
    }
    #[doc = "Bit 11 - Fast Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_fastmdix(&mut self) -> EMAC_PC_FASTMDIX_W {
        EMAC_PC_FASTMDIX_W::new(self)
    }
    #[doc = "Bit 12 - Robust Auto MDI-X"]
    #[inline(always)]
    pub fn emac_pc_rbstmdix(&mut self) -> EMAC_PC_RBSTMDIX_W {
        EMAC_PC_RBSTMDIX_W::new(self)
    }
    #[doc = "Bit 13 - MDI Swap"]
    #[inline(always)]
    pub fn emac_pc_mdiswap(&mut self) -> EMAC_PC_MDISWAP_W {
        EMAC_PC_MDISWAP_W::new(self)
    }
    #[doc = "Bit 14 - Polarity Swap"]
    #[inline(always)]
    pub fn emac_pc_polswap(&mut self) -> EMAC_PC_POLSWAP_W {
        EMAC_PC_POLSWAP_W::new(self)
    }
    #[doc = "Bits 15:19 - Fast Link Down Mode"]
    #[inline(always)]
    pub fn emac_pc_fastldmode(&mut self) -> EMAC_PC_FASTLDMODE_W {
        EMAC_PC_FASTLDMODE_W::new(self)
    }
    #[doc = "Bit 20 - TDR Auto Run"]
    #[inline(always)]
    pub fn emac_pc_tdrrun(&mut self) -> EMAC_PC_TDRRUN_W {
        EMAC_PC_TDRRUN_W::new(self)
    }
    #[doc = "Bit 21 - Link Loss Recovery"]
    #[inline(always)]
    pub fn emac_pc_lrr(&mut self) -> EMAC_PC_LRR_W {
        EMAC_PC_LRR_W::new(self)
    }
    #[doc = "Bit 22 - Isolate MII in Link Loss"]
    #[inline(always)]
    pub fn emac_pc_isomiill(&mut self) -> EMAC_PC_ISOMIILL_W {
        EMAC_PC_ISOMIILL_W::new(self)
    }
    #[doc = "Bit 23 - RXER Detection During Idle"]
    #[inline(always)]
    pub fn emac_pc_rxeridle(&mut self) -> EMAC_PC_RXERIDLE_W {
        EMAC_PC_RXERIDLE_W::new(self)
    }
    #[doc = "Bit 24 - Odd Nibble TXER Detection Disable"]
    #[inline(always)]
    pub fn emac_pc_nibdetdis(&mut self) -> EMAC_PC_NIBDETDIS_W {
        EMAC_PC_NIBDETDIS_W::new(self)
    }
    #[doc = "Bit 25 - PHY Soft Restart"]
    #[inline(always)]
    pub fn emac_pc_digrestart(&mut self) -> EMAC_PC_DIGRESTART_W {
        EMAC_PC_DIGRESTART_W::new(self)
    }
    #[doc = "Bits 28:30 - Ethernet Interface Select"]
    #[inline(always)]
    pub fn emac_pc_pintfs(&mut self) -> EMAC_PC_PINTFS_W {
        EMAC_PC_PINTFS_W::new(self)
    }
    #[doc = "Bit 31 - PHY Select"]
    #[inline(always)]
    pub fn emac_pc_phyext(&mut self) -> EMAC_PC_PHYEXT_W {
        EMAC_PC_PHYEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Peripheral Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
