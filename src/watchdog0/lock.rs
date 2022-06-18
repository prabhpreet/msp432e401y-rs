#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WDT_LOCK_A {
    #[doc = "0: Unlocked"]
    WDT_LOCK_UNLOCKED = 0,
    #[doc = "1: Locked"]
    WDT_LOCK_LOCKED = 1,
}
impl From<WDT_LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDT_LOCK` reader - Watchdog Lock"]
pub type WDT_LOCK_R = crate::FieldReader<u32, WDT_LOCK_A>;
impl WDT_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDT_LOCK_A> {
        match self.bits {
            0 => Some(WDT_LOCK_A::WDT_LOCK_UNLOCKED),
            1 => Some(WDT_LOCK_A::WDT_LOCK_LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WDT_LOCK_UNLOCKED`"]
    #[inline(always)]
    pub fn is_wdt_lock_unlocked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `WDT_LOCK_LOCKED`"]
    #[inline(always)]
    pub fn is_wdt_lock_locked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_LOCKED
    }
}
#[doc = "Field `WDT_LOCK` writer - Watchdog Lock"]
pub type WDT_LOCK_W<'a> = crate::FieldWriter<'a, u32, LOCK_SPEC, u32, WDT_LOCK_A, 32, 0>;
impl<'a> WDT_LOCK_W<'a> {
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn wdt_lock_unlocked(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::WDT_LOCK_UNLOCKED)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn wdt_lock_locked(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::WDT_LOCK_LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W {
        WDT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
