#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_MISC_BORMIS` reader - BOR Masked Interrupt Status"]
pub type SYSCTL_MISC_BORMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MISC_BORMIS` writer - BOR Masked Interrupt Status"]
pub type SYSCTL_MISC_BORMIS_W<'a> = crate::BitWriter<'a, u32, MISC_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_MISC_MOFMIS` reader - Main Oscillator Failure Masked Interrupt Status"]
pub type SYSCTL_MISC_MOFMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MISC_MOFMIS` writer - Main Oscillator Failure Masked Interrupt Status"]
pub type SYSCTL_MISC_MOFMIS_W<'a> = crate::BitWriter<'a, u32, MISC_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_MISC_PLLLMIS` reader - PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_PLLLMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MISC_PLLLMIS` writer - PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_PLLLMIS_W<'a> = crate::BitWriter<'a, u32, MISC_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_MISC_MOSCPUPMIS` reader - MOSC Power Up Masked Interrupt Status"]
pub type SYSCTL_MISC_MOSCPUPMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MISC_MOSCPUPMIS` writer - MOSC Power Up Masked Interrupt Status"]
pub type SYSCTL_MISC_MOSCPUPMIS_W<'a> = crate::BitWriter<'a, u32, MISC_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&self) -> SYSCTL_MISC_BORMIS_R {
        SYSCTL_MISC_BORMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_mofmis(&self) -> SYSCTL_MISC_MOFMIS_R {
        SYSCTL_MISC_MOFMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&self) -> SYSCTL_MISC_PLLLMIS_R {
        SYSCTL_MISC_PLLLMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_moscpupmis(&self) -> SYSCTL_MISC_MOSCPUPMIS_R {
        SYSCTL_MISC_MOSCPUPMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BOR Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_bormis(&mut self) -> SYSCTL_MISC_BORMIS_W {
        SYSCTL_MISC_BORMIS_W::new(self)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_mofmis(&mut self) -> SYSCTL_MISC_MOFMIS_W {
        SYSCTL_MISC_MOFMIS_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&mut self) -> SYSCTL_MISC_PLLLMIS_W {
        SYSCTL_MISC_PLLLMIS_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_moscpupmis(&mut self) -> SYSCTL_MISC_MOSCPUPMIS_W {
        SYSCTL_MISC_MOSCPUPMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R](R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
