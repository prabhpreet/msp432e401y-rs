#[doc = "Register `ACTSS` reader"]
pub struct R(crate::R<ACTSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTSS` writer"]
pub struct W(crate::W<ACTSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTSS_SPEC>;
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
impl From<crate::W<ACTSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_ACTSS_ASEN0` reader - ADC SS0 Enable"]
pub type ADC_ACTSS_ASEN0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ASEN0` writer - ADC SS0 Enable"]
pub type ADC_ACTSS_ASEN0_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 0>;
#[doc = "Field `ADC_ACTSS_ASEN1` reader - ADC SS1 Enable"]
pub type ADC_ACTSS_ASEN1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ASEN1` writer - ADC SS1 Enable"]
pub type ADC_ACTSS_ASEN1_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 1>;
#[doc = "Field `ADC_ACTSS_ASEN2` reader - ADC SS2 Enable"]
pub type ADC_ACTSS_ASEN2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ASEN2` writer - ADC SS2 Enable"]
pub type ADC_ACTSS_ASEN2_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 2>;
#[doc = "Field `ADC_ACTSS_ASEN3` reader - ADC SS3 Enable"]
pub type ADC_ACTSS_ASEN3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ASEN3` writer - ADC SS3 Enable"]
pub type ADC_ACTSS_ASEN3_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 3>;
#[doc = "Field `ADC_ACTSS_ADEN0` reader - ADC SS1 DMA Enable"]
pub type ADC_ACTSS_ADEN0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ADEN0` writer - ADC SS1 DMA Enable"]
pub type ADC_ACTSS_ADEN0_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 8>;
#[doc = "Field `ADC_ACTSS_ADEN1` reader - ADC SS1 DMA Enable"]
pub type ADC_ACTSS_ADEN1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ADEN1` writer - ADC SS1 DMA Enable"]
pub type ADC_ACTSS_ADEN1_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 9>;
#[doc = "Field `ADC_ACTSS_ADEN2` reader - ADC SS2 DMA Enable"]
pub type ADC_ACTSS_ADEN2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ADEN2` writer - ADC SS2 DMA Enable"]
pub type ADC_ACTSS_ADEN2_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 10>;
#[doc = "Field `ADC_ACTSS_ADEN3` reader - ADC SS3 DMA Enable"]
pub type ADC_ACTSS_ADEN3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_ADEN3` writer - ADC SS3 DMA Enable"]
pub type ADC_ACTSS_ADEN3_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 11>;
#[doc = "Field `ADC_ACTSS_BUSY` reader - ADC Busy"]
pub type ADC_ACTSS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ADC_ACTSS_BUSY` writer - ADC Busy"]
pub type ADC_ACTSS_BUSY_W<'a> = crate::BitWriter<'a, u32, ACTSS_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&self) -> ADC_ACTSS_ASEN0_R {
        ADC_ACTSS_ASEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&self) -> ADC_ACTSS_ASEN1_R {
        ADC_ACTSS_ASEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&self) -> ADC_ACTSS_ASEN2_R {
        ADC_ACTSS_ASEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&self) -> ADC_ACTSS_ASEN3_R {
        ADC_ACTSS_ASEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden0(&self) -> ADC_ACTSS_ADEN0_R {
        ADC_ACTSS_ADEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden1(&self) -> ADC_ACTSS_ADEN1_R {
        ADC_ACTSS_ADEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden2(&self) -> ADC_ACTSS_ADEN2_R {
        ADC_ACTSS_ADEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden3(&self) -> ADC_ACTSS_ADEN3_R {
        ADC_ACTSS_ADEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn adc_actss_busy(&self) -> ADC_ACTSS_BUSY_R {
        ADC_ACTSS_BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&mut self) -> ADC_ACTSS_ASEN0_W {
        ADC_ACTSS_ASEN0_W::new(self)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&mut self) -> ADC_ACTSS_ASEN1_W {
        ADC_ACTSS_ASEN1_W::new(self)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&mut self) -> ADC_ACTSS_ASEN2_W {
        ADC_ACTSS_ASEN2_W::new(self)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&mut self) -> ADC_ACTSS_ASEN3_W {
        ADC_ACTSS_ASEN3_W::new(self)
    }
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden0(&mut self) -> ADC_ACTSS_ADEN0_W {
        ADC_ACTSS_ADEN0_W::new(self)
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden1(&mut self) -> ADC_ACTSS_ADEN1_W {
        ADC_ACTSS_ADEN1_W::new(self)
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden2(&mut self) -> ADC_ACTSS_ADEN2_W {
        ADC_ACTSS_ADEN2_W::new(self)
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden3(&mut self) -> ADC_ACTSS_ADEN3_W {
        ADC_ACTSS_ADEN3_W::new(self)
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn adc_actss_busy(&mut self) -> ADC_ACTSS_BUSY_W {
        ADC_ACTSS_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Active Sample Sequencer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actss](index.html) module"]
pub struct ACTSS_SPEC;
impl crate::RegisterSpec for ACTSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actss::R](R) reader structure"]
impl crate::Readable for ACTSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actss::W](W) writer structure"]
impl crate::Writable for ACTSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTSS to value 0"]
impl crate::Resettable for ACTSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
