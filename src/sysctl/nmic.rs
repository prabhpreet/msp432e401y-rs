#[doc = "Register `NMIC` reader"]
pub struct R(crate::R<NMIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIC` writer"]
pub struct W(crate::W<NMIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIC_SPEC>;
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
impl From<crate::W<NMIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_NMIC_EXTERNAL` reader - External Pin NMI"]
pub type SYSCTL_NMIC_EXTERNAL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_EXTERNAL` writer - External Pin NMI"]
pub type SYSCTL_NMIC_EXTERNAL_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_NMIC_POWER` reader - Power/Brown Out Event NMI"]
pub type SYSCTL_NMIC_POWER_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_POWER` writer - Power/Brown Out Event NMI"]
pub type SYSCTL_NMIC_POWER_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_NMIC_WDT0` reader - Watch Dog Timer (WDT) 0 NMI"]
pub type SYSCTL_NMIC_WDT0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_WDT0` writer - Watch Dog Timer (WDT) 0 NMI"]
pub type SYSCTL_NMIC_WDT0_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_NMIC_WDT1` reader - Watch Dog Timer (WDT) 1 NMI"]
pub type SYSCTL_NMIC_WDT1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_WDT1` writer - Watch Dog Timer (WDT) 1 NMI"]
pub type SYSCTL_NMIC_WDT1_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_NMIC_TAMPER` reader - Tamper Event NMI"]
pub type SYSCTL_NMIC_TAMPER_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_TAMPER` writer - Tamper Event NMI"]
pub type SYSCTL_NMIC_TAMPER_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 9>;
#[doc = "Field `SYSCTL_NMIC_MOSCFAIL` reader - MOSC Failure NMI"]
pub type SYSCTL_NMIC_MOSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NMIC_MOSCFAIL` writer - MOSC Failure NMI"]
pub type SYSCTL_NMIC_MOSCFAIL_W<'a> = crate::BitWriter<'a, u32, NMIC_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_external(&self) -> SYSCTL_NMIC_EXTERNAL_R {
        SYSCTL_NMIC_EXTERNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_power(&self) -> SYSCTL_NMIC_POWER_R {
        SYSCTL_NMIC_POWER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt0(&self) -> SYSCTL_NMIC_WDT0_R {
        SYSCTL_NMIC_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt1(&self) -> SYSCTL_NMIC_WDT1_R {
        SYSCTL_NMIC_WDT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_tamper(&self) -> SYSCTL_NMIC_TAMPER_R {
        SYSCTL_NMIC_TAMPER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_moscfail(&self) -> SYSCTL_NMIC_MOSCFAIL_R {
        SYSCTL_NMIC_MOSCFAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Pin NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_external(&mut self) -> SYSCTL_NMIC_EXTERNAL_W {
        SYSCTL_NMIC_EXTERNAL_W::new(self)
    }
    #[doc = "Bit 2 - Power/Brown Out Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_power(&mut self) -> SYSCTL_NMIC_POWER_W {
        SYSCTL_NMIC_POWER_W::new(self)
    }
    #[doc = "Bit 3 - Watch Dog Timer (WDT) 0 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt0(&mut self) -> SYSCTL_NMIC_WDT0_W {
        SYSCTL_NMIC_WDT0_W::new(self)
    }
    #[doc = "Bit 5 - Watch Dog Timer (WDT) 1 NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_wdt1(&mut self) -> SYSCTL_NMIC_WDT1_W {
        SYSCTL_NMIC_WDT1_W::new(self)
    }
    #[doc = "Bit 9 - Tamper Event NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_tamper(&mut self) -> SYSCTL_NMIC_TAMPER_W {
        SYSCTL_NMIC_TAMPER_W::new(self)
    }
    #[doc = "Bit 16 - MOSC Failure NMI"]
    #[inline(always)]
    pub fn sysctl_nmic_moscfail(&mut self) -> SYSCTL_NMIC_MOSCFAIL_W {
        SYSCTL_NMIC_MOSCFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmic](index.html) module"]
pub struct NMIC_SPEC;
impl crate::RegisterSpec for NMIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmic::R](R) reader structure"]
impl crate::Readable for NMIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmic::W](W) writer structure"]
impl crate::Writable for NMIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIC to value 0"]
impl crate::Resettable for NMIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
