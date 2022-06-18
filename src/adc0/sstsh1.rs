#[doc = "Register `SSTSH1` reader"]
pub struct R(crate::R<SSTSH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTSH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTSH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTSH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTSH1` writer"]
pub struct W(crate::W<SSTSH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTSH1_SPEC>;
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
impl From<crate::W<SSTSH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTSH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSTSH1_TSH0` reader - 1st Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSTSH1_TSH0` writer - 1st Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH0_W<'a> = crate::FieldWriter<'a, u32, SSTSH1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSTSH1_TSH1` reader - 2nd Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSTSH1_TSH1` writer - 2nd Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH1_W<'a> = crate::FieldWriter<'a, u32, SSTSH1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSTSH1_TSH2` reader - 3rd Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSTSH1_TSH2` writer - 3rd Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH2_W<'a> = crate::FieldWriter<'a, u32, SSTSH1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `ADC_SSTSH1_TSH3` reader - 4th Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSTSH1_TSH3` writer - 4th Sample and Hold Period Select"]
pub type ADC_SSTSH1_TSH3_W<'a> = crate::FieldWriter<'a, u32, SSTSH1_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh0(&self) -> ADC_SSTSH1_TSH0_R {
        ADC_SSTSH1_TSH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh1(&self) -> ADC_SSTSH1_TSH1_R {
        ADC_SSTSH1_TSH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh2(&self) -> ADC_SSTSH1_TSH2_R {
        ADC_SSTSH1_TSH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh3(&self) -> ADC_SSTSH1_TSH3_R {
        ADC_SSTSH1_TSH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh0(&mut self) -> ADC_SSTSH1_TSH0_W {
        ADC_SSTSH1_TSH0_W::new(self)
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh1(&mut self) -> ADC_SSTSH1_TSH1_W {
        ADC_SSTSH1_TSH1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh2(&mut self) -> ADC_SSTSH1_TSH2_W {
        ADC_SSTSH1_TSH2_W::new(self)
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh1_tsh3(&mut self) -> ADC_SSTSH1_TSH3_W {
        ADC_SSTSH1_TSH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 1 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh1](index.html) module"]
pub struct SSTSH1_SPEC;
impl crate::RegisterSpec for SSTSH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstsh1::R](R) reader structure"]
impl crate::Readable for SSTSH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstsh1::W](W) writer structure"]
impl crate::Writable for SSTSH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTSH1 to value 0"]
impl crate::Resettable for SSTSH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
