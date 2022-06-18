#[doc = "Register `LPMCNTRL` reader"]
pub struct R(crate::R<LPMCNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMCNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMCNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMCNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMCNTRL` writer"]
pub struct W(crate::W<LPMCNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMCNTRL_SPEC>;
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
impl From<crate::W<LPMCNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMCNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_LPMCNTRL_TXLPM` reader - Transmit LPM Transaction Enable"]
pub type USB_LPMCNTRL_TXLPM_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMCNTRL_TXLPM` writer - Transmit LPM Transaction Enable"]
pub type USB_LPMCNTRL_TXLPM_W<'a> = crate::BitWriter<'a, u8, LPMCNTRL_SPEC, bool, 0>;
#[doc = "Field `USB_LPMCNTRL_RES` reader - LPM Resume"]
pub type USB_LPMCNTRL_RES_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMCNTRL_RES` writer - LPM Resume"]
pub type USB_LPMCNTRL_RES_W<'a> = crate::BitWriter<'a, u8, LPMCNTRL_SPEC, bool, 1>;
#[doc = "LPM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_LPMCNTRL_EN_A {
    #[doc = "0: LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    USB_LPMCNTRL_EN_NONE = 0,
    #[doc = "1: LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    USB_LPMCNTRL_EN_EXT = 1,
    #[doc = "3: The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    USB_LPMCNTRL_EN_LPMEXT = 3,
}
impl From<USB_LPMCNTRL_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_LPMCNTRL_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_LPMCNTRL_EN` reader - LPM Enable"]
pub type USB_LPMCNTRL_EN_R = crate::FieldReader<u8, USB_LPMCNTRL_EN_A>;
impl USB_LPMCNTRL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_LPMCNTRL_EN_A> {
        match self.bits {
            0 => Some(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_NONE),
            1 => Some(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_EXT),
            3 => Some(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_LPMEXT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_NONE`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_none(&self) -> bool {
        *self == USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_NONE
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_EXT`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_ext(&self) -> bool {
        *self == USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_EXT
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_LPMEXT`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_lpmext(&self) -> bool {
        *self == USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_LPMEXT
    }
}
#[doc = "Field `USB_LPMCNTRL_EN` writer - LPM Enable"]
pub type USB_LPMCNTRL_EN_W<'a> =
    crate::FieldWriter<'a, u8, LPMCNTRL_SPEC, u8, USB_LPMCNTRL_EN_A, 2, 2>;
impl<'a> USB_LPMCNTRL_EN_W<'a> {
    #[doc = "LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_none(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_NONE)
    }
    #[doc = "LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_ext(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_EXT)
    }
    #[doc = "The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_lpmext(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_EN_A::USB_LPMCNTRL_EN_LPMEXT)
    }
}
#[doc = "Field `USB_LPMCNTRL_NAK` reader - LPM NAK"]
pub type USB_LPMCNTRL_NAK_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMCNTRL_NAK` writer - LPM NAK"]
pub type USB_LPMCNTRL_NAK_W<'a> = crate::BitWriter<'a, u8, LPMCNTRL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_txlpm(&self) -> USB_LPMCNTRL_TXLPM_R {
        USB_LPMCNTRL_TXLPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline(always)]
    pub fn usb_lpmcntrl_res(&self) -> USB_LPMCNTRL_RES_R {
        USB_LPMCNTRL_RES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en(&self) -> USB_LPMCNTRL_EN_R {
        USB_LPMCNTRL_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline(always)]
    pub fn usb_lpmcntrl_nak(&self) -> USB_LPMCNTRL_NAK_R {
        USB_LPMCNTRL_NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_txlpm(&mut self) -> USB_LPMCNTRL_TXLPM_W {
        USB_LPMCNTRL_TXLPM_W::new(self)
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline(always)]
    pub fn usb_lpmcntrl_res(&mut self) -> USB_LPMCNTRL_RES_W {
        USB_LPMCNTRL_RES_W::new(self)
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en(&mut self) -> USB_LPMCNTRL_EN_W {
        USB_LPMCNTRL_EN_W::new(self)
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline(always)]
    pub fn usb_lpmcntrl_nak(&mut self) -> USB_LPMCNTRL_NAK_W {
        USB_LPMCNTRL_NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcntrl](index.html) module"]
pub struct LPMCNTRL_SPEC;
impl crate::RegisterSpec for LPMCNTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpmcntrl::R](R) reader structure"]
impl crate::Readable for LPMCNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmcntrl::W](W) writer structure"]
impl crate::Writable for LPMCNTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMCNTRL to value 0"]
impl crate::Resettable for LPMCNTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
