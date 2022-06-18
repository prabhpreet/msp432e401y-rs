#[doc = "Register `VDCISC` reader"]
pub struct R(crate::R<VDCISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDCISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDCISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDCISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDCISC` writer"]
pub struct W(crate::W<VDCISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDCISC_SPEC>;
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
impl From<crate::W<VDCISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDCISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VDCISC_VD` reader - VBUS Droop Interrupt Status and Clear"]
pub type USB_VDCISC_VD_R = crate::BitReader<bool>;
#[doc = "Field `USB_VDCISC_VD` writer - VBUS Droop Interrupt Status and Clear"]
pub type USB_VDCISC_VD_W<'a> = crate::BitWriter<'a, u32, VDCISC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - VBUS Droop Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_vdcisc_vd(&self) -> USB_VDCISC_VD_R {
        USB_VDCISC_VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS Droop Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_vdcisc_vd(&mut self) -> USB_VDCISC_VD_W {
        USB_VDCISC_VD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB VBUS Droop Control Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcisc](index.html) module"]
pub struct VDCISC_SPEC;
impl crate::RegisterSpec for VDCISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdcisc::R](R) reader structure"]
impl crate::Readable for VDCISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdcisc::W](W) writer structure"]
impl crate::Writable for VDCISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDCISC to value 0"]
impl crate::Resettable for VDCISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
