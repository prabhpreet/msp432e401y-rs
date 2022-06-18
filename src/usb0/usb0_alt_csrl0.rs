#[doc = "Register `CSRL0` writer"]
pub struct W(crate::W<USB0_ALT_CSRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_CSRL0_SPEC>;
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
impl From<crate::W<USB0_ALT_CSRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_CSRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CSRL0_SETUP` writer - Setup Packet"]
pub type USB_CSRL0_SETUP_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_CSRL0_SPEC, bool, 3>;
#[doc = "Field `USB_CSRL0_ERROR` writer - Error"]
pub type USB_CSRL0_ERROR_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_CSRL0_SPEC, bool, 4>;
#[doc = "Field `USB_CSRL0_REQPKT` writer - Request Packet"]
pub type USB_CSRL0_REQPKT_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_CSRL0_SPEC, bool, 5>;
#[doc = "Field `USB_CSRL0_STATUS` writer - STATUS Packet"]
pub type USB_CSRL0_STATUS_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_CSRL0_SPEC, bool, 6>;
#[doc = "Field `USB_CSRL0_NAKTO` writer - NAK Timeout"]
pub type USB_CSRL0_NAKTO_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_CSRL0_SPEC, bool, 7>;
impl W {
    #[doc = "Bit 3 - Setup Packet"]
    #[inline(always)]
    pub fn usb_csrl0_setup(&mut self) -> USB_CSRL0_SETUP_W {
        USB_CSRL0_SETUP_W::new(self)
    }
    #[doc = "Bit 4 - Error"]
    #[inline(always)]
    pub fn usb_csrl0_error(&mut self) -> USB_CSRL0_ERROR_W {
        USB_CSRL0_ERROR_W::new(self)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    pub fn usb_csrl0_reqpkt(&mut self) -> USB_CSRL0_REQPKT_W {
        USB_CSRL0_REQPKT_W::new(self)
    }
    #[doc = "Bit 6 - STATUS Packet"]
    #[inline(always)]
    pub fn usb_csrl0_status(&mut self) -> USB_CSRL0_STATUS_W {
        USB_CSRL0_STATUS_W::new(self)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_csrl0_nakto(&mut self) -> USB_CSRL0_NAKTO_W {
        USB_CSRL0_NAKTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control and Status Endpoint 0 Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_csrl0](index.html) module"]
pub struct USB0_ALT_CSRL0_SPEC;
impl crate::RegisterSpec for USB0_ALT_CSRL0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_csrl0::W](W) writer structure"]
impl crate::Writable for USB0_ALT_CSRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSRL0 to value 0"]
impl crate::Resettable for USB0_ALT_CSRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
