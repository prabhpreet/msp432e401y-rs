#[doc = "Register `SSDC3` reader"]
pub struct R(crate::R<SSDC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSDC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSDC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSDC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSDC3` writer"]
pub struct W(crate::W<SSDC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSDC3_SPEC>;
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
impl From<crate::W<SSDC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSDC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSDC3_S0DCSEL` reader - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC3_S0DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC3_S0DCSEL` writer - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC3_S0DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC3_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc3_s0dcsel(&self) -> ADC_SSDC3_S0DCSEL_R {
        ADC_SSDC3_S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc3_s0dcsel(&mut self) -> ADC_SSDC3_S0DCSEL_W {
        ADC_SSDC3_S0DCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 3 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc3](index.html) module"]
pub struct SSDC3_SPEC;
impl crate::RegisterSpec for SSDC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssdc3::R](R) reader structure"]
impl crate::Readable for SSDC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssdc3::W](W) writer structure"]
impl crate::Writable for SSDC3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSDC3 to value 0"]
impl crate::Resettable for SSDC3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
