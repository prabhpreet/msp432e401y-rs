#[doc = "Register `SSFSTAT1` reader"]
pub struct R(crate::R<SSFSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSFSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSFSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSFSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSFSTAT1` writer"]
pub struct W(crate::W<SSFSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSFSTAT1_SPEC>;
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
impl From<crate::W<SSFSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSFSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSFSTAT1_TPTR` reader - FIFO Tail Pointer"]
pub type ADC_SSFSTAT1_TPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSFSTAT1_TPTR` writer - FIFO Tail Pointer"]
pub type ADC_SSFSTAT1_TPTR_W<'a> = crate::FieldWriter<'a, u32, SSFSTAT1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSFSTAT1_HPTR` reader - FIFO Head Pointer"]
pub type ADC_SSFSTAT1_HPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSFSTAT1_HPTR` writer - FIFO Head Pointer"]
pub type ADC_SSFSTAT1_HPTR_W<'a> = crate::FieldWriter<'a, u32, SSFSTAT1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSFSTAT1_EMPTY` reader - FIFO Empty"]
pub type ADC_SSFSTAT1_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSFSTAT1_EMPTY` writer - FIFO Empty"]
pub type ADC_SSFSTAT1_EMPTY_W<'a> = crate::BitWriter<'a, u32, SSFSTAT1_SPEC, bool, 8>;
#[doc = "Field `ADC_SSFSTAT1_FULL` reader - FIFO Full"]
pub type ADC_SSFSTAT1_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSFSTAT1_FULL` writer - FIFO Full"]
pub type ADC_SSFSTAT1_FULL_W<'a> = crate::BitWriter<'a, u32, SSFSTAT1_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat1_tptr(&self) -> ADC_SSFSTAT1_TPTR_R {
        ADC_SSFSTAT1_TPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat1_hptr(&self) -> ADC_SSFSTAT1_HPTR_R {
        ADC_SSFSTAT1_HPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat1_empty(&self) -> ADC_SSFSTAT1_EMPTY_R {
        ADC_SSFSTAT1_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat1_full(&self) -> ADC_SSFSTAT1_FULL_R {
        ADC_SSFSTAT1_FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat1_tptr(&mut self) -> ADC_SSFSTAT1_TPTR_W {
        ADC_SSFSTAT1_TPTR_W::new(self)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat1_hptr(&mut self) -> ADC_SSFSTAT1_HPTR_W {
        ADC_SSFSTAT1_HPTR_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat1_empty(&mut self) -> ADC_SSFSTAT1_EMPTY_W {
        ADC_SSFSTAT1_EMPTY_W::new(self)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat1_full(&mut self) -> ADC_SSFSTAT1_FULL_W {
        ADC_SSFSTAT1_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat1](index.html) module"]
pub struct SSFSTAT1_SPEC;
impl crate::RegisterSpec for SSFSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssfstat1::R](R) reader structure"]
impl crate::Readable for SSFSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssfstat1::W](W) writer structure"]
impl crate::Writable for SSFSTAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSFSTAT1 to value 0"]
impl crate::Resettable for SSFSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
