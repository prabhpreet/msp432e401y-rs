#[doc = "Register `CALLD1` writer"]
pub struct W(crate::W<CALLD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALLD1_SPEC>;
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
impl From<crate::W<CALLD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALLD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CALLD1_DOM` writer - Day of Month"]
pub type HIB_CALLD1_DOM_W<'a> = crate::FieldWriter<'a, u32, CALLD1_SPEC, u8, u8, 5, 0>;
#[doc = "Field `HIB_CALLD1_MON` writer - Month"]
pub type HIB_CALLD1_MON_W<'a> = crate::FieldWriter<'a, u32, CALLD1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `HIB_CALLD1_YEAR` writer - Year Value"]
pub type HIB_CALLD1_YEAR_W<'a> = crate::FieldWriter<'a, u32, CALLD1_SPEC, u8, u8, 7, 16>;
#[doc = "Field `HIB_CALLD1_DOW` writer - Day of Week"]
pub type HIB_CALLD1_DOW_W<'a> = crate::FieldWriter<'a, u32, CALLD1_SPEC, u8, u8, 3, 24>;
impl W {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_calld1_dom(&mut self) -> HIB_CALLD1_DOM_W {
        HIB_CALLD1_DOM_W::new(self)
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_calld1_mon(&mut self) -> HIB_CALLD1_MON_W {
        HIB_CALLD1_MON_W::new(self)
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_calld1_year(&mut self) -> HIB_CALLD1_YEAR_W {
        HIB_CALLD1_YEAR_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_calld1_dow(&mut self) -> HIB_CALLD1_DOW_W {
        HIB_CALLD1_DOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar Load\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calld1](index.html) module"]
pub struct CALLD1_SPEC;
impl crate::RegisterSpec for CALLD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [calld1::W](W) writer structure"]
impl crate::Writable for CALLD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALLD1 to value 0"]
impl crate::Resettable for CALLD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
