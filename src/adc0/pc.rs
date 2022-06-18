#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Conversion Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_PC_MCR_A {
    #[doc = "1: Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_8 = 1,
    #[doc = "3: Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_4 = 3,
    #[doc = "5: Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_2 = 5,
    #[doc = "7: Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PC_MCR_FULL = 7,
}
impl From<ADC_PC_MCR_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_PC_MCR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC_PC_MCR` reader - Conversion Rate"]
pub type ADC_PC_MCR_R = crate::FieldReader<u8, ADC_PC_MCR_A>;
impl ADC_PC_MCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC_PC_MCR_A> {
        match self.bits {
            1 => Some(ADC_PC_MCR_A::ADC_PC_MCR_1_8),
            3 => Some(ADC_PC_MCR_A::ADC_PC_MCR_1_4),
            5 => Some(ADC_PC_MCR_A::ADC_PC_MCR_1_2),
            7 => Some(ADC_PC_MCR_A::ADC_PC_MCR_FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_8`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_8(&self) -> bool {
        *self == ADC_PC_MCR_A::ADC_PC_MCR_1_8
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_4`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_4(&self) -> bool {
        *self == ADC_PC_MCR_A::ADC_PC_MCR_1_4
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_2`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_2(&self) -> bool {
        *self == ADC_PC_MCR_A::ADC_PC_MCR_1_2
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_FULL`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_full(&self) -> bool {
        *self == ADC_PC_MCR_A::ADC_PC_MCR_FULL
    }
}
#[doc = "Field `ADC_PC_MCR` writer - Conversion Rate"]
pub type ADC_PC_MCR_W<'a> = crate::FieldWriter<'a, u32, PC_SPEC, u8, ADC_PC_MCR_A, 4, 0>;
impl<'a> ADC_PC_MCR_W<'a> {
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_8(self) -> &'a mut W {
        self.variant(ADC_PC_MCR_A::ADC_PC_MCR_1_8)
    }
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_4(self) -> &'a mut W {
        self.variant(ADC_PC_MCR_A::ADC_PC_MCR_1_4)
    }
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_2(self) -> &'a mut W {
        self.variant(ADC_PC_MCR_A::ADC_PC_MCR_1_2)
    }
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline(always)]
    pub fn adc_pc_mcr_full(self) -> &'a mut W {
        self.variant(ADC_PC_MCR_A::ADC_PC_MCR_FULL)
    }
}
impl R {
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn adc_pc_mcr(&self) -> ADC_PC_MCR_R {
        ADC_PC_MCR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn adc_pc_mcr(&mut self) -> ADC_PC_MCR_W {
        ADC_PC_MCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
