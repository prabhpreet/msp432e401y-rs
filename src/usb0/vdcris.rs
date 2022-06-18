#[doc = "Register `VDCRIS` reader"]
pub struct R(crate::R<VDCRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDCRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDCRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDCRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDCRIS` writer"]
pub struct W(crate::W<VDCRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDCRIS_SPEC>;
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
impl From<crate::W<VDCRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDCRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VDCRIS_VD` reader - VBUS Droop Raw Interrupt Status"]
pub type USB_VDCRIS_VD_R = crate::BitReader<bool>;
#[doc = "Field `USB_VDCRIS_VD` writer - VBUS Droop Raw Interrupt Status"]
pub type USB_VDCRIS_VD_W<'a> = crate::BitWriter<'a, u32, VDCRIS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Raw Interrupt Status"]
    #[inline(always)]
    pub fn usb_vdcris_vd(&self) -> USB_VDCRIS_VD_R {
        USB_VDCRIS_VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Raw Interrupt Status"]
    #[inline(always)]
    pub fn usb_vdcris_vd(&mut self) -> USB_VDCRIS_VD_W {
        USB_VDCRIS_VD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB VBUS Droop Control Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcris](index.html) module"]
pub struct VDCRIS_SPEC;
impl crate::RegisterSpec for VDCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdcris::R](R) reader structure"]
impl crate::Readable for VDCRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdcris::W](W) writer structure"]
impl crate::Writable for VDCRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDCRIS to value 0"]
impl crate::Resettable for VDCRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
