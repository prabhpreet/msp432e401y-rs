#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_RIS_INR0` reader - SS0 Raw Interrupt Status"]
pub type ADC_RIS_INR0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_INR0` writer - SS0 Raw Interrupt Status"]
pub type ADC_RIS_INR0_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `ADC_RIS_INR1` reader - SS1 Raw Interrupt Status"]
pub type ADC_RIS_INR1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_INR1` writer - SS1 Raw Interrupt Status"]
pub type ADC_RIS_INR1_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `ADC_RIS_INR2` reader - SS2 Raw Interrupt Status"]
pub type ADC_RIS_INR2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_INR2` writer - SS2 Raw Interrupt Status"]
pub type ADC_RIS_INR2_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `ADC_RIS_INR3` reader - SS3 Raw Interrupt Status"]
pub type ADC_RIS_INR3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_INR3` writer - SS3 Raw Interrupt Status"]
pub type ADC_RIS_INR3_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `ADC_RIS_DMAINR0` reader - SS0 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_DMAINR0` writer - SS0 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR0_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 8>;
#[doc = "Field `ADC_RIS_DMAINR1` reader - SS1 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_DMAINR1` writer - SS1 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR1_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 9>;
#[doc = "Field `ADC_RIS_DMAINR2` reader - SS2 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_DMAINR2` writer - SS2 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR2_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 10>;
#[doc = "Field `ADC_RIS_DMAINR3` reader - SS3 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_DMAINR3` writer - SS3 DMA Raw Interrupt Status"]
pub type ADC_RIS_DMAINR3_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 11>;
#[doc = "Field `ADC_RIS_INRDC` reader - Digital Comparator Raw Interrupt Status"]
pub type ADC_RIS_INRDC_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RIS_INRDC` writer - Digital Comparator Raw Interrupt Status"]
pub type ADC_RIS_INRDC_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&self) -> ADC_RIS_INR0_R {
        ADC_RIS_INR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&self) -> ADC_RIS_INR1_R {
        ADC_RIS_INR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&self) -> ADC_RIS_INR2_R {
        ADC_RIS_INR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&self) -> ADC_RIS_INR3_R {
        ADC_RIS_INR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SS0 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr0(&self) -> ADC_RIS_DMAINR0_R {
        ADC_RIS_DMAINR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SS1 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr1(&self) -> ADC_RIS_DMAINR1_R {
        ADC_RIS_DMAINR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SS2 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr2(&self) -> ADC_RIS_DMAINR2_R {
        ADC_RIS_DMAINR2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SS3 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr3(&self) -> ADC_RIS_DMAINR3_R {
        ADC_RIS_DMAINR3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inrdc(&self) -> ADC_RIS_INRDC_R {
        ADC_RIS_INRDC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&mut self) -> ADC_RIS_INR0_W {
        ADC_RIS_INR0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&mut self) -> ADC_RIS_INR1_W {
        ADC_RIS_INR1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&mut self) -> ADC_RIS_INR2_W {
        ADC_RIS_INR2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&mut self) -> ADC_RIS_INR3_W {
        ADC_RIS_INR3_W::new(self)
    }
    #[doc = "Bit 8 - SS0 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr0(&mut self) -> ADC_RIS_DMAINR0_W {
        ADC_RIS_DMAINR0_W::new(self)
    }
    #[doc = "Bit 9 - SS1 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr1(&mut self) -> ADC_RIS_DMAINR1_W {
        ADC_RIS_DMAINR1_W::new(self)
    }
    #[doc = "Bit 10 - SS2 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr2(&mut self) -> ADC_RIS_DMAINR2_W {
        ADC_RIS_DMAINR2_W::new(self)
    }
    #[doc = "Bit 11 - SS3 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr3(&mut self) -> ADC_RIS_DMAINR3_W {
        ADC_RIS_DMAINR3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inrdc(&mut self) -> ADC_RIS_INRDC_W {
        ADC_RIS_INRDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
