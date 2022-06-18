#[doc = "Register `SSOP1` reader"]
pub struct R(crate::R<SSOP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSOP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSOP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSOP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSOP1` writer"]
pub struct W(crate::W<SSOP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSOP1_SPEC>;
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
impl From<crate::W<SSOP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSOP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSOP1_S0DCOP` reader - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP1_S0DCOP_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSOP1_S0DCOP` writer - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP1_S0DCOP_W<'a> = crate::BitWriter<'a, u32, SSOP1_SPEC, bool, 0>;
#[doc = "Field `ADC_SSOP1_S1DCOP` reader - Sample 1 Digital Comparator Operation"]
pub type ADC_SSOP1_S1DCOP_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSOP1_S1DCOP` writer - Sample 1 Digital Comparator Operation"]
pub type ADC_SSOP1_S1DCOP_W<'a> = crate::BitWriter<'a, u32, SSOP1_SPEC, bool, 4>;
#[doc = "Field `ADC_SSOP1_S2DCOP` reader - Sample 2 Digital Comparator Operation"]
pub type ADC_SSOP1_S2DCOP_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSOP1_S2DCOP` writer - Sample 2 Digital Comparator Operation"]
pub type ADC_SSOP1_S2DCOP_W<'a> = crate::BitWriter<'a, u32, SSOP1_SPEC, bool, 8>;
#[doc = "Field `ADC_SSOP1_S3DCOP` reader - Sample 3 Digital Comparator Operation"]
pub type ADC_SSOP1_S3DCOP_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSOP1_S3DCOP` writer - Sample 3 Digital Comparator Operation"]
pub type ADC_SSOP1_S3DCOP_W<'a> = crate::BitWriter<'a, u32, SSOP1_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s0dcop(&self) -> ADC_SSOP1_S0DCOP_R {
        ADC_SSOP1_S0DCOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s1dcop(&self) -> ADC_SSOP1_S1DCOP_R {
        ADC_SSOP1_S1DCOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s2dcop(&self) -> ADC_SSOP1_S2DCOP_R {
        ADC_SSOP1_S2DCOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s3dcop(&self) -> ADC_SSOP1_S3DCOP_R {
        ADC_SSOP1_S3DCOP_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s0dcop(&mut self) -> ADC_SSOP1_S0DCOP_W {
        ADC_SSOP1_S0DCOP_W::new(self)
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s1dcop(&mut self) -> ADC_SSOP1_S1DCOP_W {
        ADC_SSOP1_S1DCOP_W::new(self)
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s2dcop(&mut self) -> ADC_SSOP1_S2DCOP_W {
        ADC_SSOP1_S2DCOP_W::new(self)
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop1_s3dcop(&mut self) -> ADC_SSOP1_S3DCOP_W {
        ADC_SSOP1_S3DCOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 1 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop1](index.html) module"]
pub struct SSOP1_SPEC;
impl crate::RegisterSpec for SSOP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssop1::R](R) reader structure"]
impl crate::Readable for SSOP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssop1::W](W) writer structure"]
impl crate::Writable for SSOP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSOP1 to value 0"]
impl crate::Resettable for SSOP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
