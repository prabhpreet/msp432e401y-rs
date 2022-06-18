#[doc = "Register `DCRIC` writer"]
pub struct W(crate::W<DCRIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRIC_SPEC>;
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
impl From<crate::W<DCRIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCRIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_DCRIC_DCINT0` writer - Digital Comparator Interrupt 0"]
pub type ADC_DCRIC_DCINT0_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 0>;
#[doc = "Field `ADC_DCRIC_DCINT1` writer - Digital Comparator Interrupt 1"]
pub type ADC_DCRIC_DCINT1_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 1>;
#[doc = "Field `ADC_DCRIC_DCINT2` writer - Digital Comparator Interrupt 2"]
pub type ADC_DCRIC_DCINT2_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 2>;
#[doc = "Field `ADC_DCRIC_DCINT3` writer - Digital Comparator Interrupt 3"]
pub type ADC_DCRIC_DCINT3_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 3>;
#[doc = "Field `ADC_DCRIC_DCINT4` writer - Digital Comparator Interrupt 4"]
pub type ADC_DCRIC_DCINT4_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 4>;
#[doc = "Field `ADC_DCRIC_DCINT5` writer - Digital Comparator Interrupt 5"]
pub type ADC_DCRIC_DCINT5_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 5>;
#[doc = "Field `ADC_DCRIC_DCINT6` writer - Digital Comparator Interrupt 6"]
pub type ADC_DCRIC_DCINT6_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 6>;
#[doc = "Field `ADC_DCRIC_DCINT7` writer - Digital Comparator Interrupt 7"]
pub type ADC_DCRIC_DCINT7_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 7>;
#[doc = "Field `ADC_DCRIC_DCTRIG0` writer - Digital Comparator Trigger 0"]
pub type ADC_DCRIC_DCTRIG0_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 16>;
#[doc = "Field `ADC_DCRIC_DCTRIG1` writer - Digital Comparator Trigger 1"]
pub type ADC_DCRIC_DCTRIG1_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 17>;
#[doc = "Field `ADC_DCRIC_DCTRIG2` writer - Digital Comparator Trigger 2"]
pub type ADC_DCRIC_DCTRIG2_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 18>;
#[doc = "Field `ADC_DCRIC_DCTRIG3` writer - Digital Comparator Trigger 3"]
pub type ADC_DCRIC_DCTRIG3_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 19>;
#[doc = "Field `ADC_DCRIC_DCTRIG4` writer - Digital Comparator Trigger 4"]
pub type ADC_DCRIC_DCTRIG4_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 20>;
#[doc = "Field `ADC_DCRIC_DCTRIG5` writer - Digital Comparator Trigger 5"]
pub type ADC_DCRIC_DCTRIG5_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 21>;
#[doc = "Field `ADC_DCRIC_DCTRIG6` writer - Digital Comparator Trigger 6"]
pub type ADC_DCRIC_DCTRIG6_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 22>;
#[doc = "Field `ADC_DCRIC_DCTRIG7` writer - Digital Comparator Trigger 7"]
pub type ADC_DCRIC_DCTRIG7_W<'a> = crate::BitWriter<'a, u32, DCRIC_SPEC, bool, 23>;
impl W {
    #[doc = "Bit 0 - Digital Comparator Interrupt 0"]
    #[inline(always)]
    pub fn adc_dcric_dcint0(&mut self) -> ADC_DCRIC_DCINT0_W {
        ADC_DCRIC_DCINT0_W::new(self)
    }
    #[doc = "Bit 1 - Digital Comparator Interrupt 1"]
    #[inline(always)]
    pub fn adc_dcric_dcint1(&mut self) -> ADC_DCRIC_DCINT1_W {
        ADC_DCRIC_DCINT1_W::new(self)
    }
    #[doc = "Bit 2 - Digital Comparator Interrupt 2"]
    #[inline(always)]
    pub fn adc_dcric_dcint2(&mut self) -> ADC_DCRIC_DCINT2_W {
        ADC_DCRIC_DCINT2_W::new(self)
    }
    #[doc = "Bit 3 - Digital Comparator Interrupt 3"]
    #[inline(always)]
    pub fn adc_dcric_dcint3(&mut self) -> ADC_DCRIC_DCINT3_W {
        ADC_DCRIC_DCINT3_W::new(self)
    }
    #[doc = "Bit 4 - Digital Comparator Interrupt 4"]
    #[inline(always)]
    pub fn adc_dcric_dcint4(&mut self) -> ADC_DCRIC_DCINT4_W {
        ADC_DCRIC_DCINT4_W::new(self)
    }
    #[doc = "Bit 5 - Digital Comparator Interrupt 5"]
    #[inline(always)]
    pub fn adc_dcric_dcint5(&mut self) -> ADC_DCRIC_DCINT5_W {
        ADC_DCRIC_DCINT5_W::new(self)
    }
    #[doc = "Bit 6 - Digital Comparator Interrupt 6"]
    #[inline(always)]
    pub fn adc_dcric_dcint6(&mut self) -> ADC_DCRIC_DCINT6_W {
        ADC_DCRIC_DCINT6_W::new(self)
    }
    #[doc = "Bit 7 - Digital Comparator Interrupt 7"]
    #[inline(always)]
    pub fn adc_dcric_dcint7(&mut self) -> ADC_DCRIC_DCINT7_W {
        ADC_DCRIC_DCINT7_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Trigger 0"]
    #[inline(always)]
    pub fn adc_dcric_dctrig0(&mut self) -> ADC_DCRIC_DCTRIG0_W {
        ADC_DCRIC_DCTRIG0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Trigger 1"]
    #[inline(always)]
    pub fn adc_dcric_dctrig1(&mut self) -> ADC_DCRIC_DCTRIG1_W {
        ADC_DCRIC_DCTRIG1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Trigger 2"]
    #[inline(always)]
    pub fn adc_dcric_dctrig2(&mut self) -> ADC_DCRIC_DCTRIG2_W {
        ADC_DCRIC_DCTRIG2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Trigger 3"]
    #[inline(always)]
    pub fn adc_dcric_dctrig3(&mut self) -> ADC_DCRIC_DCTRIG3_W {
        ADC_DCRIC_DCTRIG3_W::new(self)
    }
    #[doc = "Bit 20 - Digital Comparator Trigger 4"]
    #[inline(always)]
    pub fn adc_dcric_dctrig4(&mut self) -> ADC_DCRIC_DCTRIG4_W {
        ADC_DCRIC_DCTRIG4_W::new(self)
    }
    #[doc = "Bit 21 - Digital Comparator Trigger 5"]
    #[inline(always)]
    pub fn adc_dcric_dctrig5(&mut self) -> ADC_DCRIC_DCTRIG5_W {
        ADC_DCRIC_DCTRIG5_W::new(self)
    }
    #[doc = "Bit 22 - Digital Comparator Trigger 6"]
    #[inline(always)]
    pub fn adc_dcric_dctrig6(&mut self) -> ADC_DCRIC_DCTRIG6_W {
        ADC_DCRIC_DCTRIG6_W::new(self)
    }
    #[doc = "Bit 23 - Digital Comparator Trigger 7"]
    #[inline(always)]
    pub fn adc_dcric_dctrig7(&mut self) -> ADC_DCRIC_DCTRIG7_W {
        ADC_DCRIC_DCTRIG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Digital Comparator Reset Initial Conditions\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcric](index.html) module"]
pub struct DCRIC_SPEC;
impl crate::RegisterSpec for DCRIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcric::W](W) writer structure"]
impl crate::Writable for DCRIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRIC to value 0"]
impl crate::Resettable for DCRIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
