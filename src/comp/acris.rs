#[doc = "Register `ACRIS` reader"]
pub struct R(crate::R<ACRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACRIS` writer"]
pub struct W(crate::W<ACRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACRIS_SPEC>;
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
impl From<crate::W<ACRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_ACRIS_IN0` reader - Comparator 0 Interrupt Status"]
pub type COMP_ACRIS_IN0_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACRIS_IN0` writer - Comparator 0 Interrupt Status"]
pub type COMP_ACRIS_IN0_W<'a> = crate::BitWriter<'a, u32, ACRIS_SPEC, bool, 0>;
#[doc = "Field `COMP_ACRIS_IN1` reader - Comparator 1 Interrupt Status"]
pub type COMP_ACRIS_IN1_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACRIS_IN1` writer - Comparator 1 Interrupt Status"]
pub type COMP_ACRIS_IN1_W<'a> = crate::BitWriter<'a, u32, ACRIS_SPEC, bool, 1>;
#[doc = "Field `COMP_ACRIS_IN2` reader - Comparator 2 Interrupt Status"]
pub type COMP_ACRIS_IN2_R = crate::BitReader<bool>;
#[doc = "Field `COMP_ACRIS_IN2` writer - Comparator 2 Interrupt Status"]
pub type COMP_ACRIS_IN2_W<'a> = crate::BitWriter<'a, u32, ACRIS_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in0(&self) -> COMP_ACRIS_IN0_R {
        COMP_ACRIS_IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in1(&self) -> COMP_ACRIS_IN1_R {
        COMP_ACRIS_IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in2(&self) -> COMP_ACRIS_IN2_R {
        COMP_ACRIS_IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in0(&mut self) -> COMP_ACRIS_IN0_W {
        COMP_ACRIS_IN0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in1(&mut self) -> COMP_ACRIS_IN1_W {
        COMP_ACRIS_IN1_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 2 Interrupt Status"]
    #[inline(always)]
    pub fn comp_acris_in2(&mut self) -> COMP_ACRIS_IN2_W {
        COMP_ACRIS_IN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acris](index.html) module"]
pub struct ACRIS_SPEC;
impl crate::RegisterSpec for ACRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acris::R](R) reader structure"]
impl crate::Readable for ACRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acris::W](W) writer structure"]
impl crate::Writable for ACRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACRIS to value 0"]
impl crate::Resettable for ACRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
