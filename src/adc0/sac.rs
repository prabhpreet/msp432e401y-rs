#[doc = "Register `SAC` reader"]
pub struct R(crate::R<SAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAC` writer"]
pub struct W(crate::W<SAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAC_SPEC>;
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
impl From<crate::W<SAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hardware Averaging Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SAC_AVG_A {
    #[doc = "0: No hardware oversampling"]
    ADC_SAC_AVG_OFF = 0,
    #[doc = "1: 2x hardware oversampling"]
    ADC_SAC_AVG_2X = 1,
    #[doc = "2: 4x hardware oversampling"]
    ADC_SAC_AVG_4X = 2,
    #[doc = "3: 8x hardware oversampling"]
    ADC_SAC_AVG_8X = 3,
    #[doc = "4: 16x hardware oversampling"]
    ADC_SAC_AVG_16X = 4,
    #[doc = "5: 32x hardware oversampling"]
    ADC_SAC_AVG_32X = 5,
    #[doc = "6: 64x hardware oversampling"]
    ADC_SAC_AVG_64X = 6,
}
impl From<ADC_SAC_AVG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SAC_AVG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_SAC_AVG` reader - Hardware Averaging Control"]
pub type ADC_SAC_AVG_R = crate::FieldReader<u8, ADC_SAC_AVG_A>;
impl ADC_SAC_AVG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_SAC_AVG_A> {
        match self.bits {
            0 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF),
            1 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_2X),
            2 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_4X),
            3 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_8X),
            4 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_16X),
            5 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_32X),
            6 => Some(ADC_SAC_AVG_A::ADC_SAC_AVG_64X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_OFF`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_off(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_OFF
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_2X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_2x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_2X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_4X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_4x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_4X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_8X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_8x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_8X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_16X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_16x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_16X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_32X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_32x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_32X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_64X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_64x(&self) -> bool {
        *self == ADC_SAC_AVG_A::ADC_SAC_AVG_64X
    }
}
#[doc = "Field `ADC_SAC_AVG` writer - Hardware Averaging Control"]
pub type ADC_SAC_AVG_W<'a> = crate::FieldWriter<'a, u32, SAC_SPEC, u8, ADC_SAC_AVG_A, 3, 0>;
impl<'a> ADC_SAC_AVG_W<'a> {
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_off(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_2x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_4x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_8x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_16x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_32x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_64x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVG_A::ADC_SAC_AVG_64X)
    }
}
impl R {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&self) -> ADC_SAC_AVG_R {
        ADC_SAC_AVG_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&mut self) -> ADC_SAC_AVG_W {
        ADC_SAC_AVG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Averaging Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac](index.html) module"]
pub struct SAC_SPEC;
impl crate::RegisterSpec for SAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sac::R](R) reader structure"]
impl crate::Readable for SAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sac::W](W) writer structure"]
impl crate::Writable for SAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAC to value 0"]
impl crate::Resettable for SAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
