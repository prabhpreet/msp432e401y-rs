#[doc = "Register `TXCSRL1` reader"]
pub struct R(crate::R<USB0_ALT_TXCSRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_ALT_TXCSRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_ALT_TXCSRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_ALT_TXCSRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCSRL1` writer"]
pub struct W(crate::W<USB0_ALT_TXCSRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_TXCSRL1_SPEC>;
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
impl From<crate::W<USB0_ALT_TXCSRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_TXCSRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXCSRL1_UNDRN` reader - Underrun"]
pub type USB_TXCSRL1_UNDRN_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL1_UNDRN` writer - Underrun"]
pub type USB_TXCSRL1_UNDRN_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_TXCSRL1_SPEC, bool, 2>;
#[doc = "Field `USB_TXCSRL1_STALL` reader - Send STALL"]
pub type USB_TXCSRL1_STALL_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRL1_STALL` writer - Send STALL"]
pub type USB_TXCSRL1_STALL_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_TXCSRL1_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn usb_txcsrl1_undrn(&self) -> USB_TXCSRL1_UNDRN_R {
        USB_TXCSRL1_UNDRN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn usb_txcsrl1_stall(&self) -> USB_TXCSRL1_STALL_R {
        USB_TXCSRL1_STALL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Underrun"]
    #[inline(always)]
    pub fn usb_txcsrl1_undrn(&mut self) -> USB_TXCSRL1_UNDRN_W {
        USB_TXCSRL1_UNDRN_W::new(self)
    }
    #[doc = "Bit 4 - Send STALL"]
    #[inline(always)]
    pub fn usb_txcsrl1_stall(&mut self) -> USB_TXCSRL1_STALL_W {
        USB_TXCSRL1_STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Control and Status Endpoint 1 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_txcsrl1](index.html) module"]
pub struct USB0_ALT_TXCSRL1_SPEC;
impl crate::RegisterSpec for USB0_ALT_TXCSRL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb0_alt_txcsrl1::R](R) reader structure"]
impl crate::Readable for USB0_ALT_TXCSRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_txcsrl1::W](W) writer structure"]
impl crate::Writable for USB0_ALT_TXCSRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCSRL1 to value 0"]
impl crate::Resettable for USB0_ALT_TXCSRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
