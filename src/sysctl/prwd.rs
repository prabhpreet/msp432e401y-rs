#[doc = "Register `PRWD` reader"]
pub struct R(crate::R<PRWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRWD` writer"]
pub struct W(crate::W<PRWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRWD_SPEC>;
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
impl From<crate::W<PRWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRWD_R0` reader - Watchdog Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWD_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRWD_R0` writer - Watchdog Timer 0 Peripheral Ready"]
pub type SYSCTL_PRWD_R0_W<'a> = crate::BitWriter<'a, u32, PRWD_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PRWD_R1` reader - Watchdog Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWD_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRWD_R1` writer - Watchdog Timer 1 Peripheral Ready"]
pub type SYSCTL_PRWD_R1_W<'a> = crate::BitWriter<'a, u32, PRWD_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r0(&self) -> SYSCTL_PRWD_R0_R {
        SYSCTL_PRWD_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r1(&self) -> SYSCTL_PRWD_R1_R {
        SYSCTL_PRWD_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer 0 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r0(&mut self) -> SYSCTL_PRWD_R0_W {
        SYSCTL_PRWD_R0_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer 1 Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prwd_r1(&mut self) -> SYSCTL_PRWD_R1_W {
        SYSCTL_PRWD_R1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prwd](index.html) module"]
pub struct PRWD_SPEC;
impl crate::RegisterSpec for PRWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prwd::R](R) reader structure"]
impl crate::Readable for PRWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prwd::W](W) writer structure"]
impl crate::Writable for PRWD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRWD to value 0"]
impl crate::Resettable for PRWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
