#[doc = "Register `RXCSRH3` reader"]
pub struct R(crate::R<RXCSRH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCSRH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCSRH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCSRH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRH3` writer"]
pub struct W(crate::W<RXCSRH3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCSRH3_SPEC>;
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
impl From<crate::W<RXCSRH3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCSRH3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRH3_INCOMPRX` reader - Incomplete RX Transmission Status"]
pub type USB_RXCSRH3_INCOMPRX_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_INCOMPRX` writer - Incomplete RX Transmission Status"]
pub type USB_RXCSRH3_INCOMPRX_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 0>;
#[doc = "Field `USB_RXCSRH3_DT` reader - Data Toggle"]
pub type USB_RXCSRH3_DT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_DT` writer - Data Toggle"]
pub type USB_RXCSRH3_DT_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 1>;
#[doc = "Field `USB_RXCSRH3_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH3_DTWE_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH3_DTWE_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRH3_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH3_DMAMOD_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH3_DMAMOD_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRH3_PIDERR` reader - PID Error"]
pub type USB_RXCSRH3_PIDERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_PIDERR` writer - PID Error"]
pub type USB_RXCSRH3_PIDERR_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRH3_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH3_DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH3_DMAEN_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 5>;
#[doc = "Field `USB_RXCSRH3_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH3_AUTORQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH3_AUTORQ_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 6>;
#[doc = "Field `USB_RXCSRH3_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH3_AUTOCL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH3_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH3_AUTOCL_W<'a> = crate::BitWriter<'a, u8, RXCSRH3_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh3_incomprx(&self) -> USB_RXCSRH3_INCOMPRX_R {
        USB_RXCSRH3_INCOMPRX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dt(&self) -> USB_RXCSRH3_DT_R {
        USB_RXCSRH3_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dtwe(&self) -> USB_RXCSRH3_DTWE_R {
        USB_RXCSRH3_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dmamod(&self) -> USB_RXCSRH3_DMAMOD_R {
        USB_RXCSRH3_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh3_piderr(&self) -> USB_RXCSRH3_PIDERR_R {
        USB_RXCSRH3_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dmaen(&self) -> USB_RXCSRH3_DMAEN_R {
        USB_RXCSRH3_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh3_autorq(&self) -> USB_RXCSRH3_AUTORQ_R {
        USB_RXCSRH3_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh3_autocl(&self) -> USB_RXCSRH3_AUTOCL_R {
        USB_RXCSRH3_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh3_incomprx(&mut self) -> USB_RXCSRH3_INCOMPRX_W {
        USB_RXCSRH3_INCOMPRX_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dt(&mut self) -> USB_RXCSRH3_DT_W {
        USB_RXCSRH3_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dtwe(&mut self) -> USB_RXCSRH3_DTWE_W {
        USB_RXCSRH3_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dmamod(&mut self) -> USB_RXCSRH3_DMAMOD_W {
        USB_RXCSRH3_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh3_piderr(&mut self) -> USB_RXCSRH3_PIDERR_W {
        USB_RXCSRH3_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh3_dmaen(&mut self) -> USB_RXCSRH3_DMAEN_W {
        USB_RXCSRH3_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh3_autorq(&mut self) -> USB_RXCSRH3_AUTORQ_W {
        USB_RXCSRH3_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh3_autocl(&mut self) -> USB_RXCSRH3_AUTOCL_W {
        USB_RXCSRH3_AUTOCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh3](index.html) module"]
pub struct RXCSRH3_SPEC;
impl crate::RegisterSpec for RXCSRH3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxcsrh3::R](R) reader structure"]
impl crate::Readable for RXCSRH3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcsrh3::W](W) writer structure"]
impl crate::Writable for RXCSRH3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRH3 to value 0"]
impl crate::Resettable for RXCSRH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
