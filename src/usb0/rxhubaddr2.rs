#[doc = "Register `RXHUBADDR2` reader"]
pub struct R(crate::R<RXHUBADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXHUBADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXHUBADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXHUBADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXHUBADDR2` writer"]
pub struct W(crate::W<RXHUBADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXHUBADDR2_SPEC>;
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
impl From<crate::W<RXHUBADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXHUBADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXHUBADDR2_ADDR` reader - Hub Address"]
pub type USB_RXHUBADDR2_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RXHUBADDR2_ADDR` writer - Hub Address"]
pub type USB_RXHUBADDR2_ADDR_W<'a> = crate::FieldWriter<'a, u8, RXHUBADDR2_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr2_addr(&self) -> USB_RXHUBADDR2_ADDR_R {
        USB_RXHUBADDR2_ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_rxhubaddr2_addr(&mut self) -> USB_RXHUBADDR2_ADDR_W {
        USB_RXHUBADDR2_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Hub Address Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr2](index.html) module"]
pub struct RXHUBADDR2_SPEC;
impl crate::RegisterSpec for RXHUBADDR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxhubaddr2::R](R) reader structure"]
impl crate::Readable for RXHUBADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxhubaddr2::W](W) writer structure"]
impl crate::Writable for RXHUBADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXHUBADDR2 to value 0"]
impl crate::Resettable for RXHUBADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
