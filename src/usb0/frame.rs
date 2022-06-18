#[doc = "Register `FRAME` reader"]
pub struct R(crate::R<FRAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAME` writer"]
pub struct W(crate::W<FRAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_SPEC>;
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
impl From<crate::W<FRAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_FRAME` reader - Frame Number"]
pub type USB_FRAME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_FRAME` writer - Frame Number"]
pub type USB_FRAME_W<'a> = crate::FieldWriter<'a, u16, FRAME_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    pub fn usb_frame(&self) -> USB_FRAME_R {
        USB_FRAME_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Number"]
    #[inline(always)]
    pub fn usb_frame(&mut self) -> USB_FRAME_W {
        USB_FRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Frame Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame](index.html) module"]
pub struct FRAME_SPEC;
impl crate::RegisterSpec for FRAME_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [frame::R](R) reader structure"]
impl crate::Readable for FRAME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frame::W](W) writer structure"]
impl crate::Writable for FRAME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FRAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
