#[doc = "Register `CSRL0` writer"]
pub struct W(crate::W<CSRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSRL0_SPEC>;
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
impl From<crate::W<CSRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CSRL0_RXRDY` writer - Receive Packet Ready"]
pub type USB_CSRL0_RXRDY_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 0>;
#[doc = "Field `USB_CSRL0_TXRDY` writer - Transmit Packet Ready"]
pub type USB_CSRL0_TXRDY_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 1>;
#[doc = "Field `USB_CSRL0_STALLED` writer - Endpoint Stalled"]
pub type USB_CSRL0_STALLED_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 2>;
#[doc = "Field `USB_CSRL0_DATAEND` writer - Data End"]
pub type USB_CSRL0_DATAEND_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 3>;
#[doc = "Field `USB_CSRL0_SETEND` writer - Setup End"]
pub type USB_CSRL0_SETEND_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 4>;
#[doc = "Field `USB_CSRL0_STALL` writer - Send Stall"]
pub type USB_CSRL0_STALL_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 5>;
#[doc = "Field `USB_CSRL0_RXRDYC` writer - RXRDY Clear"]
pub type USB_CSRL0_RXRDYC_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 6>;
#[doc = "Field `USB_CSRL0_SETENDC` writer - Setup End Clear"]
pub type USB_CSRL0_SETENDC_W<'a> = crate::BitWriter<'a, u8, CSRL0_SPEC, bool, 7>;
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_csrl0_rxrdy(&mut self) -> USB_CSRL0_RXRDY_W {
        USB_CSRL0_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_csrl0_txrdy(&mut self) -> USB_CSRL0_TXRDY_W {
        USB_CSRL0_TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_csrl0_stalled(&mut self) -> USB_CSRL0_STALLED_W {
        USB_CSRL0_STALLED_W::new(self)
    }
    #[doc = "Bit 3 - Data End"]
    #[inline(always)]
    pub fn usb_csrl0_dataend(&mut self) -> USB_CSRL0_DATAEND_W {
        USB_CSRL0_DATAEND_W::new(self)
    }
    #[doc = "Bit 4 - Setup End"]
    #[inline(always)]
    pub fn usb_csrl0_setend(&mut self) -> USB_CSRL0_SETEND_W {
        USB_CSRL0_SETEND_W::new(self)
    }
    #[doc = "Bit 5 - Send Stall"]
    #[inline(always)]
    pub fn usb_csrl0_stall(&mut self) -> USB_CSRL0_STALL_W {
        USB_CSRL0_STALL_W::new(self)
    }
    #[doc = "Bit 6 - RXRDY Clear"]
    #[inline(always)]
    pub fn usb_csrl0_rxrdyc(&mut self) -> USB_CSRL0_RXRDYC_W {
        USB_CSRL0_RXRDYC_W::new(self)
    }
    #[doc = "Bit 7 - Setup End Clear"]
    #[inline(always)]
    pub fn usb_csrl0_setendc(&mut self) -> USB_CSRL0_SETENDC_W {
        USB_CSRL0_SETENDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control and Status Endpoint 0 Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrl0](index.html) module"]
pub struct CSRL0_SPEC;
impl crate::RegisterSpec for CSRL0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [csrl0::W](W) writer structure"]
impl crate::Writable for CSRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSRL0 to value 0"]
impl crate::Resettable for CSRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
