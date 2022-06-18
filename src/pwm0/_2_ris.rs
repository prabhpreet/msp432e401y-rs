#[doc = "Register `_2_RIS` reader"]
pub struct R(crate::R<_2_RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_2_RIS` writer"]
pub struct W(crate::W<_2_RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_2_RIS_SPEC>;
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
impl From<crate::W<_2_RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_2_RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_2_RIS_INTCNTZERO` reader - Counter=0 Interrupt Status"]
pub type PWM_2_RIS_INTCNTZERO_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCNTZERO` writer - Counter=0 Interrupt Status"]
pub type PWM_2_RIS_INTCNTZERO_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 0>;
#[doc = "Field `PWM_2_RIS_INTCNTLOAD` reader - Counter=Load Interrupt Status"]
pub type PWM_2_RIS_INTCNTLOAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCNTLOAD` writer - Counter=Load Interrupt Status"]
pub type PWM_2_RIS_INTCNTLOAD_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 1>;
#[doc = "Field `PWM_2_RIS_INTCMPAU` reader - Comparator A Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPAU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCMPAU` writer - Comparator A Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPAU_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 2>;
#[doc = "Field `PWM_2_RIS_INTCMPAD` reader - Comparator A Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPAD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCMPAD` writer - Comparator A Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPAD_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 3>;
#[doc = "Field `PWM_2_RIS_INTCMPBU` reader - Comparator B Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPBU_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCMPBU` writer - Comparator B Up Interrupt Status"]
pub type PWM_2_RIS_INTCMPBU_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 4>;
#[doc = "Field `PWM_2_RIS_INTCMPBD` reader - Comparator B Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPBD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_RIS_INTCMPBD` writer - Comparator B Down Interrupt Status"]
pub type PWM_2_RIS_INTCMPBD_W<'a> = crate::BitWriter<'a, u32, _2_RIS_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntzero(&self) -> PWM_2_RIS_INTCNTZERO_R {
        PWM_2_RIS_INTCNTZERO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntload(&self) -> PWM_2_RIS_INTCNTLOAD_R {
        PWM_2_RIS_INTCNTLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpau(&self) -> PWM_2_RIS_INTCMPAU_R {
        PWM_2_RIS_INTCMPAU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpad(&self) -> PWM_2_RIS_INTCMPAD_R {
        PWM_2_RIS_INTCMPAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbu(&self) -> PWM_2_RIS_INTCMPBU_R {
        PWM_2_RIS_INTCMPBU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbd(&self) -> PWM_2_RIS_INTCMPBD_R {
        PWM_2_RIS_INTCMPBD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter=0 Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntzero(&mut self) -> PWM_2_RIS_INTCNTZERO_W {
        PWM_2_RIS_INTCNTZERO_W::new(self)
    }
    #[doc = "Bit 1 - Counter=Load Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcntload(&mut self) -> PWM_2_RIS_INTCNTLOAD_W {
        PWM_2_RIS_INTCNTLOAD_W::new(self)
    }
    #[doc = "Bit 2 - Comparator A Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpau(&mut self) -> PWM_2_RIS_INTCMPAU_W {
        PWM_2_RIS_INTCMPAU_W::new(self)
    }
    #[doc = "Bit 3 - Comparator A Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpad(&mut self) -> PWM_2_RIS_INTCMPAD_W {
        PWM_2_RIS_INTCMPAD_W::new(self)
    }
    #[doc = "Bit 4 - Comparator B Up Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbu(&mut self) -> PWM_2_RIS_INTCMPBU_W {
        PWM_2_RIS_INTCMPBU_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Down Interrupt Status"]
    #[inline(always)]
    pub fn pwm_2_ris_intcmpbd(&mut self) -> PWM_2_RIS_INTCMPBD_W {
        PWM_2_RIS_INTCMPBD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM2 Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_ris](index.html) module"]
pub struct _2_RIS_SPEC;
impl crate::RegisterSpec for _2_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_ris::R](R) reader structure"]
impl crate::Readable for _2_RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_2_ris::W](W) writer structure"]
impl crate::Writable for _2_RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _2_RIS to value 0"]
impl crate::Resettable for _2_RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
