#[doc = "Register `_1_DBRISE` reader"]
pub struct R(crate::R<_1_DBRISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_DBRISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_DBRISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_DBRISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1_DBRISE` writer"]
pub struct W(crate::W<_1_DBRISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1_DBRISE_SPEC>;
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
impl From<crate::W<_1_DBRISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1_DBRISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_1_DBRISE_RISEDELAY` reader - Dead-Band Rise Delay"]
pub type PWM_1_DBRISE_RISEDELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PWM_1_DBRISE_RISEDELAY` writer - Dead-Band Rise Delay"]
pub type PWM_1_DBRISE_RISEDELAY_W<'a> =
    crate::FieldWriter<'a, u32, _1_DBRISE_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    pub fn pwm_1_dbrise_risedelay(&self) -> PWM_1_DBRISE_RISEDELAY_R {
        PWM_1_DBRISE_RISEDELAY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    pub fn pwm_1_dbrise_risedelay(&mut self) -> PWM_1_DBRISE_RISEDELAY_W {
        PWM_1_DBRISE_RISEDELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM1 Dead-Band Rising-Edge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_dbrise](index.html) module"]
pub struct _1_DBRISE_SPEC;
impl crate::RegisterSpec for _1_DBRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_dbrise::R](R) reader structure"]
impl crate::Readable for _1_DBRISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1_dbrise::W](W) writer structure"]
impl crate::Writable for _1_DBRISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _1_DBRISE to value 0"]
impl crate::Resettable for _1_DBRISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
