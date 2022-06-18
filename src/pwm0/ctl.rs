#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_CTL_GLOBALSYNC0` reader - Update PWM Generator 0"]
pub type PWM_CTL_GLOBALSYNC0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_CTL_GLOBALSYNC0` writer - Update PWM Generator 0"]
pub type PWM_CTL_GLOBALSYNC0_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `PWM_CTL_GLOBALSYNC1` reader - Update PWM Generator 1"]
pub type PWM_CTL_GLOBALSYNC1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_CTL_GLOBALSYNC1` writer - Update PWM Generator 1"]
pub type PWM_CTL_GLOBALSYNC1_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `PWM_CTL_GLOBALSYNC2` reader - Update PWM Generator 2"]
pub type PWM_CTL_GLOBALSYNC2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_CTL_GLOBALSYNC2` writer - Update PWM Generator 2"]
pub type PWM_CTL_GLOBALSYNC2_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `PWM_CTL_GLOBALSYNC3` reader - Update PWM Generator 3"]
pub type PWM_CTL_GLOBALSYNC3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_CTL_GLOBALSYNC3` writer - Update PWM Generator 3"]
pub type PWM_CTL_GLOBALSYNC3_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync0(&self) -> PWM_CTL_GLOBALSYNC0_R {
        PWM_CTL_GLOBALSYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync1(&self) -> PWM_CTL_GLOBALSYNC1_R {
        PWM_CTL_GLOBALSYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync2(&self) -> PWM_CTL_GLOBALSYNC2_R {
        PWM_CTL_GLOBALSYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync3(&self) -> PWM_CTL_GLOBALSYNC3_R {
        PWM_CTL_GLOBALSYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update PWM Generator 0"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync0(&mut self) -> PWM_CTL_GLOBALSYNC0_W {
        PWM_CTL_GLOBALSYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Update PWM Generator 1"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync1(&mut self) -> PWM_CTL_GLOBALSYNC1_W {
        PWM_CTL_GLOBALSYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Update PWM Generator 2"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync2(&mut self) -> PWM_CTL_GLOBALSYNC2_W {
        PWM_CTL_GLOBALSYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Update PWM Generator 3"]
    #[inline(always)]
    pub fn pwm_ctl_globalsync3(&mut self) -> PWM_CTL_GLOBALSYNC3_W {
        PWM_CTL_GLOBALSYNC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Master Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
