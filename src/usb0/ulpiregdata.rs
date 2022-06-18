#[doc = "Register `ULPIREGDATA` reader"]
pub struct R(crate::R<ULPIREGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPIREGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPIREGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPIREGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULPIREGDATA` writer"]
pub struct W(crate::W<ULPIREGDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULPIREGDATA_SPEC>;
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
impl From<crate::W<ULPIREGDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULPIREGDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ULPIREGDATA_REGDATA` reader - Register Data"]
pub type USB_ULPIREGDATA_REGDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_ULPIREGDATA_REGDATA` writer - Register Data"]
pub type USB_ULPIREGDATA_REGDATA_W<'a> = crate::FieldWriter<'a, u8, ULPIREGDATA_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Register Data"]
    #[inline(always)]
    pub fn usb_ulpiregdata_regdata(&self) -> USB_ULPIREGDATA_REGDATA_R {
        USB_ULPIREGDATA_REGDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Data"]
    #[inline(always)]
    pub fn usb_ulpiregdata_regdata(&mut self) -> USB_ULPIREGDATA_REGDATA_W {
        USB_ULPIREGDATA_REGDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB ULPI Register Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregdata](index.html) module"]
pub struct ULPIREGDATA_SPEC;
impl crate::RegisterSpec for ULPIREGDATA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ulpiregdata::R](R) reader structure"]
impl crate::Readable for ULPIREGDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulpiregdata::W](W) writer structure"]
impl crate::Writable for ULPIREGDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULPIREGDATA to value 0"]
impl crate::Resettable for ULPIREGDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
