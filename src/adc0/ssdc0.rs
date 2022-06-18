#[doc = "Register `SSDC0` reader"]
pub struct R(crate::R<SSDC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSDC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSDC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSDC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSDC0` writer"]
pub struct W(crate::W<SSDC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSDC0_SPEC>;
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
impl From<crate::W<SSDC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSDC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSDC0_S0DCSEL` reader - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC0_S0DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S0DCSEL` writer - Sample 0 Digital Comparator Select"]
pub type ADC_SSDC0_S0DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSDC0_S1DCSEL` reader - Sample 1 Digital Comparator Select"]
pub type ADC_SSDC0_S1DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S1DCSEL` writer - Sample 1 Digital Comparator Select"]
pub type ADC_SSDC0_S1DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSDC0_S2DCSEL` reader - Sample 2 Digital Comparator Select"]
pub type ADC_SSDC0_S2DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S2DCSEL` writer - Sample 2 Digital Comparator Select"]
pub type ADC_SSDC0_S2DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 8>;
#[doc = "Field `ADC_SSDC0_S3DCSEL` reader - Sample 3 Digital Comparator Select"]
pub type ADC_SSDC0_S3DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S3DCSEL` writer - Sample 3 Digital Comparator Select"]
pub type ADC_SSDC0_S3DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 12>;
#[doc = "Field `ADC_SSDC0_S4DCSEL` reader - Sample 4 Digital Comparator Select"]
pub type ADC_SSDC0_S4DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S4DCSEL` writer - Sample 4 Digital Comparator Select"]
pub type ADC_SSDC0_S4DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `ADC_SSDC0_S5DCSEL` reader - Sample 5 Digital Comparator Select"]
pub type ADC_SSDC0_S5DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S5DCSEL` writer - Sample 5 Digital Comparator Select"]
pub type ADC_SSDC0_S5DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 20>;
#[doc = "Field `ADC_SSDC0_S6DCSEL` reader - Sample 6 Digital Comparator Select"]
pub type ADC_SSDC0_S6DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S6DCSEL` writer - Sample 6 Digital Comparator Select"]
pub type ADC_SSDC0_S6DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 24>;
#[doc = "Field `ADC_SSDC0_S7DCSEL` reader - Sample 7 Digital Comparator Select"]
pub type ADC_SSDC0_S7DCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSDC0_S7DCSEL` writer - Sample 7 Digital Comparator Select"]
pub type ADC_SSDC0_S7DCSEL_W<'a> = crate::FieldWriter<'a, u32, SSDC0_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s0dcsel(&self) -> ADC_SSDC0_S0DCSEL_R {
        ADC_SSDC0_S0DCSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s1dcsel(&self) -> ADC_SSDC0_S1DCSEL_R {
        ADC_SSDC0_S1DCSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s2dcsel(&self) -> ADC_SSDC0_S2DCSEL_R {
        ADC_SSDC0_S2DCSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s3dcsel(&self) -> ADC_SSDC0_S3DCSEL_R {
        ADC_SSDC0_S3DCSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s4dcsel(&self) -> ADC_SSDC0_S4DCSEL_R {
        ADC_SSDC0_S4DCSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s5dcsel(&self) -> ADC_SSDC0_S5DCSEL_R {
        ADC_SSDC0_S5DCSEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s6dcsel(&self) -> ADC_SSDC0_S6DCSEL_R {
        ADC_SSDC0_S6DCSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s7dcsel(&self) -> ADC_SSDC0_S7DCSEL_R {
        ADC_SSDC0_S7DCSEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s0dcsel(&mut self) -> ADC_SSDC0_S0DCSEL_W {
        ADC_SSDC0_S0DCSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s1dcsel(&mut self) -> ADC_SSDC0_S1DCSEL_W {
        ADC_SSDC0_S1DCSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s2dcsel(&mut self) -> ADC_SSDC0_S2DCSEL_W {
        ADC_SSDC0_S2DCSEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s3dcsel(&mut self) -> ADC_SSDC0_S3DCSEL_W {
        ADC_SSDC0_S3DCSEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Sample 4 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s4dcsel(&mut self) -> ADC_SSDC0_S4DCSEL_W {
        ADC_SSDC0_S4DCSEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Sample 5 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s5dcsel(&mut self) -> ADC_SSDC0_S5DCSEL_W {
        ADC_SSDC0_S5DCSEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Sample 6 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s6dcsel(&mut self) -> ADC_SSDC0_S6DCSEL_W {
        ADC_SSDC0_S6DCSEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Sample 7 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc0_s7dcsel(&mut self) -> ADC_SSDC0_S7DCSEL_W {
        ADC_SSDC0_S7DCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 0 Digital Comparator Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssdc0](index.html) module"]
pub struct SSDC0_SPEC;
impl crate::RegisterSpec for SSDC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssdc0::R](R) reader structure"]
impl crate::Readable for SSDC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssdc0::W](W) writer structure"]
impl crate::Writable for SSDC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSDC0 to value 0"]
impl crate::Resettable for SSDC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
