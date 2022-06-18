#[doc = "Register `MMCRXRIS` reader"]
pub struct R(crate::R<MMCRXRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRXRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRXRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRXRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRXRIS` writer"]
pub struct W(crate::W<MMCRXRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRXRIS_SPEC>;
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
impl From<crate::W<MMCRXRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRXRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MMCRXRIS_GBF` reader - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_GBF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXRIS_GBF` writer - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_GBF_W<'a> = crate::BitWriter<'a, u32, MMCRXRIS_SPEC, bool, 0>;
#[doc = "Field `EMAC_MMCRXRIS_CRCERR` reader - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_CRCERR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXRIS_CRCERR` writer - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_CRCERR_W<'a> = crate::BitWriter<'a, u32, MMCRXRIS_SPEC, bool, 5>;
#[doc = "Field `EMAC_MMCRXRIS_ALGNERR` reader - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_ALGNERR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXRIS_ALGNERR` writer - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_ALGNERR_W<'a> = crate::BitWriter<'a, u32, MMCRXRIS_SPEC, bool, 6>;
#[doc = "Field `EMAC_MMCRXRIS_UCGF` reader - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_UCGF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCRXRIS_UCGF` writer - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub type EMAC_MMCRXRIS_UCGF_W<'a> = crate::BitWriter<'a, u32, MMCRXRIS_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_gbf(&self) -> EMAC_MMCRXRIS_GBF_R {
        EMAC_MMCRXRIS_GBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_crcerr(&self) -> EMAC_MMCRXRIS_CRCERR_R {
        EMAC_MMCRXRIS_CRCERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_algnerr(&self) -> EMAC_MMCRXRIS_ALGNERR_R {
        EMAC_MMCRXRIS_ALGNERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_ucgf(&self) -> EMAC_MMCRXRIS_UCGF_R {
        EMAC_MMCRXRIS_UCGF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_gbf(&mut self) -> EMAC_MMCRXRIS_GBF_W {
        EMAC_MMCRXRIS_GBF_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_crcerr(&mut self) -> EMAC_MMCRXRIS_CRCERR_W {
        EMAC_MMCRXRIS_CRCERR_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_algnerr(&mut self) -> EMAC_MMCRXRIS_ALGNERR_W {
        EMAC_MMCRXRIS_ALGNERR_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmcrxris_ucgf(&mut self) -> EMAC_MMCRXRIS_UCGF_W {
        EMAC_MMCRXRIS_UCGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrxris](index.html) module"]
pub struct MMCRXRIS_SPEC;
impl crate::RegisterSpec for MMCRXRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrxris::R](R) reader structure"]
impl crate::Readable for MMCRXRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrxris::W](W) writer structure"]
impl crate::Writable for MMCRXRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRXRIS to value 0"]
impl crate::Resettable for MMCRXRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
