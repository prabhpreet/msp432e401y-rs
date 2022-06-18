#[doc = "Register `MMCRXIM` reader"]
pub struct R(crate::R<MMCRXIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRXIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRXIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRXIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRXIM` writer"]
pub struct W(crate::W<MMCRXIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRXIM_SPEC>;
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
impl From<crate::W<MMCRXIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRXIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MMCRXIM_GBF` reader - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_GBF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXIM_GBF` writer - MMC Receive Good Bad Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_GBF_W<'a> = crate::BitWriter<'a, u32, MMCRXIM_SPEC, bool, 0>;
#[doc = "Field `EMAC_MMCRXIM_CRCERR` reader - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_CRCERR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXIM_CRCERR` writer - MMC Receive CRC Error Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_CRCERR_W<'a> = crate::BitWriter<'a, u32, MMCRXIM_SPEC, bool, 5>;
#[doc = "Field `EMAC_MMCRXIM_ALGNERR` reader - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_ALGNERR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXIM_ALGNERR` writer - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_ALGNERR_W<'a> = crate::BitWriter<'a, u32, MMCRXIM_SPEC, bool, 6>;
#[doc = "Field `EMAC_MMCRXIM_UCGF` reader - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_UCGF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXIM_UCGF` writer - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
pub type EMAC_MMCRXIM_UCGF_W<'a> = crate::BitWriter<'a, u32, MMCRXIM_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_gbf(&self) -> EMAC_MMCRXIM_GBF_R {
        EMAC_MMCRXIM_GBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_crcerr(&self) -> EMAC_MMCRXIM_CRCERR_R {
        EMAC_MMCRXIM_CRCERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_algnerr(&self) -> EMAC_MMCRXIM_ALGNERR_R {
        EMAC_MMCRXIM_ALGNERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_ucgf(&self) -> EMAC_MMCRXIM_UCGF_R {
        EMAC_MMCRXIM_UCGF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_gbf(&mut self) -> EMAC_MMCRXIM_GBF_W {
        EMAC_MMCRXIM_GBF_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_crcerr(&mut self) -> EMAC_MMCRXIM_CRCERR_W {
        EMAC_MMCRXIM_CRCERR_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_algnerr(&mut self) -> EMAC_MMCRXIM_ALGNERR_W {
        EMAC_MMCRXIM_ALGNERR_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn emac_mmcrxim_ucgf(&mut self) -> EMAC_MMCRXIM_UCGF_W {
        EMAC_MMCRXIM_UCGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MMC Receive Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrxim](index.html) module"]
pub struct MMCRXIM_SPEC;
impl crate::RegisterSpec for MMCRXIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrxim::R](R) reader structure"]
impl crate::Readable for MMCRXIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrxim::W](W) writer structure"]
impl crate::Writable for MMCRXIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRXIM to value 0"]
impl crate::Resettable for MMCRXIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
