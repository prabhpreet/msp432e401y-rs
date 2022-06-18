#[doc = "Register `IS` reader"]
pub struct R(crate::R<USB0_ALT_IS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_ALT_IS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_ALT_IS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_ALT_IS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS` writer"]
pub struct W(crate::W<USB0_ALT_IS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_IS_SPEC>;
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
impl From<crate::W<USB0_ALT_IS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_IS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_IS_RESET` reader - RESET Signaling Detected"]
pub type USB_IS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_RESET` writer - RESET Signaling Detected"]
pub type USB_IS_RESET_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_IS_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 2 - RESET Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_reset(&self) -> USB_IS_RESET_R {
        USB_IS_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RESET Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_reset(&mut self) -> USB_IS_RESET_W {
        USB_IS_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB General Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_is](index.html) module"]
pub struct USB0_ALT_IS_SPEC;
impl crate::RegisterSpec for USB0_ALT_IS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb0_alt_is::R](R) reader structure"]
impl crate::Readable for USB0_ALT_IS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_is::W](W) writer structure"]
impl crate::Writable for USB0_ALT_IS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for USB0_ALT_IS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
