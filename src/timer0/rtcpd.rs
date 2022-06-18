#[doc = "Register `RTCPD` reader"]
pub struct R(crate::R<RTCPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPD` writer"]
pub struct W(crate::W<RTCPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPD_SPEC>;
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
impl From<crate::W<RTCPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_RTCPD_RTCPD` reader - RTC Predivide Counter Value"]
pub type TIMER_RTCPD_RTCPD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_RTCPD_RTCPD` writer - RTC Predivide Counter Value"]
pub type TIMER_RTCPD_RTCPD_W<'a> = crate::FieldWriter<'a, u32, RTCPD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn timer_rtcpd_rtcpd(&self) -> TIMER_RTCPD_RTCPD_R {
        TIMER_RTCPD_RTCPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn timer_rtcpd_rtcpd(&mut self) -> TIMER_RTCPD_RTCPD_W {
        TIMER_RTCPD_RTCPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM RTC Predivide\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcpd](index.html) module"]
pub struct RTCPD_SPEC;
impl crate::RegisterSpec for RTCPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcpd::R](R) reader structure"]
impl crate::Readable for RTCPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcpd::W](W) writer structure"]
impl crate::Writable for RTCPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPD to value 0"]
impl crate::Resettable for RTCPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
