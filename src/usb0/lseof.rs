#[doc = "Register `LSEOF` reader"]
pub struct R(crate::R<LSEOF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSEOF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSEOF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSEOF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSEOF` writer"]
pub struct W(crate::W<LSEOF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSEOF_SPEC>;
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
impl From<crate::W<LSEOF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSEOF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_LSEOF_LSEOFG` reader - Low-Speed End-of-Frame Gap"]
pub type USB_LSEOF_LSEOFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_LSEOF_LSEOFG` writer - Low-Speed End-of-Frame Gap"]
pub type USB_LSEOF_LSEOFG_W<'a> = crate::FieldWriter<'a, u8, LSEOF_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_lseof_lseofg(&self) -> USB_LSEOF_LSEOFG_R {
        USB_LSEOF_LSEOFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Speed End-of-Frame Gap"]
    #[inline(always)]
    pub fn usb_lseof_lseofg(&mut self) -> USB_LSEOF_LSEOFG_W {
        USB_LSEOF_LSEOFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lseof](index.html) module"]
pub struct LSEOF_SPEC;
impl crate::RegisterSpec for LSEOF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lseof::R](R) reader structure"]
impl crate::Readable for LSEOF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lseof::W](W) writer structure"]
impl crate::Writable for LSEOF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSEOF to value 0"]
impl crate::Resettable for LSEOF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
