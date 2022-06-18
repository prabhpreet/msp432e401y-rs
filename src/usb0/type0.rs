#[doc = "Register `TYPE0` reader"]
pub struct R(crate::R<TYPE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE0` writer"]
pub struct W(crate::W<TYPE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE0_SPEC>;
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
impl From<crate::W<TYPE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TYPE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operating Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_TYPE0_SPEED_A {
    #[doc = "1: High"]
    USB_TYPE0_SPEED_HIGH = 1,
    #[doc = "2: Full"]
    USB_TYPE0_SPEED_FULL = 2,
    #[doc = "3: Low"]
    USB_TYPE0_SPEED_LOW = 3,
}
impl From<USB_TYPE0_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_TYPE0_SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_TYPE0_SPEED` reader - Operating Speed"]
pub type USB_TYPE0_SPEED_R = crate::FieldReader<u8, USB_TYPE0_SPEED_A>;
impl USB_TYPE0_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_TYPE0_SPEED_A> {
        match self.bits {
            1 => Some(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_HIGH),
            2 => Some(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL),
            3 => Some(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_HIGH`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_high(&self) -> bool {
        *self == USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_FULL`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_full(&self) -> bool {
        *self == USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_LOW`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_low(&self) -> bool {
        *self == USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW
    }
}
#[doc = "Field `USB_TYPE0_SPEED` writer - Operating Speed"]
pub type USB_TYPE0_SPEED_W<'a> =
    crate::FieldWriter<'a, u8, TYPE0_SPEC, u8, USB_TYPE0_SPEED_A, 2, 6>;
impl<'a> USB_TYPE0_SPEED_W<'a> {
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_type0_speed_high(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_type0_speed_full(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_type0_speed_low(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEED_A::USB_TYPE0_SPEED_LOW)
    }
}
impl R {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_type0_speed(&self) -> USB_TYPE0_SPEED_R {
        USB_TYPE0_SPEED_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_type0_speed(&mut self) -> USB_TYPE0_SPEED_W {
        USB_TYPE0_SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Type Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type0](index.html) module"]
pub struct TYPE0_SPEC;
impl crate::RegisterSpec for TYPE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [type0::R](R) reader structure"]
impl crate::Readable for TYPE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type0::W](W) writer structure"]
impl crate::Writable for TYPE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TYPE0 to value 0"]
impl crate::Resettable for TYPE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
