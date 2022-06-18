#[doc = "Register `LOAD` reader"]
pub struct R(crate::R<LOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD` writer"]
pub struct W(crate::W<LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_SPEC>;
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
impl From<crate::W<LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_LOAD` reader - Watchdog Load Value"]
pub type WDT_LOAD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_LOAD` writer - Watchdog Load Value"]
pub type WDT_LOAD_W<'a> = crate::FieldWriter<'a, u32, LOAD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdt_load(&self) -> WDT_LOAD_R {
        WDT_LOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Load Value"]
    #[inline(always)]
    pub fn wdt_load(&mut self) -> WDT_LOAD_W {
        WDT_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](index.html) module"]
pub struct LOAD_SPEC;
impl crate::RegisterSpec for LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load::R](R) reader structure"]
impl crate::Readable for LOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load::W](W) writer structure"]
impl crate::Writable for LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOAD to value 0"]
impl crate::Resettable for LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
