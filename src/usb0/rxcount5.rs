#[doc = "Register `RXCOUNT5` reader"]
pub struct R(crate::R<RXCOUNT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCOUNT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCOUNT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCOUNT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCOUNT5` writer"]
pub struct W(crate::W<RXCOUNT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCOUNT5_SPEC>;
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
impl From<crate::W<RXCOUNT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCOUNT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCOUNT5_COUNT` reader - Receive Packet Count"]
pub type USB_RXCOUNT5_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RXCOUNT5_COUNT` writer - Receive Packet Count"]
pub type USB_RXCOUNT5_COUNT_W<'a> = crate::FieldWriter<'a, u16, RXCOUNT5_SPEC, u16, u16, 13, 0>;
impl R {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount5_count(&self) -> USB_RXCOUNT5_COUNT_R {
        USB_RXCOUNT5_COUNT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount5_count(&mut self) -> USB_RXCOUNT5_COUNT_W {
        USB_RXCOUNT5_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Byte Count Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount5](index.html) module"]
pub struct RXCOUNT5_SPEC;
impl crate::RegisterSpec for RXCOUNT5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxcount5::R](R) reader structure"]
impl crate::Readable for RXCOUNT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcount5::W](W) writer structure"]
impl crate::Writable for RXCOUNT5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCOUNT5 to value 0"]
impl crate::Resettable for RXCOUNT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
