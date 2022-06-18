#[doc = "Register `USTAT` reader"]
pub struct R(crate::R<USTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USTAT` writer"]
pub struct W(crate::W<USTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USTAT_SPEC>;
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
impl From<crate::W<USTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_USTAT_UV0` reader - SS0 FIFO Underflow"]
pub type ADC_USTAT_UV0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_USTAT_UV0` writer - SS0 FIFO Underflow"]
pub type ADC_USTAT_UV0_W<'a> = crate::BitWriter<'a, u32, USTAT_SPEC, bool, 0>;
#[doc = "Field `ADC_USTAT_UV1` reader - SS1 FIFO Underflow"]
pub type ADC_USTAT_UV1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_USTAT_UV1` writer - SS1 FIFO Underflow"]
pub type ADC_USTAT_UV1_W<'a> = crate::BitWriter<'a, u32, USTAT_SPEC, bool, 1>;
#[doc = "Field `ADC_USTAT_UV2` reader - SS2 FIFO Underflow"]
pub type ADC_USTAT_UV2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_USTAT_UV2` writer - SS2 FIFO Underflow"]
pub type ADC_USTAT_UV2_W<'a> = crate::BitWriter<'a, u32, USTAT_SPEC, bool, 2>;
#[doc = "Field `ADC_USTAT_UV3` reader - SS3 FIFO Underflow"]
pub type ADC_USTAT_UV3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_USTAT_UV3` writer - SS3 FIFO Underflow"]
pub type ADC_USTAT_UV3_W<'a> = crate::BitWriter<'a, u32, USTAT_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&self) -> ADC_USTAT_UV0_R {
        ADC_USTAT_UV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&self) -> ADC_USTAT_UV1_R {
        ADC_USTAT_UV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&self) -> ADC_USTAT_UV2_R {
        ADC_USTAT_UV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&self) -> ADC_USTAT_UV3_R {
        ADC_USTAT_UV3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&mut self) -> ADC_USTAT_UV0_W {
        ADC_USTAT_UV0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&mut self) -> ADC_USTAT_UV1_W {
        ADC_USTAT_UV1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&mut self) -> ADC_USTAT_UV2_W {
        ADC_USTAT_UV2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&mut self) -> ADC_USTAT_UV3_W {
        ADC_USTAT_UV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Underflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ustat](index.html) module"]
pub struct USTAT_SPEC;
impl crate::RegisterSpec for USTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ustat::R](R) reader structure"]
impl crate::Readable for USTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ustat::W](W) writer structure"]
impl crate::Writable for USTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USTAT to value 0"]
impl crate::Resettable for USTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
