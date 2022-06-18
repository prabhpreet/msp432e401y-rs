#[doc = "Register `RXIS` reader"]
pub struct R(crate::R<RXIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIS` writer"]
pub struct W(crate::W<RXIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIS_SPEC>;
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
impl From<crate::W<RXIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXIS_EP1` reader - RX Endpoint 1 Interrupt"]
pub type USB_RXIS_EP1_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP1` writer - RX Endpoint 1 Interrupt"]
pub type USB_RXIS_EP1_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 1>;
#[doc = "Field `USB_RXIS_EP2` reader - RX Endpoint 2 Interrupt"]
pub type USB_RXIS_EP2_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP2` writer - RX Endpoint 2 Interrupt"]
pub type USB_RXIS_EP2_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 2>;
#[doc = "Field `USB_RXIS_EP3` reader - RX Endpoint 3 Interrupt"]
pub type USB_RXIS_EP3_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP3` writer - RX Endpoint 3 Interrupt"]
pub type USB_RXIS_EP3_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 3>;
#[doc = "Field `USB_RXIS_EP4` reader - RX Endpoint 4 Interrupt"]
pub type USB_RXIS_EP4_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP4` writer - RX Endpoint 4 Interrupt"]
pub type USB_RXIS_EP4_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 4>;
#[doc = "Field `USB_RXIS_EP5` reader - RX Endpoint 5 Interrupt"]
pub type USB_RXIS_EP5_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP5` writer - RX Endpoint 5 Interrupt"]
pub type USB_RXIS_EP5_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 5>;
#[doc = "Field `USB_RXIS_EP6` reader - RX Endpoint 6 Interrupt"]
pub type USB_RXIS_EP6_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP6` writer - RX Endpoint 6 Interrupt"]
pub type USB_RXIS_EP6_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 6>;
#[doc = "Field `USB_RXIS_EP7` reader - RX Endpoint 7 Interrupt"]
pub type USB_RXIS_EP7_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXIS_EP7` writer - RX Endpoint 7 Interrupt"]
pub type USB_RXIS_EP7_W<'a> = crate::BitWriter<'a, u16, RXIS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep1(&self) -> USB_RXIS_EP1_R {
        USB_RXIS_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep2(&self) -> USB_RXIS_EP2_R {
        USB_RXIS_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep3(&self) -> USB_RXIS_EP3_R {
        USB_RXIS_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep4(&self) -> USB_RXIS_EP4_R {
        USB_RXIS_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep5(&self) -> USB_RXIS_EP5_R {
        USB_RXIS_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep6(&self) -> USB_RXIS_EP6_R {
        USB_RXIS_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep7(&self) -> USB_RXIS_EP7_R {
        USB_RXIS_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RX Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep1(&mut self) -> USB_RXIS_EP1_W {
        USB_RXIS_EP1_W::new(self)
    }
    #[doc = "Bit 2 - RX Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep2(&mut self) -> USB_RXIS_EP2_W {
        USB_RXIS_EP2_W::new(self)
    }
    #[doc = "Bit 3 - RX Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep3(&mut self) -> USB_RXIS_EP3_W {
        USB_RXIS_EP3_W::new(self)
    }
    #[doc = "Bit 4 - RX Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep4(&mut self) -> USB_RXIS_EP4_W {
        USB_RXIS_EP4_W::new(self)
    }
    #[doc = "Bit 5 - RX Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep5(&mut self) -> USB_RXIS_EP5_W {
        USB_RXIS_EP5_W::new(self)
    }
    #[doc = "Bit 6 - RX Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep6(&mut self) -> USB_RXIS_EP6_W {
        USB_RXIS_EP6_W::new(self)
    }
    #[doc = "Bit 7 - RX Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn usb_rxis_ep7(&mut self) -> USB_RXIS_EP7_W {
        USB_RXIS_EP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxis](index.html) module"]
pub struct RXIS_SPEC;
impl crate::RegisterSpec for RXIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxis::R](R) reader structure"]
impl crate::Readable for RXIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxis::W](W) writer structure"]
impl crate::Writable for RXIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXIS to value 0"]
impl crate::Resettable for RXIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
