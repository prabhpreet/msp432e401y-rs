#[doc = "Register `SRADC` reader"]
pub struct R(crate::R<SRADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRADC` writer"]
pub struct W(crate::W<SRADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRADC_SPEC>;
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
impl From<crate::W<SRADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRADC_R0` reader - ADC Module 0 Software Reset"]
pub type SYSCTL_SRADC_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRADC_R0` writer - ADC Module 0 Software Reset"]
pub type SYSCTL_SRADC_R0_W<'a> = crate::BitWriter<'a, u32, SRADC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SRADC_R1` reader - ADC Module 1 Software Reset"]
pub type SYSCTL_SRADC_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRADC_R1` writer - ADC Module 1 Software Reset"]
pub type SYSCTL_SRADC_R1_W<'a> = crate::BitWriter<'a, u32, SRADC_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r0(&self) -> SYSCTL_SRADC_R0_R {
        SYSCTL_SRADC_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r1(&self) -> SYSCTL_SRADC_R1_R {
        SYSCTL_SRADC_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r0(&mut self) -> SYSCTL_SRADC_R0_W {
        SYSCTL_SRADC_R0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r1(&mut self) -> SYSCTL_SRADC_R1_W {
        SYSCTL_SRADC_R1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog-to-Digital Converter Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sradc](index.html) module"]
pub struct SRADC_SPEC;
impl crate::RegisterSpec for SRADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sradc::R](R) reader structure"]
impl crate::Readable for SRADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sradc::W](W) writer structure"]
impl crate::Writable for SRADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRADC to value 0"]
impl crate::Resettable for SRADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
