#[doc = "Register `RQPKTCOUNT4` reader"]
pub struct R(crate::R<RQPKTCOUNT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RQPKTCOUNT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RQPKTCOUNT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RQPKTCOUNT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RQPKTCOUNT4` writer"]
pub struct W(crate::W<RQPKTCOUNT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQPKTCOUNT4_SPEC>;
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
impl From<crate::W<RQPKTCOUNT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQPKTCOUNT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RQPKTCOUNT4_COUNT` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT4_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RQPKTCOUNT4_COUNT` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT4_COUNT_W<'a> =
    crate::FieldWriter<'a, u16, RQPKTCOUNT4_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount4_count(&self) -> USB_RQPKTCOUNT4_COUNT_R {
        USB_RQPKTCOUNT4_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount4_count(&mut self) -> USB_RQPKTCOUNT4_COUNT_W {
        USB_RQPKTCOUNT4_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount4](index.html) module"]
pub struct RQPKTCOUNT4_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rqpktcount4::R](R) reader structure"]
impl crate::Readable for RQPKTCOUNT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rqpktcount4::W](W) writer structure"]
impl crate::Writable for RQPKTCOUNT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RQPKTCOUNT4 to value 0"]
impl crate::Resettable for RQPKTCOUNT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
