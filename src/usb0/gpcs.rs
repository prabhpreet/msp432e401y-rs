#[doc = "Register `GPCS` reader"]
pub struct R(crate::R<GPCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCS` writer"]
pub struct W(crate::W<GPCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCS_SPEC>;
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
impl From<crate::W<GPCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_GPCS_DEVMOD_A {
    #[doc = "0: Use USB0VBUS and USB0ID pin"]
    USB_GPCS_DEVMOD_OTG = 0,
    #[doc = "2: Force USB0VBUS and USB0ID low"]
    USB_GPCS_DEVMOD_HOST = 2,
    #[doc = "3: Force USB0VBUS and USB0ID high"]
    USB_GPCS_DEVMOD_DEV = 3,
    #[doc = "4: Use USB0VBUS and force USB0ID low"]
    USB_GPCS_DEVMOD_HOSTVBUS = 4,
    #[doc = "5: Use USB0VBUS and force USB0ID high"]
    USB_GPCS_DEVMOD_DEVVBUS = 5,
}
impl From<USB_GPCS_DEVMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_GPCS_DEVMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_GPCS_DEVMOD` reader - Device Mode"]
pub type USB_GPCS_DEVMOD_R = crate::FieldReader<u8, USB_GPCS_DEVMOD_A>;
impl USB_GPCS_DEVMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_GPCS_DEVMOD_A> {
        match self.bits {
            0 => Some(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_OTG),
            2 => Some(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOST),
            3 => Some(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEV),
            4 => Some(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOSTVBUS),
            5 => Some(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEVVBUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_OTG`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_otg(&self) -> bool {
        *self == USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_OTG
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_HOST`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_host(&self) -> bool {
        *self == USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOST
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_DEV`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_dev(&self) -> bool {
        *self == USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEV
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_HOSTVBUS`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_hostvbus(&self) -> bool {
        *self == USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOSTVBUS
    }
    #[doc = "Checks if the value of the field is `USB_GPCS_DEVMOD_DEVVBUS`"]
    #[inline(always)]
    pub fn is_usb_gpcs_devmod_devvbus(&self) -> bool {
        *self == USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEVVBUS
    }
}
#[doc = "Field `USB_GPCS_DEVMOD` writer - Device Mode"]
pub type USB_GPCS_DEVMOD_W<'a> =
    crate::FieldWriter<'a, u32, GPCS_SPEC, u8, USB_GPCS_DEVMOD_A, 3, 0>;
impl<'a> USB_GPCS_DEVMOD_W<'a> {
    #[doc = "Use USB0VBUS and USB0ID pin"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_otg(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_OTG)
    }
    #[doc = "Force USB0VBUS and USB0ID low"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_host(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOST)
    }
    #[doc = "Force USB0VBUS and USB0ID high"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_dev(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEV)
    }
    #[doc = "Use USB0VBUS and force USB0ID low"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_hostvbus(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_HOSTVBUS)
    }
    #[doc = "Use USB0VBUS and force USB0ID high"]
    #[inline(always)]
    pub fn usb_gpcs_devmod_devvbus(self) -> &'a mut W {
        self.variant(USB_GPCS_DEVMOD_A::USB_GPCS_DEVMOD_DEVVBUS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmod(&self) -> USB_GPCS_DEVMOD_R {
        USB_GPCS_DEVMOD_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline(always)]
    pub fn usb_gpcs_devmod(&mut self) -> USB_GPCS_DEVMOD_W {
        USB_GPCS_DEVMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB General-Purpose Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcs](index.html) module"]
pub struct GPCS_SPEC;
impl crate::RegisterSpec for GPCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpcs::R](R) reader structure"]
impl crate::Readable for GPCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpcs::W](W) writer structure"]
impl crate::Writable for GPCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCS to value 0"]
impl crate::Resettable for GPCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
