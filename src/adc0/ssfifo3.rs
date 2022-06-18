#[doc = "Register `SSFIFO3` reader"]
pub struct R(crate::R<SSFIFO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSFIFO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSFIFO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSFIFO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSFIFO3` writer"]
pub struct W(crate::W<SSFIFO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSFIFO3_SPEC>;
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
impl From<crate::W<SSFIFO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSFIFO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSFIFO3_DATA` reader - Conversion Result Data"]
pub type ADC_SSFIFO3_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADC_SSFIFO3_DATA` writer - Conversion Result Data"]
pub type ADC_SSFIFO3_DATA_W<'a> = crate::FieldWriter<'a, u32, SSFIFO3_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo3_data(&self) -> ADC_SSFIFO3_DATA_R {
        ADC_SSFIFO3_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Conversion Result Data"]
    #[inline(always)]
    pub fn adc_ssfifo3_data(&mut self) -> ADC_SSFIFO3_DATA_W {
        ADC_SSFIFO3_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Result FIFO 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo3](index.html) module"]
pub struct SSFIFO3_SPEC;
impl crate::RegisterSpec for SSFIFO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssfifo3::R](R) reader structure"]
impl crate::Readable for SSFIFO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssfifo3::W](W) writer structure"]
impl crate::Writable for SSFIFO3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSFIFO3 to value 0"]
impl crate::Resettable for SSFIFO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
