#[doc = "Register `DMAEV` reader"]
pub struct R(crate::R<DMAEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEV` writer"]
pub struct W(crate::W<DMAEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEV_SPEC>;
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
impl From<crate::W<DMAEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_DMAEV_TATODMAEN` reader - GPTM A Time-Out Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TATODMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_TATODMAEN` writer - GPTM A Time-Out Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TATODMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 0>;
#[doc = "Field `TIMER_DMAEV_CAMDMAEN` reader - GPTM A Capture Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CAMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_CAMDMAEN` writer - GPTM A Capture Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CAMDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 1>;
#[doc = "Field `TIMER_DMAEV_CAEDMAEN` reader - GPTM A Capture Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CAEDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_CAEDMAEN` writer - GPTM A Capture Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CAEDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 2>;
#[doc = "Field `TIMER_DMAEV_RTCDMAEN` reader - GPTM A RTC Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_RTCDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_RTCDMAEN` writer - GPTM A RTC Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_RTCDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 3>;
#[doc = "Field `TIMER_DMAEV_TAMDMAEN` reader - GPTM A Mode Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TAMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_TAMDMAEN` writer - GPTM A Mode Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TAMDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 4>;
#[doc = "Field `TIMER_DMAEV_TBTODMAEN` reader - GPTM B Time-Out Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TBTODMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_TBTODMAEN` writer - GPTM B Time-Out Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TBTODMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 8>;
#[doc = "Field `TIMER_DMAEV_CBMDMAEN` reader - GPTM B Capture Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CBMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_CBMDMAEN` writer - GPTM B Capture Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CBMDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 9>;
#[doc = "Field `TIMER_DMAEV_CBEDMAEN` reader - GPTM B Capture Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CBEDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_CBEDMAEN` writer - GPTM B Capture Event DMA Trigger Enable"]
pub type TIMER_DMAEV_CBEDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 10>;
#[doc = "Field `TIMER_DMAEV_TBMDMAEN` reader - GPTM B Mode Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TBMDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_DMAEV_TBMDMAEN` writer - GPTM B Mode Match Event DMA Trigger Enable"]
pub type TIMER_DMAEV_TBMDMAEN_W<'a> = crate::BitWriter<'a, u32, DMAEV_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tatodmaen(&self) -> TIMER_DMAEV_TATODMAEN_R {
        TIMER_DMAEV_TATODMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_camdmaen(&self) -> TIMER_DMAEV_CAMDMAEN_R {
        TIMER_DMAEV_CAMDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_caedmaen(&self) -> TIMER_DMAEV_CAEDMAEN_R {
        TIMER_DMAEV_CAEDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_rtcdmaen(&self) -> TIMER_DMAEV_RTCDMAEN_R {
        TIMER_DMAEV_RTCDMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tamdmaen(&self) -> TIMER_DMAEV_TAMDMAEN_R {
        TIMER_DMAEV_TAMDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbtodmaen(&self) -> TIMER_DMAEV_TBTODMAEN_R {
        TIMER_DMAEV_TBTODMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbmdmaen(&self) -> TIMER_DMAEV_CBMDMAEN_R {
        TIMER_DMAEV_CBMDMAEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbedmaen(&self) -> TIMER_DMAEV_CBEDMAEN_R {
        TIMER_DMAEV_CBEDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbmdmaen(&self) -> TIMER_DMAEV_TBMDMAEN_R {
        TIMER_DMAEV_TBMDMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tatodmaen(&mut self) -> TIMER_DMAEV_TATODMAEN_W {
        TIMER_DMAEV_TATODMAEN_W::new(self)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_camdmaen(&mut self) -> TIMER_DMAEV_CAMDMAEN_W {
        TIMER_DMAEV_CAMDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_caedmaen(&mut self) -> TIMER_DMAEV_CAEDMAEN_W {
        TIMER_DMAEV_CAEDMAEN_W::new(self)
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_rtcdmaen(&mut self) -> TIMER_DMAEV_RTCDMAEN_W {
        TIMER_DMAEV_RTCDMAEN_W::new(self)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tamdmaen(&mut self) -> TIMER_DMAEV_TAMDMAEN_W {
        TIMER_DMAEV_TAMDMAEN_W::new(self)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbtodmaen(&mut self) -> TIMER_DMAEV_TBTODMAEN_W {
        TIMER_DMAEV_TBTODMAEN_W::new(self)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbmdmaen(&mut self) -> TIMER_DMAEV_CBMDMAEN_W {
        TIMER_DMAEV_CBMDMAEN_W::new(self)
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbedmaen(&mut self) -> TIMER_DMAEV_CBEDMAEN_W {
        TIMER_DMAEV_CBEDMAEN_W::new(self)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbmdmaen(&mut self) -> TIMER_DMAEV_TBMDMAEN_W {
        TIMER_DMAEV_TBMDMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM DMA Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaev](index.html) module"]
pub struct DMAEV_SPEC;
impl crate::RegisterSpec for DMAEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaev::R](R) reader structure"]
impl crate::Readable for DMAEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaev::W](W) writer structure"]
impl crate::Writable for DMAEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAEV to value 0"]
impl crate::Resettable for DMAEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
