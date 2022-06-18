#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_RIS_PMT` reader - PMT Interrupt Status"]
pub type EMAC_RIS_PMT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_PMT` writer - PMT Interrupt Status"]
pub type EMAC_RIS_PMT_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `EMAC_RIS_MMC` reader - MMC Interrupt Status"]
pub type EMAC_RIS_MMC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_MMC` writer - MMC Interrupt Status"]
pub type EMAC_RIS_MMC_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `EMAC_RIS_MMCRX` reader - MMC Receive Interrupt Status"]
pub type EMAC_RIS_MMCRX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_MMCRX` writer - MMC Receive Interrupt Status"]
pub type EMAC_RIS_MMCRX_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
#[doc = "Field `EMAC_RIS_MMCTX` reader - MMC Transmit Interrupt Status"]
pub type EMAC_RIS_MMCTX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_MMCTX` writer - MMC Transmit Interrupt Status"]
pub type EMAC_RIS_MMCTX_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 6>;
#[doc = "Field `EMAC_RIS_TS` reader - Timestamp Interrupt Status"]
pub type EMAC_RIS_TS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_TS` writer - Timestamp Interrupt Status"]
pub type EMAC_RIS_TS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 9>;
#[doc = "Field `EMAC_RIS_LPI` reader - LPI Interrupt Status"]
pub type EMAC_RIS_LPI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RIS_LPI` writer - LPI Interrupt Status"]
pub type EMAC_RIS_LPI_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_pmt(&self) -> EMAC_RIS_PMT_R {
        EMAC_RIS_PMT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmc(&self) -> EMAC_RIS_MMC_R {
        EMAC_RIS_MMC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmcrx(&self) -> EMAC_RIS_MMCRX_R {
        EMAC_RIS_MMCRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmctx(&self) -> EMAC_RIS_MMCTX_R {
        EMAC_RIS_MMCTX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_ts(&self) -> EMAC_RIS_TS_R {
        EMAC_RIS_TS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_lpi(&self) -> EMAC_RIS_LPI_R {
        EMAC_RIS_LPI_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_pmt(&mut self) -> EMAC_RIS_PMT_W {
        EMAC_RIS_PMT_W::new(self)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmc(&mut self) -> EMAC_RIS_MMC_W {
        EMAC_RIS_MMC_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmcrx(&mut self) -> EMAC_RIS_MMCRX_W {
        EMAC_RIS_MMCRX_W::new(self)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_mmctx(&mut self) -> EMAC_RIS_MMCTX_W {
        EMAC_RIS_MMCTX_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_ts(&mut self) -> EMAC_RIS_TS_W {
        EMAC_RIS_TS_W::new(self)
    }
    #[doc = "Bit 10 - LPI Interrupt Status"]
    #[inline(always)]
    pub fn emac_ris_lpi(&mut self) -> EMAC_RIS_LPI_W {
        EMAC_RIS_LPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
