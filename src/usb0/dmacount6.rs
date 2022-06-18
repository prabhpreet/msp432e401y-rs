#[doc = "Register `DMACOUNT6` reader"]
pub struct R(crate::R<DMACOUNT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACOUNT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACOUNT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACOUNT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACOUNT6` writer"]
pub struct W(crate::W<DMACOUNT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACOUNT6_SPEC>;
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
impl From<crate::W<DMACOUNT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACOUNT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMACOUNT6_COUNT` reader - DMA Count"]
pub type USB_DMACOUNT6_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_DMACOUNT6_COUNT` writer - DMA Count"]
pub type USB_DMACOUNT6_COUNT_W<'a> = crate::FieldWriter<'a, u32, DMACOUNT6_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount6_count(&self) -> USB_DMACOUNT6_COUNT_R {
        USB_DMACOUNT6_COUNT_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Count"]
    #[inline(always)]
    pub fn usb_dmacount6_count(&mut self) -> USB_DMACOUNT6_COUNT_W {
        USB_DMACOUNT6_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Count 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount6](index.html) module"]
pub struct DMACOUNT6_SPEC;
impl crate::RegisterSpec for DMACOUNT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacount6::R](R) reader structure"]
impl crate::Readable for DMACOUNT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacount6::W](W) writer structure"]
impl crate::Writable for DMACOUNT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACOUNT6 to value 0"]
impl crate::Resettable for DMACOUNT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
