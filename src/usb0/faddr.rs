#[doc = "Register `FADDR` reader"]
pub struct R(crate::R<FADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FADDR` writer"]
pub struct W(crate::W<FADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FADDR_SPEC>;
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
impl From<crate::W<FADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_FADDR` reader - Function Address"]
pub type USB_FADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_FADDR` writer - Function Address"]
pub type USB_FADDR_W<'a> = crate::FieldWriter<'a, u8, FADDR_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Function Address"]
    #[inline(always)]
    pub fn usb_faddr(&self) -> USB_FADDR_R {
        USB_FADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function Address"]
    #[inline(always)]
    pub fn usb_faddr(&mut self) -> USB_FADDR_W {
        USB_FADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Functional Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](index.html) module"]
pub struct FADDR_SPEC;
impl crate::RegisterSpec for FADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [faddr::R](R) reader structure"]
impl crate::Readable for FADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faddr::W](W) writer structure"]
impl crate::Writable for FADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
