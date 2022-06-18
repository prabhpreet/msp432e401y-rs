#[doc = "Register `EPCIM` reader"]
pub struct R(crate::R<EPCIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCIM` writer"]
pub struct W(crate::W<EPCIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCIM_SPEC>;
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
impl From<crate::W<EPCIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EPCIM_PF` reader - USB Power Fault Interrupt Mask"]
pub type USB_EPCIM_PF_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPCIM_PF` writer - USB Power Fault Interrupt Mask"]
pub type USB_EPCIM_PF_W<'a> = crate::BitWriter<'a, u32, EPCIM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Mask"]
    #[inline(always)]
    pub fn usb_epcim_pf(&self) -> USB_EPCIM_PF_R {
        USB_EPCIM_PF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Power Fault Interrupt Mask"]
    #[inline(always)]
    pub fn usb_epcim_pf(&mut self) -> USB_EPCIM_PF_W {
        USB_EPCIM_PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB External Power Control Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcim](index.html) module"]
pub struct EPCIM_SPEC;
impl crate::RegisterSpec for EPCIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epcim::R](R) reader structure"]
impl crate::Readable for EPCIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcim::W](W) writer structure"]
impl crate::Writable for EPCIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPCIM to value 0"]
impl crate::Resettable for EPCIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
