#[doc = "Register `TXCSRH1` reader"]
pub struct R(crate::R<TXCSRH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCSRH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCSRH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCSRH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCSRH1` writer"]
pub struct W(crate::W<TXCSRH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCSRH1_SPEC>;
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
impl From<crate::W<TXCSRH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCSRH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXCSRH1_DT` reader - Data Toggle"]
pub type USB_TXCSRH1_DT_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_DT` writer - Data Toggle"]
pub type USB_TXCSRH1_DT_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 0>;
#[doc = "Field `USB_TXCSRH1_DTWE` reader - Data Toggle Write Enable"]
pub type USB_TXCSRH1_DTWE_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_DTWE` writer - Data Toggle Write Enable"]
pub type USB_TXCSRH1_DTWE_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 1>;
#[doc = "Field `USB_TXCSRH1_DMAMOD` reader - DMA Request Mode"]
pub type USB_TXCSRH1_DMAMOD_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_DMAMOD` writer - DMA Request Mode"]
pub type USB_TXCSRH1_DMAMOD_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 2>;
#[doc = "Field `USB_TXCSRH1_FDT` reader - Force Data Toggle"]
pub type USB_TXCSRH1_FDT_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_FDT` writer - Force Data Toggle"]
pub type USB_TXCSRH1_FDT_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 3>;
#[doc = "Field `USB_TXCSRH1_DMAEN` reader - DMA Request Enable"]
pub type USB_TXCSRH1_DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_DMAEN` writer - DMA Request Enable"]
pub type USB_TXCSRH1_DMAEN_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 4>;
#[doc = "Field `USB_TXCSRH1_MODE` reader - Mode"]
pub type USB_TXCSRH1_MODE_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_MODE` writer - Mode"]
pub type USB_TXCSRH1_MODE_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 5>;
#[doc = "Field `USB_TXCSRH1_ISO` reader - Isochronous Transfers"]
pub type USB_TXCSRH1_ISO_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_ISO` writer - Isochronous Transfers"]
pub type USB_TXCSRH1_ISO_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 6>;
#[doc = "Field `USB_TXCSRH1_AUTOSET` reader - Auto Set"]
pub type USB_TXCSRH1_AUTOSET_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXCSRH1_AUTOSET` writer - Auto Set"]
pub type USB_TXCSRH1_AUTOSET_W<'a> = crate::BitWriter<'a, u8, TXCSRH1_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh1_dt(&self) -> USB_TXCSRH1_DT_R {
        USB_TXCSRH1_DT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_txcsrh1_dtwe(&self) -> USB_TXCSRH1_DTWE_R {
        USB_TXCSRH1_DTWE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_txcsrh1_dmamod(&self) -> USB_TXCSRH1_DMAMOD_R {
        USB_TXCSRH1_DMAMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh1_fdt(&self) -> USB_TXCSRH1_FDT_R {
        USB_TXCSRH1_FDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_txcsrh1_dmaen(&self) -> USB_TXCSRH1_DMAEN_R {
        USB_TXCSRH1_DMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode"]
    #[inline(always)]
    pub fn usb_txcsrh1_mode(&self) -> USB_TXCSRH1_MODE_R {
        USB_TXCSRH1_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_txcsrh1_iso(&self) -> USB_TXCSRH1_ISO_R {
        USB_TXCSRH1_ISO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Set"]
    #[inline(always)]
    pub fn usb_txcsrh1_autoset(&self) -> USB_TXCSRH1_AUTOSET_R {
        USB_TXCSRH1_AUTOSET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh1_dt(&mut self) -> USB_TXCSRH1_DT_W {
        USB_TXCSRH1_DT_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_txcsrh1_dtwe(&mut self) -> USB_TXCSRH1_DTWE_W {
        USB_TXCSRH1_DTWE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Request Mode"]
    #[inline(always)]
    pub fn usb_txcsrh1_dmamod(&mut self) -> USB_TXCSRH1_DMAMOD_W {
        USB_TXCSRH1_DMAMOD_W::new(self)
    }
    #[doc = "Bit 3 - Force Data Toggle"]
    #[inline(always)]
    pub fn usb_txcsrh1_fdt(&mut self) -> USB_TXCSRH1_FDT_W {
        USB_TXCSRH1_FDT_W::new(self)
    }
    #[doc = "Bit 4 - DMA Request Enable"]
    #[inline(always)]
    pub fn usb_txcsrh1_dmaen(&mut self) -> USB_TXCSRH1_DMAEN_W {
        USB_TXCSRH1_DMAEN_W::new(self)
    }
    #[doc = "Bit 5 - Mode"]
    #[inline(always)]
    pub fn usb_txcsrh1_mode(&mut self) -> USB_TXCSRH1_MODE_W {
        USB_TXCSRH1_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Isochronous Transfers"]
    #[inline(always)]
    pub fn usb_txcsrh1_iso(&mut self) -> USB_TXCSRH1_ISO_W {
        USB_TXCSRH1_ISO_W::new(self)
    }
    #[doc = "Bit 7 - Auto Set"]
    #[inline(always)]
    pub fn usb_txcsrh1_autoset(&mut self) -> USB_TXCSRH1_AUTOSET_W {
        USB_TXCSRH1_AUTOSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Control and Status Endpoint 1 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh1](index.html) module"]
pub struct TXCSRH1_SPEC;
impl crate::RegisterSpec for TXCSRH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txcsrh1::R](R) reader structure"]
impl crate::Readable for TXCSRH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcsrh1::W](W) writer structure"]
impl crate::Writable for TXCSRH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCSRH1 to value 0"]
impl crate::Resettable for TXCSRH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
