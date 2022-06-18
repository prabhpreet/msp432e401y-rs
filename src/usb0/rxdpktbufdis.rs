#[doc = "Register `RXDPKTBUFDIS` reader"]
pub struct R(crate::R<RXDPKTBUFDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDPKTBUFDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDPKTBUFDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDPKTBUFDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDPKTBUFDIS` writer"]
pub struct W(crate::W<RXDPKTBUFDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDPKTBUFDIS_SPEC>;
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
impl From<crate::W<RXDPKTBUFDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDPKTBUFDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXDPKTBUFDIS_EP1` reader - EP1 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP1_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP1` writer - EP1 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP1_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 1>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP2` reader - EP2 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP2_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP2` writer - EP2 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP2_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 2>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP3` reader - EP3 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP3_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP3` writer - EP3 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP3_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 3>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP4` reader - EP4 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP4_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP4` writer - EP4 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP4_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 4>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP5` reader - EP5 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP5_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP5` writer - EP5 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP5_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 5>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP6` reader - EP6 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP6_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP6` writer - EP6 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP6_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 6>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP7` reader - EP7 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP7_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXDPKTBUFDIS_EP7` writer - EP7 RX Double-Packet Buffer Disable"]
pub type USB_RXDPKTBUFDIS_EP7_W<'a> = crate::BitWriter<'a, u16, RXDPKTBUFDIS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 1 - EP1 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep1(&self) -> USB_RXDPKTBUFDIS_EP1_R {
        USB_RXDPKTBUFDIS_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP2 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep2(&self) -> USB_RXDPKTBUFDIS_EP2_R {
        USB_RXDPKTBUFDIS_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP3 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep3(&self) -> USB_RXDPKTBUFDIS_EP3_R {
        USB_RXDPKTBUFDIS_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP4 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep4(&self) -> USB_RXDPKTBUFDIS_EP4_R {
        USB_RXDPKTBUFDIS_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP5 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep5(&self) -> USB_RXDPKTBUFDIS_EP5_R {
        USB_RXDPKTBUFDIS_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EP6 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep6(&self) -> USB_RXDPKTBUFDIS_EP6_R {
        USB_RXDPKTBUFDIS_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EP7 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep7(&self) -> USB_RXDPKTBUFDIS_EP7_R {
        USB_RXDPKTBUFDIS_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EP1 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep1(&mut self) -> USB_RXDPKTBUFDIS_EP1_W {
        USB_RXDPKTBUFDIS_EP1_W::new(self)
    }
    #[doc = "Bit 2 - EP2 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep2(&mut self) -> USB_RXDPKTBUFDIS_EP2_W {
        USB_RXDPKTBUFDIS_EP2_W::new(self)
    }
    #[doc = "Bit 3 - EP3 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep3(&mut self) -> USB_RXDPKTBUFDIS_EP3_W {
        USB_RXDPKTBUFDIS_EP3_W::new(self)
    }
    #[doc = "Bit 4 - EP4 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep4(&mut self) -> USB_RXDPKTBUFDIS_EP4_W {
        USB_RXDPKTBUFDIS_EP4_W::new(self)
    }
    #[doc = "Bit 5 - EP5 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep5(&mut self) -> USB_RXDPKTBUFDIS_EP5_W {
        USB_RXDPKTBUFDIS_EP5_W::new(self)
    }
    #[doc = "Bit 6 - EP6 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep6(&mut self) -> USB_RXDPKTBUFDIS_EP6_W {
        USB_RXDPKTBUFDIS_EP6_W::new(self)
    }
    #[doc = "Bit 7 - EP7 RX Double-Packet Buffer Disable"]
    #[inline(always)]
    pub fn usb_rxdpktbufdis_ep7(&mut self) -> USB_RXDPKTBUFDIS_EP7_W {
        USB_RXDPKTBUFDIS_EP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Double Packet Buffer Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdpktbufdis](index.html) module"]
pub struct RXDPKTBUFDIS_SPEC;
impl crate::RegisterSpec for RXDPKTBUFDIS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxdpktbufdis::R](R) reader structure"]
impl crate::Readable for RXDPKTBUFDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdpktbufdis::W](W) writer structure"]
impl crate::Writable for RXDPKTBUFDIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDPKTBUFDIS to value 0"]
impl crate::Resettable for RXDPKTBUFDIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
