#[doc = "Register `PCADC` reader"]
pub struct R(crate::R<PCADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCADC` writer"]
pub struct W(crate::W<PCADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCADC_SPEC>;
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
impl From<crate::W<PCADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCADC_P0` reader - ADC Module 0 Power Control"]
pub type SYSCTL_PCADC_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCADC_P0` writer - ADC Module 0 Power Control"]
pub type SYSCTL_PCADC_P0_W<'a> = crate::BitWriter<'a, u32, PCADC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PCADC_P1` reader - ADC Module 1 Power Control"]
pub type SYSCTL_PCADC_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCADC_P1` writer - ADC Module 1 Power Control"]
pub type SYSCTL_PCADC_P1_W<'a> = crate::BitWriter<'a, u32, PCADC_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcadc_p0(&self) -> SYSCTL_PCADC_P0_R {
        SYSCTL_PCADC_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcadc_p1(&self) -> SYSCTL_PCADC_P1_R {
        SYSCTL_PCADC_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcadc_p0(&mut self) -> SYSCTL_PCADC_P0_W {
        SYSCTL_PCADC_P0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcadc_p1(&mut self) -> SYSCTL_PCADC_P1_W {
        SYSCTL_PCADC_P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog-to-Digital Converter Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcadc](index.html) module"]
pub struct PCADC_SPEC;
impl crate::RegisterSpec for PCADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcadc::R](R) reader structure"]
impl crate::Readable for PCADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcadc::W](W) writer structure"]
impl crate::Writable for PCADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCADC to value 0"]
impl crate::Resettable for PCADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
