#[doc = "Register `_0_INTEN` reader"]
pub struct R(crate::R<_0_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0_INTEN` writer"]
pub struct W(crate::W<_0_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0_INTEN_SPEC>;
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
impl From<crate::W<_0_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_0_INTEN_INTCNTZERO` reader - Interrupt for Counter=0"]
pub type PWM_0_INTEN_INTCNTZERO_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCNTZERO` writer - Interrupt for Counter=0"]
pub type PWM_0_INTEN_INTCNTZERO_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 0>;
#[doc = "Field `PWM_0_INTEN_INTCNTLOAD` reader - Interrupt for Counter=PWMnLOAD"]
pub type PWM_0_INTEN_INTCNTLOAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCNTLOAD` writer - Interrupt for Counter=PWMnLOAD"]
pub type PWM_0_INTEN_INTCNTLOAD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 1>;
#[doc = "Field `PWM_0_INTEN_INTCMPAU` reader - Interrupt for Counter=PWMnCMPA Up"]
pub type PWM_0_INTEN_INTCMPAU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCMPAU` writer - Interrupt for Counter=PWMnCMPA Up"]
pub type PWM_0_INTEN_INTCMPAU_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 2>;
#[doc = "Field `PWM_0_INTEN_INTCMPAD` reader - Interrupt for Counter=PWMnCMPA Down"]
pub type PWM_0_INTEN_INTCMPAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCMPAD` writer - Interrupt for Counter=PWMnCMPA Down"]
pub type PWM_0_INTEN_INTCMPAD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 3>;
#[doc = "Field `PWM_0_INTEN_INTCMPBU` reader - Interrupt for Counter=PWMnCMPB Up"]
pub type PWM_0_INTEN_INTCMPBU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCMPBU` writer - Interrupt for Counter=PWMnCMPB Up"]
pub type PWM_0_INTEN_INTCMPBU_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 4>;
#[doc = "Field `PWM_0_INTEN_INTCMPBD` reader - Interrupt for Counter=PWMnCMPB Down"]
pub type PWM_0_INTEN_INTCMPBD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_INTCMPBD` writer - Interrupt for Counter=PWMnCMPB Down"]
pub type PWM_0_INTEN_INTCMPBD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 5>;
#[doc = "Field `PWM_0_INTEN_TRCNTZERO` reader - Trigger for Counter=0"]
pub type PWM_0_INTEN_TRCNTZERO_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCNTZERO` writer - Trigger for Counter=0"]
pub type PWM_0_INTEN_TRCNTZERO_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 8>;
#[doc = "Field `PWM_0_INTEN_TRCNTLOAD` reader - Trigger for Counter=PWMnLOAD"]
pub type PWM_0_INTEN_TRCNTLOAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCNTLOAD` writer - Trigger for Counter=PWMnLOAD"]
pub type PWM_0_INTEN_TRCNTLOAD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 9>;
#[doc = "Field `PWM_0_INTEN_TRCMPAU` reader - Trigger for Counter=PWMnCMPA Up"]
pub type PWM_0_INTEN_TRCMPAU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCMPAU` writer - Trigger for Counter=PWMnCMPA Up"]
pub type PWM_0_INTEN_TRCMPAU_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 10>;
#[doc = "Field `PWM_0_INTEN_TRCMPAD` reader - Trigger for Counter=PWMnCMPA Down"]
pub type PWM_0_INTEN_TRCMPAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCMPAD` writer - Trigger for Counter=PWMnCMPA Down"]
pub type PWM_0_INTEN_TRCMPAD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 11>;
#[doc = "Field `PWM_0_INTEN_TRCMPBU` reader - Trigger for Counter=PWMnCMPB Up"]
pub type PWM_0_INTEN_TRCMPBU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCMPBU` writer - Trigger for Counter=PWMnCMPB Up"]
pub type PWM_0_INTEN_TRCMPBU_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 12>;
#[doc = "Field `PWM_0_INTEN_TRCMPBD` reader - Trigger for Counter=PWMnCMPB Down"]
pub type PWM_0_INTEN_TRCMPBD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_0_INTEN_TRCMPBD` writer - Trigger for Counter=PWMnCMPB Down"]
pub type PWM_0_INTEN_TRCMPBD_W<'a> = crate::BitWriter<'a, u32, _0_INTEN_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_0_inten_intcntzero(&self) -> PWM_0_INTEN_INTCNTZERO_R {
        PWM_0_INTEN_INTCNTZERO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_0_inten_intcntload(&self) -> PWM_0_INTEN_INTCNTLOAD_R {
        PWM_0_INTEN_INTCNTLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpau(&self) -> PWM_0_INTEN_INTCMPAU_R {
        PWM_0_INTEN_INTCMPAU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpad(&self) -> PWM_0_INTEN_INTCMPAD_R {
        PWM_0_INTEN_INTCMPAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpbu(&self) -> PWM_0_INTEN_INTCMPBU_R {
        PWM_0_INTEN_INTCMPBU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpbd(&self) -> PWM_0_INTEN_INTCMPBD_R {
        PWM_0_INTEN_INTCMPBD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_0_inten_trcntzero(&self) -> PWM_0_INTEN_TRCNTZERO_R {
        PWM_0_INTEN_TRCNTZERO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_0_inten_trcntload(&self) -> PWM_0_INTEN_TRCNTLOAD_R {
        PWM_0_INTEN_TRCNTLOAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpau(&self) -> PWM_0_INTEN_TRCMPAU_R {
        PWM_0_INTEN_TRCMPAU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpad(&self) -> PWM_0_INTEN_TRCMPAD_R {
        PWM_0_INTEN_TRCMPAD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpbu(&self) -> PWM_0_INTEN_TRCMPBU_R {
        PWM_0_INTEN_TRCMPBU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpbd(&self) -> PWM_0_INTEN_TRCMPBD_R {
        PWM_0_INTEN_TRCMPBD_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt for Counter=0"]
    #[inline(always)]
    pub fn pwm_0_inten_intcntzero(&mut self) -> PWM_0_INTEN_INTCNTZERO_W {
        PWM_0_INTEN_INTCNTZERO_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_0_inten_intcntload(&mut self) -> PWM_0_INTEN_INTCNTLOAD_W {
        PWM_0_INTEN_INTCNTLOAD_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpau(&mut self) -> PWM_0_INTEN_INTCMPAU_W {
        PWM_0_INTEN_INTCMPAU_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpad(&mut self) -> PWM_0_INTEN_INTCMPAD_W {
        PWM_0_INTEN_INTCMPAD_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpbu(&mut self) -> PWM_0_INTEN_INTCMPBU_W {
        PWM_0_INTEN_INTCMPBU_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_0_inten_intcmpbd(&mut self) -> PWM_0_INTEN_INTCMPBD_W {
        PWM_0_INTEN_INTCMPBD_W::new(self)
    }
    #[doc = "Bit 8 - Trigger for Counter=0"]
    #[inline(always)]
    pub fn pwm_0_inten_trcntzero(&mut self) -> PWM_0_INTEN_TRCNTZERO_W {
        PWM_0_INTEN_TRCNTZERO_W::new(self)
    }
    #[doc = "Bit 9 - Trigger for Counter=PWMnLOAD"]
    #[inline(always)]
    pub fn pwm_0_inten_trcntload(&mut self) -> PWM_0_INTEN_TRCNTLOAD_W {
        PWM_0_INTEN_TRCNTLOAD_W::new(self)
    }
    #[doc = "Bit 10 - Trigger for Counter=PWMnCMPA Up"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpau(&mut self) -> PWM_0_INTEN_TRCMPAU_W {
        PWM_0_INTEN_TRCMPAU_W::new(self)
    }
    #[doc = "Bit 11 - Trigger for Counter=PWMnCMPA Down"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpad(&mut self) -> PWM_0_INTEN_TRCMPAD_W {
        PWM_0_INTEN_TRCMPAD_W::new(self)
    }
    #[doc = "Bit 12 - Trigger for Counter=PWMnCMPB Up"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpbu(&mut self) -> PWM_0_INTEN_TRCMPBU_W {
        PWM_0_INTEN_TRCMPBU_W::new(self)
    }
    #[doc = "Bit 13 - Trigger for Counter=PWMnCMPB Down"]
    #[inline(always)]
    pub fn pwm_0_inten_trcmpbd(&mut self) -> PWM_0_INTEN_TRCMPBD_W {
        PWM_0_INTEN_TRCMPBD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM0 Interrupt and Trigger Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_inten](index.html) module"]
pub struct _0_INTEN_SPEC;
impl crate::RegisterSpec for _0_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_inten::R](R) reader structure"]
impl crate::Readable for _0_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0_inten::W](W) writer structure"]
impl crate::Writable for _0_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0_INTEN to value 0"]
impl crate::Resettable for _0_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
