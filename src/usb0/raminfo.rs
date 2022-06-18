#[doc = "Register `RAMINFO` reader"]
pub struct R(crate::R<RAMINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMINFO` writer"]
pub struct W(crate::W<RAMINFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMINFO_SPEC>;
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
impl From<crate::W<RAMINFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMINFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RAMINFO_RAMBITS` reader - RAM Address Bus Width"]
pub type USB_RAMINFO_RAMBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RAMINFO_RAMBITS` writer - RAM Address Bus Width"]
pub type USB_RAMINFO_RAMBITS_W<'a> = crate::FieldWriter<'a, u8, RAMINFO_SPEC, u8, u8, 4, 0>;
#[doc = "Field `USB_RAMINFO_DMACHAN` reader - DMA Channels"]
pub type USB_RAMINFO_DMACHAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RAMINFO_DMACHAN` writer - DMA Channels"]
pub type USB_RAMINFO_DMACHAN_W<'a> = crate::FieldWriter<'a, u8, RAMINFO_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline(always)]
    pub fn usb_raminfo_rambits(&self) -> USB_RAMINFO_RAMBITS_R {
        USB_RAMINFO_RAMBITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline(always)]
    pub fn usb_raminfo_dmachan(&self) -> USB_RAMINFO_DMACHAN_R {
        USB_RAMINFO_DMACHAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline(always)]
    pub fn usb_raminfo_rambits(&mut self) -> USB_RAMINFO_RAMBITS_W {
        USB_RAMINFO_RAMBITS_W::new(self)
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline(always)]
    pub fn usb_raminfo_dmachan(&mut self) -> USB_RAMINFO_DMACHAN_W {
        USB_RAMINFO_DMACHAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB RAM Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raminfo](index.html) module"]
pub struct RAMINFO_SPEC;
impl crate::RegisterSpec for RAMINFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [raminfo::R](R) reader structure"]
impl crate::Readable for RAMINFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [raminfo::W](W) writer structure"]
impl crate::Writable for RAMINFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMINFO to value 0"]
impl crate::Resettable for RAMINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
