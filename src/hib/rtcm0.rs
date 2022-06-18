#[doc = "Register `RTCM0` reader"]
pub struct R(crate::R<RTCM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCM0` writer"]
pub struct W(crate::W<RTCM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCM0_SPEC>;
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
impl From<crate::W<RTCM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RTCM0` reader - RTC Match 0"]
pub type HIB_RTCM0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_RTCM0` writer - RTC Match 0"]
pub type HIB_RTCM0_W<'a> = crate::FieldWriter<'a, u32, RTCM0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    pub fn hib_rtcm0(&self) -> HIB_RTCM0_R {
        HIB_RTCM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Match 0"]
    #[inline(always)]
    pub fn hib_rtcm0(&mut self) -> HIB_RTCM0_W {
        HIB_RTCM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation RTC Match 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcm0](index.html) module"]
pub struct RTCM0_SPEC;
impl crate::RegisterSpec for RTCM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcm0::R](R) reader structure"]
impl crate::Readable for RTCM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcm0::W](W) writer structure"]
impl crate::Writable for RTCM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCM0 to value 0"]
impl crate::Resettable for RTCM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
