#[doc = "Register `OSTAT` reader"]
pub struct R(crate::R<OSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTAT` writer"]
pub struct W(crate::W<OSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTAT_SPEC>;
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
impl From<crate::W<OSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_OSTAT_OV0` reader - SS0 FIFO Overflow"]
pub type ADC_OSTAT_OV0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_OSTAT_OV0` writer - SS0 FIFO Overflow"]
pub type ADC_OSTAT_OV0_W<'a> = crate::BitWriter<'a, u32, OSTAT_SPEC, bool, 0>;
#[doc = "Field `ADC_OSTAT_OV1` reader - SS1 FIFO Overflow"]
pub type ADC_OSTAT_OV1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_OSTAT_OV1` writer - SS1 FIFO Overflow"]
pub type ADC_OSTAT_OV1_W<'a> = crate::BitWriter<'a, u32, OSTAT_SPEC, bool, 1>;
#[doc = "Field `ADC_OSTAT_OV2` reader - SS2 FIFO Overflow"]
pub type ADC_OSTAT_OV2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_OSTAT_OV2` writer - SS2 FIFO Overflow"]
pub type ADC_OSTAT_OV2_W<'a> = crate::BitWriter<'a, u32, OSTAT_SPEC, bool, 2>;
#[doc = "Field `ADC_OSTAT_OV3` reader - SS3 FIFO Overflow"]
pub type ADC_OSTAT_OV3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_OSTAT_OV3` writer - SS3 FIFO Overflow"]
pub type ADC_OSTAT_OV3_W<'a> = crate::BitWriter<'a, u32, OSTAT_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov0(&self) -> ADC_OSTAT_OV0_R {
        ADC_OSTAT_OV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov1(&self) -> ADC_OSTAT_OV1_R {
        ADC_OSTAT_OV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov2(&self) -> ADC_OSTAT_OV2_R {
        ADC_OSTAT_OV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov3(&self) -> ADC_OSTAT_OV3_R {
        ADC_OSTAT_OV3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov0(&mut self) -> ADC_OSTAT_OV0_W {
        ADC_OSTAT_OV0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov1(&mut self) -> ADC_OSTAT_OV1_W {
        ADC_OSTAT_OV1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov2(&mut self) -> ADC_OSTAT_OV2_W {
        ADC_OSTAT_OV2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 FIFO Overflow"]
    #[inline(always)]
    pub fn adc_ostat_ov3(&mut self) -> ADC_OSTAT_OV3_W {
        ADC_OSTAT_OV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Overflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostat](index.html) module"]
pub struct OSTAT_SPEC;
impl crate::RegisterSpec for OSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ostat::R](R) reader structure"]
impl crate::Readable for OSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostat::W](W) writer structure"]
impl crate::Writable for OSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSTAT to value 0"]
impl crate::Resettable for OSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
