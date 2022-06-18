#[doc = "Register `PRADC` reader"]
pub struct R(crate::R<PRADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRADC` writer"]
pub struct W(crate::W<PRADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRADC_SPEC>;
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
impl From<crate::W<PRADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRADC_R0` reader - ADC Module 0 Peripheral Ready"]
pub type SYSCTL_PRADC_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRADC_R0` writer - ADC Module 0 Peripheral Ready"]
pub type SYSCTL_PRADC_R0_W<'a> = crate::BitWriter<'a, u32, PRADC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRADC_R1` reader - ADC Module 1 Peripheral Ready"]
pub type SYSCTL_PRADC_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRADC_R1` writer - ADC Module 1 Peripheral Ready"]
pub type SYSCTL_PRADC_R1_W<'a> = crate::BitWriter<'a, u32, PRADC_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pradc_r0(&self) -> SYSCTL_PRADC_R0_R {
        SYSCTL_PRADC_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pradc_r1(&self) -> SYSCTL_PRADC_R1_R {
        SYSCTL_PRADC_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pradc_r0(&mut self) -> SYSCTL_PRADC_R0_W {
        SYSCTL_PRADC_R0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_pradc_r1(&mut self) -> SYSCTL_PRADC_R1_W {
        SYSCTL_PRADC_R1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog-to-Digital Converter Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pradc](index.html) module"]
pub struct PRADC_SPEC;
impl crate::RegisterSpec for PRADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pradc::R](R) reader structure"]
impl crate::Readable for PRADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pradc::W](W) writer structure"]
impl crate::Writable for PRADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRADC to value 0"]
impl crate::Resettable for PRADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
