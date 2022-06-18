#[doc = "Register `TXHUBPORT4` reader"]
pub struct R(crate::R<TXHUBPORT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXHUBPORT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXHUBPORT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXHUBPORT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXHUBPORT4` writer"]
pub struct W(crate::W<TXHUBPORT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXHUBPORT4_SPEC>;
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
impl From<crate::W<TXHUBPORT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXHUBPORT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXHUBPORT4_PORT` reader - Hub Port"]
pub type USB_TXHUBPORT4_PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_TXHUBPORT4_PORT` writer - Hub Port"]
pub type USB_TXHUBPORT4_PORT_W<'a> = crate::FieldWriter<'a, u8, TXHUBPORT4_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport4_port(&self) -> USB_TXHUBPORT4_PORT_R {
        USB_TXHUBPORT4_PORT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport4_port(&mut self) -> USB_TXHUBPORT4_PORT_W {
        USB_TXHUBPORT4_PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Hub Port Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport4](index.html) module"]
pub struct TXHUBPORT4_SPEC;
impl crate::RegisterSpec for TXHUBPORT4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txhubport4::R](R) reader structure"]
impl crate::Readable for TXHUBPORT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txhubport4::W](W) writer structure"]
impl crate::Writable for TXHUBPORT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXHUBPORT4 to value 0"]
impl crate::Resettable for TXHUBPORT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
