#[doc = "Register `SCGCWD` reader"]
pub struct R(crate::R<SCGCWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCWD` writer"]
pub struct W(crate::W<SCGCWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCWD_SPEC>;
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
impl From<crate::W<SCGCWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCWD_S0` reader - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCWD_S0` writer - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S0_W<'a> = crate::BitWriter<'a, u32, SCGCWD_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCWD_S1` reader - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCWD_S1` writer - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCWD_S1_W<'a> = crate::BitWriter<'a, u32, SCGCWD_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s0(&self) -> SYSCTL_SCGCWD_S0_R {
        SYSCTL_SCGCWD_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s1(&self) -> SYSCTL_SCGCWD_S1_R {
        SYSCTL_SCGCWD_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s0(&mut self) -> SYSCTL_SCGCWD_S0_W {
        SYSCTL_SCGCWD_S0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcwd_s1(&mut self) -> SYSCTL_SCGCWD_S1_W {
        SYSCTL_SCGCWD_S1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcwd](index.html) module"]
pub struct SCGCWD_SPEC;
impl crate::RegisterSpec for SCGCWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcwd::R](R) reader structure"]
impl crate::Readable for SCGCWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcwd::W](W) writer structure"]
impl crate::Writable for SCGCWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCWD to value 0"]
impl crate::Resettable for SCGCWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
