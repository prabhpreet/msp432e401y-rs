#[doc = "Register `DMACOUNT0` reader"]
pub struct R(crate::R<DMACOUNT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACOUNT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACOUNT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACOUNT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACOUNT0` writer"]
pub struct W(crate::W<DMACOUNT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACOUNT0_SPEC>;
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
impl From<crate::W<DMACOUNT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACOUNT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMACOUNT0_COUNT` reader - DMA Count"]
pub type USB_DMACOUNT0_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_DMACOUNT0_COUNT` writer - DMA Count"]
pub type USB_DMACOUNT0_COUNT_W<'a> = crate::FieldWriter<'a, u32, DMACOUNT0_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount0_count(&self) -> USB_DMACOUNT0_COUNT_R {
        USB_DMACOUNT0_COUNT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount0_count(&mut self) -> USB_DMACOUNT0_COUNT_W {
        USB_DMACOUNT0_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount0](index.html) module"]
pub struct DMACOUNT0_SPEC;
impl crate::RegisterSpec for DMACOUNT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacount0::R](R) reader structure"]
impl crate::Readable for DMACOUNT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacount0::W](W) writer structure"]
impl crate::Writable for DMACOUNT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACOUNT0 to value 0"]
impl crate::Resettable for DMACOUNT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
