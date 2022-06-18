#[doc = "Register `DMACOUNT7` reader"]
pub struct R(crate::R<DMACOUNT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACOUNT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACOUNT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACOUNT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACOUNT7` writer"]
pub struct W(crate::W<DMACOUNT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACOUNT7_SPEC>;
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
impl From<crate::W<DMACOUNT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACOUNT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMACOUNT7_COUNT` reader - DMA Count"]
pub type USB_DMACOUNT7_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_DMACOUNT7_COUNT` writer - DMA Count"]
pub type USB_DMACOUNT7_COUNT_W<'a> = crate::FieldWriter<'a, u32, DMACOUNT7_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount7_count(&self) -> USB_DMACOUNT7_COUNT_R {
        USB_DMACOUNT7_COUNT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount7_count(&mut self) -> USB_DMACOUNT7_COUNT_W {
        USB_DMACOUNT7_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Count 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount7](index.html) module"]
pub struct DMACOUNT7_SPEC;
impl crate::RegisterSpec for DMACOUNT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacount7::R](R) reader structure"]
impl crate::Readable for DMACOUNT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacount7::W](W) writer structure"]
impl crate::Writable for DMACOUNT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACOUNT7 to value 0"]
impl crate::Resettable for DMACOUNT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
