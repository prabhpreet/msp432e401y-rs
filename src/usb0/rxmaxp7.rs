#[doc = "Register `RXMAXP7` reader"]
pub struct R(crate::R<RXMAXP7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMAXP7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMAXP7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMAXP7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMAXP7` writer"]
pub struct W(crate::W<RXMAXP7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMAXP7_SPEC>;
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
impl From<crate::W<RXMAXP7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMAXP7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXMAXP7_MAXLOAD` reader - Maximum Payload"]
pub type USB_RXMAXP7_MAXLOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RXMAXP7_MAXLOAD` writer - Maximum Payload"]
pub type USB_RXMAXP7_MAXLOAD_W<'a> = crate::FieldWriter<'a, u16, RXMAXP7_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_rxmaxp7_maxload(&self) -> USB_RXMAXP7_MAXLOAD_R {
        USB_RXMAXP7_MAXLOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload"]
    #[inline(always)]
    pub fn usb_rxmaxp7_maxload(&mut self) -> USB_RXMAXP7_MAXLOAD_W {
        USB_RXMAXP7_MAXLOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Maximum Receive Data Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp7](index.html) module"]
pub struct RXMAXP7_SPEC;
impl crate::RegisterSpec for RXMAXP7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rxmaxp7::R](R) reader structure"]
impl crate::Readable for RXMAXP7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmaxp7::W](W) writer structure"]
impl crate::Writable for RXMAXP7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXMAXP7 to value 0"]
impl crate::Resettable for RXMAXP7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
