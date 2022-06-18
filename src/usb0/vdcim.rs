#[doc = "Register `VDCIM` reader"]
pub struct R(crate::R<VDCIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDCIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDCIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDCIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDCIM` writer"]
pub struct W(crate::W<VDCIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDCIM_SPEC>;
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
impl From<crate::W<VDCIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDCIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VDCIM_VD` reader - VBUS Droop Interrupt Mask"]
pub type USB_VDCIM_VD_R = crate::BitReader<bool>;
#[doc = "Field `USB_VDCIM_VD` writer - VBUS Droop Interrupt Mask"]
pub type USB_VDCIM_VD_W<'a> = crate::BitWriter<'a, u32, VDCIM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Interrupt Mask"]
    #[inline(always)]
    pub fn usb_vdcim_vd(&self) -> USB_VDCIM_VD_R {
        USB_VDCIM_VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Interrupt Mask"]
    #[inline(always)]
    pub fn usb_vdcim_vd(&mut self) -> USB_VDCIM_VD_W {
        USB_VDCIM_VD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB VBUS Droop Control Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcim](index.html) module"]
pub struct VDCIM_SPEC;
impl crate::RegisterSpec for VDCIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdcim::R](R) reader structure"]
impl crate::Readable for VDCIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdcim::W](W) writer structure"]
impl crate::Writable for VDCIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDCIM to value 0"]
impl crate::Resettable for VDCIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
