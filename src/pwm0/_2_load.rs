#[doc = "Register `_2_LOAD` reader"]
pub struct R(crate::R<_2_LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_2_LOAD` writer"]
pub struct W(crate::W<_2_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_2_LOAD_SPEC>;
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
impl From<crate::W<_2_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_2_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_2_LOAD_LOAD` reader - Counter Load Value"]
pub type PWM_2_LOAD_LOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PWM_2_LOAD_LOAD` writer - Counter Load Value"]
pub type PWM_2_LOAD_LOAD_W<'a> = crate::FieldWriter<'a, u32, _2_LOAD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Counter Load Value"]
    #[inline(always)]
    pub fn pwm_2_load_load(&self) -> PWM_2_LOAD_LOAD_R {
        PWM_2_LOAD_LOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Load Value"]
    #[inline(always)]
    pub fn pwm_2_load_load(&mut self) -> PWM_2_LOAD_LOAD_W {
        PWM_2_LOAD_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM2 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_load](index.html) module"]
pub struct _2_LOAD_SPEC;
impl crate::RegisterSpec for _2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_load::R](R) reader structure"]
impl crate::Readable for _2_LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_2_load::W](W) writer structure"]
impl crate::Writable for _2_LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _2_LOAD to value 0"]
impl crate::Resettable for _2_LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
