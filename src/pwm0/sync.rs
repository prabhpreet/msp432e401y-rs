#[doc = "Register `SYNC` reader"]
pub struct R(crate::R<SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC` writer"]
pub struct W(crate::W<SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_SPEC>;
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
impl From<crate::W<SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_SYNC_SYNC0` reader - Reset Generator 0 Counter"]
pub type PWM_SYNC_SYNC0_R = crate::BitReader<bool>;
#[doc = "Field `PWM_SYNC_SYNC0` writer - Reset Generator 0 Counter"]
pub type PWM_SYNC_SYNC0_W<'a> = crate::BitWriter<'a, u32, SYNC_SPEC, bool, 0>;
#[doc = "Field `PWM_SYNC_SYNC1` reader - Reset Generator 1 Counter"]
pub type PWM_SYNC_SYNC1_R = crate::BitReader<bool>;
#[doc = "Field `PWM_SYNC_SYNC1` writer - Reset Generator 1 Counter"]
pub type PWM_SYNC_SYNC1_W<'a> = crate::BitWriter<'a, u32, SYNC_SPEC, bool, 1>;
#[doc = "Field `PWM_SYNC_SYNC2` reader - Reset Generator 2 Counter"]
pub type PWM_SYNC_SYNC2_R = crate::BitReader<bool>;
#[doc = "Field `PWM_SYNC_SYNC2` writer - Reset Generator 2 Counter"]
pub type PWM_SYNC_SYNC2_W<'a> = crate::BitWriter<'a, u32, SYNC_SPEC, bool, 2>;
#[doc = "Field `PWM_SYNC_SYNC3` reader - Reset Generator 3 Counter"]
pub type PWM_SYNC_SYNC3_R = crate::BitReader<bool>;
#[doc = "Field `PWM_SYNC_SYNC3` writer - Reset Generator 3 Counter"]
pub type PWM_SYNC_SYNC3_W<'a> = crate::BitWriter<'a, u32, SYNC_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync0(&self) -> PWM_SYNC_SYNC0_R {
        PWM_SYNC_SYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync1(&self) -> PWM_SYNC_SYNC1_R {
        PWM_SYNC_SYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync2(&self) -> PWM_SYNC_SYNC2_R {
        PWM_SYNC_SYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Generator 3 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync3(&self) -> PWM_SYNC_SYNC3_R {
        PWM_SYNC_SYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync0(&mut self) -> PWM_SYNC_SYNC0_W {
        PWM_SYNC_SYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync1(&mut self) -> PWM_SYNC_SYNC1_W {
        PWM_SYNC_SYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync2(&mut self) -> PWM_SYNC_SYNC2_W {
        PWM_SYNC_SYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Reset Generator 3 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync3(&mut self) -> PWM_SYNC_SYNC3_W {
        PWM_SYNC_SYNC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Time Base Sync\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](index.html) module"]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync::R](R) reader structure"]
impl crate::Readable for SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync::W](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
