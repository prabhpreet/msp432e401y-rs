#[doc = "Register `SSOP3` reader"]
pub struct R(crate::R<SSOP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSOP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSOP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSOP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSOP3` writer"]
pub struct W(crate::W<SSOP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSOP3_SPEC>;
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
impl From<crate::W<SSOP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSOP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSOP3_S0DCOP` reader - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP3_S0DCOP_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSOP3_S0DCOP` writer - Sample 0 Digital Comparator Operation"]
pub type ADC_SSOP3_S0DCOP_W<'a> = crate::BitWriter<'a, u32, SSOP3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop3_s0dcop(&self) -> ADC_SSOP3_S0DCOP_R {
        ADC_SSOP3_S0DCOP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop3_s0dcop(&mut self) -> ADC_SSOP3_S0DCOP_W {
        ADC_SSOP3_S0DCOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence 3 Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssop3](index.html) module"]
pub struct SSOP3_SPEC;
impl crate::RegisterSpec for SSOP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssop3::R](R) reader structure"]
impl crate::Readable for SSOP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssop3::W](W) writer structure"]
impl crate::Writable for SSOP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSOP3 to value 0"]
impl crate::Resettable for SSOP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
