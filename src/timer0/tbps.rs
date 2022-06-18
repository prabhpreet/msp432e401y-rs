#[doc = "Register `TBPS` reader"]
pub struct R(crate::R<TBPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPS` writer"]
pub struct W(crate::W<TBPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPS_SPEC>;
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
impl From<crate::W<TBPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TBPS_PSS` reader - GPTM Timer A Prescaler Value"]
pub type TIMER_TBPS_PSS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER_TBPS_PSS` writer - GPTM Timer A Prescaler Value"]
pub type TIMER_TBPS_PSS_W<'a> = crate::FieldWriter<'a, u32, TBPS_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn timer_tbps_pss(&self) -> TIMER_TBPS_PSS_R {
        TIMER_TBPS_PSS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer A Prescaler Value"]
    #[inline(always)]
    pub fn timer_tbps_pss(&mut self) -> TIMER_TBPS_PSS_W {
        TIMER_TBPS_PSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B Prescale Snapshot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbps](index.html) module"]
pub struct TBPS_SPEC;
impl crate::RegisterSpec for TBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbps::R](R) reader structure"]
impl crate::Readable for TBPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbps::W](W) writer structure"]
impl crate::Writable for TBPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TBPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
