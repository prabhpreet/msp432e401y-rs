#[doc = "Register `_3_MINFLTPER` reader"]
pub struct R(crate::R<_3_MINFLTPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_3_MINFLTPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_3_MINFLTPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_3_MINFLTPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_3_MINFLTPER` writer"]
pub struct W(crate::W<_3_MINFLTPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_3_MINFLTPER_SPEC>;
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
impl From<crate::W<_3_MINFLTPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_3_MINFLTPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_3_MINFLTPER_MFP` reader - Minimum Fault Period"]
pub type PWM_3_MINFLTPER_MFP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PWM_3_MINFLTPER_MFP` writer - Minimum Fault Period"]
pub type PWM_3_MINFLTPER_MFP_W<'a> =
    crate::FieldWriter<'a, u32, _3_MINFLTPER_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_3_minfltper_mfp(&self) -> PWM_3_MINFLTPER_MFP_R {
        PWM_3_MINFLTPER_MFP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_3_minfltper_mfp(&mut self) -> PWM_3_MINFLTPER_MFP_W {
        PWM_3_MINFLTPER_MFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM3 Minimum Fault Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_3_minfltper](index.html) module"]
pub struct _3_MINFLTPER_SPEC;
impl crate::RegisterSpec for _3_MINFLTPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_3_minfltper::R](R) reader structure"]
impl crate::Readable for _3_MINFLTPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_3_minfltper::W](W) writer structure"]
impl crate::Writable for _3_MINFLTPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _3_MINFLTPER to value 0"]
impl crate::Resettable for _3_MINFLTPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
