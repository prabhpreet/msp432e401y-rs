#[doc = "Register `PSSI` reader"]
pub struct R(crate::R<PSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSI` writer"]
pub struct W(crate::W<PSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSI_SPEC>;
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
impl From<crate::W<PSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_PSSI_SS0` reader - SS0 Initiate"]
pub type ADC_PSSI_SS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_SS0` writer - SS0 Initiate"]
pub type ADC_PSSI_SS0_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 0>;
#[doc = "Field `ADC_PSSI_SS1` reader - SS1 Initiate"]
pub type ADC_PSSI_SS1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_SS1` writer - SS1 Initiate"]
pub type ADC_PSSI_SS1_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 1>;
#[doc = "Field `ADC_PSSI_SS2` reader - SS2 Initiate"]
pub type ADC_PSSI_SS2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_SS2` writer - SS2 Initiate"]
pub type ADC_PSSI_SS2_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 2>;
#[doc = "Field `ADC_PSSI_SS3` reader - SS3 Initiate"]
pub type ADC_PSSI_SS3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_SS3` writer - SS3 Initiate"]
pub type ADC_PSSI_SS3_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 3>;
#[doc = "Field `ADC_PSSI_SYNCWAIT` reader - Synchronize Wait"]
pub type ADC_PSSI_SYNCWAIT_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_SYNCWAIT` writer - Synchronize Wait"]
pub type ADC_PSSI_SYNCWAIT_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 27>;
#[doc = "Field `ADC_PSSI_GSYNC` reader - Global Synchronize"]
pub type ADC_PSSI_GSYNC_R = crate::BitReader<bool>;
#[doc = "Field `ADC_PSSI_GSYNC` writer - Global Synchronize"]
pub type ADC_PSSI_GSYNC_W<'a> = crate::BitWriter<'a, u32, PSSI_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&self) -> ADC_PSSI_SS0_R {
        ADC_PSSI_SS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&self) -> ADC_PSSI_SS1_R {
        ADC_PSSI_SS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&self) -> ADC_PSSI_SS2_R {
        ADC_PSSI_SS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&self) -> ADC_PSSI_SS3_R {
        ADC_PSSI_SS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn adc_pssi_syncwait(&self) -> ADC_PSSI_SYNCWAIT_R {
        ADC_PSSI_SYNCWAIT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn adc_pssi_gsync(&self) -> ADC_PSSI_GSYNC_R {
        ADC_PSSI_GSYNC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&mut self) -> ADC_PSSI_SS0_W {
        ADC_PSSI_SS0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&mut self) -> ADC_PSSI_SS1_W {
        ADC_PSSI_SS1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&mut self) -> ADC_PSSI_SS2_W {
        ADC_PSSI_SS2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&mut self) -> ADC_PSSI_SS3_W {
        ADC_PSSI_SS3_W::new(self)
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn adc_pssi_syncwait(&mut self) -> ADC_PSSI_SYNCWAIT_W {
        ADC_PSSI_SYNCWAIT_W::new(self)
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn adc_pssi_gsync(&mut self) -> ADC_PSSI_GSYNC_W {
        ADC_PSSI_GSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Processor Sample Sequence Initiate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi](index.html) module"]
pub struct PSSI_SPEC;
impl crate::RegisterSpec for PSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssi::R](R) reader structure"]
impl crate::Readable for PSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssi::W](W) writer structure"]
impl crate::Writable for PSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSSI to value 0"]
impl crate::Resettable for PSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
