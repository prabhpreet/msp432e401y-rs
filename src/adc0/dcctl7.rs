#[doc = "Register `DCCTL7` reader"]
pub struct R(crate::R<DCCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCTL7` writer"]
pub struct W(crate::W<DCCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCTL7_SPEC>;
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
impl From<crate::W<DCCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparison Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_DCCTL7_CIM_A {
    #[doc = "0: Always"]
    ADC_DCCTL7_CIM_ALWAYS = 0,
    #[doc = "1: Once"]
    ADC_DCCTL7_CIM_ONCE = 1,
    #[doc = "2: Hysteresis Always"]
    ADC_DCCTL7_CIM_HALWAYS = 2,
    #[doc = "3: Hysteresis Once"]
    ADC_DCCTL7_CIM_HONCE = 3,
}
impl From<ADC_DCCTL7_CIM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL7_CIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_DCCTL7_CIM` reader - Comparison Interrupt Mode"]
pub type ADC_DCCTL7_CIM_R = crate::FieldReader<u8, ADC_DCCTL7_CIM_A>;
impl ADC_DCCTL7_CIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DCCTL7_CIM_A {
        match self.bits {
            0 => ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ALWAYS,
            1 => ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ONCE,
            2 => ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HALWAYS,
            3 => ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIM_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cim_always(&self) -> bool {
        *self == ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIM_ONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cim_once(&self) -> bool {
        *self == ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ONCE
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIM_HALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cim_halways(&self) -> bool {
        *self == ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIM_HONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cim_honce(&self) -> bool {
        *self == ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HONCE
    }
}
#[doc = "Field `ADC_DCCTL7_CIM` writer - Comparison Interrupt Mode"]
pub type ADC_DCCTL7_CIM_W<'a> =
    crate::FieldWriterSafe<'a, u32, DCCTL7_SPEC, u8, ADC_DCCTL7_CIM_A, 2, 0>;
impl<'a> ADC_DCCTL7_CIM_W<'a> {
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl7_cim_always(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl7_cim_once(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl7_cim_halways(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl7_cim_honce(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIM_A::ADC_DCCTL7_CIM_HONCE)
    }
}
#[doc = "Comparison Interrupt Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_DCCTL7_CIC_A {
    #[doc = "0: Low Band"]
    ADC_DCCTL7_CIC_LOW = 0,
    #[doc = "1: Mid Band"]
    ADC_DCCTL7_CIC_MID = 1,
    #[doc = "3: High Band"]
    ADC_DCCTL7_CIC_HIGH = 3,
}
impl From<ADC_DCCTL7_CIC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL7_CIC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_DCCTL7_CIC` reader - Comparison Interrupt Condition"]
pub type ADC_DCCTL7_CIC_R = crate::FieldReader<u8, ADC_DCCTL7_CIC_A>;
impl ADC_DCCTL7_CIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_DCCTL7_CIC_A> {
        match self.bits {
            0 => Some(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_LOW),
            1 => Some(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_MID),
            3 => Some(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIC_LOW`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cic_low(&self) -> bool {
        *self == ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_LOW
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIC_MID`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cic_mid(&self) -> bool {
        *self == ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_MID
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CIC_HIGH`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_cic_high(&self) -> bool {
        *self == ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_HIGH
    }
}
#[doc = "Field `ADC_DCCTL7_CIC` writer - Comparison Interrupt Condition"]
pub type ADC_DCCTL7_CIC_W<'a> =
    crate::FieldWriter<'a, u32, DCCTL7_SPEC, u8, ADC_DCCTL7_CIC_A, 2, 2>;
impl<'a> ADC_DCCTL7_CIC_W<'a> {
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl7_cic_low(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl7_cic_mid(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl7_cic_high(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CIC_A::ADC_DCCTL7_CIC_HIGH)
    }
}
#[doc = "Field `ADC_DCCTL7_CIE` reader - Comparison Interrupt Enable"]
pub type ADC_DCCTL7_CIE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DCCTL7_CIE` writer - Comparison Interrupt Enable"]
pub type ADC_DCCTL7_CIE_W<'a> = crate::BitWriter<'a, u32, DCCTL7_SPEC, bool, 4>;
#[doc = "Comparison Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_DCCTL7_CTM_A {
    #[doc = "0: Always"]
    ADC_DCCTL7_CTM_ALWAYS = 0,
    #[doc = "1: Once"]
    ADC_DCCTL7_CTM_ONCE = 1,
    #[doc = "2: Hysteresis Always"]
    ADC_DCCTL7_CTM_HALWAYS = 2,
    #[doc = "3: Hysteresis Once"]
    ADC_DCCTL7_CTM_HONCE = 3,
}
impl From<ADC_DCCTL7_CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL7_CTM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_DCCTL7_CTM` reader - Comparison Trigger Mode"]
pub type ADC_DCCTL7_CTM_R = crate::FieldReader<u8, ADC_DCCTL7_CTM_A>;
impl ADC_DCCTL7_CTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DCCTL7_CTM_A {
        match self.bits {
            0 => ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ALWAYS,
            1 => ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ONCE,
            2 => ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HALWAYS,
            3 => ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTM_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctm_always(&self) -> bool {
        *self == ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTM_ONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctm_once(&self) -> bool {
        *self == ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ONCE
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTM_HALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctm_halways(&self) -> bool {
        *self == ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTM_HONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctm_honce(&self) -> bool {
        *self == ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HONCE
    }
}
#[doc = "Field `ADC_DCCTL7_CTM` writer - Comparison Trigger Mode"]
pub type ADC_DCCTL7_CTM_W<'a> =
    crate::FieldWriterSafe<'a, u32, DCCTL7_SPEC, u8, ADC_DCCTL7_CTM_A, 2, 8>;
impl<'a> ADC_DCCTL7_CTM_W<'a> {
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm_always(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm_once(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm_halways(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm_honce(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTM_A::ADC_DCCTL7_CTM_HONCE)
    }
}
#[doc = "Comparison Trigger Condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_DCCTL7_CTC_A {
    #[doc = "0: Low Band"]
    ADC_DCCTL7_CTC_LOW = 0,
    #[doc = "1: Mid Band"]
    ADC_DCCTL7_CTC_MID = 1,
    #[doc = "3: High Band"]
    ADC_DCCTL7_CTC_HIGH = 3,
}
impl From<ADC_DCCTL7_CTC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DCCTL7_CTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_DCCTL7_CTC` reader - Comparison Trigger Condition"]
pub type ADC_DCCTL7_CTC_R = crate::FieldReader<u8, ADC_DCCTL7_CTC_A>;
impl ADC_DCCTL7_CTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_DCCTL7_CTC_A> {
        match self.bits {
            0 => Some(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_LOW),
            1 => Some(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_MID),
            3 => Some(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTC_LOW`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctc_low(&self) -> bool {
        *self == ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_LOW
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTC_MID`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctc_mid(&self) -> bool {
        *self == ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_MID
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL7_CTC_HIGH`"]
    #[inline(always)]
    pub fn is_adc_dcctl7_ctc_high(&self) -> bool {
        *self == ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_HIGH
    }
}
#[doc = "Field `ADC_DCCTL7_CTC` writer - Comparison Trigger Condition"]
pub type ADC_DCCTL7_CTC_W<'a> =
    crate::FieldWriter<'a, u32, DCCTL7_SPEC, u8, ADC_DCCTL7_CTC_A, 2, 10>;
impl<'a> ADC_DCCTL7_CTC_W<'a> {
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl7_ctc_low(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl7_ctc_mid(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl7_ctc_high(self) -> &'a mut W {
        self.variant(ADC_DCCTL7_CTC_A::ADC_DCCTL7_CTC_HIGH)
    }
}
#[doc = "Field `ADC_DCCTL7_CTE` reader - Comparison Trigger Enable"]
pub type ADC_DCCTL7_CTE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DCCTL7_CTE` writer - Comparison Trigger Enable"]
pub type ADC_DCCTL7_CTE_W<'a> = crate::BitWriter<'a, u32, DCCTL7_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn adc_dcctl7_cim(&self) -> ADC_DCCTL7_CIM_R {
        ADC_DCCTL7_CIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn adc_dcctl7_cic(&self) -> ADC_DCCTL7_CIC_R {
        ADC_DCCTL7_CIC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn adc_dcctl7_cie(&self) -> ADC_DCCTL7_CIE_R {
        ADC_DCCTL7_CIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm(&self) -> ADC_DCCTL7_CTM_R {
        ADC_DCCTL7_CTM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn adc_dcctl7_ctc(&self) -> ADC_DCCTL7_CTC_R {
        ADC_DCCTL7_CTC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn adc_dcctl7_cte(&self) -> ADC_DCCTL7_CTE_R {
        ADC_DCCTL7_CTE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn adc_dcctl7_cim(&mut self) -> ADC_DCCTL7_CIM_W {
        ADC_DCCTL7_CIM_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn adc_dcctl7_cic(&mut self) -> ADC_DCCTL7_CIC_W {
        ADC_DCCTL7_CIC_W::new(self)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn adc_dcctl7_cie(&mut self) -> ADC_DCCTL7_CIE_W {
        ADC_DCCTL7_CIE_W::new(self)
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn adc_dcctl7_ctm(&mut self) -> ADC_DCCTL7_CTM_W {
        ADC_DCCTL7_CTM_W::new(self)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn adc_dcctl7_ctc(&mut self) -> ADC_DCCTL7_CTC_W {
        ADC_DCCTL7_CTC_W::new(self)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn adc_dcctl7_cte(&mut self) -> ADC_DCCTL7_CTE_W {
        ADC_DCCTL7_CTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Digital Comparator Control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcctl7](index.html) module"]
pub struct DCCTL7_SPEC;
impl crate::RegisterSpec for DCCTL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcctl7::R](R) reader structure"]
impl crate::Readable for DCCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcctl7::W](W) writer structure"]
impl crate::Writable for DCCTL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCCTL7 to value 0"]
impl crate::Resettable for DCCTL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
