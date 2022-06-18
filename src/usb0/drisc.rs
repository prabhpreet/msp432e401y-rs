#[doc = "Register `DRISC` writer"]
pub struct W(crate::W<DRISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRISC_SPEC>;
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
impl From<crate::W<DRISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DRISC_RESUME` writer - RESUME Interrupt Status and Clear"]
pub type USB_DRISC_RESUME_W<'a> = crate::BitWriter<'a, u32, DRISC_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 0 - RESUME Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_drisc_resume(&mut self) -> USB_DRISC_RESUME_W {
        USB_DRISC_RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device RESUME Interrupt Status and Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drisc](index.html) module"]
pub struct DRISC_SPEC;
impl crate::RegisterSpec for DRISC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [drisc::W](W) writer structure"]
impl crate::Writable for DRISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRISC to value 0"]
impl crate::Resettable for DRISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
