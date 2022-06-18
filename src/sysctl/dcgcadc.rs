#[doc = "Register `DCGCADC` reader"]
pub struct R(crate::R<DCGCADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCADC` writer"]
pub struct W(crate::W<DCGCADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCADC_SPEC>;
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
impl From<crate::W<DCGCADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCADC_D0` reader - ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCADC_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCADC_D0` writer - ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCADC_D0_W<'a> = crate::BitWriter<'a, u32, DCGCADC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCADC_D1` reader - ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCADC_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCADC_D1` writer - ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCADC_D1_W<'a> = crate::BitWriter<'a, u32, DCGCADC_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcadc_d0(&self) -> SYSCTL_DCGCADC_D0_R {
        SYSCTL_DCGCADC_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcadc_d1(&self) -> SYSCTL_DCGCADC_D1_R {
        SYSCTL_DCGCADC_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcadc_d0(&mut self) -> SYSCTL_DCGCADC_D0_W {
        SYSCTL_DCGCADC_D0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcadc_d1(&mut self) -> SYSCTL_DCGCADC_D1_W {
        SYSCTL_DCGCADC_D1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcadc](index.html) module"]
pub struct DCGCADC_SPEC;
impl crate::RegisterSpec for DCGCADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcadc::R](R) reader structure"]
impl crate::Readable for DCGCADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcadc::W](W) writer structure"]
impl crate::Writable for DCGCADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCADC to value 0"]
impl crate::Resettable for DCGCADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
