#[doc = "Register `TXHUBPORT5` reader"]
pub struct R(crate::R<TXHUBPORT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXHUBPORT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXHUBPORT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXHUBPORT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXHUBPORT5` writer"]
pub struct W(crate::W<TXHUBPORT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXHUBPORT5_SPEC>;
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
impl From<crate::W<TXHUBPORT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXHUBPORT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXHUBPORT5_PORT` reader - Hub Port"]
pub type USB_TXHUBPORT5_PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_TXHUBPORT5_PORT` writer - Hub Port"]
pub type USB_TXHUBPORT5_PORT_W<'a> = crate::FieldWriter<'a, u8, TXHUBPORT5_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport5_port(&self) -> USB_TXHUBPORT5_PORT_R {
        USB_TXHUBPORT5_PORT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hub Port"]
    #[inline(always)]
    pub fn usb_txhubport5_port(&mut self) -> USB_TXHUBPORT5_PORT_W {
        USB_TXHUBPORT5_PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Transmit Hub Port Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport5](index.html) module"]
pub struct TXHUBPORT5_SPEC;
impl crate::RegisterSpec for TXHUBPORT5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txhubport5::R](R) reader structure"]
impl crate::Readable for TXHUBPORT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txhubport5::W](W) writer structure"]
impl crate::Writable for TXHUBPORT5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXHUBPORT5 to value 0"]
impl crate::Resettable for TXHUBPORT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
