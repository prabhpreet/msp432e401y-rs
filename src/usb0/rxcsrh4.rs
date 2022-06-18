#[doc = "Register `RXCSRH4` reader"]
pub struct R(crate::R<RXCSRH4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCSRH4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCSRH4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCSRH4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCSRH4` writer"]
pub struct W(crate::W<RXCSRH4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCSRH4_SPEC>;
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
impl From<crate::W<RXCSRH4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCSRH4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCSRH4_INCOMPRX` reader - Incomplete RX Transmission Status"]
pub type USB_RXCSRH4_INCOMPRX_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_INCOMPRX` writer - Incomplete RX Transmission Status"]
pub type USB_RXCSRH4_INCOMPRX_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 0>;
#[doc = "Field `USB_RXCSRH4_DT` reader - Data Toggle"]
pub type USB_RXCSRH4_DT_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_DT` writer - Data Toggle"]
pub type USB_RXCSRH4_DT_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 1>;
#[doc = "Field `USB_RXCSRH4_DTWE` reader - Data Toggle Write Enable"]
pub type USB_RXCSRH4_DTWE_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_DTWE` writer - Data Toggle Write Enable"]
pub type USB_RXCSRH4_DTWE_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 2>;
#[doc = "Field `USB_RXCSRH4_DMAMOD` reader - DMA Request Mode"]
pub type USB_RXCSRH4_DMAMOD_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_DMAMOD` writer - DMA Request Mode"]
pub type USB_RXCSRH4_DMAMOD_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 3>;
#[doc = "Field `USB_RXCSRH4_PIDERR` reader - PID Error"]
pub type USB_RXCSRH4_PIDERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_PIDERR` writer - PID Error"]
pub type USB_RXCSRH4_PIDERR_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 4>;
#[doc = "Field `USB_RXCSRH4_DMAEN` reader - DMA Request Enable"]
pub type USB_RXCSRH4_DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_DMAEN` writer - DMA Request Enable"]
pub type USB_RXCSRH4_DMAEN_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 5>;
#[doc = "Field `USB_RXCSRH4_AUTORQ` reader - Auto Request"]
pub type USB_RXCSRH4_AUTORQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_AUTORQ` writer - Auto Request"]
pub type USB_RXCSRH4_AUTORQ_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 6>;
#[doc = "Field `USB_RXCSRH4_AUTOCL` reader - Auto Clear"]
pub type USB_RXCSRH4_AUTOCL_R = crate::BitReader<bool>;
#[doc = "Field `USB_RXCSRH4_AUTOCL` writer - Auto Clear"]
pub type USB_RXCSRH4_AUTOCL_W<'a> = crate::BitWriter<'a, u8, RXCSRH4_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh4_incomprx(&self) -> USB_RXCSRH4_INCOMPRX_R {
        USB_RXCSRH4_INCOMPRX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dt(&self) -> USB_RXCSRH4_DT_R {
        USB_RXCSRH4_DT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dtwe(&self) -> USB_RXCSRH4_DTWE_R {
        USB_RXCSRH4_DTWE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dmamod(&self) -> USB_RXCSRH4_DMAMOD_R {
        USB_RXCSRH4_DMAMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh4_piderr(&self) -> USB_RXCSRH4_PIDERR_R {
        USB_RXCSRH4_PIDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dmaen(&self) -> USB_RXCSRH4_DMAEN_R {
        USB_RXCSRH4_DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh4_autorq(&self) -> USB_RXCSRH4_AUTORQ_R {
        USB_RXCSRH4_AUTORQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh4_autocl(&self) -> USB_RXCSRH4_AUTOCL_R {
        USB_RXCSRH4_AUTOCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Incomplete RX Transmission Status"]
    #[inline(always)]
    pub fn usb_rxcsrh4_incomprx(&mut self) -> USB_RXCSRH4_INCOMPRX_W {
        USB_RXCSRH4_INCOMPRX_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dt(&mut self) -> USB_RXCSRH4_DT_W {
        USB_RXCSRH4_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dtwe(&mut self) -> USB_RXCSRH4_DTWE_W {
        USB_RXCSRH4_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dmamod(&mut self) -> USB_RXCSRH4_DMAMOD_W {
        USB_RXCSRH4_DMAMOD_W::new(self)
    }
    #[doc = "Bit 4 - PID Error"]
    #[inline(always)]
    pub fn usb_rxcsrh4_piderr(&mut self) -> USB_RXCSRH4_PIDERR_W {
        USB_RXCSRH4_PIDERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_rxcsrh4_dmaen(&mut self) -> USB_RXCSRH4_DMAEN_W {
        USB_RXCSRH4_DMAEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto Request"]
    #[inline(always)]
    pub fn usb_rxcsrh4_autorq(&mut self) -> USB_RXCSRH4_AUTORQ_W {
        USB_RXCSRH4_AUTORQ_W::new(self)
    }
    #[doc = "Bit 7 - Auto Clear"]
    #[inline(always)]
    pub fn usb_rxcsrh4_autocl(&mut self) -> USB_RXCSRH4_AUTOCL_W {
        USB_RXCSRH4_AUTOCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Control and Status Endpoint 4 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh4](index.html) module"]
pub struct RXCSRH4_SPEC;
impl crate::RegisterSpec for RXCSRH4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxcsrh4::R](R) reader structure"]
impl crate::Readable for RXCSRH4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcsrh4::W](W) writer structure"]
impl crate::Writable for RXCSRH4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCSRH4 to value 0"]
impl crate::Resettable for RXCSRH4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
