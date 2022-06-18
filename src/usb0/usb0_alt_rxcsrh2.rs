#[doc = "Register `RXCSRH2` reader"]
pub struct R(crate::R<USB0_ALT_RXCSRH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_ALT_RXCSRH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_ALT_RXCSRH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_ALT_RXCSRH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRH2` writer"]
pub struct W(crate::W<USB0_ALT_RXCSRH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_RXCSRH2_SPEC>;
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
impl From<crate::W<USB0_ALT_RXCSRH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_RXCSRH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRH2_DISNYET` reader - Disable NYET"]
pub type USB_RXCSRH2_DISNYET_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_DISNYET` writer - Disable NYET"]
pub type USB_RXCSRH2_DISNYET_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_RXCSRH2_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRH2_ISO` reader - Isochronous Transfers"]
pub type USB_RXCSRH2_ISO_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_ISO` writer - Isochronous Transfers"]
pub type USB_RXCSRH2_ISO_W<'a> = crate::BitWriter<'a, u8, USB0_ALT_RXCSRH2_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh2_disnyet(&self) -> USB_RXCSRH2_DISNYET_R {
        USB_RXCSRH2_DISNYET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh2_iso(&self) -> USB_RXCSRH2_ISO_R {
        USB_RXCSRH2_ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Disable NYET"]
    #[inline(always)]
    pub fn usb_rxcsrh2_disnyet(&mut self) -> USB_RXCSRH2_DISNYET_W {
        USB_RXCSRH2_DISNYET_W::new(self)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_rxcsrh2_iso(&mut self) -> USB_RXCSRH2_ISO_W {
        USB_RXCSRH2_ISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_rxcsrh2](index.html) module"]
pub struct USB0_ALT_RXCSRH2_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXCSRH2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb0_alt_rxcsrh2::R](R) reader structure"]
impl crate::Readable for USB0_ALT_RXCSRH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_rxcsrh2::W](W) writer structure"]
impl crate::Writable for USB0_ALT_RXCSRH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRH2 to value 0"]
impl crate::Resettable for USB0_ALT_RXCSRH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
