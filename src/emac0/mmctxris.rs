#[doc = "Register `MMCTXRIS` reader"]
pub struct R(crate::R<MMCTXRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTXRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTXRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTXRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTXRIS` writer"]
pub struct W(crate::W<MMCTXRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTXRIS_SPEC>;
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
impl From<crate::W<MMCTXRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTXRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MMCTXRIS_GBF` reader - MMC Transmit Good Bad Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_GBF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCTXRIS_GBF` writer - MMC Transmit Good Bad Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_GBF_W<'a> = crate::BitWriter<'a, u32, MMCTXRIS_SPEC, bool, 1>;
#[doc = "Field `EMAC_MMCTXRIS_SCOLLGF` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_SCOLLGF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCTXRIS_SCOLLGF` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_SCOLLGF_W<'a> = crate::BitWriter<'a, u32, MMCTXRIS_SPEC, bool, 14>;
#[doc = "Field `EMAC_MMCTXRIS_MCOLLGF` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_MCOLLGF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCTXRIS_MCOLLGF` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_MCOLLGF_W<'a> = crate::BitWriter<'a, u32, MMCTXRIS_SPEC, bool, 15>;
#[doc = "Field `EMAC_MMCTXRIS_OCTCNT` reader - Octet Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_OCTCNT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MMCTXRIS_OCTCNT` writer - Octet Counter Interrupt Status"]
pub type EMAC_MMCTXRIS_OCTCNT_W<'a> = crate::BitWriter<'a, u32, MMCTXRIS_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_gbf(&self) -> EMAC_MMCTXRIS_GBF_R {
        EMAC_MMCTXRIS_GBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_scollgf(&self) -> EMAC_MMCTXRIS_SCOLLGF_R {
        EMAC_MMCTXRIS_SCOLLGF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_mcollgf(&self) -> EMAC_MMCTXRIS_MCOLLGF_R {
        EMAC_MMCTXRIS_MCOLLGF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_octcnt(&self) -> EMAC_MMCTXRIS_OCTCNT_R {
        EMAC_MMCTXRIS_OCTCNT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_gbf(&mut self) -> EMAC_MMCTXRIS_GBF_W {
        EMAC_MMCTXRIS_GBF_W::new(self)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_scollgf(&mut self) -> EMAC_MMCTXRIS_SCOLLGF_W {
        EMAC_MMCTXRIS_SCOLLGF_W::new(self)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_mcollgf(&mut self) -> EMAC_MMCTXRIS_MCOLLGF_W {
        EMAC_MMCTXRIS_MCOLLGF_W::new(self)
    }
    #[doc = "Bit 20 - Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn emac_mmctxris_octcnt(&mut self) -> EMAC_MMCTXRIS_OCTCNT_W {
        EMAC_MMCTXRIS_OCTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctxris](index.html) module"]
pub struct MMCTXRIS_SPEC;
impl crate::RegisterSpec for MMCTXRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctxris::R](R) reader structure"]
impl crate::Readable for MMCTXRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctxris::W](W) writer structure"]
impl crate::Writable for MMCTXRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTXRIS to value 0"]
impl crate::Resettable for MMCTXRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
