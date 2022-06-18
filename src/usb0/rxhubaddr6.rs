#[doc = "Register `RXHUBADDR6` reader"]
pub struct R(crate::R<RXHUBADDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXHUBADDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXHUBADDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXHUBADDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXHUBADDR6` writer"]
pub struct W(crate::W<RXHUBADDR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXHUBADDR6_SPEC>;
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
impl From<crate::W<RXHUBADDR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXHUBADDR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXHUBADDR6_ADDR` reader - Hub Address"]
pub type USB_RXHUBADDR6_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RXHUBADDR6_ADDR` writer - Hub Address"]
pub type USB_RXHUBADDR6_ADDR_W<'a> = crate::FieldWriter<'a, u8, RXHUBADDR6_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr6_addr(&self) -> USB_RXHUBADDR6_ADDR_R {
        USB_RXHUBADDR6_ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr6_addr(&mut self) -> USB_RXHUBADDR6_ADDR_W {
        USB_RXHUBADDR6_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Hub Address Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr6](index.html) module"]
pub struct RXHUBADDR6_SPEC;
impl crate::RegisterSpec for RXHUBADDR6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxhubaddr6::R](R) reader structure"]
impl crate::Readable for RXHUBADDR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxhubaddr6::W](W) writer structure"]
impl crate::Writable for RXHUBADDR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXHUBADDR6 to value 0"]
impl crate::Resettable for RXHUBADDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
