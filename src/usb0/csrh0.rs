#[doc = "Register `CSRH0` writer"]
pub struct W(crate::W<CSRH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSRH0_SPEC>;
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
impl From<crate::W<CSRH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSRH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CSRH0_FLUSH` writer - Flush FIFO"]
pub type USB_CSRH0_FLUSH_W<'a> = crate::BitWriter<'a, u8, CSRH0_SPEC, bool, 0>;
#[doc = "Field `USB_CSRH0_DT` writer - Data Toggle"]
pub type USB_CSRH0_DT_W<'a> = crate::BitWriter<'a, u8, CSRH0_SPEC, bool, 1>;
#[doc = "Field `USB_CSRH0_DTWE` writer - Data Toggle Write Enable"]
pub type USB_CSRH0_DTWE_W<'a> = crate::BitWriter<'a, u8, CSRH0_SPEC, bool, 2>;
#[doc = "Field `USB_CSRH0_DISPING` writer - PING Disable"]
pub type USB_CSRH0_DISPING_W<'a> = crate::BitWriter<'a, u8, CSRH0_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Flush FIFO"]
    #[inline(always)]
    pub fn usb_csrh0_flush(&mut self) -> USB_CSRH0_FLUSH_W {
        USB_CSRH0_FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - Data Toggle"]
    #[inline(always)]
    pub fn usb_csrh0_dt(&mut self) -> USB_CSRH0_DT_W {
        USB_CSRH0_DT_W::new(self)
    }
    #[doc = "Bit 2 - Data Toggle Write Enable"]
    #[inline(always)]
    pub fn usb_csrh0_dtwe(&mut self) -> USB_CSRH0_DTWE_W {
        USB_CSRH0_DTWE_W::new(self)
    }
    #[doc = "Bit 3 - PING Disable"]
    #[inline(always)]
    pub fn usb_csrh0_disping(&mut self) -> USB_CSRH0_DISPING_W {
        USB_CSRH0_DISPING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control and Status Endpoint 0 High\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrh0](index.html) module"]
pub struct CSRH0_SPEC;
impl crate::RegisterSpec for CSRH0_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [csrh0::W](W) writer structure"]
impl crate::Writable for CSRH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSRH0 to value 0"]
impl crate::Resettable for CSRH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
