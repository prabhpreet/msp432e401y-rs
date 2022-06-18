#[doc = "Register `RXCSRH2` reader"]
pub struct R(crate::R<RXCSRH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCSRH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCSRH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCSRH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRH2` writer"]
pub struct W(crate::W<RXCSRH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCSRH2_SPEC>;
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
impl From<crate::W<RXCSRH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCSRH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRH2_INCOMPRX` reader - Incomplete RX Transmission Status"]
pub type USB_RXCSRH2_INCOMPRX_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_INCOMPRX` writer - Incomplete RX Transmission Status"]
pub type USB_RXCSRH2_INCOMPRX_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 0>;
#[doc = "Field `USB_RXCSRH2_DT` reader - Data Toggle"]
pub type USB_RXCSRH2_DT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_DT` writer - Data Toggle"]
pub type USB_RXCSRH2_DT_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 1>;
#[doc = "Field `USB_RXCSRH2_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH2_DTWE_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH2_DTWE_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRH2_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH2_DMAMOD_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH2_DMAMOD_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRH2_PIDERR` reader - PID Error"]
pub type USB_RXCSRH2_PIDERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_PIDERR` writer - PID Error"]
pub type USB_RXCSRH2_PIDERR_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRH2_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH2_DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH2_DMAEN_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 5>;
#[doc = "Field `USB_RXCSRH2_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH2_AUTORQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH2_AUTORQ_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 6>;
#[doc = "Field `USB_RXCSRH2_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH2_AUTOCL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH2_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH2_AUTOCL_W<'a> = crate::BitWriter<'a, u8, RXCSRH2_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh2_incomprx(&self) -> USB_RXCSRH2_INCOMPRX_R {
        USB_RXCSRH2_INCOMPRX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dt(&self) -> USB_RXCSRH2_DT_R {
        USB_RXCSRH2_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dtwe(&self) -> USB_RXCSRH2_DTWE_R {
        USB_RXCSRH2_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dmamod(&self) -> USB_RXCSRH2_DMAMOD_R {
        USB_RXCSRH2_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh2_piderr(&self) -> USB_RXCSRH2_PIDERR_R {
        USB_RXCSRH2_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dmaen(&self) -> USB_RXCSRH2_DMAEN_R {
        USB_RXCSRH2_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh2_autorq(&self) -> USB_RXCSRH2_AUTORQ_R {
        USB_RXCSRH2_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh2_autocl(&self) -> USB_RXCSRH2_AUTOCL_R {
        USB_RXCSRH2_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh2_incomprx(&mut self) -> USB_RXCSRH2_INCOMPRX_W {
        USB_RXCSRH2_INCOMPRX_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dt(&mut self) -> USB_RXCSRH2_DT_W {
        USB_RXCSRH2_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dtwe(&mut self) -> USB_RXCSRH2_DTWE_W {
        USB_RXCSRH2_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dmamod(&mut self) -> USB_RXCSRH2_DMAMOD_W {
        USB_RXCSRH2_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh2_piderr(&mut self) -> USB_RXCSRH2_PIDERR_W {
        USB_RXCSRH2_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh2_dmaen(&mut self) -> USB_RXCSRH2_DMAEN_W {
        USB_RXCSRH2_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh2_autorq(&mut self) -> USB_RXCSRH2_AUTORQ_W {
        USB_RXCSRH2_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh2_autocl(&mut self) -> USB_RXCSRH2_AUTOCL_W {
        USB_RXCSRH2_AUTOCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh2](index.html) module"]
pub struct RXCSRH2_SPEC;
impl crate::RegisterSpec for RXCSRH2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxcsrh2::R](R) reader structure"]
impl crate::Readable for RXCSRH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcsrh2::W](W) writer structure"]
impl crate::Writable for RXCSRH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRH2 to value 0"]
impl crate::Resettable for RXCSRH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
