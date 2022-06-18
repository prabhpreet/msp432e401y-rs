#[doc = "Register `DMACOUNT2` reader"]
pub struct R(crate::R<DMACOUNT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACOUNT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACOUNT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACOUNT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACOUNT2` writer"]
pub struct W(crate::W<DMACOUNT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACOUNT2_SPEC>;
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
impl From<crate::W<DMACOUNT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACOUNT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMACOUNT2_COUNT` reader - DMA Count"]
pub type USB_DMACOUNT2_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_DMACOUNT2_COUNT` writer - DMA Count"]
pub type USB_DMACOUNT2_COUNT_W<'a> = crate::FieldWriter<'a, u32, DMACOUNT2_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount2_count(&self) -> USB_DMACOUNT2_COUNT_R {
        USB_DMACOUNT2_COUNT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount2_count(&mut self) -> USB_DMACOUNT2_COUNT_W {
        USB_DMACOUNT2_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Count 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount2](index.html) module"]
pub struct DMACOUNT2_SPEC;
impl crate::RegisterSpec for DMACOUNT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacount2::R](R) reader structure"]
impl crate::Readable for DMACOUNT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacount2::W](W) writer structure"]
impl crate::Writable for DMACOUNT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACOUNT2 to value 0"]
impl crate::Resettable for DMACOUNT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
