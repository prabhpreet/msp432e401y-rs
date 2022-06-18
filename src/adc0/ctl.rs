#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Voltage Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CTL_VREF_A {
    #[doc = "0: VDDA and GNDA are the voltage references"]
    ADC_CTL_VREF_INTERNAL = 0,
    #[doc = "1: The external VREFA+ and VREFA- inputs are the voltage references"]
    ADC_CTL_VREF_EXT_3V = 1,
}
impl From<ADC_CTL_VREF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CTL_VREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_CTL_VREF` reader - Voltage Reference Select"]
pub type ADC_CTL_VREF_R = crate::BitReader<ADC_CTL_VREF_A>;
impl ADC_CTL_VREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CTL_VREF_A {
        match self.bits {
            false => ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL,
            true => ADC_CTL_VREF_A::ADC_CTL_VREF_EXT_3V,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_CTL_VREF_INTERNAL`"]
    #[inline(always)]
    pub fn is_adc_ctl_vref_internal(&self) -> bool {
        *self == ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_CTL_VREF_EXT_3V`"]
    #[inline(always)]
    pub fn is_adc_ctl_vref_ext_3v(&self) -> bool {
        *self == ADC_CTL_VREF_A::ADC_CTL_VREF_EXT_3V
    }
}
#[doc = "Field `ADC_CTL_VREF` writer - Voltage Reference Select"]
pub type ADC_CTL_VREF_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, ADC_CTL_VREF_A, 0>;
impl<'a> ADC_CTL_VREF_W<'a> {
    #[doc = "VDDA and GNDA are the voltage references"]
    #[inline(always)]
    pub fn adc_ctl_vref_internal(self) -> &'a mut W {
        self.variant(ADC_CTL_VREF_A::ADC_CTL_VREF_INTERNAL)
    }
    #[doc = "The external VREFA+ and VREFA- inputs are the voltage references"]
    #[inline(always)]
    pub fn adc_ctl_vref_ext_3v(self) -> &'a mut W {
        self.variant(ADC_CTL_VREF_A::ADC_CTL_VREF_EXT_3V)
    }
}
#[doc = "Field `ADC_CTL_DITHER` reader - Dither Mode Enable"]
pub type ADC_CTL_DITHER_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CTL_DITHER` writer - Dither Mode Enable"]
pub type ADC_CTL_DITHER_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - Voltage Reference Select"]
    #[inline(always)]
    pub fn adc_ctl_vref(&self) -> ADC_CTL_VREF_R {
        ADC_CTL_VREF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Dither Mode Enable"]
    #[inline(always)]
    pub fn adc_ctl_dither(&self) -> ADC_CTL_DITHER_R {
        ADC_CTL_DITHER_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Reference Select"]
    #[inline(always)]
    pub fn adc_ctl_vref(&mut self) -> ADC_CTL_VREF_W {
        ADC_CTL_VREF_W::new(self)
    }
    #[doc = "Bit 6 - Dither Mode Enable"]
    #[inline(always)]
    pub fn adc_ctl_dither(&mut self) -> ADC_CTL_DITHER_W {
        ADC_CTL_DITHER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
