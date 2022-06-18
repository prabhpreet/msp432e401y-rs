#[doc = "Register `DEVCTL` reader"]
pub struct R(crate::R<DEVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCTL` writer"]
pub struct W(crate::W<DEVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCTL_SPEC>;
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
impl From<crate::W<DEVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DEVCTL_SESSION` reader - Session Start/End (OTG only)"]
pub type USB_DEVCTL_SESSION_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_SESSION` writer - Session Start/End (OTG only)"]
pub type USB_DEVCTL_SESSION_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 0>;
#[doc = "Field `USB_DEVCTL_HOSTREQ` reader - Host Request (OTG only)"]
pub type USB_DEVCTL_HOSTREQ_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_HOSTREQ` writer - Host Request (OTG only)"]
pub type USB_DEVCTL_HOSTREQ_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 1>;
#[doc = "Field `USB_DEVCTL_HOST` reader - Host Mode"]
pub type USB_DEVCTL_HOST_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_HOST` writer - Host Mode"]
pub type USB_DEVCTL_HOST_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 2>;
#[doc = "VBUS Level (OTG only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_DEVCTL_VBUS_A {
    #[doc = "0: Below SessionEnd"]
    USB_DEVCTL_VBUS_NONE = 0,
    #[doc = "1: Above SessionEnd, below AValid"]
    USB_DEVCTL_VBUS_SEND = 1,
    #[doc = "2: Above AValid, below VBUSValid"]
    USB_DEVCTL_VBUS_AVALID = 2,
    #[doc = "3: Above VBUSValid"]
    USB_DEVCTL_VBUS_VALID = 3,
}
impl From<USB_DEVCTL_VBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_DEVCTL_VBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_DEVCTL_VBUS` reader - VBUS Level (OTG only)"]
pub type USB_DEVCTL_VBUS_R = crate::FieldReader<u8, USB_DEVCTL_VBUS_A>;
impl USB_DEVCTL_VBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_DEVCTL_VBUS_A {
        match self.bits {
            0 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE,
            1 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND,
            2 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID,
            3 => USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_NONE`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_none(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_SEND`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_send(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_AVALID`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_avalid(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID
    }
    #[doc = "Checks if the value of the field is `USB_DEVCTL_VBUS_VALID`"]
    #[inline(always)]
    pub fn is_usb_devctl_vbus_valid(&self) -> bool {
        *self == USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID
    }
}
#[doc = "Field `USB_DEVCTL_VBUS` writer - VBUS Level (OTG only)"]
pub type USB_DEVCTL_VBUS_W<'a> =
    crate::FieldWriterSafe<'a, u8, DEVCTL_SPEC, u8, USB_DEVCTL_VBUS_A, 2, 3>;
impl<'a> USB_DEVCTL_VBUS_W<'a> {
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn usb_devctl_vbus_none(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_NONE)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_send(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_SEND)
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_avalid(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_AVALID)
    }
    #[doc = "Above VBUSValid"]
    #[inline(always)]
    pub fn usb_devctl_vbus_valid(self) -> &'a mut W {
        self.variant(USB_DEVCTL_VBUS_A::USB_DEVCTL_VBUS_VALID)
    }
}
#[doc = "Field `USB_DEVCTL_LSDEV` reader - Low-Speed Device Detected"]
pub type USB_DEVCTL_LSDEV_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_LSDEV` writer - Low-Speed Device Detected"]
pub type USB_DEVCTL_LSDEV_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 5>;
#[doc = "Field `USB_DEVCTL_FSDEV` reader - Full-Speed Device Detected"]
pub type USB_DEVCTL_FSDEV_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_FSDEV` writer - Full-Speed Device Detected"]
pub type USB_DEVCTL_FSDEV_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 6>;
#[doc = "Field `USB_DEVCTL_DEV` reader - Device Mode (OTG only)"]
pub type USB_DEVCTL_DEV_R = crate::BitReader<bool>;
#[doc = "Field `USB_DEVCTL_DEV` writer - Device Mode (OTG only)"]
pub type USB_DEVCTL_DEV_W<'a> = crate::BitWriter<'a, u8, DEVCTL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_session(&self) -> USB_DEVCTL_SESSION_R {
        USB_DEVCTL_SESSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_hostreq(&self) -> USB_DEVCTL_HOSTREQ_R {
        USB_DEVCTL_HOSTREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn usb_devctl_host(&self) -> USB_DEVCTL_HOST_R {
        USB_DEVCTL_HOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_vbus(&self) -> USB_DEVCTL_VBUS_R {
        USB_DEVCTL_VBUS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_lsdev(&self) -> USB_DEVCTL_LSDEV_R {
        USB_DEVCTL_LSDEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_fsdev(&self) -> USB_DEVCTL_FSDEV_R {
        USB_DEVCTL_FSDEV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_dev(&self) -> USB_DEVCTL_DEV_R {
        USB_DEVCTL_DEV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_session(&mut self) -> USB_DEVCTL_SESSION_W {
        USB_DEVCTL_SESSION_W::new(self)
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_hostreq(&mut self) -> USB_DEVCTL_HOSTREQ_W {
        USB_DEVCTL_HOSTREQ_W::new(self)
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline(always)]
    pub fn usb_devctl_host(&mut self) -> USB_DEVCTL_HOST_W {
        USB_DEVCTL_HOST_W::new(self)
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_vbus(&mut self) -> USB_DEVCTL_VBUS_W {
        USB_DEVCTL_VBUS_W::new(self)
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_lsdev(&mut self) -> USB_DEVCTL_LSDEV_W {
        USB_DEVCTL_LSDEV_W::new(self)
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline(always)]
    pub fn usb_devctl_fsdev(&mut self) -> USB_DEVCTL_FSDEV_W {
        USB_DEVCTL_FSDEV_W::new(self)
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline(always)]
    pub fn usb_devctl_dev(&mut self) -> USB_DEVCTL_DEV_W {
        USB_DEVCTL_DEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devctl](index.html) module"]
pub struct DEVCTL_SPEC;
impl crate::RegisterSpec for DEVCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [devctl::R](R) reader structure"]
impl crate::Readable for DEVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devctl::W](W) writer structure"]
impl crate::Writable for DEVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVCTL to value 0"]
impl crate::Resettable for DEVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
