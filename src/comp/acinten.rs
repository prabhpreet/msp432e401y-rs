#[doc = "Register `ACINTEN` reader"]
pub struct R(crate::R<ACINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACINTEN` writer"]
pub struct W(crate::W<ACINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACINTEN_SPEC>;
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
impl From<crate::W<ACINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_ACINTEN_IN0` reader - Comparator 0 Interrupt Enable"]
pub type COMP_ACINTEN_IN0_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACINTEN_IN0` writer - Comparator 0 Interrupt Enable"]
pub type COMP_ACINTEN_IN0_W<'a> = crate::BitWriter<'a, u32, ACINTEN_SPEC, bool, 0>;
#[doc = "Field `COMP_ACINTEN_IN1` reader - Comparator 1 Interrupt Enable"]
pub type COMP_ACINTEN_IN1_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACINTEN_IN1` writer - Comparator 1 Interrupt Enable"]
pub type COMP_ACINTEN_IN1_W<'a> = crate::BitWriter<'a, u32, ACINTEN_SPEC, bool, 1>;
#[doc = "Field `COMP_ACINTEN_IN2` reader - Comparator 2 Interrupt Enable"]
pub type COMP_ACINTEN_IN2_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACINTEN_IN2` writer - Comparator 2 Interrupt Enable"]
pub type COMP_ACINTEN_IN2_W<'a> = crate::BitWriter<'a, u32, ACINTEN_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in0(&self) -> COMP_ACINTEN_IN0_R {
        COMP_ACINTEN_IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in1(&self) -> COMP_ACINTEN_IN1_R {
        COMP_ACINTEN_IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in2(&self) -> COMP_ACINTEN_IN2_R {
        COMP_ACINTEN_IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in0(&mut self) -> COMP_ACINTEN_IN0_W {
        COMP_ACINTEN_IN0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in1(&mut self) -> COMP_ACINTEN_IN1_W {
        COMP_ACINTEN_IN1_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 2 Interrupt Enable"]
    #[inline(always)]
    pub fn comp_acinten_in2(&mut self) -> COMP_ACINTEN_IN2_W {
        COMP_ACINTEN_IN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acinten](index.html) module"]
pub struct ACINTEN_SPEC;
impl crate::RegisterSpec for ACINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acinten::R](R) reader structure"]
impl crate::Readable for ACINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acinten::W](W) writer structure"]
impl crate::Writable for ACINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACINTEN to value 0"]
impl crate::Resettable for ACINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
