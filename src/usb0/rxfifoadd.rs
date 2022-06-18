#[doc = "Register `RXFIFOADD` reader"]
pub struct R(crate::R<RXFIFOADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFOADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFOADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFOADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFIFOADD` writer"]
pub struct W(crate::W<RXFIFOADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFIFOADD_SPEC>;
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
impl From<crate::W<RXFIFOADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFIFOADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXFIFOADD_ADDR` reader - Transmit/Receive Start Address"]
pub type USB_RXFIFOADD_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RXFIFOADD_ADDR` writer - Transmit/Receive Start Address"]
pub type USB_RXFIFOADD_ADDR_W<'a> = crate::FieldWriter<'a, u16, RXFIFOADD_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_rxfifoadd_addr(&self) -> USB_RXFIFOADD_ADDR_R {
        USB_RXFIFOADD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_rxfifoadd_addr(&mut self) -> USB_RXFIFOADD_ADDR_W {
        USB_RXFIFOADD_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive FIFO Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifoadd](index.html) module"]
pub struct RXFIFOADD_SPEC;
impl crate::RegisterSpec for RXFIFOADD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxfifoadd::R](R) reader structure"]
impl crate::Readable for RXFIFOADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfifoadd::W](W) writer structure"]
impl crate::Writable for RXFIFOADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXFIFOADD to value 0"]
impl crate::Resettable for RXFIFOADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
