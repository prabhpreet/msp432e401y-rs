#[doc = "Register `SCGCADC` reader"]
pub struct R(crate::R<SCGCADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCADC` writer"]
pub struct W(crate::W<SCGCADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCADC_SPEC>;
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
impl From<crate::W<SCGCADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCADC_S0` reader - ADC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCADC_S0` writer - ADC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S0_W<'a> = crate::BitWriter<'a, u32, SCGCADC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCADC_S1` reader - ADC Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCADC_S1` writer - ADC Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S1_W<'a> = crate::BitWriter<'a, u32, SCGCADC_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s0(&self) -> SYSCTL_SCGCADC_S0_R {
        SYSCTL_SCGCADC_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s1(&self) -> SYSCTL_SCGCADC_S1_R {
        SYSCTL_SCGCADC_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s0(&mut self) -> SYSCTL_SCGCADC_S0_W {
        SYSCTL_SCGCADC_S0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s1(&mut self) -> SYSCTL_SCGCADC_S1_W {
        SYSCTL_SCGCADC_S1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcadc](index.html) module"]
pub struct SCGCADC_SPEC;
impl crate::RegisterSpec for SCGCADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcadc::R](R) reader structure"]
impl crate::Readable for SCGCADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcadc::W](W) writer structure"]
impl crate::Writable for SCGCADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCADC to value 0"]
impl crate::Resettable for SCGCADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
