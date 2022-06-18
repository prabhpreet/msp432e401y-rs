#[doc = "Register `DMABUSMOD` reader"]
pub struct R(crate::R<DMABUSMOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABUSMOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABUSMOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABUSMOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABUSMOD` writer"]
pub struct W(crate::W<DMABUSMOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABUSMOD_SPEC>;
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
impl From<crate::W<DMABUSMOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABUSMOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_DMABUSMOD_SWR` reader - DMA Software Reset"]
pub type EMAC_DMABUSMOD_SWR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_SWR` writer - DMA Software Reset"]
pub type EMAC_DMABUSMOD_SWR_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 0>;
#[doc = "Field `EMAC_DMABUSMOD_DA` reader - DMA Arbitration Scheme"]
pub type EMAC_DMABUSMOD_DA_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_DA` writer - DMA Arbitration Scheme"]
pub type EMAC_DMABUSMOD_DA_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 1>;
#[doc = "Field `EMAC_DMABUSMOD_DSL` reader - Descriptor Skip Length"]
pub type EMAC_DMABUSMOD_DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_DMABUSMOD_DSL` writer - Descriptor Skip Length"]
pub type EMAC_DMABUSMOD_DSL_W<'a> = crate::FieldWriter<'a, u32, DMABUSMOD_SPEC, u8, u8, 5, 2>;
#[doc = "Field `EMAC_DMABUSMOD_ATDS` reader - Alternate Descriptor Size"]
pub type EMAC_DMABUSMOD_ATDS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_ATDS` writer - Alternate Descriptor Size"]
pub type EMAC_DMABUSMOD_ATDS_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 7>;
#[doc = "Field `EMAC_DMABUSMOD_PBL` reader - Programmable Burst Length"]
pub type EMAC_DMABUSMOD_PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_DMABUSMOD_PBL` writer - Programmable Burst Length"]
pub type EMAC_DMABUSMOD_PBL_W<'a> = crate::FieldWriter<'a, u32, DMABUSMOD_SPEC, u8, u8, 6, 8>;
#[doc = "Field `EMAC_DMABUSMOD_PR` reader - Priority Ratio"]
pub type EMAC_DMABUSMOD_PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_DMABUSMOD_PR` writer - Priority Ratio"]
pub type EMAC_DMABUSMOD_PR_W<'a> = crate::FieldWriter<'a, u32, DMABUSMOD_SPEC, u8, u8, 2, 14>;
#[doc = "Field `EMAC_DMABUSMOD_FB` reader - Fixed Burst"]
pub type EMAC_DMABUSMOD_FB_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_FB` writer - Fixed Burst"]
pub type EMAC_DMABUSMOD_FB_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 16>;
#[doc = "Field `EMAC_DMABUSMOD_RPBL` reader - RX DMA Programmable Burst Length (PBL)"]
pub type EMAC_DMABUSMOD_RPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_DMABUSMOD_RPBL` writer - RX DMA Programmable Burst Length (PBL)"]
pub type EMAC_DMABUSMOD_RPBL_W<'a> = crate::FieldWriter<'a, u32, DMABUSMOD_SPEC, u8, u8, 6, 17>;
#[doc = "Field `EMAC_DMABUSMOD_USP` reader - Use Separate Programmable Burst Length (PBL)"]
pub type EMAC_DMABUSMOD_USP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_USP` writer - Use Separate Programmable Burst Length (PBL)"]
pub type EMAC_DMABUSMOD_USP_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 23>;
#[doc = "Field `EMAC_DMABUSMOD_8XPBL` reader - 8 x Programmable Burst Length (PBL) Mode"]
pub type EMAC_DMABUSMOD_8XPBL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_8XPBL` writer - 8 x Programmable Burst Length (PBL) Mode"]
pub type EMAC_DMABUSMOD_8XPBL_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 24>;
#[doc = "Field `EMAC_DMABUSMOD_AAL` reader - Address Aligned Beats"]
pub type EMAC_DMABUSMOD_AAL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_AAL` writer - Address Aligned Beats"]
pub type EMAC_DMABUSMOD_AAL_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 25>;
#[doc = "Field `EMAC_DMABUSMOD_MB` reader - Mixed Burst"]
pub type EMAC_DMABUSMOD_MB_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_MB` writer - Mixed Burst"]
pub type EMAC_DMABUSMOD_MB_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 26>;
#[doc = "Field `EMAC_DMABUSMOD_TXPR` reader - Transmit Priority"]
pub type EMAC_DMABUSMOD_TXPR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_TXPR` writer - Transmit Priority"]
pub type EMAC_DMABUSMOD_TXPR_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 27>;
#[doc = "Field `EMAC_DMABUSMOD_RIB` reader - Rebuild Burst"]
pub type EMAC_DMABUSMOD_RIB_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMABUSMOD_RIB` writer - Rebuild Burst"]
pub type EMAC_DMABUSMOD_RIB_W<'a> = crate::BitWriter<'a, u32, DMABUSMOD_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - DMA Software Reset"]
    #[inline(always)]
    pub fn emac_dmabusmod_swr(&self) -> EMAC_DMABUSMOD_SWR_R {
        EMAC_DMABUSMOD_SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn emac_dmabusmod_da(&self) -> EMAC_DMABUSMOD_DA_R {
        EMAC_DMABUSMOD_DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_dsl(&self) -> EMAC_DMABUSMOD_DSL_R {
        EMAC_DMABUSMOD_DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn emac_dmabusmod_atds(&self) -> EMAC_DMABUSMOD_ATDS_R {
        EMAC_DMABUSMOD_ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_pbl(&self) -> EMAC_DMABUSMOD_PBL_R {
        EMAC_DMABUSMOD_PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn emac_dmabusmod_pr(&self) -> EMAC_DMABUSMOD_PR_R {
        EMAC_DMABUSMOD_PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_fb(&self) -> EMAC_DMABUSMOD_FB_R {
        EMAC_DMABUSMOD_FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - RX DMA Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_rpbl(&self) -> EMAC_DMABUSMOD_RPBL_R {
        EMAC_DMABUSMOD_RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_usp(&self) -> EMAC_DMABUSMOD_USP_R {
        EMAC_DMABUSMOD_USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 8 x Programmable Burst Length (PBL) Mode"]
    #[inline(always)]
    pub fn emac_dmabusmod_8xpbl(&self) -> EMAC_DMABUSMOD_8XPBL_R {
        EMAC_DMABUSMOD_8XPBL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn emac_dmabusmod_aal(&self) -> EMAC_DMABUSMOD_AAL_R {
        EMAC_DMABUSMOD_AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_mb(&self) -> EMAC_DMABUSMOD_MB_R {
        EMAC_DMABUSMOD_MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn emac_dmabusmod_txpr(&self) -> EMAC_DMABUSMOD_TXPR_R {
        EMAC_DMABUSMOD_TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Rebuild Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_rib(&self) -> EMAC_DMABUSMOD_RIB_R {
        EMAC_DMABUSMOD_RIB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Software Reset"]
    #[inline(always)]
    pub fn emac_dmabusmod_swr(&mut self) -> EMAC_DMABUSMOD_SWR_W {
        EMAC_DMABUSMOD_SWR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn emac_dmabusmod_da(&mut self) -> EMAC_DMABUSMOD_DA_W {
        EMAC_DMABUSMOD_DA_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_dsl(&mut self) -> EMAC_DMABUSMOD_DSL_W {
        EMAC_DMABUSMOD_DSL_W::new(self)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn emac_dmabusmod_atds(&mut self) -> EMAC_DMABUSMOD_ATDS_W {
        EMAC_DMABUSMOD_ATDS_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn emac_dmabusmod_pbl(&mut self) -> EMAC_DMABUSMOD_PBL_W {
        EMAC_DMABUSMOD_PBL_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn emac_dmabusmod_pr(&mut self) -> EMAC_DMABUSMOD_PR_W {
        EMAC_DMABUSMOD_PR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_fb(&mut self) -> EMAC_DMABUSMOD_FB_W {
        EMAC_DMABUSMOD_FB_W::new(self)
    }
    #[doc = "Bits 17:22 - RX DMA Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_rpbl(&mut self) -> EMAC_DMABUSMOD_RPBL_W {
        EMAC_DMABUSMOD_RPBL_W::new(self)
    }
    #[doc = "Bit 23 - Use Separate Programmable Burst Length (PBL)"]
    #[inline(always)]
    pub fn emac_dmabusmod_usp(&mut self) -> EMAC_DMABUSMOD_USP_W {
        EMAC_DMABUSMOD_USP_W::new(self)
    }
    #[doc = "Bit 24 - 8 x Programmable Burst Length (PBL) Mode"]
    #[inline(always)]
    pub fn emac_dmabusmod_8xpbl(&mut self) -> EMAC_DMABUSMOD_8XPBL_W {
        EMAC_DMABUSMOD_8XPBL_W::new(self)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn emac_dmabusmod_aal(&mut self) -> EMAC_DMABUSMOD_AAL_W {
        EMAC_DMABUSMOD_AAL_W::new(self)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_mb(&mut self) -> EMAC_DMABUSMOD_MB_W {
        EMAC_DMABUSMOD_MB_W::new(self)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn emac_dmabusmod_txpr(&mut self) -> EMAC_DMABUSMOD_TXPR_W {
        EMAC_DMABUSMOD_TXPR_W::new(self)
    }
    #[doc = "Bit 31 - Rebuild Burst"]
    #[inline(always)]
    pub fn emac_dmabusmod_rib(&mut self) -> EMAC_DMABUSMOD_RIB_W {
        EMAC_DMABUSMOD_RIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC DMA Bus Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabusmod](index.html) module"]
pub struct DMABUSMOD_SPEC;
impl crate::RegisterSpec for DMABUSMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabusmod::R](R) reader structure"]
impl crate::Readable for DMABUSMOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabusmod::W](W) writer structure"]
impl crate::Writable for DMABUSMOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABUSMOD to value 0"]
impl crate::Resettable for DMABUSMOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
