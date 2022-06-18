#[doc = "Register `RXCSRL5` reader"]
pub struct R(crate::R<RXCSRL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCSRL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCSRL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCSRL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRL5` writer"]
pub struct W(crate::W<RXCSRL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCSRL5_SPEC>;
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
impl From<crate::W<RXCSRL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCSRL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRL5_RXRDY` reader - Receive Packet Ready"]
pub type USB_RXCSRL5_RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_RXRDY` writer - Receive Packet Ready"]
pub type USB_RXCSRL5_RXRDY_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 0>;
#[doc = "Field `USB_RXCSRL5_FULL` reader - FIFO Full"]
pub type USB_RXCSRL5_FULL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_FULL` writer - FIFO Full"]
pub type USB_RXCSRL5_FULL_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 1>;
#[doc = "Field `USB_RXCSRL5_OVER` reader - Overrun"]
pub type USB_RXCSRL5_OVER_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_OVER` writer - Overrun"]
pub type USB_RXCSRL5_OVER_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRL5_DATAERR` reader - Data Error"]
pub type USB_RXCSRL5_DATAERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_DATAERR` writer - Data Error"]
pub type USB_RXCSRL5_DATAERR_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRL5_FLUSH` reader - Flush FIFO"]
pub type USB_RXCSRL5_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_FLUSH` writer - Flush FIFO"]
pub type USB_RXCSRL5_FLUSH_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRL5_STALL` reader - Send STALL"]
pub type USB_RXCSRL5_STALL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_STALL` writer - Send STALL"]
pub type USB_RXCSRL5_STALL_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 5>;
#[doc = "Field `USB_RXCSRL5_STALLED` reader - Endpoint Stalled"]
pub type USB_RXCSRL5_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_STALLED` writer - Endpoint Stalled"]
pub type USB_RXCSRL5_STALLED_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 6>;
#[doc = "Field `USB_RXCSRL5_CLRDT` reader - Clear Data Toggle"]
pub type USB_RXCSRL5_CLRDT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL5_CLRDT` writer - Clear Data Toggle"]
pub type USB_RXCSRL5_CLRDT_W<'a> = crate::BitWriter<'a, u8, RXCSRL5_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl5_rxrdy(&self) -> USB_RXCSRL5_RXRDY_R {
        USB_RXCSRL5_RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl5_full(&self) -> USB_RXCSRL5_FULL_R {
        USB_RXCSRL5_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl5_over(&self) -> USB_RXCSRL5_OVER_R {
        USB_RXCSRL5_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl5_dataerr(&self) -> USB_RXCSRL5_DATAERR_R {
        USB_RXCSRL5_DATAERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl5_flush(&self) -> USB_RXCSRL5_FLUSH_R {
        USB_RXCSRL5_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stall(&self) -> USB_RXCSRL5_STALL_R {
        USB_RXCSRL5_STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stalled(&self) -> USB_RXCSRL5_STALLED_R {
        USB_RXCSRL5_STALLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl5_clrdt(&self) -> USB_RXCSRL5_CLRDT_R {
        USB_RXCSRL5_CLRDT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl5_rxrdy(&mut self) -> USB_RXCSRL5_RXRDY_W {
        USB_RXCSRL5_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl5_full(&mut self) -> USB_RXCSRL5_FULL_W {
        USB_RXCSRL5_FULL_W::new(self)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl5_over(&mut self) -> USB_RXCSRL5_OVER_W {
        USB_RXCSRL5_OVER_W::new(self)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl5_dataerr(&mut self) -> USB_RXCSRL5_DATAERR_W {
        USB_RXCSRL5_DATAERR_W::new(self)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl5_flush(&mut self) -> USB_RXCSRL5_FLUSH_W {
        USB_RXCSRL5_FLUSH_W::new(self)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stall(&mut self) -> USB_RXCSRL5_STALL_W {
        USB_RXCSRL5_STALL_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl5_stalled(&mut self) -> USB_RXCSRL5_STALLED_W {
        USB_RXCSRL5_STALLED_W::new(self)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl5_clrdt(&mut self) -> USB_RXCSRL5_CLRDT_W {
        USB_RXCSRL5_CLRDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 5 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl5](index.html) module"]
pub struct RXCSRL5_SPEC;
impl crate::RegisterSpec for RXCSRL5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxcsrl5::R](R) reader structure"]
impl crate::Readable for RXCSRL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcsrl5::W](W) writer structure"]
impl crate::Writable for RXCSRL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRL5 to value 0"]
impl crate::Resettable for RXCSRL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
