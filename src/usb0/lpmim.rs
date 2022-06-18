#[doc = "Register `LPMIM` reader"]
pub struct R(crate::R<LPMIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMIM` writer"]
pub struct W(crate::W<LPMIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMIM_SPEC>;
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
impl From<crate::W<LPMIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_LPMIM_STALL` reader - LPM STALL Interrupt Mask"]
pub type USB_LPMIM_STALL_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_STALL` writer - LPM STALL Interrupt Mask"]
pub type USB_LPMIM_STALL_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 0>;
#[doc = "Field `USB_LPMIM_NY` reader - LPM NY Interrupt Mask"]
pub type USB_LPMIM_NY_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_NY` writer - LPM NY Interrupt Mask"]
pub type USB_LPMIM_NY_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 1>;
#[doc = "Field `USB_LPMIM_ACK` reader - LPM ACK Interrupt Mask"]
pub type USB_LPMIM_ACK_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_ACK` writer - LPM ACK Interrupt Mask"]
pub type USB_LPMIM_ACK_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 2>;
#[doc = "Field `USB_LPMIM_NC` reader - LPM NC Interrupt Mask"]
pub type USB_LPMIM_NC_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_NC` writer - LPM NC Interrupt Mask"]
pub type USB_LPMIM_NC_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 3>;
#[doc = "Field `USB_LPMIM_RES` reader - LPM Resume Interrupt Mask"]
pub type USB_LPMIM_RES_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_RES` writer - LPM Resume Interrupt Mask"]
pub type USB_LPMIM_RES_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 4>;
#[doc = "Field `USB_LPMIM_ERR` reader - LPM Error Interrupt Mask"]
pub type USB_LPMIM_ERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMIM_ERR` writer - LPM Error Interrupt Mask"]
pub type USB_LPMIM_ERR_W<'a> = crate::BitWriter<'a, u8, LPMIM_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - LPM STALL Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_stall(&self) -> USB_LPMIM_STALL_R {
        USB_LPMIM_STALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM NY Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_ny(&self) -> USB_LPMIM_NY_R {
        USB_LPMIM_NY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_ack(&self) -> USB_LPMIM_ACK_R {
        USB_LPMIM_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPM NC Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_nc(&self) -> USB_LPMIM_NC_R {
        USB_LPMIM_NC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_res(&self) -> USB_LPMIM_RES_R {
        USB_LPMIM_RES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPM Error Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_err(&self) -> USB_LPMIM_ERR_R {
        USB_LPMIM_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM STALL Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_stall(&mut self) -> USB_LPMIM_STALL_W {
        USB_LPMIM_STALL_W::new(self)
    }
    #[doc = "Bit 1 - LPM NY Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_ny(&mut self) -> USB_LPMIM_NY_W {
        USB_LPMIM_NY_W::new(self)
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_ack(&mut self) -> USB_LPMIM_ACK_W {
        USB_LPMIM_ACK_W::new(self)
    }
    #[doc = "Bit 3 - LPM NC Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_nc(&mut self) -> USB_LPMIM_NC_W {
        USB_LPMIM_NC_W::new(self)
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_res(&mut self) -> USB_LPMIM_RES_W {
        USB_LPMIM_RES_W::new(self)
    }
    #[doc = "Bit 5 - LPM Error Interrupt Mask"]
    #[inline(always)]
    pub fn usb_lpmim_err(&mut self) -> USB_LPMIM_ERR_W {
        USB_LPMIM_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmim](index.html) module"]
pub struct LPMIM_SPEC;
impl crate::RegisterSpec for LPMIM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpmim::R](R) reader structure"]
impl crate::Readable for LPMIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmim::W](W) writer structure"]
impl crate::Writable for LPMIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMIM to value 0"]
impl crate::Resettable for LPMIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
