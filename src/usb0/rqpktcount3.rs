#[doc = "Register `RQPKTCOUNT3` reader"]
pub struct R(crate::R<RQPKTCOUNT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RQPKTCOUNT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RQPKTCOUNT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RQPKTCOUNT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RQPKTCOUNT3` writer"]
pub struct W(crate::W<RQPKTCOUNT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQPKTCOUNT3_SPEC>;
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
impl From<crate::W<RQPKTCOUNT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQPKTCOUNT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RQPKTCOUNT3` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RQPKTCOUNT3` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT3_W<'a> = crate::FieldWriter<'a, u16, RQPKTCOUNT3_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount3(&self) -> USB_RQPKTCOUNT3_R {
        USB_RQPKTCOUNT3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount3(&mut self) -> USB_RQPKTCOUNT3_W {
        USB_RQPKTCOUNT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount3](index.html) module"]
pub struct RQPKTCOUNT3_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rqpktcount3::R](R) reader structure"]
impl crate::Readable for RQPKTCOUNT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rqpktcount3::W](W) writer structure"]
impl crate::Writable for RQPKTCOUNT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RQPKTCOUNT3 to value 0"]
impl crate::Resettable for RQPKTCOUNT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
