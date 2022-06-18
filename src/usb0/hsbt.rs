#[doc = "Register `HSBT` reader"]
pub struct R(crate::R<HSBT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSBT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSBT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSBT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSBT` writer"]
pub struct W(crate::W<HSBT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSBT_SPEC>;
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
impl From<crate::W<HSBT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSBT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_HSBT_HSBT` reader - High Speed Timeout Adder"]
pub type USB_HSBT_HSBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_HSBT_HSBT` writer - High Speed Timeout Adder"]
pub type USB_HSBT_HSBT_W<'a> = crate::FieldWriter<'a, u16, HSBT_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - High Speed Timeout Adder"]
    #[inline(always)]
    pub fn usb_hsbt_hsbt(&self) -> USB_HSBT_HSBT_R {
        USB_HSBT_HSBT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - High Speed Timeout Adder"]
    #[inline(always)]
    pub fn usb_hsbt_hsbt(&mut self) -> USB_HSBT_HSBT_W {
        USB_HSBT_HSBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB High Speed Time-out Adder\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbt](index.html) module"]
pub struct HSBT_SPEC;
impl crate::RegisterSpec for HSBT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hsbt::R](R) reader structure"]
impl crate::Readable for HSBT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsbt::W](W) writer structure"]
impl crate::Writable for HSBT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSBT to value 0"]
impl crate::Resettable for HSBT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
