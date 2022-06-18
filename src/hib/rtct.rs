#[doc = "Register `RTCT` reader"]
pub struct R(crate::R<RTCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCT` writer"]
pub struct W(crate::W<RTCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCT_SPEC>;
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
impl From<crate::W<RTCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RTCT_TRIM` reader - RTC Trim Value"]
pub type HIB_RTCT_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIB_RTCT_TRIM` writer - RTC Trim Value"]
pub type HIB_RTCT_TRIM_W<'a> = crate::FieldWriter<'a, u32, RTCT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    pub fn hib_rtct_trim(&self) -> HIB_RTCT_TRIM_R {
        HIB_RTCT_TRIM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Trim Value"]
    #[inline(always)]
    pub fn hib_rtct_trim(&mut self) -> HIB_RTCT_TRIM_W {
        HIB_RTCT_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation RTC Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtct](index.html) module"]
pub struct RTCT_SPEC;
impl crate::RegisterSpec for RTCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtct::R](R) reader structure"]
impl crate::Readable for RTCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtct::W](W) writer structure"]
impl crate::Writable for RTCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCT to value 0"]
impl crate::Resettable for RTCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
