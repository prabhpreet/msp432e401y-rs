#[doc = "Register `VPLEN` reader"]
pub struct R(crate::R<VPLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VPLEN` writer"]
pub struct W(crate::W<VPLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VPLEN_SPEC>;
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
impl From<crate::W<VPLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VPLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VPLEN_VPLEN` reader - VBUS Pulse Length"]
pub type USB_VPLEN_VPLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_VPLEN_VPLEN` writer - VBUS Pulse Length"]
pub type USB_VPLEN_VPLEN_W<'a> = crate::FieldWriter<'a, u8, VPLEN_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    pub fn usb_vplen_vplen(&self) -> USB_VPLEN_VPLEN_R {
        USB_VPLEN_VPLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - VBUS Pulse Length"]
    #[inline(always)]
    pub fn usb_vplen_vplen(&mut self) -> USB_VPLEN_VPLEN_W {
        USB_VPLEN_VPLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OTG VBUS Pulse Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vplen](index.html) module"]
pub struct VPLEN_SPEC;
impl crate::RegisterSpec for VPLEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vplen::R](R) reader structure"]
impl crate::Readable for VPLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vplen::W](W) writer structure"]
impl crate::Writable for VPLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VPLEN to value 0"]
impl crate::Resettable for VPLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
