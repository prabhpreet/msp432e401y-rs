#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controller Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_PP_TYPE_A {
    #[doc = "0: The first-generation USB controller revision"]
    USB_PP_TYPE_0 = 0,
    #[doc = "1: The second-generation USB controller revision"]
    USB_PP_TYPE_1 = 1,
}
impl From<USB_PP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_PP_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_PP_TYPE` reader - Controller Type"]
pub type USB_PP_TYPE_R = crate::FieldReader<u8, USB_PP_TYPE_A>;
impl USB_PP_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_PP_TYPE_A> {
        match self.bits {
            0 => Some(USB_PP_TYPE_A::USB_PP_TYPE_0),
            1 => Some(USB_PP_TYPE_A::USB_PP_TYPE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_PP_TYPE_0`"]
    #[inline(always)]
    pub fn is_usb_pp_type_0(&self) -> bool {
        *self == USB_PP_TYPE_A::USB_PP_TYPE_0
    }
    #[doc = "Checks if the value of the field is `USB_PP_TYPE_1`"]
    #[inline(always)]
    pub fn is_usb_pp_type_1(&self) -> bool {
        *self == USB_PP_TYPE_A::USB_PP_TYPE_1
    }
}
#[doc = "Field `USB_PP_TYPE` writer - Controller Type"]
pub type USB_PP_TYPE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, USB_PP_TYPE_A, 4, 0>;
impl<'a> USB_PP_TYPE_W<'a> {
    #[doc = "The first-generation USB controller revision"]
    #[inline(always)]
    pub fn usb_pp_type_0(self) -> &'a mut W {
        self.variant(USB_PP_TYPE_A::USB_PP_TYPE_0)
    }
    #[doc = "The second-generation USB controller revision"]
    #[inline(always)]
    pub fn usb_pp_type_1(self) -> &'a mut W {
        self.variant(USB_PP_TYPE_A::USB_PP_TYPE_1)
    }
}
#[doc = "Field `USB_PP_PHY` reader - PHY Present"]
pub type USB_PP_PHY_R = crate::BitReader<bool>;
#[doc = "Field `USB_PP_PHY` writer - PHY Present"]
pub type USB_PP_PHY_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 4>;
#[doc = "Field `USB_PP_ULPI` reader - ULPI Present"]
pub type USB_PP_ULPI_R = crate::BitReader<bool>;
#[doc = "Field `USB_PP_ULPI` writer - ULPI Present"]
pub type USB_PP_ULPI_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 5>;
#[doc = "USB Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_PP_USB_A {
    #[doc = "1: DEVICE"]
    USB_PP_USB_DEVICE = 1,
    #[doc = "2: HOST"]
    USB_PP_USB_HOSTDEVICE = 2,
    #[doc = "3: OTG"]
    USB_PP_USB_OTG = 3,
}
impl From<USB_PP_USB_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_PP_USB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_PP_USB` reader - USB Capability"]
pub type USB_PP_USB_R = crate::FieldReader<u8, USB_PP_USB_A>;
impl USB_PP_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_PP_USB_A> {
        match self.bits {
            1 => Some(USB_PP_USB_A::USB_PP_USB_DEVICE),
            2 => Some(USB_PP_USB_A::USB_PP_USB_HOSTDEVICE),
            3 => Some(USB_PP_USB_A::USB_PP_USB_OTG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_DEVICE`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_device(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_DEVICE
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_HOSTDEVICE`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_hostdevice(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_HOSTDEVICE
    }
    #[doc = "Checks if the value of the field is `USB_PP_USB_OTG`"]
    #[inline(always)]
    pub fn is_usb_pp_usb_otg(&self) -> bool {
        *self == USB_PP_USB_A::USB_PP_USB_OTG
    }
}
#[doc = "Field `USB_PP_USB` writer - USB Capability"]
pub type USB_PP_USB_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, USB_PP_USB_A, 2, 6>;
impl<'a> USB_PP_USB_W<'a> {
    #[doc = "DEVICE"]
    #[inline(always)]
    pub fn usb_pp_usb_device(self) -> &'a mut W {
        self.variant(USB_PP_USB_A::USB_PP_USB_DEVICE)
    }
    #[doc = "HOST"]
    #[inline(always)]
    pub fn usb_pp_usb_hostdevice(self) -> &'a mut W {
        self.variant(USB_PP_USB_A::USB_PP_USB_HOSTDEVICE)
    }
    #[doc = "OTG"]
    #[inline(always)]
    pub fn usb_pp_usb_otg(self) -> &'a mut W {
        self.variant(USB_PP_USB_A::USB_PP_USB_OTG)
    }
}
#[doc = "Field `USB_PP_ECNT` reader - Endpoint Count"]
pub type USB_PP_ECNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_PP_ECNT` writer - Endpoint Count"]
pub type USB_PP_ECNT_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn usb_pp_type(&self) -> USB_PP_TYPE_R {
        USB_PP_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn usb_pp_phy(&self) -> USB_PP_PHY_R {
        USB_PP_PHY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline(always)]
    pub fn usb_pp_ulpi(&self) -> USB_PP_ULPI_R {
        USB_PP_ULPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb_pp_usb(&self) -> USB_PP_USB_R {
        USB_PP_USB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn usb_pp_ecnt(&self) -> USB_PP_ECNT_R {
        USB_PP_ECNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn usb_pp_type(&mut self) -> USB_PP_TYPE_W {
        USB_PP_TYPE_W::new(self)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn usb_pp_phy(&mut self) -> USB_PP_PHY_W {
        USB_PP_PHY_W::new(self)
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline(always)]
    pub fn usb_pp_ulpi(&mut self) -> USB_PP_ULPI_W {
        USB_PP_ULPI_W::new(self)
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb_pp_usb(&mut self) -> USB_PP_USB_W {
        USB_PP_USB_W::new(self)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn usb_pp_ecnt(&mut self) -> USB_PP_ECNT_W {
        USB_PP_ECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
