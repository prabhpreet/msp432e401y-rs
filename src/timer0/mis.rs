#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_MIS_TATOMIS` reader - GPTM Timer A Time-Out Masked Interrupt"]
pub type TIMER_MIS_TATOMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_TATOMIS` writer - GPTM Timer A Time-Out Masked Interrupt"]
pub type TIMER_MIS_TATOMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `TIMER_MIS_CAMMIS` reader - GPTM Timer A Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CAMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_CAMMIS` writer - GPTM Timer A Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CAMMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 1>;
#[doc = "Field `TIMER_MIS_CAEMIS` reader - GPTM Timer A Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CAEMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_CAEMIS` writer - GPTM Timer A Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CAEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `TIMER_MIS_RTCMIS` reader - GPTM RTC Masked Interrupt"]
pub type TIMER_MIS_RTCMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_RTCMIS` writer - GPTM RTC Masked Interrupt"]
pub type TIMER_MIS_RTCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `TIMER_MIS_TAMMIS` reader - GPTM Timer A Match Masked Interrupt"]
pub type TIMER_MIS_TAMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_TAMMIS` writer - GPTM Timer A Match Masked Interrupt"]
pub type TIMER_MIS_TAMMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
#[doc = "Field `TIMER_MIS_DMAAMIS` reader - GPTM Timer A DMA Done Masked Interrupt"]
pub type TIMER_MIS_DMAAMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_DMAAMIS` writer - GPTM Timer A DMA Done Masked Interrupt"]
pub type TIMER_MIS_DMAAMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 5>;
#[doc = "Field `TIMER_MIS_TBTOMIS` reader - GPTM Timer B Time-Out Masked Interrupt"]
pub type TIMER_MIS_TBTOMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_TBTOMIS` writer - GPTM Timer B Time-Out Masked Interrupt"]
pub type TIMER_MIS_TBTOMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 8>;
#[doc = "Field `TIMER_MIS_CBMMIS` reader - GPTM Timer B Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CBMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_CBMMIS` writer - GPTM Timer B Capture Mode Match Masked Interrupt"]
pub type TIMER_MIS_CBMMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 9>;
#[doc = "Field `TIMER_MIS_CBEMIS` reader - GPTM Timer B Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CBEMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_CBEMIS` writer - GPTM Timer B Capture Mode Event Masked Interrupt"]
pub type TIMER_MIS_CBEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 10>;
#[doc = "Field `TIMER_MIS_TBMMIS` reader - GPTM Timer B Match Masked Interrupt"]
pub type TIMER_MIS_TBMMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_TBMMIS` writer - GPTM Timer B Match Masked Interrupt"]
pub type TIMER_MIS_TBMMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 11>;
#[doc = "Field `TIMER_MIS_DMABMIS` reader - GPTM Timer B DMA Done Masked Interrupt"]
pub type TIMER_MIS_DMABMIS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_MIS_DMABMIS` writer - GPTM Timer B DMA Done Masked Interrupt"]
pub type TIMER_MIS_DMABMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&self) -> TIMER_MIS_TATOMIS_R {
        TIMER_MIS_TATOMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&self) -> TIMER_MIS_CAMMIS_R {
        TIMER_MIS_CAMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&self) -> TIMER_MIS_CAEMIS_R {
        TIMER_MIS_CAEMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&self) -> TIMER_MIS_RTCMIS_R {
        TIMER_MIS_RTCMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tammis(&self) -> TIMER_MIS_TAMMIS_R {
        TIMER_MIS_TAMMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmaamis(&self) -> TIMER_MIS_DMAAMIS_R {
        TIMER_MIS_DMAAMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&self) -> TIMER_MIS_TBTOMIS_R {
        TIMER_MIS_TBTOMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&self) -> TIMER_MIS_CBMMIS_R {
        TIMER_MIS_CBMMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&self) -> TIMER_MIS_CBEMIS_R {
        TIMER_MIS_CBEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbmmis(&self) -> TIMER_MIS_TBMMIS_R {
        TIMER_MIS_TBMMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmabmis(&self) -> TIMER_MIS_DMABMIS_R {
        TIMER_MIS_DMABMIS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&mut self) -> TIMER_MIS_TATOMIS_W {
        TIMER_MIS_TATOMIS_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&mut self) -> TIMER_MIS_CAMMIS_W {
        TIMER_MIS_CAMMIS_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&mut self) -> TIMER_MIS_CAEMIS_W {
        TIMER_MIS_CAEMIS_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&mut self) -> TIMER_MIS_RTCMIS_W {
        TIMER_MIS_RTCMIS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tammis(&mut self) -> TIMER_MIS_TAMMIS_W {
        TIMER_MIS_TAMMIS_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmaamis(&mut self) -> TIMER_MIS_DMAAMIS_W {
        TIMER_MIS_DMAAMIS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&mut self) -> TIMER_MIS_TBTOMIS_W {
        TIMER_MIS_TBTOMIS_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&mut self) -> TIMER_MIS_CBMMIS_W {
        TIMER_MIS_CBMMIS_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&mut self) -> TIMER_MIS_CBEMIS_W {
        TIMER_MIS_CBEMIS_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbmmis(&mut self) -> TIMER_MIS_TBMMIS_W {
        TIMER_MIS_TBMMIS_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmabmis(&mut self) -> TIMER_MIS_DMABMIS_W {
        TIMER_MIS_DMABMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
