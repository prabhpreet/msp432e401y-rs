#[doc = "Register `TXMAXP7` reader"]
pub struct R(crate::R<TXMAXP7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMAXP7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMAXP7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMAXP7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXMAXP7` writer"]
pub struct W(crate::W<TXMAXP7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMAXP7_SPEC>;
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
impl From<crate::W<TXMAXP7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMAXP7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_TXMAXP7_MAXLOAD` reader - Maximum Payload"]
pub type USB_TXMAXP7_MAXLOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_TXMAXP7_MAXLOAD` writer - Maximum Payload"]
pub type USB_TXMAXP7_MAXLOAD_W<'a> = crate::FieldWriter<'a, u16, TXMAXP7_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_txmaxp7_maxload(&self) -> USB_TXMAXP7_MAXLOAD_R {
        USB_TXMAXP7_MAXLOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_txmaxp7_maxload(&mut self) -> USB_TXMAXP7_MAXLOAD_W {
        USB_TXMAXP7_MAXLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Maximum Transmit Data Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp7](index.html) module"]
pub struct TXMAXP7_SPEC;
impl crate::RegisterSpec for TXMAXP7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [txmaxp7::R](R) reader structure"]
impl crate::Readable for TXMAXP7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmaxp7::W](W) writer structure"]
impl crate::Writable for TXMAXP7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXMAXP7 to value 0"]
impl crate::Resettable for TXMAXP7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
