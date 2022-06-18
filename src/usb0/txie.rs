#[doc = "Register `TXIE` reader"]
pub struct R(crate::R<TXIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXIE` writer"]
pub struct W(crate::W<TXIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXIE_SPEC>;
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
impl From<crate::W<TXIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXIE_EP0` reader - TX and RX Endpoint 0 Interrupt Enable"]
pub type USB_TXIE_EP0_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP0` writer - TX and RX Endpoint 0 Interrupt Enable"]
pub type USB_TXIE_EP0_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 0>;
#[doc = "Field `USB_TXIE_EP1` reader - TX Endpoint 1 Interrupt Enable"]
pub type USB_TXIE_EP1_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP1` writer - TX Endpoint 1 Interrupt Enable"]
pub type USB_TXIE_EP1_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 1>;
#[doc = "Field `USB_TXIE_EP2` reader - TX Endpoint 2 Interrupt Enable"]
pub type USB_TXIE_EP2_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP2` writer - TX Endpoint 2 Interrupt Enable"]
pub type USB_TXIE_EP2_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 2>;
#[doc = "Field `USB_TXIE_EP3` reader - TX Endpoint 3 Interrupt Enable"]
pub type USB_TXIE_EP3_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP3` writer - TX Endpoint 3 Interrupt Enable"]
pub type USB_TXIE_EP3_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 3>;
#[doc = "Field `USB_TXIE_EP4` reader - TX Endpoint 4 Interrupt Enable"]
pub type USB_TXIE_EP4_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP4` writer - TX Endpoint 4 Interrupt Enable"]
pub type USB_TXIE_EP4_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 4>;
#[doc = "Field `USB_TXIE_EP5` reader - TX Endpoint 5 Interrupt Enable"]
pub type USB_TXIE_EP5_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP5` writer - TX Endpoint 5 Interrupt Enable"]
pub type USB_TXIE_EP5_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 5>;
#[doc = "Field `USB_TXIE_EP6` reader - TX Endpoint 6 Interrupt Enable"]
pub type USB_TXIE_EP6_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP6` writer - TX Endpoint 6 Interrupt Enable"]
pub type USB_TXIE_EP6_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 6>;
#[doc = "Field `USB_TXIE_EP7` reader - TX Endpoint 7 Interrupt Enable"]
pub type USB_TXIE_EP7_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXIE_EP7` writer - TX Endpoint 7 Interrupt Enable"]
pub type USB_TXIE_EP7_W<'a> = crate::BitWriter<'a, u16, TXIE_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - TX and RX Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep0(&self) -> USB_TXIE_EP0_R {
        USB_TXIE_EP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep1(&self) -> USB_TXIE_EP1_R {
        USB_TXIE_EP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep2(&self) -> USB_TXIE_EP2_R {
        USB_TXIE_EP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep3(&self) -> USB_TXIE_EP3_R {
        USB_TXIE_EP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep4(&self) -> USB_TXIE_EP4_R {
        USB_TXIE_EP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep5(&self) -> USB_TXIE_EP5_R {
        USB_TXIE_EP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep6(&self) -> USB_TXIE_EP6_R {
        USB_TXIE_EP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep7(&self) -> USB_TXIE_EP7_R {
        USB_TXIE_EP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX and RX Endpoint 0 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep0(&mut self) -> USB_TXIE_EP0_W {
        USB_TXIE_EP0_W::new(self)
    }
    #[doc = "Bit 1 - TX Endpoint 1 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep1(&mut self) -> USB_TXIE_EP1_W {
        USB_TXIE_EP1_W::new(self)
    }
    #[doc = "Bit 2 - TX Endpoint 2 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep2(&mut self) -> USB_TXIE_EP2_W {
        USB_TXIE_EP2_W::new(self)
    }
    #[doc = "Bit 3 - TX Endpoint 3 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep3(&mut self) -> USB_TXIE_EP3_W {
        USB_TXIE_EP3_W::new(self)
    }
    #[doc = "Bit 4 - TX Endpoint 4 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep4(&mut self) -> USB_TXIE_EP4_W {
        USB_TXIE_EP4_W::new(self)
    }
    #[doc = "Bit 5 - TX Endpoint 5 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep5(&mut self) -> USB_TXIE_EP5_W {
        USB_TXIE_EP5_W::new(self)
    }
    #[doc = "Bit 6 - TX Endpoint 6 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep6(&mut self) -> USB_TXIE_EP6_W {
        USB_TXIE_EP6_W::new(self)
    }
    #[doc = "Bit 7 - TX Endpoint 7 Interrupt Enable"]
    #[inline(always)]
    pub fn usb_txie_ep7(&mut self) -> USB_TXIE_EP7_W {
        USB_TXIE_EP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txie](index.html) module"]
pub struct TXIE_SPEC;
impl crate::RegisterSpec for TXIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [txie::R](R) reader structure"]
impl crate::Readable for TXIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txie::W](W) writer structure"]
impl crate::Writable for TXIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXIE to value 0"]
impl crate::Resettable for TXIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
