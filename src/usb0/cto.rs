#[doc = "Register `CTO` reader"]
pub struct R(crate::R<CTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTO` writer"]
pub struct W(crate::W<CTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTO_SPEC>;
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
impl From<crate::W<CTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CTO_CCTV` reader - Configurable Chirp Timeout Value"]
pub type USB_CTO_CCTV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_CTO_CCTV` writer - Configurable Chirp Timeout Value"]
pub type USB_CTO_CCTV_W<'a> = crate::FieldWriter<'a, u16, CTO_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout Value"]
    #[inline(always)]
    pub fn usb_cto_cctv(&self) -> USB_CTO_CCTV_R {
        USB_CTO_CCTV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configurable Chirp Timeout Value"]
    #[inline(always)]
    pub fn usb_cto_cctv(&mut self) -> USB_CTO_CCTV_W {
        USB_CTO_CCTV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Chirp Timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cto](index.html) module"]
pub struct CTO_SPEC;
impl crate::RegisterSpec for CTO_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cto::R](R) reader structure"]
impl crate::Readable for CTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cto::W](W) writer structure"]
impl crate::Writable for CTO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTO to value 0"]
impl crate::Resettable for CTO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
