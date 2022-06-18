#[doc = "Register `MOSCCTL` reader"]
pub struct R(crate::R<MOSCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSCCTL` writer"]
pub struct W(crate::W<MOSCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSCCTL_SPEC>;
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
impl From<crate::W<MOSCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_MOSCCTL_CVAL` reader - Clock Validation for MOSC"]
pub type SYSCTL_MOSCCTL_CVAL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MOSCCTL_CVAL` writer - Clock Validation for MOSC"]
pub type SYSCTL_MOSCCTL_CVAL_W<'a> = crate::BitWriter<'a, u32, MOSCCTL_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_MOSCCTL_MOSCIM` reader - MOSC Failure Action"]
pub type SYSCTL_MOSCCTL_MOSCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MOSCCTL_MOSCIM` writer - MOSC Failure Action"]
pub type SYSCTL_MOSCCTL_MOSCIM_W<'a> = crate::BitWriter<'a, u32, MOSCCTL_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_MOSCCTL_NOXTAL` reader - No Crystal Connected"]
pub type SYSCTL_MOSCCTL_NOXTAL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MOSCCTL_NOXTAL` writer - No Crystal Connected"]
pub type SYSCTL_MOSCCTL_NOXTAL_W<'a> = crate::BitWriter<'a, u32, MOSCCTL_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_MOSCCTL_PWRDN` reader - Power Down"]
pub type SYSCTL_MOSCCTL_PWRDN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MOSCCTL_PWRDN` writer - Power Down"]
pub type SYSCTL_MOSCCTL_PWRDN_W<'a> = crate::BitWriter<'a, u32, MOSCCTL_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_MOSCCTL_OSCRNG` reader - Oscillator Range"]
pub type SYSCTL_MOSCCTL_OSCRNG_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_MOSCCTL_OSCRNG` writer - Oscillator Range"]
pub type SYSCTL_MOSCCTL_OSCRNG_W<'a> = crate::BitWriter<'a, u32, MOSCCTL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn sysctl_moscctl_cval(&self) -> SYSCTL_MOSCCTL_CVAL_R {
        SYSCTL_MOSCCTL_CVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn sysctl_moscctl_moscim(&self) -> SYSCTL_MOSCCTL_MOSCIM_R {
        SYSCTL_MOSCCTL_MOSCIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn sysctl_moscctl_noxtal(&self) -> SYSCTL_MOSCCTL_NOXTAL_R {
        SYSCTL_MOSCCTL_NOXTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Down"]
    #[inline(always)]
    pub fn sysctl_moscctl_pwrdn(&self) -> SYSCTL_MOSCCTL_PWRDN_R {
        SYSCTL_MOSCCTL_PWRDN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Oscillator Range"]
    #[inline(always)]
    pub fn sysctl_moscctl_oscrng(&self) -> SYSCTL_MOSCCTL_OSCRNG_R {
        SYSCTL_MOSCCTL_OSCRNG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn sysctl_moscctl_cval(&mut self) -> SYSCTL_MOSCCTL_CVAL_W {
        SYSCTL_MOSCCTL_CVAL_W::new(self)
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn sysctl_moscctl_moscim(&mut self) -> SYSCTL_MOSCCTL_MOSCIM_W {
        SYSCTL_MOSCCTL_MOSCIM_W::new(self)
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn sysctl_moscctl_noxtal(&mut self) -> SYSCTL_MOSCCTL_NOXTAL_W {
        SYSCTL_MOSCCTL_NOXTAL_W::new(self)
    }
    #[doc = "Bit 3 - Power Down"]
    #[inline(always)]
    pub fn sysctl_moscctl_pwrdn(&mut self) -> SYSCTL_MOSCCTL_PWRDN_W {
        SYSCTL_MOSCCTL_PWRDN_W::new(self)
    }
    #[doc = "Bit 4 - Oscillator Range"]
    #[inline(always)]
    pub fn sysctl_moscctl_oscrng(&mut self) -> SYSCTL_MOSCCTL_OSCRNG_W {
        SYSCTL_MOSCCTL_OSCRNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moscctl](index.html) module"]
pub struct MOSCCTL_SPEC;
impl crate::RegisterSpec for MOSCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [moscctl::R](R) reader structure"]
impl crate::Readable for MOSCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [moscctl::W](W) writer structure"]
impl crate::Writable for MOSCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOSCCTL to value 0"]
impl crate::Resettable for MOSCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
