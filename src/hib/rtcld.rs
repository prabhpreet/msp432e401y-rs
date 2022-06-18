#[doc = "Register `RTCLD` reader"]
pub struct R(crate::R<RTCLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCLD` writer"]
pub struct W(crate::W<RTCLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCLD_SPEC>;
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
impl From<crate::W<RTCLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RTCLD` reader - RTC Load"]
pub type HIB_RTCLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_RTCLD` writer - RTC Load"]
pub type HIB_RTCLD_W<'a> = crate::FieldWriter<'a, u32, RTCLD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    pub fn hib_rtcld(&self) -> HIB_RTCLD_R {
        HIB_RTCLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Load"]
    #[inline(always)]
    pub fn hib_rtcld(&mut self) -> HIB_RTCLD_W {
        HIB_RTCLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation RTC Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcld](index.html) module"]
pub struct RTCLD_SPEC;
impl crate::RegisterSpec for RTCLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcld::R](R) reader structure"]
impl crate::Readable for RTCLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcld::W](W) writer structure"]
impl crate::Writable for RTCLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCLD to value 0"]
impl crate::Resettable for RTCLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
