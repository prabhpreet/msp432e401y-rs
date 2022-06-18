#[doc = "Register `SSEMUX1` reader"]
pub struct R(crate::R<SSEMUX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSEMUX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSEMUX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSEMUX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSEMUX1` writer"]
pub struct W(crate::W<SSEMUX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSEMUX1_SPEC>;
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
impl From<crate::W<SSEMUX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSEMUX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSEMUX1_EMUX0` reader - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX1_EMUX0` writer - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX0_W<'a> = crate::BitWriter<'a, u32, SSEMUX1_SPEC, bool, 0>;
#[doc = "Field `ADC_SSEMUX1_EMUX1` reader - 2th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX1_EMUX1` writer - 2th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX1_W<'a> = crate::BitWriter<'a, u32, SSEMUX1_SPEC, bool, 4>;
#[doc = "Field `ADC_SSEMUX1_EMUX2` reader - 3rd Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX1_EMUX2` writer - 3rd Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX2_W<'a> = crate::BitWriter<'a, u32, SSEMUX1_SPEC, bool, 8>;
#[doc = "Field `ADC_SSEMUX1_EMUX3` reader - 4th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX1_EMUX3` writer - 4th Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX1_EMUX3_W<'a> = crate::BitWriter<'a, u32, SSEMUX1_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux0(&self) -> ADC_SSEMUX1_EMUX0_R {
        ADC_SSEMUX1_EMUX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux1(&self) -> ADC_SSEMUX1_EMUX1_R {
        ADC_SSEMUX1_EMUX1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux2(&self) -> ADC_SSEMUX1_EMUX2_R {
        ADC_SSEMUX1_EMUX2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux3(&self) -> ADC_SSEMUX1_EMUX3_R {
        ADC_SSEMUX1_EMUX3_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux0(&mut self) -> ADC_SSEMUX1_EMUX0_W {
        ADC_SSEMUX1_EMUX0_W::new(self)
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux1(&mut self) -> ADC_SSEMUX1_EMUX1_W {
        ADC_SSEMUX1_EMUX1_W::new(self)
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux2(&mut self) -> ADC_SSEMUX1_EMUX2_W {
        ADC_SSEMUX1_EMUX2_W::new(self)
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux1_emux3(&mut self) -> ADC_SSEMUX1_EMUX3_W {
        ADC_SSEMUX1_EMUX3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux1](index.html) module"]
pub struct SSEMUX1_SPEC;
impl crate::RegisterSpec for SSEMUX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssemux1::R](R) reader structure"]
impl crate::Readable for SSEMUX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssemux1::W](W) writer structure"]
impl crate::Writable for SSEMUX1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSEMUX1 to value 0"]
impl crate::Resettable for SSEMUX1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
