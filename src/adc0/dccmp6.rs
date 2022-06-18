#[doc = "Register `DCCMP6` reader"]
pub struct R(crate::R<DCCMP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCMP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCMP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCMP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCMP6` writer"]
pub struct W(crate::W<DCCMP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCMP6_SPEC>;
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
impl From<crate::W<DCCMP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCMP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_DCCMP6_COMP0` reader - Compare 0"]
pub type ADC_DCCMP6_COMP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_DCCMP6_COMP0` writer - Compare 0"]
pub type ADC_DCCMP6_COMP0_W<'a> = crate::FieldWriter<'a, u32, DCCMP6_SPEC, u16, u16, 12, 0>;
#[doc = "Field `ADC_DCCMP6_COMP1` reader - Compare 1"]
pub type ADC_DCCMP6_COMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_DCCMP6_COMP1` writer - Compare 1"]
pub type ADC_DCCMP6_COMP1_W<'a> = crate::FieldWriter<'a, u32, DCCMP6_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp6_comp0(&self) -> ADC_DCCMP6_COMP0_R {
        ADC_DCCMP6_COMP0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp6_comp1(&self) -> ADC_DCCMP6_COMP1_R {
        ADC_DCCMP6_COMP1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp6_comp0(&mut self) -> ADC_DCCMP6_COMP0_W {
        ADC_DCCMP6_COMP0_W::new(self)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp6_comp1(&mut self) -> ADC_DCCMP6_COMP1_W {
        ADC_DCCMP6_COMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Digital Comparator Range 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp6](index.html) module"]
pub struct DCCMP6_SPEC;
impl crate::RegisterSpec for DCCMP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dccmp6::R](R) reader structure"]
impl crate::Readable for DCCMP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dccmp6::W](W) writer structure"]
impl crate::Writable for DCCMP6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCCMP6 to value 0"]
impl crate::Resettable for DCCMP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
