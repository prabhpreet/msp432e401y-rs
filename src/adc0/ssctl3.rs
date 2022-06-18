#[doc = "Register `SSCTL3` reader"]
pub struct R(crate::R<SSCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCTL3` writer"]
pub struct W(crate::W<SSCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCTL3_SPEC>;
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
impl From<crate::W<SSCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSCTL3_D0` reader - Sample Differential Input Select"]
pub type ADC_SSCTL3_D0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL3_D0` writer - Sample Differential Input Select"]
pub type ADC_SSCTL3_D0_W<'a> = crate::BitWriter<'a, u32, SSCTL3_SPEC, bool, 0>;
#[doc = "Field `ADC_SSCTL3_END0` reader - End of Sequence"]
pub type ADC_SSCTL3_END0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL3_END0` writer - End of Sequence"]
pub type ADC_SSCTL3_END0_W<'a> = crate::BitWriter<'a, u32, SSCTL3_SPEC, bool, 1>;
#[doc = "Field `ADC_SSCTL3_IE0` reader - Sample Interrupt Enable"]
pub type ADC_SSCTL3_IE0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL3_IE0` writer - Sample Interrupt Enable"]
pub type ADC_SSCTL3_IE0_W<'a> = crate::BitWriter<'a, u32, SSCTL3_SPEC, bool, 2>;
#[doc = "Field `ADC_SSCTL3_TS0` reader - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL3_TS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL3_TS0` writer - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL3_TS0_W<'a> = crate::BitWriter<'a, u32, SSCTL3_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl3_d0(&self) -> ADC_SSCTL3_D0_R {
        ADC_SSCTL3_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl3_end0(&self) -> ADC_SSCTL3_END0_R {
        ADC_SSCTL3_END0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl3_ie0(&self) -> ADC_SSCTL3_IE0_R {
        ADC_SSCTL3_IE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl3_ts0(&self) -> ADC_SSCTL3_TS0_R {
        ADC_SSCTL3_TS0_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl3_d0(&mut self) -> ADC_SSCTL3_D0_W {
        ADC_SSCTL3_D0_W::new(self)
    }
    #[doc = "Bit 1 - End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl3_end0(&mut self) -> ADC_SSCTL3_END0_W {
        ADC_SSCTL3_END0_W::new(self)
    }
    #[doc = "Bit 2 - Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl3_ie0(&mut self) -> ADC_SSCTL3_IE0_W {
        ADC_SSCTL3_IE0_W::new(self)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl3_ts0(&mut self) -> ADC_SSCTL3_TS0_W {
        ADC_SSCTL3_TS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl3](index.html) module"]
pub struct SSCTL3_SPEC;
impl crate::RegisterSpec for SSCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssctl3::R](R) reader structure"]
impl crate::Readable for SSCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssctl3::W](W) writer structure"]
impl crate::Writable for SSCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCTL3 to value 0"]
impl crate::Resettable for SSCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
