#[doc = "Register `TXINTERVAL7` reader"]
pub struct R(crate::R<TXINTERVAL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXINTERVAL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXINTERVAL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXINTERVAL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXINTERVAL7` writer"]
pub struct W(crate::W<TXINTERVAL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXINTERVAL7_SPEC>;
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
impl From<crate::W<TXINTERVAL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXINTERVAL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXINTERVAL7_TXPOLL` reader - TX Polling"]
pub type USB_TXINTERVAL7_TXPOLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_TXINTERVAL7_TXPOLL` writer - TX Polling"]
pub type USB_TXINTERVAL7_TXPOLL_W<'a> = crate::FieldWriter<'a, u8, TXINTERVAL7_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    pub fn usb_txinterval7_txpoll(&self) -> USB_TXINTERVAL7_TXPOLL_R {
        USB_TXINTERVAL7_TXPOLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    pub fn usb_txinterval7_txpoll(&mut self) -> USB_TXINTERVAL7_TXPOLL_W {
        USB_TXINTERVAL7_TXPOLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Host Transmit Interval Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval7](index.html) module"]
pub struct TXINTERVAL7_SPEC;
impl crate::RegisterSpec for TXINTERVAL7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txinterval7::R](R) reader structure"]
impl crate::Readable for TXINTERVAL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txinterval7::W](W) writer structure"]
impl crate::Writable for TXINTERVAL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXINTERVAL7 to value 0"]
impl crate::Resettable for TXINTERVAL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
