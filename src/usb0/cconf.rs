#[doc = "Register `CCONF` reader"]
pub struct R(crate::R<CCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCONF` writer"]
pub struct W(crate::W<CCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCONF_SPEC>;
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
impl From<crate::W<CCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CCONF_RXEDMA` reader - TX Early DMA Enable"]
pub type USB_CCONF_RXEDMA_R = crate::BitReader<bool>;
#[doc = "Field `USB_CCONF_RXEDMA` writer - TX Early DMA Enable"]
pub type USB_CCONF_RXEDMA_W<'a> = crate::BitWriter<'a, u8, CCONF_SPEC, bool, 0>;
#[doc = "Field `USB_CCONF_TXEDMA` reader - TX Early DMA Enable"]
pub type USB_CCONF_TXEDMA_R = crate::BitReader<bool>;
#[doc = "Field `USB_CCONF_TXEDMA` writer - TX Early DMA Enable"]
pub type USB_CCONF_TXEDMA_W<'a> = crate::BitWriter<'a, u8, CCONF_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn usb_cconf_rxedma(&self) -> USB_CCONF_RXEDMA_R {
        USB_CCONF_RXEDMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn usb_cconf_txedma(&self) -> USB_CCONF_TXEDMA_R {
        USB_CCONF_TXEDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn usb_cconf_rxedma(&mut self) -> USB_CCONF_RXEDMA_W {
        USB_CCONF_RXEDMA_W::new(self)
    }
    #[doc = "Bit 1 - TX Early DMA Enable"]
    #[inline(always)]
    pub fn usb_cconf_txedma(&mut self) -> USB_CCONF_TXEDMA_W {
        USB_CCONF_TXEDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Common Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cconf](index.html) module"]
pub struct CCONF_SPEC;
impl crate::RegisterSpec for CCONF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cconf::R](R) reader structure"]
impl crate::Readable for CCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cconf::W](W) writer structure"]
impl crate::Writable for CCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCONF to value 0"]
impl crate::Resettable for CCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
