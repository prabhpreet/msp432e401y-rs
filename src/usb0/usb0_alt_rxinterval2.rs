#[doc = "Register `RXINTERVAL2` reader"]
pub struct R(crate::R<USB0_ALT_RXINTERVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0_ALT_RXINTERVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0_ALT_RXINTERVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0_ALT_RXINTERVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXINTERVAL2` writer"]
pub struct W(crate::W<USB0_ALT_RXINTERVAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0_ALT_RXINTERVAL2_SPEC>;
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
impl From<crate::W<USB0_ALT_RXINTERVAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0_ALT_RXINTERVAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXINTERVAL2_NAKLMT` reader - NAK Limit"]
pub type USB_RXINTERVAL2_NAKLMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RXINTERVAL2_NAKLMT` writer - NAK Limit"]
pub type USB_RXINTERVAL2_NAKLMT_W<'a> =
    crate::FieldWriter<'a, u8, USB0_ALT_RXINTERVAL2_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    pub fn usb_rxinterval2_naklmt(&self) -> USB_RXINTERVAL2_NAKLMT_R {
        USB_RXINTERVAL2_NAKLMT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    pub fn usb_rxinterval2_naklmt(&mut self) -> USB_RXINTERVAL2_NAKLMT_W {
        USB_RXINTERVAL2_NAKLMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Host Receive Polling Interval Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0_alt_rxinterval2](index.html) module"]
pub struct USB0_ALT_RXINTERVAL2_SPEC;
impl crate::RegisterSpec for USB0_ALT_RXINTERVAL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb0_alt_rxinterval2::R](R) reader structure"]
impl crate::Readable for USB0_ALT_RXINTERVAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0_alt_rxinterval2::W](W) writer structure"]
impl crate::Writable for USB0_ALT_RXINTERVAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXINTERVAL2 to value 0"]
impl crate::Resettable for USB0_ALT_RXINTERVAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
