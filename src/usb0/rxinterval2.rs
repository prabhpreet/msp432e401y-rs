#[doc = "Register `RXINTERVAL2` reader"]
pub struct R(crate::R<RXINTERVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXINTERVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXINTERVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXINTERVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXINTERVAL2` writer"]
pub struct W(crate::W<RXINTERVAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXINTERVAL2_SPEC>;
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
impl From<crate::W<RXINTERVAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXINTERVAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXINTERVAL2_TXPOLL` reader - RX Polling"]
pub type USB_RXINTERVAL2_TXPOLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RXINTERVAL2_TXPOLL` writer - RX Polling"]
pub type USB_RXINTERVAL2_TXPOLL_W<'a> = crate::FieldWriter<'a, u8, RXINTERVAL2_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    pub fn usb_rxinterval2_txpoll(&self) -> USB_RXINTERVAL2_TXPOLL_R {
        USB_RXINTERVAL2_TXPOLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX Polling"]
    #[inline(always)]
    pub fn usb_rxinterval2_txpoll(&mut self) -> USB_RXINTERVAL2_TXPOLL_W {
        USB_RXINTERVAL2_TXPOLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Host Receive Polling Interval Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval2](index.html) module"]
pub struct RXINTERVAL2_SPEC;
impl crate::RegisterSpec for RXINTERVAL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxinterval2::R](R) reader structure"]
impl crate::Readable for RXINTERVAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxinterval2::W](W) writer structure"]
impl crate::Writable for RXINTERVAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXINTERVAL2 to value 0"]
impl crate::Resettable for RXINTERVAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
