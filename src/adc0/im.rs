#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_IM_MASK0` reader - SS0 Interrupt Mask"]
pub type ADC_IM_MASK0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_MASK0` writer - SS0 Interrupt Mask"]
pub type ADC_IM_MASK0_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `ADC_IM_MASK1` reader - SS1 Interrupt Mask"]
pub type ADC_IM_MASK1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_MASK1` writer - SS1 Interrupt Mask"]
pub type ADC_IM_MASK1_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 1>;
#[doc = "Field `ADC_IM_MASK2` reader - SS2 Interrupt Mask"]
pub type ADC_IM_MASK2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_MASK2` writer - SS2 Interrupt Mask"]
pub type ADC_IM_MASK2_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `ADC_IM_MASK3` reader - SS3 Interrupt Mask"]
pub type ADC_IM_MASK3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_MASK3` writer - SS3 Interrupt Mask"]
pub type ADC_IM_MASK3_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `ADC_IM_DMAMASK0` reader - SS0 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DMAMASK0` writer - SS0 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK0_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 8>;
#[doc = "Field `ADC_IM_DMAMASK1` reader - SS1 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DMAMASK1` writer - SS1 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK1_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 9>;
#[doc = "Field `ADC_IM_DMAMASK2` reader - SS2 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DMAMASK2` writer - SS2 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK2_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 10>;
#[doc = "Field `ADC_IM_DMAMASK3` reader - SS3 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DMAMASK3` writer - SS3 DMA Interrupt Mask"]
pub type ADC_IM_DMAMASK3_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 11>;
#[doc = "Field `ADC_IM_DCONSS0` reader - Digital Comparator Interrupt on SS0"]
pub type ADC_IM_DCONSS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DCONSS0` writer - Digital Comparator Interrupt on SS0"]
pub type ADC_IM_DCONSS0_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 16>;
#[doc = "Field `ADC_IM_DCONSS1` reader - Digital Comparator Interrupt on SS1"]
pub type ADC_IM_DCONSS1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DCONSS1` writer - Digital Comparator Interrupt on SS1"]
pub type ADC_IM_DCONSS1_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 17>;
#[doc = "Field `ADC_IM_DCONSS2` reader - Digital Comparator Interrupt on SS2"]
pub type ADC_IM_DCONSS2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DCONSS2` writer - Digital Comparator Interrupt on SS2"]
pub type ADC_IM_DCONSS2_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 18>;
#[doc = "Field `ADC_IM_DCONSS3` reader - Digital Comparator Interrupt on SS3"]
pub type ADC_IM_DCONSS3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IM_DCONSS3` writer - Digital Comparator Interrupt on SS3"]
pub type ADC_IM_DCONSS3_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&self) -> ADC_IM_MASK0_R {
        ADC_IM_MASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&self) -> ADC_IM_MASK1_R {
        ADC_IM_MASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&self) -> ADC_IM_MASK2_R {
        ADC_IM_MASK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&self) -> ADC_IM_MASK3_R {
        ADC_IM_MASK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask0(&self) -> ADC_IM_DMAMASK0_R {
        ADC_IM_DMAMASK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask1(&self) -> ADC_IM_DMAMASK1_R {
        ADC_IM_DMAMASK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask2(&self) -> ADC_IM_DMAMASK2_R {
        ADC_IM_DMAMASK2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask3(&self) -> ADC_IM_DMAMASK3_R {
        ADC_IM_DMAMASK3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn adc_im_dconss0(&self) -> ADC_IM_DCONSS0_R {
        ADC_IM_DCONSS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn adc_im_dconss1(&self) -> ADC_IM_DCONSS1_R {
        ADC_IM_DCONSS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn adc_im_dconss2(&self) -> ADC_IM_DCONSS2_R {
        ADC_IM_DCONSS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn adc_im_dconss3(&self) -> ADC_IM_DCONSS3_R {
        ADC_IM_DCONSS3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&mut self) -> ADC_IM_MASK0_W {
        ADC_IM_MASK0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&mut self) -> ADC_IM_MASK1_W {
        ADC_IM_MASK1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&mut self) -> ADC_IM_MASK2_W {
        ADC_IM_MASK2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&mut self) -> ADC_IM_MASK3_W {
        ADC_IM_MASK3_W::new(self)
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask0(&mut self) -> ADC_IM_DMAMASK0_W {
        ADC_IM_DMAMASK0_W::new(self)
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask1(&mut self) -> ADC_IM_DMAMASK1_W {
        ADC_IM_DMAMASK1_W::new(self)
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask2(&mut self) -> ADC_IM_DMAMASK2_W {
        ADC_IM_DMAMASK2_W::new(self)
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask3(&mut self) -> ADC_IM_DMAMASK3_W {
        ADC_IM_DMAMASK3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn adc_im_dconss0(&mut self) -> ADC_IM_DCONSS0_W {
        ADC_IM_DCONSS0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn adc_im_dconss1(&mut self) -> ADC_IM_DCONSS1_W {
        ADC_IM_DCONSS1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn adc_im_dconss2(&mut self) -> ADC_IM_DCONSS2_W {
        ADC_IM_DCONSS2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn adc_im_dconss3(&mut self) -> ADC_IM_DCONSS3_W {
        ADC_IM_DCONSS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
