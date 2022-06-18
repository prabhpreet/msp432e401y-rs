#[doc = "Register `HHSRTN` reader"]
pub struct R(crate::R<HHSRTN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HHSRTN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HHSRTN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HHSRTN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HHSRTN` writer"]
pub struct W(crate::W<HHSRTN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HHSRTN_SPEC>;
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
impl From<crate::W<HHSRTN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HHSRTN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_HHSRTN_HHSRTN` reader - HIgh Speed to UTM Operating Delay"]
pub type USB_HHSRTN_HHSRTN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_HHSRTN_HHSRTN` writer - HIgh Speed to UTM Operating Delay"]
pub type USB_HHSRTN_HHSRTN_W<'a> = crate::FieldWriter<'a, u16, HHSRTN_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - HIgh Speed to UTM Operating Delay"]
    #[inline(always)]
    pub fn usb_hhsrtn_hhsrtn(&self) -> USB_HHSRTN_HHSRTN_R {
        USB_HHSRTN_HHSRTN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - HIgh Speed to UTM Operating Delay"]
    #[inline(always)]
    pub fn usb_hhsrtn_hhsrtn(&mut self) -> USB_HHSRTN_HHSRTN_W {
        USB_HHSRTN_HHSRTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB High Speed to UTM Operating Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hhsrtn](index.html) module"]
pub struct HHSRTN_SPEC;
impl crate::RegisterSpec for HHSRTN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hhsrtn::R](R) reader structure"]
impl crate::Readable for HHSRTN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hhsrtn::W](W) writer structure"]
impl crate::Writable for HHSRTN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HHSRTN to value 0"]
impl crate::Resettable for HHSRTN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
