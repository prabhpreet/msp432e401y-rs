#[doc = "Register `ULPIREGADDR` reader"]
pub struct R(crate::R<ULPIREGADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPIREGADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPIREGADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPIREGADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULPIREGADDR` writer"]
pub struct W(crate::W<ULPIREGADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULPIREGADDR_SPEC>;
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
impl From<crate::W<ULPIREGADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULPIREGADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ULPIREGADDR_ADDR` reader - Register Address"]
pub type USB_ULPIREGADDR_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_ULPIREGADDR_ADDR` writer - Register Address"]
pub type USB_ULPIREGADDR_ADDR_W<'a> = crate::FieldWriter<'a, u8, ULPIREGADDR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Register Address"]
    #[inline(always)]
    pub fn usb_ulpiregaddr_addr(&self) -> USB_ULPIREGADDR_ADDR_R {
        USB_ULPIREGADDR_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Address"]
    #[inline(always)]
    pub fn usb_ulpiregaddr_addr(&mut self) -> USB_ULPIREGADDR_ADDR_W {
        USB_ULPIREGADDR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB ULPI Register Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregaddr](index.html) module"]
pub struct ULPIREGADDR_SPEC;
impl crate::RegisterSpec for ULPIREGADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ulpiregaddr::R](R) reader structure"]
impl crate::Readable for ULPIREGADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulpiregaddr::W](W) writer structure"]
impl crate::Writable for ULPIREGADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULPIREGADDR to value 0"]
impl crate::Resettable for ULPIREGADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
