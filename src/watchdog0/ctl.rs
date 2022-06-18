#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CTL_INTEN` reader - Watchdog Interrupt Enable"]
pub type WDT_CTL_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CTL_INTEN` writer - Watchdog Interrupt Enable"]
pub type WDT_CTL_INTEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `WDT_CTL_RESEN` reader - Watchdog Reset Enable"]
pub type WDT_CTL_RESEN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CTL_RESEN` writer - Watchdog Reset Enable"]
pub type WDT_CTL_RESEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `WDT_CTL_INTTYPE` reader - Watchdog Interrupt Type"]
pub type WDT_CTL_INTTYPE_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CTL_INTTYPE` writer - Watchdog Interrupt Type"]
pub type WDT_CTL_INTTYPE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `WDT_CTL_WRC` reader - Write Complete"]
pub type WDT_CTL_WRC_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CTL_WRC` writer - Write Complete"]
pub type WDT_CTL_WRC_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdt_ctl_inten(&self) -> WDT_CTL_INTEN_R {
        WDT_CTL_INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdt_ctl_resen(&self) -> WDT_CTL_RESEN_R {
        WDT_CTL_RESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    pub fn wdt_ctl_inttype(&self) -> WDT_CTL_INTTYPE_R {
        WDT_CTL_INTTYPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    pub fn wdt_ctl_wrc(&self) -> WDT_CTL_WRC_R {
        WDT_CTL_WRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdt_ctl_inten(&mut self) -> WDT_CTL_INTEN_W {
        WDT_CTL_INTEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Reset Enable"]
    #[inline(always)]
    pub fn wdt_ctl_resen(&mut self) -> WDT_CTL_RESEN_W {
        WDT_CTL_RESEN_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog Interrupt Type"]
    #[inline(always)]
    pub fn wdt_ctl_inttype(&mut self) -> WDT_CTL_INTTYPE_W {
        WDT_CTL_INTTYPE_W::new(self)
    }
    #[doc = "Bit 31 - Write Complete"]
    #[inline(always)]
    pub fn wdt_ctl_wrc(&mut self) -> WDT_CTL_WRC_W {
        WDT_CTL_WRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
