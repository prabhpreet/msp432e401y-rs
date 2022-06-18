#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_IMR_TATOIM` reader - GPTM Timer A Time-Out Interrupt Mask"]
pub type TIMER_IMR_TATOIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_TATOIM` writer - GPTM Timer A Time-Out Interrupt Mask"]
pub type TIMER_IMR_TATOIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 0>;
#[doc = "Field `TIMER_IMR_CAMIM` reader - GPTM Timer A Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CAMIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_CAMIM` writer - GPTM Timer A Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CAMIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 1>;
#[doc = "Field `TIMER_IMR_CAEIM` reader - GPTM Timer A Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CAEIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_CAEIM` writer - GPTM Timer A Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CAEIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 2>;
#[doc = "Field `TIMER_IMR_RTCIM` reader - GPTM RTC Interrupt Mask"]
pub type TIMER_IMR_RTCIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_RTCIM` writer - GPTM RTC Interrupt Mask"]
pub type TIMER_IMR_RTCIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 3>;
#[doc = "Field `TIMER_IMR_TAMIM` reader - GPTM Timer A Match Interrupt Mask"]
pub type TIMER_IMR_TAMIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_TAMIM` writer - GPTM Timer A Match Interrupt Mask"]
pub type TIMER_IMR_TAMIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 4>;
#[doc = "Field `TIMER_IMR_DMAAIM` reader - GPTM Timer A DMA Done Interrupt Mask"]
pub type TIMER_IMR_DMAAIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_DMAAIM` writer - GPTM Timer A DMA Done Interrupt Mask"]
pub type TIMER_IMR_DMAAIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 5>;
#[doc = "Field `TIMER_IMR_TBTOIM` reader - GPTM Timer B Time-Out Interrupt Mask"]
pub type TIMER_IMR_TBTOIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_TBTOIM` writer - GPTM Timer B Time-Out Interrupt Mask"]
pub type TIMER_IMR_TBTOIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 8>;
#[doc = "Field `TIMER_IMR_CBMIM` reader - GPTM Timer B Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CBMIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_CBMIM` writer - GPTM Timer B Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CBMIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 9>;
#[doc = "Field `TIMER_IMR_CBEIM` reader - GPTM Timer B Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CBEIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_CBEIM` writer - GPTM Timer B Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CBEIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 10>;
#[doc = "Field `TIMER_IMR_TBMIM` reader - GPTM Timer B Match Interrupt Mask"]
pub type TIMER_IMR_TBMIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_TBMIM` writer - GPTM Timer B Match Interrupt Mask"]
pub type TIMER_IMR_TBMIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 11>;
#[doc = "Field `TIMER_IMR_DMABIM` reader - GPTM Timer B DMA Done Interrupt Mask"]
pub type TIMER_IMR_DMABIM_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_IMR_DMABIM` writer - GPTM Timer B DMA Done Interrupt Mask"]
pub type TIMER_IMR_DMABIM_W<'a> = crate::BitWriter<'a, u32, IMR_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&self) -> TIMER_IMR_TATOIM_R {
        TIMER_IMR_TATOIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&self) -> TIMER_IMR_CAMIM_R {
        TIMER_IMR_CAMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&self) -> TIMER_IMR_CAEIM_R {
        TIMER_IMR_CAEIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&self) -> TIMER_IMR_RTCIM_R {
        TIMER_IMR_RTCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tamim(&self) -> TIMER_IMR_TAMIM_R {
        TIMER_IMR_TAMIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmaaim(&self) -> TIMER_IMR_DMAAIM_R {
        TIMER_IMR_DMAAIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&self) -> TIMER_IMR_TBTOIM_R {
        TIMER_IMR_TBTOIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&self) -> TIMER_IMR_CBMIM_R {
        TIMER_IMR_CBMIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&self) -> TIMER_IMR_CBEIM_R {
        TIMER_IMR_CBEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbmim(&self) -> TIMER_IMR_TBMIM_R {
        TIMER_IMR_TBMIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmabim(&self) -> TIMER_IMR_DMABIM_R {
        TIMER_IMR_DMABIM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&mut self) -> TIMER_IMR_TATOIM_W {
        TIMER_IMR_TATOIM_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&mut self) -> TIMER_IMR_CAMIM_W {
        TIMER_IMR_CAMIM_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&mut self) -> TIMER_IMR_CAEIM_W {
        TIMER_IMR_CAEIM_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&mut self) -> TIMER_IMR_RTCIM_W {
        TIMER_IMR_RTCIM_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tamim(&mut self) -> TIMER_IMR_TAMIM_W {
        TIMER_IMR_TAMIM_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmaaim(&mut self) -> TIMER_IMR_DMAAIM_W {
        TIMER_IMR_DMAAIM_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&mut self) -> TIMER_IMR_TBTOIM_W {
        TIMER_IMR_TBTOIM_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&mut self) -> TIMER_IMR_CBMIM_W {
        TIMER_IMR_CBMIM_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&mut self) -> TIMER_IMR_CBEIM_W {
        TIMER_IMR_CBEIM_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbmim(&mut self) -> TIMER_IMR_TBMIM_W {
        TIMER_IMR_TBMIM_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmabim(&mut self) -> TIMER_IMR_DMABIM_W {
        TIMER_IMR_DMABIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
