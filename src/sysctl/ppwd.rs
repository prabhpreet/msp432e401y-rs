#[doc = "Register `PPWD` reader"]
pub struct R(crate::R<PPWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPWD` writer"]
pub struct W(crate::W<PPWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPWD_SPEC>;
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
impl From<crate::W<PPWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPWD_P0` reader - Watchdog Timer 0 Present"]
pub type SYSCTL_PPWD_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPWD_P0` writer - Watchdog Timer 0 Present"]
pub type SYSCTL_PPWD_P0_W<'a> = crate::BitWriter<'a, u32, PPWD_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PPWD_P1` reader - Watchdog Timer 1 Present"]
pub type SYSCTL_PPWD_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPWD_P1` writer - Watchdog Timer 1 Present"]
pub type SYSCTL_PPWD_P1_W<'a> = crate::BitWriter<'a, u32, PPWD_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p0(&self) -> SYSCTL_PPWD_P0_R {
        SYSCTL_PPWD_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p1(&self) -> SYSCTL_PPWD_P1_R {
        SYSCTL_PPWD_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p0(&mut self) -> SYSCTL_PPWD_P0_W {
        SYSCTL_PPWD_P0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppwd_p1(&mut self) -> SYSCTL_PPWD_P1_W {
        SYSCTL_PPWD_P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppwd](index.html) module"]
pub struct PPWD_SPEC;
impl crate::RegisterSpec for PPWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppwd::R](R) reader structure"]
impl crate::Readable for PPWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppwd::W](W) writer structure"]
impl crate::Writable for PPWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPWD to value 0"]
impl crate::Resettable for PPWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
