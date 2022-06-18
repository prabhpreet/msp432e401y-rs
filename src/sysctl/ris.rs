#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RIS_BORRIS` reader - Brown-Out Reset Raw Interrupt Status"]
pub type SYSCTL_RIS_BORRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RIS_BORRIS` writer - Brown-Out Reset Raw Interrupt Status"]
pub type SYSCTL_RIS_BORRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_RIS_MOFRIS` reader - Main Oscillator Failure Raw Interrupt Status"]
pub type SYSCTL_RIS_MOFRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RIS_MOFRIS` writer - Main Oscillator Failure Raw Interrupt Status"]
pub type SYSCTL_RIS_MOFRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_RIS_PLLLRIS` reader - PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_PLLLRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RIS_PLLLRIS` writer - PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_PLLLRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_RIS_MOSCPUPRIS` reader - MOSC Power Up Raw Interrupt Status"]
pub type SYSCTL_RIS_MOSCPUPRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RIS_MOSCPUPRIS` writer - MOSC Power Up Raw Interrupt Status"]
pub type SYSCTL_RIS_MOSCPUPRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&self) -> SYSCTL_RIS_BORRIS_R {
        SYSCTL_RIS_BORRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_mofris(&self) -> SYSCTL_RIS_MOFRIS_R {
        SYSCTL_RIS_MOFRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&self) -> SYSCTL_RIS_PLLLRIS_R {
        SYSCTL_RIS_PLLLRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_moscpupris(&self) -> SYSCTL_RIS_MOSCPUPRIS_R {
        SYSCTL_RIS_MOSCPUPRIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Brown-Out Reset Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_borris(&mut self) -> SYSCTL_RIS_BORRIS_W {
        SYSCTL_RIS_BORRIS_W::new(self)
    }
    #[doc = "Bit 3 - Main Oscillator Failure Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_mofris(&mut self) -> SYSCTL_RIS_MOFRIS_W {
        SYSCTL_RIS_MOFRIS_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&mut self) -> SYSCTL_RIS_PLLLRIS_W {
        SYSCTL_RIS_PLLLRIS_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_moscpupris(&mut self) -> SYSCTL_RIS_MOSCPUPRIS_W {
        SYSCTL_RIS_MOSCPUPRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
