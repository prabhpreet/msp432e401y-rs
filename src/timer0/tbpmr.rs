#[doc = "Register `TBPMR` reader"]
pub struct R(crate::R<TBPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPMR` writer"]
pub struct W(crate::W<TBPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPMR_SPEC>;
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
impl From<crate::W<TBPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TBPMR_TBPSMR` reader - GPTM TimerB Prescale Match"]
pub type TIMER_TBPMR_TBPSMR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_TBPMR_TBPSMR` writer - GPTM TimerB Prescale Match"]
pub type TIMER_TBPMR_TBPSMR_W<'a> = crate::FieldWriter<'a, u32, TBPMR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmr(&self) -> TIMER_TBPMR_TBPSMR_R {
        TIMER_TBPMR_TBPSMR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM TimerB Prescale Match"]
    #[inline(always)]
    pub fn timer_tbpmr_tbpsmr(&mut self) -> TIMER_TBPMR_TBPSMR_W {
        TIMER_TBPMR_TBPSMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM TimerB Prescale Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpmr](index.html) module"]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpmr::R](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
