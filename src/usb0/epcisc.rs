#[doc = "Register `EPCISC` reader"]
pub struct R(crate::R<EPCISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCISC` writer"]
pub struct W(crate::W<EPCISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCISC_SPEC>;
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
impl From<crate::W<EPCISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EPCISC_PF` reader - USB Power Fault Interrupt Status and Clear"]
pub type USB_EPCISC_PF_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPCISC_PF` writer - USB Power Fault Interrupt Status and Clear"]
pub type USB_EPCISC_PF_W<'a> = crate::BitWriter<'a, u32, EPCISC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_epcisc_pf(&self) -> USB_EPCISC_PF_R {
        USB_EPCISC_PF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status and Clear"]
    #[inline(always)]
    pub fn usb_epcisc_pf(&mut self) -> USB_EPCISC_PF_W {
        USB_EPCISC_PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB External Power Control Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcisc](index.html) module"]
pub struct EPCISC_SPEC;
impl crate::RegisterSpec for EPCISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epcisc::R](R) reader structure"]
impl crate::Readable for EPCISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcisc::W](W) writer structure"]
impl crate::Writable for EPCISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPCISC to value 0"]
impl crate::Resettable for EPCISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
