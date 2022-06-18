#[doc = "Register `RQPKTCOUNT2` reader"]
pub struct R(crate::R<RQPKTCOUNT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RQPKTCOUNT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RQPKTCOUNT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RQPKTCOUNT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RQPKTCOUNT2` writer"]
pub struct W(crate::W<RQPKTCOUNT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQPKTCOUNT2_SPEC>;
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
impl From<crate::W<RQPKTCOUNT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQPKTCOUNT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RQPKTCOUNT2` reader - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_RQPKTCOUNT2` writer - Block Transfer Packet Count"]
pub type USB_RQPKTCOUNT2_W<'a> = crate::FieldWriter<'a, u16, RQPKTCOUNT2_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount2(&self) -> USB_RQPKTCOUNT2_R {
        USB_RQPKTCOUNT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block Transfer Packet Count"]
    #[inline(always)]
    pub fn usb_rqpktcount2(&mut self) -> USB_RQPKTCOUNT2_W {
        USB_RQPKTCOUNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount2](index.html) module"]
pub struct RQPKTCOUNT2_SPEC;
impl crate::RegisterSpec for RQPKTCOUNT2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rqpktcount2::R](R) reader structure"]
impl crate::Readable for RQPKTCOUNT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rqpktcount2::W](W) writer structure"]
impl crate::Writable for RQPKTCOUNT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RQPKTCOUNT2 to value 0"]
impl crate::Resettable for RQPKTCOUNT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
