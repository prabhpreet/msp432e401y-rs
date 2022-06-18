#[doc = "Register `CALLD0` writer"]
pub struct W(crate::W<CALLD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALLD0_SPEC>;
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
impl From<crate::W<CALLD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALLD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CALLD0_SEC` writer - Seconds"]
pub type HIB_CALLD0_SEC_W<'a> = crate::FieldWriter<'a, u32, CALLD0_SPEC, u8, u8, 6, 0>;
#[doc = "Field `HIB_CALLD0_MIN` writer - Minutes"]
pub type HIB_CALLD0_MIN_W<'a> = crate::FieldWriter<'a, u32, CALLD0_SPEC, u8, u8, 6, 8>;
#[doc = "Field `HIB_CALLD0_HR` writer - Hours"]
pub type HIB_CALLD0_HR_W<'a> = crate::FieldWriter<'a, u32, CALLD0_SPEC, u8, u8, 5, 16>;
#[doc = "Field `HIB_CALLD0_AMPM` writer - AM/PM Designation"]
pub type HIB_CALLD0_AMPM_W<'a> = crate::BitWriter<'a, u32, CALLD0_SPEC, bool, 22>;
impl W {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_calld0_sec(&mut self) -> HIB_CALLD0_SEC_W {
        HIB_CALLD0_SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_calld0_min(&mut self) -> HIB_CALLD0_MIN_W {
        HIB_CALLD0_MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_calld0_hr(&mut self) -> HIB_CALLD0_HR_W {
        HIB_CALLD0_HR_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_calld0_ampm(&mut self) -> HIB_CALLD0_AMPM_W {
        HIB_CALLD0_AMPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar Load 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calld0](index.html) module"]
pub struct CALLD0_SPEC;
impl crate::RegisterSpec for CALLD0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [calld0::W](W) writer structure"]
impl crate::Writable for CALLD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALLD0 to value 0"]
impl crate::Resettable for CALLD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
