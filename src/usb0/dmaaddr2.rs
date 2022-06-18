#[doc = "Register `DMAADDR2` reader"]
pub struct R(crate::R<DMAADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAADDR2` writer"]
pub struct W(crate::W<DMAADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAADDR2_SPEC>;
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
impl From<crate::W<DMAADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMAADDR2_ADDR` reader - DMA Address"]
pub type USB_DMAADDR2_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USB_DMAADDR2_ADDR` writer - DMA Address"]
pub type USB_DMAADDR2_ADDR_W<'a> = crate::FieldWriter<'a, u32, DMAADDR2_SPEC, u32, u32, 30, 2>;
impl R {
    #[doc = "Bits 2:31 - DMA Address"]
    #[inline(always)]
    pub fn usb_dmaaddr2_addr(&self) -> USB_DMAADDR2_ADDR_R {
        USB_DMAADDR2_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA Address"]
    #[inline(always)]
    pub fn usb_dmaaddr2_addr(&mut self) -> USB_DMAADDR2_ADDR_W {
        USB_DMAADDR2_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr2](index.html) module"]
pub struct DMAADDR2_SPEC;
impl crate::RegisterSpec for DMAADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaaddr2::R](R) reader structure"]
impl crate::Readable for DMAADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaaddr2::W](W) writer structure"]
impl crate::Writable for DMAADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAADDR2 to value 0"]
impl crate::Resettable for DMAADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
