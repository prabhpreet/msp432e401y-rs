#[doc = "Register `LPMRIS` reader"]
pub struct R(crate::R<LPMRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMRIS` writer"]
pub struct W(crate::W<LPMRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMRIS_SPEC>;
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
impl From<crate::W<LPMRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_LPMRIS_LPMST` reader - LPM STALL Interrupt Status"]
pub type USB_LPMRIS_LPMST_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_LPMST` writer - LPM STALL Interrupt Status"]
pub type USB_LPMRIS_LPMST_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 0>;
#[doc = "Field `USB_LPMRIS_NY` reader - LPM NY Interrupt Status"]
pub type USB_LPMRIS_NY_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_NY` writer - LPM NY Interrupt Status"]
pub type USB_LPMRIS_NY_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 1>;
#[doc = "Field `USB_LPMRIS_ACK` reader - LPM ACK Interrupt Status"]
pub type USB_LPMRIS_ACK_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_ACK` writer - LPM ACK Interrupt Status"]
pub type USB_LPMRIS_ACK_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 2>;
#[doc = "Field `USB_LPMRIS_NC` reader - LPM NC Interrupt Status"]
pub type USB_LPMRIS_NC_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_NC` writer - LPM NC Interrupt Status"]
pub type USB_LPMRIS_NC_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 3>;
#[doc = "Field `USB_LPMRIS_RES` reader - LPM Resume Interrupt Status"]
pub type USB_LPMRIS_RES_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_RES` writer - LPM Resume Interrupt Status"]
pub type USB_LPMRIS_RES_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 4>;
#[doc = "Field `USB_LPMRIS_ERR` reader - LPM Interrupt Status"]
pub type USB_LPMRIS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMRIS_ERR` writer - LPM Interrupt Status"]
pub type USB_LPMRIS_ERR_W<'a> = crate::BitWriter<'a, u8, LPMRIS_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - LPM STALL Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_lpmst(&self) -> USB_LPMRIS_LPMST_R {
        USB_LPMRIS_LPMST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM NY Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ny(&self) -> USB_LPMRIS_NY_R {
        USB_LPMRIS_NY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ack(&self) -> USB_LPMRIS_ACK_R {
        USB_LPMRIS_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPM NC Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_nc(&self) -> USB_LPMRIS_NC_R {
        USB_LPMRIS_NC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_res(&self) -> USB_LPMRIS_RES_R {
        USB_LPMRIS_RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPM Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_err(&self) -> USB_LPMRIS_ERR_R {
        USB_LPMRIS_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM STALL Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_lpmst(&mut self) -> USB_LPMRIS_LPMST_W {
        USB_LPMRIS_LPMST_W::new(self)
    }
    #[doc = "Bit 1 - LPM NY Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ny(&mut self) -> USB_LPMRIS_NY_W {
        USB_LPMRIS_NY_W::new(self)
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ack(&mut self) -> USB_LPMRIS_ACK_W {
        USB_LPMRIS_ACK_W::new(self)
    }
    #[doc = "Bit 3 - LPM NC Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_nc(&mut self) -> USB_LPMRIS_NC_W {
        USB_LPMRIS_NC_W::new(self)
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_res(&mut self) -> USB_LPMRIS_RES_W {
        USB_LPMRIS_RES_W::new(self)
    }
    #[doc = "Bit 5 - LPM Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_err(&mut self) -> USB_LPMRIS_ERR_W {
        USB_LPMRIS_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmris](index.html) module"]
pub struct LPMRIS_SPEC;
impl crate::RegisterSpec for LPMRIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpmris::R](R) reader structure"]
impl crate::Readable for LPMRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmris::W](W) writer structure"]
impl crate::Writable for LPMRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMRIS to value 0"]
impl crate::Resettable for LPMRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
