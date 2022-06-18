#[doc = "Register `IS` reader"]
pub struct R(crate::R<IS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS` writer"]
pub struct W(crate::W<IS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IS_SPEC>;
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
impl From<crate::W<IS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_IS_SUSPEND` reader - SUSPEND Signaling Detected"]
pub type USB_IS_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_SUSPEND` writer - SUSPEND Signaling Detected"]
pub type USB_IS_SUSPEND_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 0>;
#[doc = "Field `USB_IS_RESUME` reader - RESUME Signaling Detected"]
pub type USB_IS_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_RESUME` writer - RESUME Signaling Detected"]
pub type USB_IS_RESUME_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 1>;
#[doc = "Field `USB_IS_BABBLE` reader - Babble Detected"]
pub type USB_IS_BABBLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_BABBLE` writer - Babble Detected"]
pub type USB_IS_BABBLE_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 2>;
#[doc = "Field `USB_IS_SOF` reader - Start of Frame"]
pub type USB_IS_SOF_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_SOF` writer - Start of Frame"]
pub type USB_IS_SOF_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 3>;
#[doc = "Field `USB_IS_CONN` reader - Session Connect"]
pub type USB_IS_CONN_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_CONN` writer - Session Connect"]
pub type USB_IS_CONN_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 4>;
#[doc = "Field `USB_IS_DISCON` reader - Session Disconnect (OTG only)"]
pub type USB_IS_DISCON_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_DISCON` writer - Session Disconnect (OTG only)"]
pub type USB_IS_DISCON_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 5>;
#[doc = "Field `USB_IS_SESREQ` reader - SESSION REQUEST (OTG only)"]
pub type USB_IS_SESREQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_SESREQ` writer - SESSION REQUEST (OTG only)"]
pub type USB_IS_SESREQ_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 6>;
#[doc = "Field `USB_IS_VBUSERR` reader - VBUS Error (OTG only)"]
pub type USB_IS_VBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_IS_VBUSERR` writer - VBUS Error (OTG only)"]
pub type USB_IS_VBUSERR_W<'a> = crate::BitWriter<'a, u8, IS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_suspend(&self) -> USB_IS_SUSPEND_R {
        USB_IS_SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_resume(&self) -> USB_IS_RESUME_R {
        USB_IS_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn usb_is_babble(&self) -> USB_IS_BABBLE_R {
        USB_IS_BABBLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn usb_is_sof(&self) -> USB_IS_SOF_R {
        USB_IS_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn usb_is_conn(&self) -> USB_IS_CONN_R {
        USB_IS_CONN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Session Disconnect (OTG only)"]
    #[inline(always)]
    pub fn usb_is_discon(&self) -> USB_IS_DISCON_R {
        USB_IS_DISCON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SESSION REQUEST (OTG only)"]
    #[inline(always)]
    pub fn usb_is_sesreq(&self) -> USB_IS_SESREQ_R {
        USB_IS_SESREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VBUS Error (OTG only)"]
    #[inline(always)]
    pub fn usb_is_vbuserr(&self) -> USB_IS_VBUSERR_R {
        USB_IS_VBUSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SUSPEND Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_suspend(&mut self) -> USB_IS_SUSPEND_W {
        USB_IS_SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - RESUME Signaling Detected"]
    #[inline(always)]
    pub fn usb_is_resume(&mut self) -> USB_IS_RESUME_W {
        USB_IS_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Babble Detected"]
    #[inline(always)]
    pub fn usb_is_babble(&mut self) -> USB_IS_BABBLE_W {
        USB_IS_BABBLE_W::new(self)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn usb_is_sof(&mut self) -> USB_IS_SOF_W {
        USB_IS_SOF_W::new(self)
    }
    #[doc = "Bit 4 - Session Connect"]
    #[inline(always)]
    pub fn usb_is_conn(&mut self) -> USB_IS_CONN_W {
        USB_IS_CONN_W::new(self)
    }
    #[doc = "Bit 5 - Session Disconnect (OTG only)"]
    #[inline(always)]
    pub fn usb_is_discon(&mut self) -> USB_IS_DISCON_W {
        USB_IS_DISCON_W::new(self)
    }
    #[doc = "Bit 6 - SESSION REQUEST (OTG only)"]
    #[inline(always)]
    pub fn usb_is_sesreq(&mut self) -> USB_IS_SESREQ_W {
        USB_IS_SESREQ_W::new(self)
    }
    #[doc = "Bit 7 - VBUS Error (OTG only)"]
    #[inline(always)]
    pub fn usb_is_vbuserr(&mut self) -> USB_IS_VBUSERR_W {
        USB_IS_VBUSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB General Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](index.html) module"]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [is::R](R) reader structure"]
impl crate::Readable for IS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [is::W](W) writer structure"]
impl crate::Writable for IS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
