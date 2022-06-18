#[doc = "Register `RESC` reader"]
pub struct R(crate::R<RESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESC` writer"]
pub struct W(crate::W<RESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESC_SPEC>;
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
impl From<crate::W<RESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RESC_EXT` reader - External Reset"]
pub type SYSCTL_RESC_EXT_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_EXT` writer - External Reset"]
pub type SYSCTL_RESC_EXT_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_RESC_POR` reader - Power-On Reset"]
pub type SYSCTL_RESC_POR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_POR` writer - Power-On Reset"]
pub type SYSCTL_RESC_POR_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_RESC_BOR` reader - Brown-Out Reset"]
pub type SYSCTL_RESC_BOR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_BOR` writer - Brown-Out Reset"]
pub type SYSCTL_RESC_BOR_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_RESC_WDT0` reader - Watchdog Timer 0 Reset"]
pub type SYSCTL_RESC_WDT0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_WDT0` writer - Watchdog Timer 0 Reset"]
pub type SYSCTL_RESC_WDT0_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_RESC_SW` reader - Software Reset"]
pub type SYSCTL_RESC_SW_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_SW` writer - Software Reset"]
pub type SYSCTL_RESC_SW_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_RESC_WDT1` reader - Watchdog Timer 1 Reset"]
pub type SYSCTL_RESC_WDT1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_WDT1` writer - Watchdog Timer 1 Reset"]
pub type SYSCTL_RESC_WDT1_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_RESC_HIB` reader - HIB Reset"]
pub type SYSCTL_RESC_HIB_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_HIB` writer - HIB Reset"]
pub type SYSCTL_RESC_HIB_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_RESC_HSSR` reader - HSSR Reset"]
pub type SYSCTL_RESC_HSSR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_HSSR` writer - HSSR Reset"]
pub type SYSCTL_RESC_HSSR_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 12>;
#[doc = "Field `SYSCTL_RESC_MOSCFAIL` reader - MOSC Failure Reset"]
pub type SYSCTL_RESC_MOSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RESC_MOSCFAIL` writer - MOSC Failure Reset"]
pub type SYSCTL_RESC_MOSCFAIL_W<'a> = crate::BitWriter<'a, u32, RESC_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&self) -> SYSCTL_RESC_EXT_R {
        SYSCTL_RESC_EXT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&self) -> SYSCTL_RESC_POR_R {
        SYSCTL_RESC_POR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&self) -> SYSCTL_RESC_BOR_R {
        SYSCTL_RESC_BOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt0(&self) -> SYSCTL_RESC_WDT0_R {
        SYSCTL_RESC_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&self) -> SYSCTL_RESC_SW_R {
        SYSCTL_RESC_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt1(&self) -> SYSCTL_RESC_WDT1_R {
        SYSCTL_RESC_WDT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HIB Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hib(&self) -> SYSCTL_RESC_HIB_R {
        SYSCTL_RESC_HIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - HSSR Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hssr(&self) -> SYSCTL_RESC_HSSR_R {
        SYSCTL_RESC_HSSR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    pub fn sysctl_resc_moscfail(&self) -> SYSCTL_RESC_MOSCFAIL_R {
        SYSCTL_RESC_MOSCFAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset"]
    #[inline(always)]
    pub fn sysctl_resc_ext(&mut self) -> SYSCTL_RESC_EXT_W {
        SYSCTL_RESC_EXT_W::new(self)
    }
    #[doc = "Bit 1 - Power-On Reset"]
    #[inline(always)]
    pub fn sysctl_resc_por(&mut self) -> SYSCTL_RESC_POR_W {
        SYSCTL_RESC_POR_W::new(self)
    }
    #[doc = "Bit 2 - Brown-Out Reset"]
    #[inline(always)]
    pub fn sysctl_resc_bor(&mut self) -> SYSCTL_RESC_BOR_W {
        SYSCTL_RESC_BOR_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt0(&mut self) -> SYSCTL_RESC_WDT0_W {
        SYSCTL_RESC_WDT0_W::new(self)
    }
    #[doc = "Bit 4 - Software Reset"]
    #[inline(always)]
    pub fn sysctl_resc_sw(&mut self) -> SYSCTL_RESC_SW_W {
        SYSCTL_RESC_SW_W::new(self)
    }
    #[doc = "Bit 5 - Watchdog Timer 1 Reset"]
    #[inline(always)]
    pub fn sysctl_resc_wdt1(&mut self) -> SYSCTL_RESC_WDT1_W {
        SYSCTL_RESC_WDT1_W::new(self)
    }
    #[doc = "Bit 6 - HIB Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hib(&mut self) -> SYSCTL_RESC_HIB_W {
        SYSCTL_RESC_HIB_W::new(self)
    }
    #[doc = "Bit 12 - HSSR Reset"]
    #[inline(always)]
    pub fn sysctl_resc_hssr(&mut self) -> SYSCTL_RESC_HSSR_W {
        SYSCTL_RESC_HSSR_W::new(self)
    }
    #[doc = "Bit 16 - MOSC Failure Reset"]
    #[inline(always)]
    pub fn sysctl_resc_moscfail(&mut self) -> SYSCTL_RESC_MOSCFAIL_W {
        SYSCTL_RESC_MOSCFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resc](index.html) module"]
pub struct RESC_SPEC;
impl crate::RegisterSpec for RESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resc::R](R) reader structure"]
impl crate::Readable for RESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resc::W](W) writer structure"]
impl crate::Writable for RESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESC to value 0"]
impl crate::Resettable for RESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
