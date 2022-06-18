#[doc = "Register `NAKLMT` reader"]
pub struct R(crate::R<NAKLMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NAKLMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NAKLMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NAKLMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NAKLMT` writer"]
pub struct W(crate::W<NAKLMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NAKLMT_SPEC>;
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
impl From<crate::W<NAKLMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NAKLMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_NAKLMT_NAKLMT` reader - EP0 NAK Limit"]
pub type USB_NAKLMT_NAKLMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_NAKLMT_NAKLMT` writer - EP0 NAK Limit"]
pub type USB_NAKLMT_NAKLMT_W<'a> = crate::FieldWriter<'a, u8, NAKLMT_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    pub fn usb_naklmt_naklmt(&self) -> USB_NAKLMT_NAKLMT_R {
        USB_NAKLMT_NAKLMT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EP0 NAK Limit"]
    #[inline(always)]
    pub fn usb_naklmt_naklmt(&mut self) -> USB_NAKLMT_NAKLMT_W {
        USB_NAKLMT_NAKLMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB NAK Limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [naklmt](index.html) module"]
pub struct NAKLMT_SPEC;
impl crate::RegisterSpec for NAKLMT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [naklmt::R](R) reader structure"]
impl crate::Readable for NAKLMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [naklmt::W](W) writer structure"]
impl crate::Writable for NAKLMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NAKLMT to value 0"]
impl crate::Resettable for NAKLMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
