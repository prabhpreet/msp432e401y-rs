#[doc = "Register `ADCEV` reader"]
pub struct R(crate::R<ADCEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCEV` writer"]
pub struct W(crate::W<ADCEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCEV_SPEC>;
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
impl From<crate::W<ADCEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_ADCEV_TATOADCEN` reader - GPTM A Time-Out Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TATOADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_TATOADCEN` writer - GPTM A Time-Out Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TATOADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 0>;
#[doc = "Field `TIMER_ADCEV_CAMADCEN` reader - GPTM A Capture Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CAMADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_CAMADCEN` writer - GPTM A Capture Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CAMADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 1>;
#[doc = "Field `TIMER_ADCEV_CAEADCEN` reader - GPTM A Capture Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CAEADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_CAEADCEN` writer - GPTM A Capture Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CAEADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 2>;
#[doc = "Field `TIMER_ADCEV_RTCADCEN` reader - GPTM RTC Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_RTCADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_RTCADCEN` writer - GPTM RTC Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_RTCADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 3>;
#[doc = "Field `TIMER_ADCEV_TAMADCEN` reader - GPTM A Mode Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TAMADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_TAMADCEN` writer - GPTM A Mode Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TAMADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 4>;
#[doc = "Field `TIMER_ADCEV_TBTOADCEN` reader - GPTM B Time-Out Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TBTOADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_TBTOADCEN` writer - GPTM B Time-Out Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TBTOADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 8>;
#[doc = "Field `TIMER_ADCEV_CBMADCEN` reader - GPTM B Capture Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CBMADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_CBMADCEN` writer - GPTM B Capture Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CBMADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 9>;
#[doc = "Field `TIMER_ADCEV_CBEADCEN` reader - GPTM B Capture Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CBEADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_CBEADCEN` writer - GPTM B Capture Event ADC Trigger Enable"]
pub type TIMER_ADCEV_CBEADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 10>;
#[doc = "Field `TIMER_ADCEV_TBMADCEN` reader - GPTM B Mode Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TBMADCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ADCEV_TBMADCEN` writer - GPTM B Mode Match Event ADC Trigger Enable"]
pub type TIMER_ADCEV_TBMADCEN_W<'a> = crate::BitWriter<'a, u32, ADCEV_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tatoadcen(&self) -> TIMER_ADCEV_TATOADCEN_R {
        TIMER_ADCEV_TATOADCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_camadcen(&self) -> TIMER_ADCEV_CAMADCEN_R {
        TIMER_ADCEV_CAMADCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_caeadcen(&self) -> TIMER_ADCEV_CAEADCEN_R {
        TIMER_ADCEV_CAEADCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_rtcadcen(&self) -> TIMER_ADCEV_RTCADCEN_R {
        TIMER_ADCEV_RTCADCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tamadcen(&self) -> TIMER_ADCEV_TAMADCEN_R {
        TIMER_ADCEV_TAMADCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbtoadcen(&self) -> TIMER_ADCEV_TBTOADCEN_R {
        TIMER_ADCEV_TBTOADCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbmadcen(&self) -> TIMER_ADCEV_CBMADCEN_R {
        TIMER_ADCEV_CBMADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbeadcen(&self) -> TIMER_ADCEV_CBEADCEN_R {
        TIMER_ADCEV_CBEADCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbmadcen(&self) -> TIMER_ADCEV_TBMADCEN_R {
        TIMER_ADCEV_TBMADCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tatoadcen(&mut self) -> TIMER_ADCEV_TATOADCEN_W {
        TIMER_ADCEV_TATOADCEN_W::new(self)
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_camadcen(&mut self) -> TIMER_ADCEV_CAMADCEN_W {
        TIMER_ADCEV_CAMADCEN_W::new(self)
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_caeadcen(&mut self) -> TIMER_ADCEV_CAEADCEN_W {
        TIMER_ADCEV_CAEADCEN_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_rtcadcen(&mut self) -> TIMER_ADCEV_RTCADCEN_W {
        TIMER_ADCEV_RTCADCEN_W::new(self)
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tamadcen(&mut self) -> TIMER_ADCEV_TAMADCEN_W {
        TIMER_ADCEV_TAMADCEN_W::new(self)
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbtoadcen(&mut self) -> TIMER_ADCEV_TBTOADCEN_W {
        TIMER_ADCEV_TBTOADCEN_W::new(self)
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbmadcen(&mut self) -> TIMER_ADCEV_CBMADCEN_W {
        TIMER_ADCEV_CBMADCEN_W::new(self)
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbeadcen(&mut self) -> TIMER_ADCEV_CBEADCEN_W {
        TIMER_ADCEV_CBEADCEN_W::new(self)
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbmadcen(&mut self) -> TIMER_ADCEV_TBMADCEN_W {
        TIMER_ADCEV_TBMADCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM ADC Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcev](index.html) module"]
pub struct ADCEV_SPEC;
impl crate::RegisterSpec for ADCEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcev::R](R) reader structure"]
impl crate::Readable for ADCEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcev::W](W) writer structure"]
impl crate::Writable for ADCEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCEV to value 0"]
impl crate::Resettable for ADCEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
