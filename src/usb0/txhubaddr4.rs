#[doc = "Register `TXHUBADDR4` reader"]
pub struct R(crate::R<TXHUBADDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXHUBADDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXHUBADDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXHUBADDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXHUBADDR4` writer"]
pub struct W(crate::W<TXHUBADDR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXHUBADDR4_SPEC>;
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
impl From<crate::W<TXHUBADDR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXHUBADDR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXHUBADDR4_ADDR` reader - Hub Address"]
pub type USB_TXHUBADDR4_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_TXHUBADDR4_ADDR` writer - Hub Address"]
pub type USB_TXHUBADDR4_ADDR_W<'a> = crate::FieldWriter<'a, u8, TXHUBADDR4_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_txhubaddr4_addr(&self) -> USB_TXHUBADDR4_ADDR_R {
        USB_TXHUBADDR4_ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Address"]
    #[inline(always)]
    pub fn usb_txhubaddr4_addr(&mut self) -> USB_TXHUBADDR4_ADDR_W {
        USB_TXHUBADDR4_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Hub Address Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr4](index.html) module"]
pub struct TXHUBADDR4_SPEC;
impl crate::RegisterSpec for TXHUBADDR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txhubaddr4::R](R) reader structure"]
impl crate::Readable for TXHUBADDR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txhubaddr4::W](W) writer structure"]
impl crate::Writable for TXHUBADDR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXHUBADDR4 to value 0"]
impl crate::Resettable for TXHUBADDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
