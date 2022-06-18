#[doc = "Register `VDC` reader"]
pub struct R(crate::R<VDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDC` writer"]
pub struct W(crate::W<VDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDC_SPEC>;
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
impl From<crate::W<VDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VDC_VBDEN` reader - VBUS Droop Enable"]
pub type USB_VDC_VBDEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_VDC_VBDEN` writer - VBUS Droop Enable"]
pub type USB_VDC_VBDEN_W<'a> = crate::BitWriter<'a, u32, VDC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    pub fn usb_vdc_vbden(&self) -> USB_VDC_VBDEN_R {
        USB_VDC_VBDEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Enable"]
    #[inline(always)]
    pub fn usb_vdc_vbden(&mut self) -> USB_VDC_VBDEN_W {
        USB_VDC_VBDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB VBUS Droop Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdc](index.html) module"]
pub struct VDC_SPEC;
impl crate::RegisterSpec for VDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdc::R](R) reader structure"]
impl crate::Readable for VDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdc::W](W) writer structure"]
impl crate::Writable for VDC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDC to value 0"]
impl crate::Resettable for VDC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
