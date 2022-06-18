#[doc = "Register `RXCSRL2` reader"]
pub struct R(crate::R<RXCSRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCSRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCSRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCSRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRL2` writer"]
pub struct W(crate::W<RXCSRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCSRL2_SPEC>;
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
impl From<crate::W<RXCSRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCSRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRL2_RXRDY` reader - Receive Packet Ready"]
pub type USB_RXCSRL2_RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_RXRDY` writer - Receive Packet Ready"]
pub type USB_RXCSRL2_RXRDY_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 0>;
#[doc = "Field `USB_RXCSRL2_FULL` reader - FIFO Full"]
pub type USB_RXCSRL2_FULL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_FULL` writer - FIFO Full"]
pub type USB_RXCSRL2_FULL_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 1>;
#[doc = "Field `USB_RXCSRL2_OVER` reader - Overrun"]
pub type USB_RXCSRL2_OVER_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_OVER` writer - Overrun"]
pub type USB_RXCSRL2_OVER_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRL2_DATAERR` reader - Data Error"]
pub type USB_RXCSRL2_DATAERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_DATAERR` writer - Data Error"]
pub type USB_RXCSRL2_DATAERR_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRL2_FLUSH` reader - Flush FIFO"]
pub type USB_RXCSRL2_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_FLUSH` writer - Flush FIFO"]
pub type USB_RXCSRL2_FLUSH_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRL2_STALL` reader - Send STALL"]
pub type USB_RXCSRL2_STALL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_STALL` writer - Send STALL"]
pub type USB_RXCSRL2_STALL_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 5>;
#[doc = "Field `USB_RXCSRL2_STALLED` reader - Endpoint Stalled"]
pub type USB_RXCSRL2_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_STALLED` writer - Endpoint Stalled"]
pub type USB_RXCSRL2_STALLED_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 6>;
#[doc = "Field `USB_RXCSRL2_CLRDT` reader - Clear Data Toggle"]
pub type USB_RXCSRL2_CLRDT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRL2_CLRDT` writer - Clear Data Toggle"]
pub type USB_RXCSRL2_CLRDT_W<'a> = crate::BitWriter<'a, u8, RXCSRL2_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl2_rxrdy(&self) -> USB_RXCSRL2_RXRDY_R {
        USB_RXCSRL2_RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl2_full(&self) -> USB_RXCSRL2_FULL_R {
        USB_RXCSRL2_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl2_over(&self) -> USB_RXCSRL2_OVER_R {
        USB_RXCSRL2_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl2_dataerr(&self) -> USB_RXCSRL2_DATAERR_R {
        USB_RXCSRL2_DATAERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl2_flush(&self) -> USB_RXCSRL2_FLUSH_R {
        USB_RXCSRL2_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl2_stall(&self) -> USB_RXCSRL2_STALL_R {
        USB_RXCSRL2_STALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl2_stalled(&self) -> USB_RXCSRL2_STALLED_R {
        USB_RXCSRL2_STALLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl2_clrdt(&self) -> USB_RXCSRL2_CLRDT_R {
        USB_RXCSRL2_CLRDT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Packet Ready"]
    #[inline(always)]
    pub fn usb_rxcsrl2_rxrdy(&mut self) -> USB_RXCSRL2_RXRDY_W {
        USB_RXCSRL2_RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full"]
    #[inline(always)]
    pub fn usb_rxcsrl2_full(&mut self) -> USB_RXCSRL2_FULL_W {
        USB_RXCSRL2_FULL_W::new(self)
    }
    #[doc = "Bit 2 - Overrun"]
    #[inline(always)]
    pub fn usb_rxcsrl2_over(&mut self) -> USB_RXCSRL2_OVER_W {
        USB_RXCSRL2_OVER_W::new(self)
    }
    #[doc = "Bit 3 - Data Error"]
    #[inline(always)]
    pub fn usb_rxcsrl2_dataerr(&mut self) -> USB_RXCSRL2_DATAERR_W {
        USB_RXCSRL2_DATAERR_W::new(self)
    }
    #[doc = "Bit 4 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_rxcsrl2_flush(&mut self) -> USB_RXCSRL2_FLUSH_W {
        USB_RXCSRL2_FLUSH_W::new(self)
    }
    #[doc = "Bit 5 - Send STALL"]
    #[inline(always)]
    pub fn usb_rxcsrl2_stall(&mut self) -> USB_RXCSRL2_STALL_W {
        USB_RXCSRL2_STALL_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_rxcsrl2_stalled(&mut self) -> USB_RXCSRL2_STALLED_W {
        USB_RXCSRL2_STALLED_W::new(self)
    }
    #[doc = "Bit 7 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrl2_clrdt(&mut self) -> USB_RXCSRL2_CLRDT_W {
        USB_RXCSRL2_CLRDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 2 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl2](index.html) module"]
pub struct RXCSRL2_SPEC;
impl crate::RegisterSpec for RXCSRL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxcsrl2::R](R) reader structure"]
impl crate::Readable for RXCSRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcsrl2::W](W) writer structure"]
impl crate::Writable for RXCSRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRL2 to value 0"]
impl crate::Resettable for RXCSRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
