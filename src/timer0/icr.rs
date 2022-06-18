#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_ICR_TATOCINT` writer - GPTM Timer A Time-Out Raw Interrupt"]
pub type TIMER_ICR_TATOCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 0>;
#[doc = "Field `TIMER_ICR_CAMCINT` writer - GPTM Timer A Capture Mode Match Interrupt Clear"]
pub type TIMER_ICR_CAMCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 1>;
#[doc = "Field `TIMER_ICR_CAECINT` writer - GPTM Timer A Capture Mode Event Interrupt Clear"]
pub type TIMER_ICR_CAECINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 2>;
#[doc = "Field `TIMER_ICR_RTCCINT` writer - GPTM RTC Interrupt Clear"]
pub type TIMER_ICR_RTCCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 3>;
#[doc = "Field `TIMER_ICR_TAMCINT` writer - GPTM Timer A Match Interrupt Clear"]
pub type TIMER_ICR_TAMCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 4>;
#[doc = "Field `TIMER_ICR_DMAAINT` writer - GPTM Timer A DMA Done Interrupt Clear"]
pub type TIMER_ICR_DMAAINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 5>;
#[doc = "Field `TIMER_ICR_TBTOCINT` writer - GPTM Timer B Time-Out Interrupt Clear"]
pub type TIMER_ICR_TBTOCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 8>;
#[doc = "Field `TIMER_ICR_CBMCINT` writer - GPTM Timer B Capture Mode Match Interrupt Clear"]
pub type TIMER_ICR_CBMCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 9>;
#[doc = "Field `TIMER_ICR_CBECINT` writer - GPTM Timer B Capture Mode Event Interrupt Clear"]
pub type TIMER_ICR_CBECINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 10>;
#[doc = "Field `TIMER_ICR_TBMCINT` writer - GPTM Timer B Match Interrupt Clear"]
pub type TIMER_ICR_TBMCINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 11>;
#[doc = "Field `TIMER_ICR_DMABINT` writer - GPTM Timer B DMA Done Interrupt Clear"]
pub type TIMER_ICR_DMABINT_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 13>;
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_icr_tatocint(&mut self) -> TIMER_ICR_TATOCINT_W {
        TIMER_ICR_TATOCINT_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_camcint(&mut self) -> TIMER_ICR_CAMCINT_W {
        TIMER_ICR_CAMCINT_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_caecint(&mut self) -> TIMER_ICR_CAECINT_W {
        TIMER_ICR_CAECINT_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_rtccint(&mut self) -> TIMER_ICR_RTCCINT_W {
        TIMER_ICR_RTCCINT_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tamcint(&mut self) -> TIMER_ICR_TAMCINT_W {
        TIMER_ICR_TAMCINT_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_dmaaint(&mut self) -> TIMER_ICR_DMAAINT_W {
        TIMER_ICR_DMAAINT_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tbtocint(&mut self) -> TIMER_ICR_TBTOCINT_W {
        TIMER_ICR_TBTOCINT_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbmcint(&mut self) -> TIMER_ICR_CBMCINT_W {
        TIMER_ICR_CBMCINT_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbecint(&mut self) -> TIMER_ICR_CBECINT_W {
        TIMER_ICR_CBECINT_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tbmcint(&mut self) -> TIMER_ICR_TBMCINT_W {
        TIMER_ICR_TBMCINT_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_dmabint(&mut self) -> TIMER_ICR_DMABINT_W {
        TIMER_ICR_DMABINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
