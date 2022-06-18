#[doc = "Register `RTCC` reader"]
pub struct R(crate::R<RTCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCC` writer"]
pub struct W(crate::W<RTCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCC_SPEC>;
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
impl From<crate::W<RTCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RTCC` reader - RTC Counter"]
pub type HIB_RTCC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HIB_RTCC` writer - RTC Counter"]
pub type HIB_RTCC_W<'a> = crate::FieldWriter<'a, u32, RTCC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    pub fn hib_rtcc(&self) -> HIB_RTCC_R {
        HIB_RTCC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC Counter"]
    #[inline(always)]
    pub fn hib_rtcc(&mut self) -> HIB_RTCC_W {
        HIB_RTCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation RTC Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcc](index.html) module"]
pub struct RTCC_SPEC;
impl crate::RegisterSpec for RTCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcc::R](R) reader structure"]
impl crate::Readable for RTCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcc::W](W) writer structure"]
impl crate::Writable for RTCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCC to value 0"]
impl crate::Resettable for RTCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
