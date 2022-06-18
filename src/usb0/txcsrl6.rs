#[doc = "Register `TXCSRL6` reader"]
pub struct R(crate::R<TXCSRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCSRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCSRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCSRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCSRL6` writer"]
pub struct W(crate::W<TXCSRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCSRL6_SPEC>;
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
impl From<crate::W<TXCSRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCSRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXCSRL6_TXRDY` reader - Transmit Packet Ready"]
pub type USB_TXCSRL6_TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_TXRDY` writer - Transmit Packet Ready"]
pub type USB_TXCSRL6_TXRDY_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 0>;
#[doc = "Field `USB_TXCSRL6_FIFONE` reader - FIFO Not Empty"]
pub type USB_TXCSRL6_FIFONE_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_FIFONE` writer - FIFO Not Empty"]
pub type USB_TXCSRL6_FIFONE_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 1>;
#[doc = "Field `USB_TXCSRL6_ERROR` reader - Error"]
pub type USB_TXCSRL6_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_ERROR` writer - Error"]
pub type USB_TXCSRL6_ERROR_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 2>;
#[doc = "Field `USB_TXCSRL6_FLUSH` reader - Flush FIFO"]
pub type USB_TXCSRL6_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_FLUSH` writer - Flush FIFO"]
pub type USB_TXCSRL6_FLUSH_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 3>;
#[doc = "Field `USB_TXCSRL6_SETUP` reader - Setup Packet"]
pub type USB_TXCSRL6_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_SETUP` writer - Setup Packet"]
pub type USB_TXCSRL6_SETUP_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 4>;
#[doc = "Field `USB_TXCSRL6_STALLED` reader - Endpoint Stalled"]
pub type USB_TXCSRL6_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_STALLED` writer - Endpoint Stalled"]
pub type USB_TXCSRL6_STALLED_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 5>;
#[doc = "Field `USB_TXCSRL6_CLRDT` reader - Clear Data Toggle"]
pub type USB_TXCSRL6_CLRDT_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_CLRDT` writer - Clear Data Toggle"]
pub type USB_TXCSRL6_CLRDT_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 6>;
#[doc = "Field `USB_TXCSRL6_NAKTO` reader - NAK Timeout"]
pub type USB_TXCSRL6_NAKTO_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL6_NAKTO` writer - NAK Timeout"]
pub type USB_TXCSRL6_NAKTO_W<'a> = crate::BitWriter<'a, u8, TXCSRL6_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl6_txrdy(&self) -> USB_TXCSRL6_TXRDY_R {
        USB_TXCSRL6_TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl6_fifone(&self) -> USB_TXCSRL6_FIFONE_R {
        USB_TXCSRL6_FIFONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl6_error(&self) -> USB_TXCSRL6_ERROR_R {
        USB_TXCSRL6_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl6_flush(&self) -> USB_TXCSRL6_FLUSH_R {
        USB_TXCSRL6_FLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl6_setup(&self) -> USB_TXCSRL6_SETUP_R {
        USB_TXCSRL6_SETUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl6_stalled(&self) -> USB_TXCSRL6_STALLED_R {
        USB_TXCSRL6_STALLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl6_clrdt(&self) -> USB_TXCSRL6_CLRDT_R {
        USB_TXCSRL6_CLRDT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl6_nakto(&self) -> USB_TXCSRL6_NAKTO_R {
        USB_TXCSRL6_NAKTO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl6_txrdy(&mut self) -> USB_TXCSRL6_TXRDY_W {
        USB_TXCSRL6_TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl6_fifone(&mut self) -> USB_TXCSRL6_FIFONE_W {
        USB_TXCSRL6_FIFONE_W::new(self)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl6_error(&mut self) -> USB_TXCSRL6_ERROR_W {
        USB_TXCSRL6_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl6_flush(&mut self) -> USB_TXCSRL6_FLUSH_W {
        USB_TXCSRL6_FLUSH_W::new(self)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl6_setup(&mut self) -> USB_TXCSRL6_SETUP_W {
        USB_TXCSRL6_SETUP_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl6_stalled(&mut self) -> USB_TXCSRL6_STALLED_W {
        USB_TXCSRL6_STALLED_W::new(self)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl6_clrdt(&mut self) -> USB_TXCSRL6_CLRDT_W {
        USB_TXCSRL6_CLRDT_W::new(self)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl6_nakto(&mut self) -> USB_TXCSRL6_NAKTO_W {
        USB_TXCSRL6_NAKTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Control and Status Endpoint 6 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl6](index.html) module"]
pub struct TXCSRL6_SPEC;
impl crate::RegisterSpec for TXCSRL6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txcsrl6::R](R) reader structure"]
impl crate::Readable for TXCSRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcsrl6::W](W) writer structure"]
impl crate::Writable for TXCSRL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCSRL6 to value 0"]
impl crate::Resettable for TXCSRL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
