#[doc = "Register `SSMUX1` reader"]
pub struct R(crate::R<SSMUX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSMUX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSMUX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSMUX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSMUX1` writer"]
pub struct W(crate::W<SSMUX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSMUX1_SPEC>;
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
impl From<crate::W<SSMUX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSMUX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSMUX1_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX1_MUX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX1_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX1_MUX0_W<'a> = crate::FieldWriter<'a, u32, SSMUX1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSMUX1_MUX1` reader - 2nd Sample Input Select"]
pub type ADC_SSMUX1_MUX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX1_MUX1` writer - 2nd Sample Input Select"]
pub type ADC_SSMUX1_MUX1_W<'a> = crate::FieldWriter<'a, u32, SSMUX1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSMUX1_MUX2` reader - 3rd Sample Input Select"]
pub type ADC_SSMUX1_MUX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX1_MUX2` writer - 3rd Sample Input Select"]
pub type ADC_SSMUX1_MUX2_W<'a> = crate::FieldWriter<'a, u32, SSMUX1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `ADC_SSMUX1_MUX3` reader - 4th Sample Input Select"]
pub type ADC_SSMUX1_MUX3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX1_MUX3` writer - 4th Sample Input Select"]
pub type ADC_SSMUX1_MUX3_W<'a> = crate::FieldWriter<'a, u32, SSMUX1_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux0(&self) -> ADC_SSMUX1_MUX0_R {
        ADC_SSMUX1_MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux1(&self) -> ADC_SSMUX1_MUX1_R {
        ADC_SSMUX1_MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux2(&self) -> ADC_SSMUX1_MUX2_R {
        ADC_SSMUX1_MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux3(&self) -> ADC_SSMUX1_MUX3_R {
        ADC_SSMUX1_MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux0(&mut self) -> ADC_SSMUX1_MUX0_W {
        ADC_SSMUX1_MUX0_W::new(self)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux1(&mut self) -> ADC_SSMUX1_MUX1_W {
        ADC_SSMUX1_MUX1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux2(&mut self) -> ADC_SSMUX1_MUX2_W {
        ADC_SSMUX1_MUX2_W::new(self)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux1_mux3(&mut self) -> ADC_SSMUX1_MUX3_W {
        ADC_SSMUX1_MUX3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux1](index.html) module"]
pub struct SSMUX1_SPEC;
impl crate::RegisterSpec for SSMUX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssmux1::R](R) reader structure"]
impl crate::Readable for SSMUX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssmux1::W](W) writer structure"]
impl crate::Writable for SSMUX1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSMUX1 to value 0"]
impl crate::Resettable for SSMUX1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
