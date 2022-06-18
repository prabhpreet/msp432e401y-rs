#[doc = "Register `EPCRIS` reader"]
pub struct R(crate::R<EPCRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPCRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPCRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPCRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPCRIS` writer"]
pub struct W(crate::W<EPCRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPCRIS_SPEC>;
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
impl From<crate::W<EPCRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPCRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_EPCRIS_PF` reader - USB Power Fault Interrupt Status"]
pub type USB_EPCRIS_PF_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPCRIS_PF` writer - USB Power Fault Interrupt Status"]
pub type USB_EPCRIS_PF_W<'a> = crate::BitWriter<'a, u32, EPCRIS_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status"]
    #[inline(always)]
    pub fn usb_epcris_pf(&self) -> USB_EPCRIS_PF_R {
        USB_EPCRIS_PF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Power Fault Interrupt Status"]
    #[inline(always)]
    pub fn usb_epcris_pf(&mut self) -> USB_EPCRIS_PF_W {
        USB_EPCRIS_PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB External Power Control Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcris](index.html) module"]
pub struct EPCRIS_SPEC;
impl crate::RegisterSpec for EPCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epcris::R](R) reader structure"]
impl crate::Readable for EPCRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epcris::W](W) writer structure"]
impl crate::Writable for EPCRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPCRIS to value 0"]
impl crate::Resettable for EPCRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
