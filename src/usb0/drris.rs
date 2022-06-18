#[doc = "Register `DRRIS` reader"]
pub struct R(crate::R<DRRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRRIS` writer"]
pub struct W(crate::W<DRRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRRIS_SPEC>;
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
impl From<crate::W<DRRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DRRIS_RESUME` reader - RESUME Interrupt Status"]
pub type USB_DRRIS_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `USB_DRRIS_RESUME` writer - RESUME Interrupt Status"]
pub type USB_DRRIS_RESUME_W<'a> = crate::BitWriter<'a, u32, DRRIS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - RESUME Interrupt Status"]
    #[inline(always)]
    pub fn usb_drris_resume(&self) -> USB_DRRIS_RESUME_R {
        USB_DRRIS_RESUME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESUME Interrupt Status"]
    #[inline(always)]
    pub fn usb_drris_resume(&mut self) -> USB_DRRIS_RESUME_W {
        USB_DRRIS_RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device RESUME Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drris](index.html) module"]
pub struct DRRIS_SPEC;
impl crate::RegisterSpec for DRRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drris::R](R) reader structure"]
impl crate::Readable for DRRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drris::W](W) writer structure"]
impl crate::Writable for DRRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRRIS to value 0"]
impl crate::Resettable for DRRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
