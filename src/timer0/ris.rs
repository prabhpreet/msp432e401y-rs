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
#[doc = "Field `TIMER_RIS_TATORIS` reader - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_RIS_TATORIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_TATORIS` writer - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_RIS_TATORIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `TIMER_RIS_CAMRIS` reader - GPTM Timer A Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_CAMRIS` writer - GPTM Timer A Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CAMRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `TIMER_RIS_CAERIS` reader - GPTM Timer A Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CAERIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_CAERIS` writer - GPTM Timer A Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CAERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `TIMER_RIS_RTCRIS` reader - GPTM RTC Raw Interrupt"]
pub type TIMER_RIS_RTCRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_RTCRIS` writer - GPTM RTC Raw Interrupt"]
pub type TIMER_RIS_RTCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `TIMER_RIS_TAMRIS` reader - GPTM Timer A Match Raw Interrupt"]
pub type TIMER_RIS_TAMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_TAMRIS` writer - GPTM Timer A Match Raw Interrupt"]
pub type TIMER_RIS_TAMRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `TIMER_RIS_DMAARIS` reader - GPTM Timer A DMA Done Raw Interrupt Status"]
pub type TIMER_RIS_DMAARIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_DMAARIS` writer - GPTM Timer A DMA Done Raw Interrupt Status"]
pub type TIMER_RIS_DMAARIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
#[doc = "Field `TIMER_RIS_TBTORIS` reader - GPTM Timer B Time-Out Raw Interrupt"]
pub type TIMER_RIS_TBTORIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_TBTORIS` writer - GPTM Timer B Time-Out Raw Interrupt"]
pub type TIMER_RIS_TBTORIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 8>;
#[doc = "Field `TIMER_RIS_CBMRIS` reader - GPTM Timer B Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_CBMRIS` writer - GPTM Timer B Capture Mode Match Raw Interrupt"]
pub type TIMER_RIS_CBMRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 9>;
#[doc = "Field `TIMER_RIS_CBERIS` reader - GPTM Timer B Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CBERIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_CBERIS` writer - GPTM Timer B Capture Mode Event Raw Interrupt"]
pub type TIMER_RIS_CBERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 10>;
#[doc = "Field `TIMER_RIS_TBMRIS` reader - GPTM Timer B Match Raw Interrupt"]
pub type TIMER_RIS_TBMRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_TBMRIS` writer - GPTM Timer B Match Raw Interrupt"]
pub type TIMER_RIS_TBMRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 11>;
#[doc = "Field `TIMER_RIS_DMABRIS` reader - GPTM Timer B DMA Done Raw Interrupt Status"]
pub type TIMER_RIS_DMABRIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RIS_DMABRIS` writer - GPTM Timer B DMA Done Raw Interrupt Status"]
pub type TIMER_RIS_DMABRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&self) -> TIMER_RIS_TATORIS_R {
        TIMER_RIS_TATORIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&self) -> TIMER_RIS_CAMRIS_R {
        TIMER_RIS_CAMRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&self) -> TIMER_RIS_CAERIS_R {
        TIMER_RIS_CAERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&self) -> TIMER_RIS_RTCRIS_R {
        TIMER_RIS_RTCRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tamris(&self) -> TIMER_RIS_TAMRIS_R {
        TIMER_RIS_TAMRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmaaris(&self) -> TIMER_RIS_DMAARIS_R {
        TIMER_RIS_DMAARIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&self) -> TIMER_RIS_TBTORIS_R {
        TIMER_RIS_TBTORIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&self) -> TIMER_RIS_CBMRIS_R {
        TIMER_RIS_CBMRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&self) -> TIMER_RIS_CBERIS_R {
        TIMER_RIS_CBERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbmris(&self) -> TIMER_RIS_TBMRIS_R {
        TIMER_RIS_TBMRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmabris(&self) -> TIMER_RIS_DMABRIS_R {
        TIMER_RIS_DMABRIS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&mut self) -> TIMER_RIS_TATORIS_W {
        TIMER_RIS_TATORIS_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&mut self) -> TIMER_RIS_CAMRIS_W {
        TIMER_RIS_CAMRIS_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&mut self) -> TIMER_RIS_CAERIS_W {
        TIMER_RIS_CAERIS_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&mut self) -> TIMER_RIS_RTCRIS_W {
        TIMER_RIS_RTCRIS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tamris(&mut self) -> TIMER_RIS_TAMRIS_W {
        TIMER_RIS_TAMRIS_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmaaris(&mut self) -> TIMER_RIS_DMAARIS_W {
        TIMER_RIS_DMAARIS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&mut self) -> TIMER_RIS_TBTORIS_W {
        TIMER_RIS_TBTORIS_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&mut self) -> TIMER_RIS_CBMRIS_W {
        TIMER_RIS_CBMRIS_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&mut self) -> TIMER_RIS_CBERIS_W {
        TIMER_RIS_CBERIS_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbmris(&mut self) -> TIMER_RIS_TBMRIS_W {
        TIMER_RIS_TBMRIS_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmabris(&mut self) -> TIMER_RIS_DMABRIS_W {
        TIMER_RIS_DMABRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
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
