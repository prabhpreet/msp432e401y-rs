#[doc = "Register `FIFO1` reader"]
pub struct R(crate::R<FIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO1` writer"]
pub struct W(crate::W<FIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO1_SPEC>;
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
impl From<crate::W<FIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_FIFO1_EPDATA` reader - Endpoint Data"]
pub type USB_FIFO1_EPDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_FIFO1_EPDATA` writer - Endpoint Data"]
pub type USB_FIFO1_EPDATA_W<'a> = crate::FieldWriter<'a, u32, FIFO1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn usb_fifo1_epdata(&self) -> USB_FIFO1_EPDATA_R {
        USB_FIFO1_EPDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Endpoint Data"]
    #[inline(always)]
    pub fn usb_fifo1_epdata(&mut self) -> USB_FIFO1_EPDATA_W {
        USB_FIFO1_EPDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB FIFO Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo1](index.html) module"]
pub struct FIFO1_SPEC;
impl crate::RegisterSpec for FIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo1::R](R) reader structure"]
impl crate::Readable for FIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo1::W](W) writer structure"]
impl crate::Writable for FIFO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for FIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
