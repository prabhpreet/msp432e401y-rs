#[doc = "Register `TXCSRL7` reader"]
pub struct R(crate::R<TXCSRL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCSRL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCSRL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCSRL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCSRL7` writer"]
pub struct W(crate::W<TXCSRL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCSRL7_SPEC>;
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
impl From<crate::W<TXCSRL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCSRL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXCSRL7_TXRDY` reader - Transmit Packet Ready"]
pub type USB_TXCSRL7_TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_TXRDY` writer - Transmit Packet Ready"]
pub type USB_TXCSRL7_TXRDY_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 0>;
#[doc = "Field `USB_TXCSRL7_FIFONE` reader - FIFO Not Empty"]
pub type USB_TXCSRL7_FIFONE_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_FIFONE` writer - FIFO Not Empty"]
pub type USB_TXCSRL7_FIFONE_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 1>;
#[doc = "Field `USB_TXCSRL7_ERROR` reader - Error"]
pub type USB_TXCSRL7_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_ERROR` writer - Error"]
pub type USB_TXCSRL7_ERROR_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 2>;
#[doc = "Field `USB_TXCSRL7_FLUSH` reader - Flush FIFO"]
pub type USB_TXCSRL7_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_FLUSH` writer - Flush FIFO"]
pub type USB_TXCSRL7_FLUSH_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 3>;
#[doc = "Field `USB_TXCSRL7_SETUP` reader - Setup Packet"]
pub type USB_TXCSRL7_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_SETUP` writer - Setup Packet"]
pub type USB_TXCSRL7_SETUP_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 4>;
#[doc = "Field `USB_TXCSRL7_STALLED` reader - Endpoint Stalled"]
pub type USB_TXCSRL7_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_STALLED` writer - Endpoint Stalled"]
pub type USB_TXCSRL7_STALLED_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 5>;
#[doc = "Field `USB_TXCSRL7_CLRDT` reader - Clear Data Toggle"]
pub type USB_TXCSRL7_CLRDT_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_CLRDT` writer - Clear Data Toggle"]
pub type USB_TXCSRL7_CLRDT_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 6>;
#[doc = "Field `USB_TXCSRL7_NAKTO` reader - NAK Timeout"]
pub type USB_TXCSRL7_NAKTO_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL7_NAKTO` writer - NAK Timeout"]
pub type USB_TXCSRL7_NAKTO_W<'a> = crate::BitWriter<'a, u8, TXCSRL7_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl7_txrdy(&self) -> USB_TXCSRL7_TXRDY_R {
        USB_TXCSRL7_TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl7_fifone(&self) -> USB_TXCSRL7_FIFONE_R {
        USB_TXCSRL7_FIFONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl7_error(&self) -> USB_TXCSRL7_ERROR_R {
        USB_TXCSRL7_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl7_flush(&self) -> USB_TXCSRL7_FLUSH_R {
        USB_TXCSRL7_FLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl7_setup(&self) -> USB_TXCSRL7_SETUP_R {
        USB_TXCSRL7_SETUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl7_stalled(&self) -> USB_TXCSRL7_STALLED_R {
        USB_TXCSRL7_STALLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl7_clrdt(&self) -> USB_TXCSRL7_CLRDT_R {
        USB_TXCSRL7_CLRDT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl7_nakto(&self) -> USB_TXCSRL7_NAKTO_R {
        USB_TXCSRL7_NAKTO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn usb_txcsrl7_txrdy(&mut self) -> USB_TXCSRL7_TXRDY_W {
        USB_TXCSRL7_TXRDY_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Not Empty"]
    #[inline(always)]
    pub fn usb_txcsrl7_fifone(&mut self) -> USB_TXCSRL7_FIFONE_W {
        USB_TXCSRL7_FIFONE_W::new(self)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn usb_txcsrl7_error(&mut self) -> USB_TXCSRL7_ERROR_W {
        USB_TXCSRL7_ERROR_W::new(self)
    }
    #[doc = "Bit 3 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_txcsrl7_flush(&mut self) -> USB_TXCSRL7_FLUSH_W {
        USB_TXCSRL7_FLUSH_W::new(self)
    }
    #[doc = "Bit 4 - Setup Packet"]
    #[inline(always)]
    pub fn usb_txcsrl7_setup(&mut self) -> USB_TXCSRL7_SETUP_W {
        USB_TXCSRL7_SETUP_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint Stalled"]
    #[inline(always)]
    pub fn usb_txcsrl7_stalled(&mut self) -> USB_TXCSRL7_STALLED_W {
        USB_TXCSRL7_STALLED_W::new(self)
    }
    #[doc = "Bit 6 - Clear Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrl7_clrdt(&mut self) -> USB_TXCSRL7_CLRDT_W {
        USB_TXCSRL7_CLRDT_W::new(self)
    }
    #[doc = "Bit 7 - NAK Timeout"]
    #[inline(always)]
    pub fn usb_txcsrl7_nakto(&mut self) -> USB_TXCSRL7_NAKTO_W {
        USB_TXCSRL7_NAKTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Control and Status Endpoint 7 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl7](index.html) module"]
pub struct TXCSRL7_SPEC;
impl crate::RegisterSpec for TXCSRL7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txcsrl7::R](R) reader structure"]
impl crate::Readable for TXCSRL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcsrl7::W](W) writer structure"]
impl crate::Writable for TXCSRL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCSRL7 to value 0"]
impl crate::Resettable for TXCSRL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
