#[doc = "Register `TBPR` reader"]
pub struct R(crate::R<TBPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPR` writer"]
pub struct W(crate::W<TBPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPR_SPEC>;
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
impl From<crate::W<TBPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TBPR_TBPSR` reader - GPTM Timer B Prescale"]
pub type TIMER_TBPR_TBPSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_TBPR_TBPSR` writer - GPTM Timer B Prescale"]
pub type TIMER_TBPR_TBPSR_W<'a> = crate::FieldWriter<'a, u32, TBPR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    pub fn timer_tbpr_tbpsr(&self) -> TIMER_TBPR_TBPSR_R {
        TIMER_TBPR_TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B Prescale"]
    #[inline(always)]
    pub fn timer_tbpr_tbpsr(&mut self) -> TIMER_TBPR_TBPSR_W {
        TIMER_TBPR_TBPSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpr](index.html) module"]
pub struct TBPR_SPEC;
impl crate::RegisterSpec for TBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpr::R](R) reader structure"]
impl crate::Readable for TBPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpr::W](W) writer structure"]
impl crate::Writable for TBPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TBPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
