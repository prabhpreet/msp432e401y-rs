#[doc = "Register `SSPRI` reader"]
pub struct R(crate::R<SSPRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPRI` writer"]
pub struct W(crate::W<SSPRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPRI_SPEC>;
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
impl From<crate::W<SSPRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSPRI_SS0` reader - SS0 Priority"]
pub type ADC_SSPRI_SS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSPRI_SS0` writer - SS0 Priority"]
pub type ADC_SSPRI_SS0_W<'a> = crate::FieldWriter<'a, u32, SSPRI_SPEC, u8, u8, 2, 0>;
#[doc = "Field `ADC_SSPRI_SS1` reader - SS1 Priority"]
pub type ADC_SSPRI_SS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSPRI_SS1` writer - SS1 Priority"]
pub type ADC_SSPRI_SS1_W<'a> = crate::FieldWriter<'a, u32, SSPRI_SPEC, u8, u8, 2, 4>;
#[doc = "Field `ADC_SSPRI_SS2` reader - SS2 Priority"]
pub type ADC_SSPRI_SS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSPRI_SS2` writer - SS2 Priority"]
pub type ADC_SSPRI_SS2_W<'a> = crate::FieldWriter<'a, u32, SSPRI_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ADC_SSPRI_SS3` reader - SS3 Priority"]
pub type ADC_SSPRI_SS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSPRI_SS3` writer - SS3 Priority"]
pub type ADC_SSPRI_SS3_W<'a> = crate::FieldWriter<'a, u32, SSPRI_SPEC, u8, u8, 2, 12>;
impl R {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&self) -> ADC_SSPRI_SS0_R {
        ADC_SSPRI_SS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&self) -> ADC_SSPRI_SS1_R {
        ADC_SSPRI_SS1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&self) -> ADC_SSPRI_SS2_R {
        ADC_SSPRI_SS2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&self) -> ADC_SSPRI_SS3_R {
        ADC_SSPRI_SS3_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&mut self) -> ADC_SSPRI_SS0_W {
        ADC_SSPRI_SS0_W::new(self)
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&mut self) -> ADC_SSPRI_SS1_W {
        ADC_SSPRI_SS1_W::new(self)
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&mut self) -> ADC_SSPRI_SS2_W {
        ADC_SSPRI_SS2_W::new(self)
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&mut self) -> ADC_SSPRI_SS3_W {
        ADC_SSPRI_SS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequencer Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspri](index.html) module"]
pub struct SSPRI_SPEC;
impl crate::RegisterSpec for SSPRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspri::R](R) reader structure"]
impl crate::Readable for SSPRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspri::W](W) writer structure"]
impl crate::Writable for SSPRI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSPRI to value 0"]
impl crate::Resettable for SSPRI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
