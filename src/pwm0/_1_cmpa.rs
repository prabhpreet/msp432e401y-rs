#[doc = "Register `_1_CMPA` reader"]
pub struct R(crate::R<_1_CMPA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_CMPA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_CMPA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_CMPA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1_CMPA` writer"]
pub struct W(crate::W<_1_CMPA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1_CMPA_SPEC>;
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
impl From<crate::W<_1_CMPA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1_CMPA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_1_CMPA_COMPA` reader - Comparator A Value"]
pub type PWM_1_CMPA_COMPA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PWM_1_CMPA_COMPA` writer - Comparator A Value"]
pub type PWM_1_CMPA_COMPA_W<'a> = crate::FieldWriter<'a, u32, _1_CMPA_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn pwm_1_cmpa_compa(&self) -> PWM_1_CMPA_COMPA_R {
        PWM_1_CMPA_COMPA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator A Value"]
    #[inline(always)]
    pub fn pwm_1_cmpa_compa(&mut self) -> PWM_1_CMPA_COMPA_W {
        PWM_1_CMPA_COMPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM1 Compare A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_cmpa](index.html) module"]
pub struct _1_CMPA_SPEC;
impl crate::RegisterSpec for _1_CMPA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_cmpa::R](R) reader structure"]
impl crate::Readable for _1_CMPA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1_cmpa::W](W) writer structure"]
impl crate::Writable for _1_CMPA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1_CMPA to value 0"]
impl crate::Resettable for _1_CMPA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
