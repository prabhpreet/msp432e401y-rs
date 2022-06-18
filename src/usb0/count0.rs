#[doc = "Register `COUNT0` reader"]
pub struct R(crate::R<COUNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT0` writer"]
pub struct W(crate::W<COUNT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT0_SPEC>;
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
impl From<crate::W<COUNT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_COUNT0_COUNT` reader - FIFO Count"]
pub type USB_COUNT0_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_COUNT0_COUNT` writer - FIFO Count"]
pub type USB_COUNT0_COUNT_W<'a> = crate::FieldWriter<'a, u8, COUNT0_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - FIFO Count"]
    #[inline(always)]
    pub fn usb_count0_count(&self) -> USB_COUNT0_COUNT_R {
        USB_COUNT0_COUNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - FIFO Count"]
    #[inline(always)]
    pub fn usb_count0_count(&mut self) -> USB_COUNT0_COUNT_W {
        USB_COUNT0_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Receive Byte Count Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0](index.html) module"]
pub struct COUNT0_SPEC;
impl crate::RegisterSpec for COUNT0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [count0::R](R) reader structure"]
impl crate::Readable for COUNT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count0::W](W) writer structure"]
impl crate::Writable for COUNT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT0 to value 0"]
impl crate::Resettable for COUNT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
