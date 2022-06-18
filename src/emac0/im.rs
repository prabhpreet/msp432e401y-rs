#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_IM_PMT` reader - PMT Interrupt Mask"]
pub type EMAC_IM_PMT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_IM_PMT` writer - PMT Interrupt Mask"]
pub type EMAC_IM_PMT_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `EMAC_IM_TSI` reader - Timestamp Interrupt Mask"]
pub type EMAC_IM_TSI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_IM_TSI` writer - Timestamp Interrupt Mask"]
pub type EMAC_IM_TSI_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 9>;
#[doc = "Field `EMAC_IM_LPII` reader - LPI Interrupt Mask"]
pub type EMAC_IM_LPII_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_IM_LPII` writer - LPI Interrupt Mask"]
pub type EMAC_IM_LPII_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_pmt(&self) -> EMAC_IM_PMT_R {
        EMAC_IM_PMT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_tsi(&self) -> EMAC_IM_TSI_R {
        EMAC_IM_TSI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_lpii(&self) -> EMAC_IM_LPII_R {
        EMAC_IM_LPII_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_pmt(&mut self) -> EMAC_IM_PMT_W {
        EMAC_IM_PMT_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_tsi(&mut self) -> EMAC_IM_TSI_W {
        EMAC_IM_TSI_W::new(self)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask"]
    #[inline(always)]
    pub fn emac_im_lpii(&mut self) -> EMAC_IM_LPII_W {
        EMAC_IM_LPII_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
