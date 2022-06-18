#[doc = "Register `RXCOUNT6` reader"]
pub struct R(crate::R<RXCOUNT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCOUNT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCOUNT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCOUNT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCOUNT6` writer"]
pub struct W(crate::W<RXCOUNT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCOUNT6_SPEC>;
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
impl From<crate::W<RXCOUNT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCOUNT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXCOUNT6_COUNT` reader - Receive Packet Count"]
pub type USB_RXCOUNT6_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RXCOUNT6_COUNT` writer - Receive Packet Count"]
pub type USB_RXCOUNT6_COUNT_W<'a> = crate::FieldWriter<'a, u16, RXCOUNT6_SPEC, u16, u16, 13, 0>;
impl R {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount6_count(&self) -> USB_RXCOUNT6_COUNT_R {
        USB_RXCOUNT6_COUNT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Receive Packet Count"]
    #[inline(always)]
    pub fn usb_rxcount6_count(&mut self) -> USB_RXCOUNT6_COUNT_W {
        USB_RXCOUNT6_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Byte Count Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount6](index.html) module"]
pub struct RXCOUNT6_SPEC;
impl crate::RegisterSpec for RXCOUNT6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxcount6::R](R) reader structure"]
impl crate::Readable for RXCOUNT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcount6::W](W) writer structure"]
impl crate::Writable for RXCOUNT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCOUNT6 to value 0"]
impl crate::Resettable for RXCOUNT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
