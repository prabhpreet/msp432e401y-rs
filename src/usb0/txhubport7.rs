#[doc = "Register `TXHUBPORT7` reader"]
pub struct R(crate::R<TXHUBPORT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXHUBPORT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXHUBPORT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXHUBPORT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXHUBPORT7` writer"]
pub struct W(crate::W<TXHUBPORT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXHUBPORT7_SPEC>;
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
impl From<crate::W<TXHUBPORT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXHUBPORT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXHUBPORT7_PORT` reader - Hub Port"]
pub type USB_TXHUBPORT7_PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_TXHUBPORT7_PORT` writer - Hub Port"]
pub type USB_TXHUBPORT7_PORT_W<'a> = crate::FieldWriter<'a, u8, TXHUBPORT7_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport7_port(&self) -> USB_TXHUBPORT7_PORT_R {
        USB_TXHUBPORT7_PORT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport7_port(&mut self) -> USB_TXHUBPORT7_PORT_W {
        USB_TXHUBPORT7_PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Hub Port Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport7](index.html) module"]
pub struct TXHUBPORT7_SPEC;
impl crate::RegisterSpec for TXHUBPORT7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txhubport7::R](R) reader structure"]
impl crate::Readable for TXHUBPORT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txhubport7::W](W) writer structure"]
impl crate::Writable for TXHUBPORT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXHUBPORT7 to value 0"]
impl crate::Resettable for TXHUBPORT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
