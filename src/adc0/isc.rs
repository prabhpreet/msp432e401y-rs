#[doc = "Register `ISC` reader"]
pub struct R(crate::R<ISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISC` writer"]
pub struct W(crate::W<ISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISC_SPEC>;
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
impl From<crate::W<ISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ISC_IN0` reader - SS0 Interrupt Status and Clear"]
pub type ADC_ISC_IN0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_IN0` writer - SS0 Interrupt Status and Clear"]
pub type ADC_ISC_IN0_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 0>;
#[doc = "Field `ADC_ISC_IN1` reader - SS1 Interrupt Status and Clear"]
pub type ADC_ISC_IN1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_IN1` writer - SS1 Interrupt Status and Clear"]
pub type ADC_ISC_IN1_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 1>;
#[doc = "Field `ADC_ISC_IN2` reader - SS2 Interrupt Status and Clear"]
pub type ADC_ISC_IN2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_IN2` writer - SS2 Interrupt Status and Clear"]
pub type ADC_ISC_IN2_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 2>;
#[doc = "Field `ADC_ISC_IN3` reader - SS3 Interrupt Status and Clear"]
pub type ADC_ISC_IN3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_IN3` writer - SS3 Interrupt Status and Clear"]
pub type ADC_ISC_IN3_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 3>;
#[doc = "Field `ADC_ISC_DMAIN0` reader - SS0 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DMAIN0` writer - SS0 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN0_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 8>;
#[doc = "Field `ADC_ISC_DMAIN1` reader - SS1 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DMAIN1` writer - SS1 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN1_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 9>;
#[doc = "Field `ADC_ISC_DMAIN2` reader - SS2 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DMAIN2` writer - SS2 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN2_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 10>;
#[doc = "Field `ADC_ISC_DMAIN3` reader - SS3 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DMAIN3` writer - SS3 DMA Interrupt Status and Clear"]
pub type ADC_ISC_DMAIN3_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 11>;
#[doc = "Field `ADC_ISC_DCINSS0` reader - Digital Comparator Interrupt Status on SS0"]
pub type ADC_ISC_DCINSS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DCINSS0` writer - Digital Comparator Interrupt Status on SS0"]
pub type ADC_ISC_DCINSS0_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 16>;
#[doc = "Field `ADC_ISC_DCINSS1` reader - Digital Comparator Interrupt Status on SS1"]
pub type ADC_ISC_DCINSS1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DCINSS1` writer - Digital Comparator Interrupt Status on SS1"]
pub type ADC_ISC_DCINSS1_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 17>;
#[doc = "Field `ADC_ISC_DCINSS2` reader - Digital Comparator Interrupt Status on SS2"]
pub type ADC_ISC_DCINSS2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DCINSS2` writer - Digital Comparator Interrupt Status on SS2"]
pub type ADC_ISC_DCINSS2_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 18>;
#[doc = "Field `ADC_ISC_DCINSS3` reader - Digital Comparator Interrupt Status on SS3"]
pub type ADC_ISC_DCINSS3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ISC_DCINSS3` writer - Digital Comparator Interrupt Status on SS3"]
pub type ADC_ISC_DCINSS3_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&self) -> ADC_ISC_IN0_R {
        ADC_ISC_IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&self) -> ADC_ISC_IN1_R {
        ADC_ISC_IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&self) -> ADC_ISC_IN2_R {
        ADC_ISC_IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&self) -> ADC_ISC_IN3_R {
        ADC_ISC_IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain0(&self) -> ADC_ISC_DMAIN0_R {
        ADC_ISC_DMAIN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain1(&self) -> ADC_ISC_DMAIN1_R {
        ADC_ISC_DMAIN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain2(&self) -> ADC_ISC_DMAIN2_R {
        ADC_ISC_DMAIN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain3(&self) -> ADC_ISC_DMAIN3_R {
        ADC_ISC_DMAIN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn adc_isc_dcinss0(&self) -> ADC_ISC_DCINSS0_R {
        ADC_ISC_DCINSS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn adc_isc_dcinss1(&self) -> ADC_ISC_DCINSS1_R {
        ADC_ISC_DCINSS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn adc_isc_dcinss2(&self) -> ADC_ISC_DCINSS2_R {
        ADC_ISC_DCINSS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn adc_isc_dcinss3(&self) -> ADC_ISC_DCINSS3_R {
        ADC_ISC_DCINSS3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&mut self) -> ADC_ISC_IN0_W {
        ADC_ISC_IN0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&mut self) -> ADC_ISC_IN1_W {
        ADC_ISC_IN1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&mut self) -> ADC_ISC_IN2_W {
        ADC_ISC_IN2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&mut self) -> ADC_ISC_IN3_W {
        ADC_ISC_IN3_W::new(self)
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain0(&mut self) -> ADC_ISC_DMAIN0_W {
        ADC_ISC_DMAIN0_W::new(self)
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain1(&mut self) -> ADC_ISC_DMAIN1_W {
        ADC_ISC_DMAIN1_W::new(self)
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain2(&mut self) -> ADC_ISC_DMAIN2_W {
        ADC_ISC_DMAIN2_W::new(self)
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain3(&mut self) -> ADC_ISC_DMAIN3_W {
        ADC_ISC_DMAIN3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn adc_isc_dcinss0(&mut self) -> ADC_ISC_DCINSS0_W {
        ADC_ISC_DCINSS0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn adc_isc_dcinss1(&mut self) -> ADC_ISC_DCINSS1_W {
        ADC_ISC_DCINSS1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn adc_isc_dcinss2(&mut self) -> ADC_ISC_DCINSS2_W {
        ADC_ISC_DCINSS2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn adc_isc_dcinss3(&mut self) -> ADC_ISC_DCINSS3_W {
        ADC_ISC_DCINSS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](index.html) module"]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isc::R](R) reader structure"]
impl crate::Readable for ISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isc::W](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
