#[doc = "Register `RXTYPE6` reader"]
pub struct R(crate::R<RXTYPE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTYPE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTYPE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTYPE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXTYPE6` writer"]
pub struct W(crate::W<RXTYPE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXTYPE6_SPEC>;
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
impl From<crate::W<RXTYPE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXTYPE6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_RXTYPE6_TEP` reader - Target Endpoint Number"]
pub type USB_RXTYPE6_TEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_RXTYPE6_TEP` writer - Target Endpoint Number"]
pub type USB_RXTYPE6_TEP_W<'a> = crate::FieldWriter<'a, u8, RXTYPE6_SPEC, u8, u8, 4, 0>;
#[doc = "Protocol\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_RXTYPE6_PROTO_A {
    #[doc = "0: Control"]
    USB_RXTYPE6_PROTO_CTRL = 0,
    #[doc = "1: Isochronous"]
    USB_RXTYPE6_PROTO_ISOC = 1,
    #[doc = "2: Bulk"]
    USB_RXTYPE6_PROTO_BULK = 2,
    #[doc = "3: Interrupt"]
    USB_RXTYPE6_PROTO_INT = 3,
}
impl From<USB_RXTYPE6_PROTO_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_RXTYPE6_PROTO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_RXTYPE6_PROTO` reader - Protocol"]
pub type USB_RXTYPE6_PROTO_R = crate::FieldReader<u8, USB_RXTYPE6_PROTO_A>;
impl USB_RXTYPE6_PROTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_RXTYPE6_PROTO_A {
        match self.bits {
            0 => USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_CTRL,
            1 => USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_ISOC,
            2 => USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_BULK,
            3 => USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_PROTO_CTRL`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_proto_ctrl(&self) -> bool {
        *self == USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_CTRL
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_PROTO_ISOC`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_proto_isoc(&self) -> bool {
        *self == USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_ISOC
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_PROTO_BULK`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_proto_bulk(&self) -> bool {
        *self == USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_BULK
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_PROTO_INT`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_proto_int(&self) -> bool {
        *self == USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_INT
    }
}
#[doc = "Field `USB_RXTYPE6_PROTO` writer - Protocol"]
pub type USB_RXTYPE6_PROTO_W<'a> =
    crate::FieldWriterSafe<'a, u8, RXTYPE6_SPEC, u8, USB_RXTYPE6_PROTO_A, 2, 4>;
impl<'a> USB_RXTYPE6_PROTO_W<'a> {
    #[doc = "Control"]
    #[inline(always)]
    pub fn usb_rxtype6_proto_ctrl(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn usb_rxtype6_proto_isoc(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_ISOC)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn usb_rxtype6_proto_bulk(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_BULK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn usb_rxtype6_proto_int(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_PROTO_A::USB_RXTYPE6_PROTO_INT)
    }
}
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_RXTYPE6_SPEED_A {
    #[doc = "0: Default"]
    USB_RXTYPE6_SPEED_DFLT = 0,
    #[doc = "1: High"]
    USB_RXTYPE6_SPEED_HIGH = 1,
    #[doc = "2: Full"]
    USB_RXTYPE6_SPEED_FULL = 2,
    #[doc = "3: Low"]
    USB_RXTYPE6_SPEED_LOW = 3,
}
impl From<USB_RXTYPE6_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_RXTYPE6_SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_RXTYPE6_SPEED` reader - Operating Speed"]
pub type USB_RXTYPE6_SPEED_R = crate::FieldReader<u8, USB_RXTYPE6_SPEED_A>;
impl USB_RXTYPE6_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_RXTYPE6_SPEED_A {
        match self.bits {
            0 => USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_DFLT,
            1 => USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_HIGH,
            2 => USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_FULL,
            3 => USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_SPEED_DFLT`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_speed_dflt(&self) -> bool {
        *self == USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_DFLT
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_SPEED_HIGH`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_speed_high(&self) -> bool {
        *self == USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_SPEED_FULL`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_speed_full(&self) -> bool {
        *self == USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_FULL
    }
    #[doc = "Checks if the value of the field is `USB_RXTYPE6_SPEED_LOW`"]
    #[inline(always)]
    pub fn is_usb_rxtype6_speed_low(&self) -> bool {
        *self == USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_LOW
    }
}
#[doc = "Field `USB_RXTYPE6_SPEED` writer - Operating Speed"]
pub type USB_RXTYPE6_SPEED_W<'a> =
    crate::FieldWriterSafe<'a, u8, RXTYPE6_SPEC, u8, USB_RXTYPE6_SPEED_A, 2, 6>;
impl<'a> USB_RXTYPE6_SPEED_W<'a> {
    #[doc = "Default"]
    #[inline(always)]
    pub fn usb_rxtype6_speed_dflt(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_DFLT)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_rxtype6_speed_high(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_rxtype6_speed_full(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_rxtype6_speed_low(self) -> &'a mut W {
        self.variant(USB_RXTYPE6_SPEED_A::USB_RXTYPE6_SPEED_LOW)
    }
}
impl R {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn usb_rxtype6_tep(&self) -> USB_RXTYPE6_TEP_R {
        USB_RXTYPE6_TEP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_rxtype6_proto(&self) -> USB_RXTYPE6_PROTO_R {
        USB_RXTYPE6_PROTO_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_rxtype6_speed(&self) -> USB_RXTYPE6_SPEED_R {
        USB_RXTYPE6_SPEED_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline(always)]
    pub fn usb_rxtype6_tep(&mut self) -> USB_RXTYPE6_TEP_W {
        USB_RXTYPE6_TEP_W::new(self)
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline(always)]
    pub fn usb_rxtype6_proto(&mut self) -> USB_RXTYPE6_PROTO_W {
        USB_RXTYPE6_PROTO_W::new(self)
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_rxtype6_speed(&mut self) -> USB_RXTYPE6_SPEED_W {
        USB_RXTYPE6_SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Host Configure Receive Type Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype6](index.html) module"]
pub struct RXTYPE6_SPEC;
impl crate::RegisterSpec for RXTYPE6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxtype6::R](R) reader structure"]
impl crate::Readable for RXTYPE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxtype6::W](W) writer structure"]
impl crate::Writable for RXTYPE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXTYPE6 to value 0"]
impl crate::Resettable for RXTYPE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
