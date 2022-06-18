#[doc = "Register `DCCMP4` reader"]
pub struct R(crate::R<DCCMP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCMP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCMP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCMP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCMP4` writer"]
pub struct W(crate::W<DCCMP4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCMP4_SPEC>;
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
impl From<crate::W<DCCMP4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCMP4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_DCCMP4_COMP0` reader - Compare 0"]
pub type ADC_DCCMP4_COMP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_DCCMP4_COMP0` writer - Compare 0"]
pub type ADC_DCCMP4_COMP0_W<'a> = crate::FieldWriter<'a, u32, DCCMP4_SPEC, u16, u16, 12, 0>;
#[doc = "Field `ADC_DCCMP4_COMP1` reader - Compare 1"]
pub type ADC_DCCMP4_COMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_DCCMP4_COMP1` writer - Compare 1"]
pub type ADC_DCCMP4_COMP1_W<'a> = crate::FieldWriter<'a, u32, DCCMP4_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp4_comp0(&self) -> ADC_DCCMP4_COMP0_R {
        ADC_DCCMP4_COMP0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp4_comp1(&self) -> ADC_DCCMP4_COMP1_R {
        ADC_DCCMP4_COMP1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp4_comp0(&mut self) -> ADC_DCCMP4_COMP0_W {
        ADC_DCCMP4_COMP0_W::new(self)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp4_comp1(&mut self) -> ADC_DCCMP4_COMP1_W {
        ADC_DCCMP4_COMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Digital Comparator Range 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccmp4](index.html) module"]
pub struct DCCMP4_SPEC;
impl crate::RegisterSpec for DCCMP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dccmp4::R](R) reader structure"]
impl crate::Readable for DCCMP4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dccmp4::W](W) writer structure"]
impl crate::Writable for DCCMP4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCCMP4 to value 0"]
impl crate::Resettable for DCCMP4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
