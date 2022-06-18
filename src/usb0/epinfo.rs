#[doc = "Register `EPINFO` reader"]
pub struct R(crate::R<EPINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINFO` writer"]
pub struct W(crate::W<EPINFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINFO_SPEC>;
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
impl From<crate::W<EPINFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EPINFO_TXEP` reader - TX Endpoints"]
pub type USB_EPINFO_TXEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_EPINFO_TXEP` writer - TX Endpoints"]
pub type USB_EPINFO_TXEP_W<'a> = crate::FieldWriter<'a, u8, EPINFO_SPEC, u8, u8, 4, 0>;
#[doc = "Field `USB_EPINFO_RXEP` reader - RX Endpoints"]
pub type USB_EPINFO_RXEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_EPINFO_RXEP` writer - RX Endpoints"]
pub type USB_EPINFO_RXEP_W<'a> = crate::FieldWriter<'a, u8, EPINFO_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_txep(&self) -> USB_EPINFO_TXEP_R {
        USB_EPINFO_TXEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_rxep(&self) -> USB_EPINFO_RXEP_R {
        USB_EPINFO_RXEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_txep(&mut self) -> USB_EPINFO_TXEP_W {
        USB_EPINFO_TXEP_W::new(self)
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_rxep(&mut self) -> USB_EPINFO_RXEP_W {
        USB_EPINFO_RXEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinfo](index.html) module"]
pub struct EPINFO_SPEC;
impl crate::RegisterSpec for EPINFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epinfo::R](R) reader structure"]
impl crate::Readable for EPINFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epinfo::W](W) writer structure"]
impl crate::Writable for EPINFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINFO to value 0"]
impl crate::Resettable for EPINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
