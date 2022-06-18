#[doc = "Register `SSMUX3` reader"]
pub struct R(crate::R<SSMUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSMUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSMUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSMUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSMUX3` writer"]
pub struct W(crate::W<SSMUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSMUX3_SPEC>;
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
impl From<crate::W<SSMUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSMUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSMUX3_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX3_MUX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX3_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX3_MUX0_W<'a> = crate::FieldWriter<'a, u32, SSMUX3_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux3_mux0(&self) -> ADC_SSMUX3_MUX0_R {
        ADC_SSMUX3_MUX0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux3_mux0(&mut self) -> ADC_SSMUX3_MUX0_W {
        ADC_SSMUX3_MUX0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux3](index.html) module"]
pub struct SSMUX3_SPEC;
impl crate::RegisterSpec for SSMUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssmux3::R](R) reader structure"]
impl crate::Readable for SSMUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssmux3::W](W) writer structure"]
impl crate::Writable for SSMUX3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSMUX3 to value 0"]
impl crate::Resettable for SSMUX3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
