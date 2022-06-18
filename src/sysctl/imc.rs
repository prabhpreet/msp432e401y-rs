#[doc = "Register `IMC` reader"]
pub struct R(crate::R<IMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMC` writer"]
pub struct W(crate::W<IMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMC_SPEC>;
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
impl From<crate::W<IMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_IMC_BORIM` reader - Brown-Out Reset Interrupt Mask"]
pub type SYSCTL_IMC_BORIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_IMC_BORIM` writer - Brown-Out Reset Interrupt Mask"]
pub type SYSCTL_IMC_BORIM_W<'a> = crate::BitWriter<'a, u32, IMC_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_IMC_MOFIM` reader - Main Oscillator Failure Interrupt Mask"]
pub type SYSCTL_IMC_MOFIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_IMC_MOFIM` writer - Main Oscillator Failure Interrupt Mask"]
pub type SYSCTL_IMC_MOFIM_W<'a> = crate::BitWriter<'a, u32, IMC_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_IMC_PLLLIM` reader - PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_PLLLIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_IMC_PLLLIM` writer - PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_PLLLIM_W<'a> = crate::BitWriter<'a, u32, IMC_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_IMC_MOSCPUPIM` reader - MOSC Power Up Interrupt Mask"]
pub type SYSCTL_IMC_MOSCPUPIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_IMC_MOSCPUPIM` writer - MOSC Power Up Interrupt Mask"]
pub type SYSCTL_IMC_MOSCPUPIM_W<'a> = crate::BitWriter<'a, u32, IMC_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&self) -> SYSCTL_IMC_BORIM_R {
        SYSCTL_IMC_BORIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_mofim(&self) -> SYSCTL_IMC_MOFIM_R {
        SYSCTL_IMC_MOFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&self) -> SYSCTL_IMC_PLLLIM_R {
        SYSCTL_IMC_PLLLIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_moscpupim(&self) -> SYSCTL_IMC_MOSCPUPIM_R {
        SYSCTL_IMC_MOSCPUPIM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Brown-Out Reset Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_borim(&mut self) -> SYSCTL_IMC_BORIM_W {
        SYSCTL_IMC_BORIM_W::new(self)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_mofim(&mut self) -> SYSCTL_IMC_MOFIM_W {
        SYSCTL_IMC_MOFIM_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&mut self) -> SYSCTL_IMC_PLLLIM_W {
        SYSCTL_IMC_PLLLIM_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_moscpupim(&mut self) -> SYSCTL_IMC_MOSCPUPIM_W {
        SYSCTL_IMC_MOSCPUPIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imc](index.html) module"]
pub struct IMC_SPEC;
impl crate::RegisterSpec for IMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imc::R](R) reader structure"]
impl crate::Readable for IMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imc::W](W) writer structure"]
impl crate::Writable for IMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMC to value 0"]
impl crate::Resettable for IMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
