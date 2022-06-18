#[doc = "Register `_3_DBCTL` reader"]
pub struct R(crate::R<_3_DBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_3_DBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_3_DBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_3_DBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_3_DBCTL` writer"]
pub struct W(crate::W<_3_DBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_3_DBCTL_SPEC>;
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
impl From<crate::W<_3_DBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_3_DBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_3_DBCTL_ENABLE` reader - Dead-Band Generator Enable"]
pub type PWM_3_DBCTL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PWM_3_DBCTL_ENABLE` writer - Dead-Band Generator Enable"]
pub type PWM_3_DBCTL_ENABLE_W<'a> = crate::BitWriter<'a, u32, _3_DBCTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    pub fn pwm_3_dbctl_enable(&self) -> PWM_3_DBCTL_ENABLE_R {
        PWM_3_DBCTL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    pub fn pwm_3_dbctl_enable(&mut self) -> PWM_3_DBCTL_ENABLE_W {
        PWM_3_DBCTL_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM3 Dead-Band Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_3_dbctl](index.html) module"]
pub struct _3_DBCTL_SPEC;
impl crate::RegisterSpec for _3_DBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_3_dbctl::R](R) reader structure"]
impl crate::Readable for _3_DBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_3_dbctl::W](W) writer structure"]
impl crate::Writable for _3_DBCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _3_DBCTL to value 0"]
impl crate::Resettable for _3_DBCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
