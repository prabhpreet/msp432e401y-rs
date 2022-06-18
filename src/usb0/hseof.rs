#[doc = "Register `HSEOF` reader"]
pub struct R(crate::R<HSEOF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEOF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEOF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEOF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEOF` writer"]
pub struct W(crate::W<HSEOF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEOF_SPEC>;
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
impl From<crate::W<HSEOF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEOF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_HSEOF_HSEOFG` reader - HIgh-Speed End-of-Frame Gap"]
pub type USB_HSEOF_HSEOFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_HSEOF_HSEOFG` writer - HIgh-Speed End-of-Frame Gap"]
pub type USB_HSEOF_HSEOFG_W<'a> = crate::FieldWriter<'a, u8, HSEOF_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - HIgh-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_hseof_hseofg(&self) -> USB_HSEOF_HSEOFG_R {
        USB_HSEOF_HSEOFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - HIgh-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_hseof_hseofg(&mut self) -> USB_HSEOF_HSEOFG_W {
        USB_HSEOF_HSEOFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB High-Speed Last Transaction to End of Frame Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hseof](index.html) module"]
pub struct HSEOF_SPEC;
impl crate::RegisterSpec for HSEOF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hseof::R](R) reader structure"]
impl crate::Readable for HSEOF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hseof::W](W) writer structure"]
impl crate::Writable for HSEOF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSEOF to value 0"]
impl crate::Resettable for HSEOF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
