#[doc = "Register `SSEMUX0` reader"]
pub struct R(crate::R<SSEMUX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSEMUX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSEMUX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSEMUX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSEMUX0` writer"]
pub struct W(crate::W<SSEMUX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSEMUX0_SPEC>;
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
impl From<crate::W<SSEMUX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSEMUX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSEMUX0_EMUX0` reader - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX0` writer - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX0_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 0>;
#[doc = "Field `ADC_SSEMUX0_EMUX1` reader - 2th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX1` writer - 2th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX1_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 4>;
#[doc = "Field `ADC_SSEMUX0_EMUX2` reader - 3rd Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX2` writer - 3rd Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX2_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 8>;
#[doc = "Field `ADC_SSEMUX0_EMUX3` reader - 4th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX3` writer - 4th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX3_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 12>;
#[doc = "Field `ADC_SSEMUX0_EMUX4` reader - 5th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX4_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX4` writer - 5th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX4_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 16>;
#[doc = "Field `ADC_SSEMUX0_EMUX5` reader - 6th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX5_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX5` writer - 6th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX5_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 20>;
#[doc = "Field `ADC_SSEMUX0_EMUX6` reader - 7th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX6_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX6` writer - 7th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX6_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 24>;
#[doc = "Field `ADC_SSEMUX0_EMUX7` reader - 8th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX7_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX0_EMUX7` writer - 8th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX0_EMUX7_W<'a> = crate::BitWriter<'a, u32, SSEMUX0_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux0(&self) -> ADC_SSEMUX0_EMUX0_R {
        ADC_SSEMUX0_EMUX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux1(&self) -> ADC_SSEMUX0_EMUX1_R {
        ADC_SSEMUX0_EMUX1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux2(&self) -> ADC_SSEMUX0_EMUX2_R {
        ADC_SSEMUX0_EMUX2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux3(&self) -> ADC_SSEMUX0_EMUX3_R {
        ADC_SSEMUX0_EMUX3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux4(&self) -> ADC_SSEMUX0_EMUX4_R {
        ADC_SSEMUX0_EMUX4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux5(&self) -> ADC_SSEMUX0_EMUX5_R {
        ADC_SSEMUX0_EMUX5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux6(&self) -> ADC_SSEMUX0_EMUX6_R {
        ADC_SSEMUX0_EMUX6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux7(&self) -> ADC_SSEMUX0_EMUX7_R {
        ADC_SSEMUX0_EMUX7_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux0(&mut self) -> ADC_SSEMUX0_EMUX0_W {
        ADC_SSEMUX0_EMUX0_W::new(self)
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux1(&mut self) -> ADC_SSEMUX0_EMUX1_W {
        ADC_SSEMUX0_EMUX1_W::new(self)
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux2(&mut self) -> ADC_SSEMUX0_EMUX2_W {
        ADC_SSEMUX0_EMUX2_W::new(self)
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux3(&mut self) -> ADC_SSEMUX0_EMUX3_W {
        ADC_SSEMUX0_EMUX3_W::new(self)
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux4(&mut self) -> ADC_SSEMUX0_EMUX4_W {
        ADC_SSEMUX0_EMUX4_W::new(self)
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux5(&mut self) -> ADC_SSEMUX0_EMUX5_W {
        ADC_SSEMUX0_EMUX5_W::new(self)
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux6(&mut self) -> ADC_SSEMUX0_EMUX6_W {
        ADC_SSEMUX0_EMUX6_W::new(self)
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux7(&mut self) -> ADC_SSEMUX0_EMUX7_W {
        ADC_SSEMUX0_EMUX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux0](index.html) module"]
pub struct SSEMUX0_SPEC;
impl crate::RegisterSpec for SSEMUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssemux0::R](R) reader structure"]
impl crate::Readable for SSEMUX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssemux0::W](W) writer structure"]
impl crate::Writable for SSEMUX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSEMUX0 to value 0"]
impl crate::Resettable for SSEMUX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
