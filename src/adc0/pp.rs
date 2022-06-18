#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Maximum Conversion Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_PP_MCR_A {
    #[doc = "7: Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PP_MCR_FULL = 7,
}
impl From<ADC_PP_MCR_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PP_MCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_PP_MCR` reader - Maximum Conversion Rate"]
pub type ADC_PP_MCR_R = crate::FieldReader<u8, ADC_PP_MCR_A>;
impl ADC_PP_MCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_PP_MCR_A> {
        match self.bits {
            7 => Some(ADC_PP_MCR_A::ADC_PP_MCR_FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PP_MCR_FULL`"]
    #[inline(always)]
    pub fn is_adc_pp_mcr_full(&self) -> bool {
        *self == ADC_PP_MCR_A::ADC_PP_MCR_FULL
    }
}
#[doc = "Field `ADC_PP_MCR` writer - Maximum Conversion Rate"]
pub type ADC_PP_MCR_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, ADC_PP_MCR_A, 4, 0>;
impl<'a> ADC_PP_MCR_W<'a> {
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline(always)]
    pub fn adc_pp_mcr_full(self) -> &'a mut W {
        self.variant(ADC_PP_MCR_A::ADC_PP_MCR_FULL)
    }
}
#[doc = "Field `ADC_PP_CH` reader - ADC Channel Count"]
pub type ADC_PP_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_PP_CH` writer - ADC Channel Count"]
pub type ADC_PP_CH_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 6, 4>;
#[doc = "Field `ADC_PP_DC` reader - Digital Comparator Count"]
pub type ADC_PP_DC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_PP_DC` writer - Digital Comparator Count"]
pub type ADC_PP_DC_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 6, 10>;
#[doc = "ADC Architecture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_PP_TYPE_A {
    #[doc = "0: SAR"]
    ADC_PP_TYPE_SAR = 0,
}
impl From<ADC_PP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PP_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_PP_TYPE` reader - ADC Architecture"]
pub type ADC_PP_TYPE_R = crate::FieldReader<u8, ADC_PP_TYPE_A>;
impl ADC_PP_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_PP_TYPE_A> {
        match self.bits {
            0 => Some(ADC_PP_TYPE_A::ADC_PP_TYPE_SAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PP_TYPE_SAR`"]
    #[inline(always)]
    pub fn is_adc_pp_type_sar(&self) -> bool {
        *self == ADC_PP_TYPE_A::ADC_PP_TYPE_SAR
    }
}
#[doc = "Field `ADC_PP_TYPE` writer - ADC Architecture"]
pub type ADC_PP_TYPE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, ADC_PP_TYPE_A, 2, 16>;
impl<'a> ADC_PP_TYPE_W<'a> {
    #[doc = "SAR"]
    #[inline(always)]
    pub fn adc_pp_type_sar(self) -> &'a mut W {
        self.variant(ADC_PP_TYPE_A::ADC_PP_TYPE_SAR)
    }
}
#[doc = "Field `ADC_PP_RSL` reader - Resolution"]
pub type ADC_PP_RSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_PP_RSL` writer - Resolution"]
pub type ADC_PP_RSL_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 5, 18>;
#[doc = "Field `ADC_PP_TS` reader - Temperature Sensor"]
pub type ADC_PP_TS_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PP_TS` writer - Temperature Sensor"]
pub type ADC_PP_TS_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 23>;
#[doc = "Field `ADC_PP_APSHT` reader - Application-Programmable Sample-and-Hold Time"]
pub type ADC_PP_APSHT_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PP_APSHT` writer - Application-Programmable Sample-and-Hold Time"]
pub type ADC_PP_APSHT_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 24>;
impl R {
    #[doc = "Bits 0:3 - Maximum Conversion Rate"]
    #[inline(always)]
    pub fn adc_pp_mcr(&self) -> ADC_PP_MCR_R {
        ADC_PP_MCR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn adc_pp_ch(&self) -> ADC_PP_CH_R {
        ADC_PP_CH_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn adc_pp_dc(&self) -> ADC_PP_DC_R {
        ADC_PP_DC_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn adc_pp_type(&self) -> ADC_PP_TYPE_R {
        ADC_PP_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn adc_pp_rsl(&self) -> ADC_PP_RSL_R {
        ADC_PP_RSL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn adc_pp_ts(&self) -> ADC_PP_TS_R {
        ADC_PP_TS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Application-Programmable Sample-and-Hold Time"]
    #[inline(always)]
    pub fn adc_pp_apsht(&self) -> ADC_PP_APSHT_R {
        ADC_PP_APSHT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Maximum Conversion Rate"]
    #[inline(always)]
    pub fn adc_pp_mcr(&mut self) -> ADC_PP_MCR_W {
        ADC_PP_MCR_W::new(self)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn adc_pp_ch(&mut self) -> ADC_PP_CH_W {
        ADC_PP_CH_W::new(self)
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn adc_pp_dc(&mut self) -> ADC_PP_DC_W {
        ADC_PP_DC_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn adc_pp_type(&mut self) -> ADC_PP_TYPE_W {
        ADC_PP_TYPE_W::new(self)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn adc_pp_rsl(&mut self) -> ADC_PP_RSL_W {
        ADC_PP_RSL_W::new(self)
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn adc_pp_ts(&mut self) -> ADC_PP_TS_W {
        ADC_PP_TS_W::new(self)
    }
    #[doc = "Bit 24 - Application-Programmable Sample-and-Hold Time"]
    #[inline(always)]
    pub fn adc_pp_apsht(&mut self) -> ADC_PP_APSHT_W {
        ADC_PP_APSHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
