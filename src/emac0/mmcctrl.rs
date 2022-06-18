#[doc = "Register `MMCCTRL` reader"]
pub struct R(crate::R<MMCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCCTRL` writer"]
pub struct W(crate::W<MMCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCTRL_SPEC>;
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
impl From<crate::W<MMCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MMCCTRL_CNTRST` reader - Counters Reset"]
pub type EMAC_MMCCTRL_CNTRST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_CNTRST` writer - Counters Reset"]
pub type EMAC_MMCCTRL_CNTRST_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 0>;
#[doc = "Field `EMAC_MMCCTRL_CNTSTPRO` reader - Counters Stop Rollover"]
pub type EMAC_MMCCTRL_CNTSTPRO_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_CNTSTPRO` writer - Counters Stop Rollover"]
pub type EMAC_MMCCTRL_CNTSTPRO_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 1>;
#[doc = "Field `EMAC_MMCCTRL_RSTONRD` reader - Reset on Read"]
pub type EMAC_MMCCTRL_RSTONRD_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_RSTONRD` writer - Reset on Read"]
pub type EMAC_MMCCTRL_RSTONRD_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 2>;
#[doc = "Field `EMAC_MMCCTRL_CNTFREEZ` reader - MMC Counter Freeze"]
pub type EMAC_MMCCTRL_CNTFREEZ_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_CNTFREEZ` writer - MMC Counter Freeze"]
pub type EMAC_MMCCTRL_CNTFREEZ_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 3>;
#[doc = "Field `EMAC_MMCCTRL_CNTPRST` reader - Counters Preset"]
pub type EMAC_MMCCTRL_CNTPRST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_CNTPRST` writer - Counters Preset"]
pub type EMAC_MMCCTRL_CNTPRST_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 4>;
#[doc = "Field `EMAC_MMCCTRL_CNTPRSTLVL` reader - Full/Half Preset Level Value"]
pub type EMAC_MMCCTRL_CNTPRSTLVL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_CNTPRSTLVL` writer - Full/Half Preset Level Value"]
pub type EMAC_MMCCTRL_CNTPRSTLVL_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 5>;
#[doc = "Field `EMAC_MMCCTRL_UCDBC` reader - Update MMC Counters for Dropped Broadcast Frames"]
pub type EMAC_MMCCTRL_UCDBC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCCTRL_UCDBC` writer - Update MMC Counters for Dropped Broadcast Frames"]
pub type EMAC_MMCCTRL_UCDBC_W<'a> = crate::BitWriter<'a, u32, MMCCTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntrst(&self) -> EMAC_MMCCTRL_CNTRST_R {
        EMAC_MMCCTRL_CNTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntstpro(&self) -> EMAC_MMCCTRL_CNTSTPRO_R {
        EMAC_MMCCTRL_CNTSTPRO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn emac_mmcctrl_rstonrd(&self) -> EMAC_MMCCTRL_RSTONRD_R {
        EMAC_MMCCTRL_RSTONRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntfreez(&self) -> EMAC_MMCCTRL_CNTFREEZ_R {
        EMAC_MMCCTRL_CNTFREEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprst(&self) -> EMAC_MMCCTRL_CNTPRST_R {
        EMAC_MMCCTRL_CNTPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprstlvl(&self) -> EMAC_MMCCTRL_CNTPRSTLVL_R {
        EMAC_MMCCTRL_CNTPRSTLVL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn emac_mmcctrl_ucdbc(&self) -> EMAC_MMCCTRL_UCDBC_R {
        EMAC_MMCCTRL_UCDBC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counters Reset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntrst(&mut self) -> EMAC_MMCCTRL_CNTRST_W {
        EMAC_MMCCTRL_CNTRST_W::new(self)
    }
    #[doc = "Bit 1 - Counters Stop Rollover"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntstpro(&mut self) -> EMAC_MMCCTRL_CNTSTPRO_W {
        EMAC_MMCCTRL_CNTSTPRO_W::new(self)
    }
    #[doc = "Bit 2 - Reset on Read"]
    #[inline(always)]
    pub fn emac_mmcctrl_rstonrd(&mut self) -> EMAC_MMCCTRL_RSTONRD_W {
        EMAC_MMCCTRL_RSTONRD_W::new(self)
    }
    #[doc = "Bit 3 - MMC Counter Freeze"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntfreez(&mut self) -> EMAC_MMCCTRL_CNTFREEZ_W {
        EMAC_MMCCTRL_CNTFREEZ_W::new(self)
    }
    #[doc = "Bit 4 - Counters Preset"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprst(&mut self) -> EMAC_MMCCTRL_CNTPRST_W {
        EMAC_MMCCTRL_CNTPRST_W::new(self)
    }
    #[doc = "Bit 5 - Full/Half Preset Level Value"]
    #[inline(always)]
    pub fn emac_mmcctrl_cntprstlvl(&mut self) -> EMAC_MMCCTRL_CNTPRSTLVL_W {
        EMAC_MMCCTRL_CNTPRSTLVL_W::new(self)
    }
    #[doc = "Bit 8 - Update MMC Counters for Dropped Broadcast Frames"]
    #[inline(always)]
    pub fn emac_mmcctrl_ucdbc(&mut self) -> EMAC_MMCCTRL_UCDBC_W {
        EMAC_MMCCTRL_UCDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MMC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcctrl](index.html) module"]
pub struct MMCCTRL_SPEC;
impl crate::RegisterSpec for MMCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcctrl::R](R) reader structure"]
impl crate::Readable for MMCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcctrl::W](W) writer structure"]
impl crate::Writable for MMCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCCTRL to value 0"]
impl crate::Resettable for MMCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
