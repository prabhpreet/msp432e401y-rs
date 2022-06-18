#[doc = "Register `EPIDX` reader"]
pub struct R(crate::R<EPIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPIDX` writer"]
pub struct W(crate::W<EPIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPIDX_SPEC>;
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
impl From<crate::W<EPIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EPIDX_EPIDX` reader - Endpoint Index"]
pub type USB_EPIDX_EPIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_EPIDX_EPIDX` writer - Endpoint Index"]
pub type USB_EPIDX_EPIDX_W<'a> = crate::FieldWriter<'a, u8, EPIDX_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Index"]
    #[inline(always)]
    pub fn usb_epidx_epidx(&self) -> USB_EPIDX_EPIDX_R {
        USB_EPIDX_EPIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Index"]
    #[inline(always)]
    pub fn usb_epidx_epidx(&mut self) -> USB_EPIDX_EPIDX_W {
        USB_EPIDX_EPIDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epidx](index.html) module"]
pub struct EPIDX_SPEC;
impl crate::RegisterSpec for EPIDX_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epidx::R](R) reader structure"]
impl crate::Readable for EPIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epidx::W](W) writer structure"]
impl crate::Writable for EPIDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPIDX to value 0"]
impl crate::Resettable for EPIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
