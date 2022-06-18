#[doc = "Register `RXCSRL3` reader"]
pub struct R(crate::R<USB0_ALT_RXCSRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_ALT_RXCSRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_ALT_RXCSRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_ALT_RXCSRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRL3` writer"]
pub struct W(crate::W<USB0_ALT_RXCSRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_RXCSRL3_SPEC>;
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
impl From<crate::W<USB0_ALT_RXCSRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_RXCSRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRL3_ERROR` reader - Error"]
pub type USB_RXCSRL3_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL3_ERROR` writer - Error"]
pub type USB_RXCSRL3_ERROR_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_RXCSRL3_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRL3_NAKTO` reader - NAK Timeout"]
pub type USB_RXCSRL3_NAKTO_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL3_NAKTO` writer - NAK Timeout"]
pub type USB_RXCSRL3_NAKTO_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_RXCSRL3_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRL3_REQPKT` reader - Request Packet"]
pub type USB_RXCSRL3_REQPKT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL3_REQPKT` writer - Request Packet"]
pub type USB_RXCSRL3_REQPKT_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_RXCSRL3_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_rxcsrl3_error(&self) -> USB_RXCSRL3_ERROR_R {
        USB_RXCSRL3_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_rxcsrl3_nakto(&self) -> USB_RXCSRL3_NAKTO_R {
        USB_RXCSRL3_NAKTO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    pub fn usb_rxcsrl3_reqpkt(&self) -> USB_RXCSRL3_REQPKT_R {
        USB_RXCSRL3_REQPKT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_rxcsrl3_error(&mut self) -> USB_RXCSRL3_ERROR_W {
        USB_RXCSRL3_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_rxcsrl3_nakto(&mut self) -> USB_RXCSRL3_NAKTO_W {
        USB_RXCSRL3_NAKTO_W::new(self)
    }
    #[doc = "Bit 5 - Request Packet"]
    #[inline(always)]
    pub fn usb_rxcsrl3_reqpkt(&mut self) -> USB_RXCSRL3_REQPKT_W {
        USB_RXCSRL3_REQPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_rxcsrl3](index.html) module"]
pub struct USB0_ALT_RXCSRL3_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXCSRL3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb0_alt_rxcsrl3::R](R) reader structure"]
impl crate::Readable for USB0_ALT_RXCSRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_rxcsrl3::W](W) writer structure"]
impl crate::Writable for USB0_ALT_RXCSRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRL3 to value 0"]
impl crate::Resettable for USB0_ALT_RXCSRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
