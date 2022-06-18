#[doc = "Register `SSFSTAT2` reader"]
pub struct R(crate::R<SSFSTAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSFSTAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSFSTAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSFSTAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSFSTAT2` writer"]
pub struct W(crate::W<SSFSTAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSFSTAT2_SPEC>;
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
impl From<crate::W<SSFSTAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSFSTAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSFSTAT2_TPTR` reader - FIFO Tail Pointer"]
pub type ADC_SSFSTAT2_TPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSFSTAT2_TPTR` writer - FIFO Tail Pointer"]
pub type ADC_SSFSTAT2_TPTR_W<'a> = crate::FieldWriter<'a, u32, SSFSTAT2_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSFSTAT2_HPTR` reader - FIFO Head Pointer"]
pub type ADC_SSFSTAT2_HPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSFSTAT2_HPTR` writer - FIFO Head Pointer"]
pub type ADC_SSFSTAT2_HPTR_W<'a> = crate::FieldWriter<'a, u32, SSFSTAT2_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSFSTAT2_EMPTY` reader - FIFO Empty"]
pub type ADC_SSFSTAT2_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSFSTAT2_EMPTY` writer - FIFO Empty"]
pub type ADC_SSFSTAT2_EMPTY_W<'a> = crate::BitWriter<'a, u32, SSFSTAT2_SPEC, bool, 8>;
#[doc = "Field `ADC_SSFSTAT2_FULL` reader - FIFO Full"]
pub type ADC_SSFSTAT2_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSFSTAT2_FULL` writer - FIFO Full"]
pub type ADC_SSFSTAT2_FULL_W<'a> = crate::BitWriter<'a, u32, SSFSTAT2_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_tptr(&self) -> ADC_SSFSTAT2_TPTR_R {
        ADC_SSFSTAT2_TPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_hptr(&self) -> ADC_SSFSTAT2_HPTR_R {
        ADC_SSFSTAT2_HPTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat2_empty(&self) -> ADC_SSFSTAT2_EMPTY_R {
        ADC_SSFSTAT2_EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat2_full(&self) -> ADC_SSFSTAT2_FULL_R {
        ADC_SSFSTAT2_FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Tail Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_tptr(&mut self) -> ADC_SSFSTAT2_TPTR_W {
        ADC_SSFSTAT2_TPTR_W::new(self)
    }
    #[doc = "Bits 4:7 - FIFO Head Pointer"]
    #[inline(always)]
    pub fn adc_ssfstat2_hptr(&mut self) -> ADC_SSFSTAT2_HPTR_W {
        ADC_SSFSTAT2_HPTR_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Empty"]
    #[inline(always)]
    pub fn adc_ssfstat2_empty(&mut self) -> ADC_SSFSTAT2_EMPTY_W {
        ADC_SSFSTAT2_EMPTY_W::new(self)
    }
    #[doc = "Bit 12 - FIFO Full"]
    #[inline(always)]
    pub fn adc_ssfstat2_full(&mut self) -> ADC_SSFSTAT2_FULL_W {
        ADC_SSFSTAT2_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence FIFO 2 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat2](index.html) module"]
pub struct SSFSTAT2_SPEC;
impl crate::RegisterSpec for SSFSTAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssfstat2::R](R) reader structure"]
impl crate::Readable for SSFSTAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssfstat2::W](W) writer structure"]
impl crate::Writable for SSFSTAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSFSTAT2 to value 0"]
impl crate::Resettable for SSFSTAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
