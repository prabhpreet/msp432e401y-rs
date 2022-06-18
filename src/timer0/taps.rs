#[doc = "Register `TAPS` reader"]
pub struct R(crate::R<TAPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPS` writer"]
pub struct W(crate::W<TAPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPS_SPEC>;
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
impl From<crate::W<TAPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TAPS_PSS` reader - GPTM Timer A Prescaler Snapshot"]
pub type TIMER_TAPS_PSS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_TAPS_PSS` writer - GPTM Timer A Prescaler Snapshot"]
pub type TIMER_TAPS_PSS_W<'a> = crate::FieldWriter<'a, u32, TAPS_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn timer_taps_pss(&self) -> TIMER_TAPS_PSS_R {
        TIMER_TAPS_PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Snapshot"]
    #[inline(always)]
    pub fn timer_taps_pss(&mut self) -> TIMER_TAPS_PSS_W {
        TIMER_TAPS_PSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer A Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taps](index.html) module"]
pub struct TAPS_SPEC;
impl crate::RegisterSpec for TAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taps::R](R) reader structure"]
impl crate::Readable for TAPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taps::W](W) writer structure"]
impl crate::Writable for TAPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TAPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
