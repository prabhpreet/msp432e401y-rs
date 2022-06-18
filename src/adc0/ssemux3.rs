#[doc = "Register `SSEMUX3` reader"]
pub struct R(crate::R<SSEMUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSEMUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSEMUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSEMUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSEMUX3` writer"]
pub struct W(crate::W<SSEMUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSEMUX3_SPEC>;
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
impl From<crate::W<SSEMUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSEMUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSEMUX3_EMUX0` reader - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX3_EMUX0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSEMUX3_EMUX0` writer - 1st Sample Input Select (Upper Bit)"]
pub type ADC_SSEMUX3_EMUX0_W<'a> = crate::BitWriter<'a, u32, SSEMUX3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux3_emux0(&self) -> ADC_SSEMUX3_EMUX0_R {
        ADC_SSEMUX3_EMUX0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux3_emux0(&mut self) -> ADC_SSEMUX3_EMUX0_W {
        ADC_SSEMUX3_EMUX0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssemux3](index.html) module"]
pub struct SSEMUX3_SPEC;
impl crate::RegisterSpec for SSEMUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssemux3::R](R) reader structure"]
impl crate::Readable for SSEMUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssemux3::W](W) writer structure"]
impl crate::Writable for SSEMUX3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSEMUX3 to value 0"]
impl crate::Resettable for SSEMUX3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
