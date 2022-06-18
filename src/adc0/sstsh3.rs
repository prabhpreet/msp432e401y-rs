#[doc = "Register `SSTSH3` reader"]
pub struct R(crate::R<SSTSH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSTSH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSTSH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSTSH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTSH3` writer"]
pub struct W(crate::W<SSTSH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSTSH3_SPEC>;
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
impl From<crate::W<SSTSH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSTSH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSTSH3_TSH0` reader - 1st Sample and Hold Period Select"]
pub type ADC_SSTSH3_TSH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSTSH3_TSH0` writer - 1st Sample and Hold Period Select"]
pub type ADC_SSTSH3_TSH0_W<'a> = crate::FieldWriter<'a, u32, SSTSH3_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh3_tsh0(&self) -> ADC_SSTSH3_TSH0_R {
        ADC_SSTSH3_TSH0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh3_tsh0(&mut self) -> ADC_SSTSH3_TSH0_W {
        ADC_SSTSH3_TSH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 3 Sample and Hold Time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstsh3](index.html) module"]
pub struct SSTSH3_SPEC;
impl crate::RegisterSpec for SSTSH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sstsh3::R](R) reader structure"]
impl crate::Readable for SSTSH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstsh3::W](W) writer structure"]
impl crate::Writable for SSTSH3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTSH3 to value 0"]
impl crate::Resettable for SSTSH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
