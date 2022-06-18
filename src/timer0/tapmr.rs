#[doc = "Register `TAPMR` reader"]
pub struct R(crate::R<TAPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAPMR` writer"]
pub struct W(crate::W<TAPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAPMR_SPEC>;
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
impl From<crate::W<TAPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TAPMR_TAPSMR` reader - GPTM TimerA Prescale Match"]
pub type TIMER_TAPMR_TAPSMR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_TAPMR_TAPSMR` writer - GPTM TimerA Prescale Match"]
pub type TIMER_TAPMR_TAPSMR_W<'a> = crate::FieldWriter<'a, u32, TAPMR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmr(&self) -> TIMER_TAPMR_TAPSMR_R {
        TIMER_TAPMR_TAPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerA Prescale Match"]
    #[inline(always)]
    pub fn timer_tapmr_tapsmr(&mut self) -> TIMER_TAPMR_TAPSMR_W {
        TIMER_TAPMR_TAPSMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM TimerA Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tapmr](index.html) module"]
pub struct TAPMR_SPEC;
impl crate::RegisterSpec for TAPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tapmr::R](R) reader structure"]
impl crate::Readable for TAPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tapmr::W](W) writer structure"]
impl crate::Writable for TAPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAPMR to value 0"]
impl crate::Resettable for TAPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
