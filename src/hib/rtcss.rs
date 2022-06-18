#[doc = "Register `RTCSS` reader"]
pub struct R(crate::R<RTCSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSS` writer"]
pub struct W(crate::W<RTCSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSS_SPEC>;
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
impl From<crate::W<RTCSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RTCSS_RTCSSC` reader - RTC Sub Seconds Count"]
pub type HIB_RTCSS_RTCSSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIB_RTCSS_RTCSSC` writer - RTC Sub Seconds Count"]
pub type HIB_RTCSS_RTCSSC_W<'a> = crate::FieldWriter<'a, u32, RTCSS_SPEC, u16, u16, 15, 0>;
#[doc = "Field `HIB_RTCSS_RTCSSM` reader - RTC Sub Seconds Match"]
pub type HIB_RTCSS_RTCSSM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIB_RTCSS_RTCSSM` writer - RTC Sub Seconds Match"]
pub type HIB_RTCSS_RTCSSM_W<'a> = crate::FieldWriter<'a, u32, RTCSS_SPEC, u16, u16, 15, 16>;
impl R {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssc(&self) -> HIB_RTCSS_RTCSSC_R {
        HIB_RTCSS_RTCSSC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssm(&self) -> HIB_RTCSS_RTCSSM_R {
        HIB_RTCSS_RTCSSM_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssc(&mut self) -> HIB_RTCSS_RTCSSC_W {
        HIB_RTCSS_RTCSSC_W::new(self)
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssm(&mut self) -> HIB_RTCSS_RTCSSM_W {
        HIB_RTCSS_RTCSSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation RTC Sub Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcss](index.html) module"]
pub struct RTCSS_SPEC;
impl crate::RegisterSpec for RTCSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcss::R](R) reader structure"]
impl crate::Readable for RTCSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcss::W](W) writer structure"]
impl crate::Writable for RTCSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCSS to value 0"]
impl crate::Resettable for RTCSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
