#[doc = "Register `SSMUX0` reader"]
pub struct R(crate::R<SSMUX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSMUX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSMUX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSMUX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSMUX0` writer"]
pub struct W(crate::W<SSMUX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSMUX0_SPEC>;
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
impl From<crate::W<SSMUX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSMUX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSMUX0_MUX0` reader - 1st Sample Input Select"]
pub type ADC_SSMUX0_MUX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX0` writer - 1st Sample Input Select"]
pub type ADC_SSMUX0_MUX0_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADC_SSMUX0_MUX1` reader - 2nd Sample Input Select"]
pub type ADC_SSMUX0_MUX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX1` writer - 2nd Sample Input Select"]
pub type ADC_SSMUX0_MUX1_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADC_SSMUX0_MUX2` reader - 3rd Sample Input Select"]
pub type ADC_SSMUX0_MUX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX2` writer - 3rd Sample Input Select"]
pub type ADC_SSMUX0_MUX2_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 8>;
#[doc = "Field `ADC_SSMUX0_MUX3` reader - 4th Sample Input Select"]
pub type ADC_SSMUX0_MUX3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX3` writer - 4th Sample Input Select"]
pub type ADC_SSMUX0_MUX3_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 12>;
#[doc = "Field `ADC_SSMUX0_MUX4` reader - 5th Sample Input Select"]
pub type ADC_SSMUX0_MUX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX4` writer - 5th Sample Input Select"]
pub type ADC_SSMUX0_MUX4_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `ADC_SSMUX0_MUX5` reader - 6th Sample Input Select"]
pub type ADC_SSMUX0_MUX5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX5` writer - 6th Sample Input Select"]
pub type ADC_SSMUX0_MUX5_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 20>;
#[doc = "Field `ADC_SSMUX0_MUX6` reader - 7th Sample Input Select"]
pub type ADC_SSMUX0_MUX6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX6` writer - 7th Sample Input Select"]
pub type ADC_SSMUX0_MUX6_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 24>;
#[doc = "Field `ADC_SSMUX0_MUX7` reader - 8th Sample Input Select"]
pub type ADC_SSMUX0_MUX7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_SSMUX0_MUX7` writer - 8th Sample Input Select"]
pub type ADC_SSMUX0_MUX7_W<'a> = crate::FieldWriter<'a, u32, SSMUX0_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&self) -> ADC_SSMUX0_MUX0_R {
        ADC_SSMUX0_MUX0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&self) -> ADC_SSMUX0_MUX1_R {
        ADC_SSMUX0_MUX1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&self) -> ADC_SSMUX0_MUX2_R {
        ADC_SSMUX0_MUX2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&self) -> ADC_SSMUX0_MUX3_R {
        ADC_SSMUX0_MUX3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&self) -> ADC_SSMUX0_MUX4_R {
        ADC_SSMUX0_MUX4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&self) -> ADC_SSMUX0_MUX5_R {
        ADC_SSMUX0_MUX5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&self) -> ADC_SSMUX0_MUX6_R {
        ADC_SSMUX0_MUX6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&self) -> ADC_SSMUX0_MUX7_R {
        ADC_SSMUX0_MUX7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&mut self) -> ADC_SSMUX0_MUX0_W {
        ADC_SSMUX0_MUX0_W::new(self)
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&mut self) -> ADC_SSMUX0_MUX1_W {
        ADC_SSMUX0_MUX1_W::new(self)
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&mut self) -> ADC_SSMUX0_MUX2_W {
        ADC_SSMUX0_MUX2_W::new(self)
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&mut self) -> ADC_SSMUX0_MUX3_W {
        ADC_SSMUX0_MUX3_W::new(self)
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&mut self) -> ADC_SSMUX0_MUX4_W {
        ADC_SSMUX0_MUX4_W::new(self)
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&mut self) -> ADC_SSMUX0_MUX5_W {
        ADC_SSMUX0_MUX5_W::new(self)
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&mut self) -> ADC_SSMUX0_MUX6_W {
        ADC_SSMUX0_MUX6_W::new(self)
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&mut self) -> ADC_SSMUX0_MUX7_W {
        ADC_SSMUX0_MUX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Input Multiplexer Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux0](index.html) module"]
pub struct SSMUX0_SPEC;
impl crate::RegisterSpec for SSMUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssmux0::R](R) reader structure"]
impl crate::Readable for SSMUX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssmux0::W](W) writer structure"]
impl crate::Writable for SSMUX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSMUX0 to value 0"]
impl crate::Resettable for SSMUX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
