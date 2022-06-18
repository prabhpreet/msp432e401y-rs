#[doc = "Register `LPMFADDR` reader"]
pub struct R(crate::R<LPMFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMFADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMFADDR` writer"]
pub struct W(crate::W<LPMFADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMFADDR_SPEC>;
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
impl From<crate::W<LPMFADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMFADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_LPMFADDR_ADDR` reader - LPM Function Address"]
pub type USB_LPMFADDR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_LPMFADDR_ADDR` writer - LPM Function Address"]
pub type USB_LPMFADDR_ADDR_W<'a> = crate::FieldWriter<'a, u8, LPMFADDR_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - LPM Function Address"]
    #[inline(always)]
    pub fn usb_lpmfaddr_addr(&self) -> USB_LPMFADDR_ADDR_R {
        USB_LPMFADDR_ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LPM Function Address"]
    #[inline(always)]
    pub fn usb_lpmfaddr_addr(&mut self) -> USB_LPMFADDR_ADDR_W {
        USB_LPMFADDR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM Function Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmfaddr](index.html) module"]
pub struct LPMFADDR_SPEC;
impl crate::RegisterSpec for LPMFADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpmfaddr::R](R) reader structure"]
impl crate::Readable for LPMFADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmfaddr::W](W) writer structure"]
impl crate::Writable for LPMFADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMFADDR to value 0"]
impl crate::Resettable for LPMFADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
