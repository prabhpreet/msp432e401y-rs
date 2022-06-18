#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_IE_SUSPND` reader - Enable SUSPEND Interrupt"]
pub type USB_IE_SUSPND_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_SUSPND` writer - Enable SUSPEND Interrupt"]
pub type USB_IE_SUSPND_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 0>;
#[doc = "Field `USB_IE_RESUME` reader - Enable RESUME Interrupt"]
pub type USB_IE_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_RESUME` writer - Enable RESUME Interrupt"]
pub type USB_IE_RESUME_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 1>;
#[doc = "Field `USB_IE_BABBLE` reader - Enable Babble Interrupt"]
pub type USB_IE_BABBLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_BABBLE` writer - Enable Babble Interrupt"]
pub type USB_IE_BABBLE_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 2>;
#[doc = "Field `USB_IE_SOF` reader - Enable Start-of-Frame Interrupt"]
pub type USB_IE_SOF_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_SOF` writer - Enable Start-of-Frame Interrupt"]
pub type USB_IE_SOF_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 3>;
#[doc = "Field `USB_IE_CONN` reader - Enable Connect Interrupt"]
pub type USB_IE_CONN_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_CONN` writer - Enable Connect Interrupt"]
pub type USB_IE_CONN_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 4>;
#[doc = "Field `USB_IE_DISCON` reader - Enable Disconnect Interrupt"]
pub type USB_IE_DISCON_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_DISCON` writer - Enable Disconnect Interrupt"]
pub type USB_IE_DISCON_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 5>;
#[doc = "Field `USB_IE_SESREQ` reader - Enable Session Request (OTG only)"]
pub type USB_IE_SESREQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_SESREQ` writer - Enable Session Request (OTG only)"]
pub type USB_IE_SESREQ_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 6>;
#[doc = "Field `USB_IE_VBUSERR` reader - Enable VBUS Error Interrupt (OTG only)"]
pub type USB_IE_VBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_IE_VBUSERR` writer - Enable VBUS Error Interrupt (OTG only)"]
pub type USB_IE_VBUSERR_W<'a> = crate::BitWriter<'a, u8, IE_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn usb_ie_suspnd(&self) -> USB_IE_SUSPND_R {
        USB_IE_SUSPND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn usb_ie_resume(&self) -> USB_IE_RESUME_R {
        USB_IE_RESUME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn usb_ie_babble(&self) -> USB_IE_BABBLE_R {
        USB_IE_BABBLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn usb_ie_sof(&self) -> USB_IE_SOF_R {
        USB_IE_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_conn(&self) -> USB_IE_CONN_R {
        USB_IE_CONN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_discon(&self) -> USB_IE_DISCON_R {
        USB_IE_DISCON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_sesreq(&self) -> USB_IE_SESREQ_R {
        USB_IE_SESREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_vbuserr(&self) -> USB_IE_VBUSERR_R {
        USB_IE_VBUSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SUSPEND Interrupt"]
    #[inline(always)]
    pub fn usb_ie_suspnd(&mut self) -> USB_IE_SUSPND_W {
        USB_IE_SUSPND_W::new(self)
    }
    #[doc = "Bit 1 - Enable RESUME Interrupt"]
    #[inline(always)]
    pub fn usb_ie_resume(&mut self) -> USB_IE_RESUME_W {
        USB_IE_RESUME_W::new(self)
    }
    #[doc = "Bit 2 - Enable Babble Interrupt"]
    #[inline(always)]
    pub fn usb_ie_babble(&mut self) -> USB_IE_BABBLE_W {
        USB_IE_BABBLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable Start-of-Frame Interrupt"]
    #[inline(always)]
    pub fn usb_ie_sof(&mut self) -> USB_IE_SOF_W {
        USB_IE_SOF_W::new(self)
    }
    #[doc = "Bit 4 - Enable Connect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_conn(&mut self) -> USB_IE_CONN_W {
        USB_IE_CONN_W::new(self)
    }
    #[doc = "Bit 5 - Enable Disconnect Interrupt"]
    #[inline(always)]
    pub fn usb_ie_discon(&mut self) -> USB_IE_DISCON_W {
        USB_IE_DISCON_W::new(self)
    }
    #[doc = "Bit 6 - Enable Session Request (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_sesreq(&mut self) -> USB_IE_SESREQ_W {
        USB_IE_SESREQ_W::new(self)
    }
    #[doc = "Bit 7 - Enable VBUS Error Interrupt (OTG only)"]
    #[inline(always)]
    pub fn usb_ie_vbuserr(&mut self) -> USB_IE_VBUSERR_W {
        USB_IE_VBUSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
