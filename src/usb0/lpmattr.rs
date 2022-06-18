#[doc = "Register `LPMATTR` reader"]
pub struct R(crate::R<LPMATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMATTR` writer"]
pub struct W(crate::W<LPMATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMATTR_SPEC>;
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
impl From<crate::W<LPMATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Link State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_LPMATTR_LS_A {
    #[doc = "1: Sleep State (L1)"]
    USB_LPMATTR_LS_L1 = 1,
}
impl From<USB_LPMATTR_LS_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_LPMATTR_LS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_LPMATTR_LS` reader - Link State"]
pub type USB_LPMATTR_LS_R = crate::FieldReader<u8, USB_LPMATTR_LS_A>;
impl USB_LPMATTR_LS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USB_LPMATTR_LS_A> {
        match self.bits {
            1 => Some(USB_LPMATTR_LS_A::USB_LPMATTR_LS_L1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USB_LPMATTR_LS_L1`"]
    #[inline(always)]
    pub fn is_usb_lpmattr_ls_l1(&self) -> bool {
        *self == USB_LPMATTR_LS_A::USB_LPMATTR_LS_L1
    }
}
#[doc = "Field `USB_LPMATTR_LS` writer - Link State"]
pub type USB_LPMATTR_LS_W<'a> =
    crate::FieldWriter<'a, u16, LPMATTR_SPEC, u8, USB_LPMATTR_LS_A, 4, 0>;
impl<'a> USB_LPMATTR_LS_W<'a> {
    #[doc = "Sleep State (L1)"]
    #[inline(always)]
    pub fn usb_lpmattr_ls_l1(self) -> &'a mut W {
        self.variant(USB_LPMATTR_LS_A::USB_LPMATTR_LS_L1)
    }
}
#[doc = "Field `USB_LPMATTR_HIRD` reader - Host Initiated Resume Duration"]
pub type USB_LPMATTR_HIRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_LPMATTR_HIRD` writer - Host Initiated Resume Duration"]
pub type USB_LPMATTR_HIRD_W<'a> = crate::FieldWriter<'a, u16, LPMATTR_SPEC, u8, u8, 4, 4>;
#[doc = "Field `USB_LPMATTR_RMTWAK` reader - Remote Wake"]
pub type USB_LPMATTR_RMTWAK_R = crate::BitReader<bool>;
#[doc = "Field `USB_LPMATTR_RMTWAK` writer - Remote Wake"]
pub type USB_LPMATTR_RMTWAK_W<'a> = crate::BitWriter<'a, u16, LPMATTR_SPEC, bool, 8>;
#[doc = "Field `USB_LPMATTR_ENDPT` reader - Endpoint"]
pub type USB_LPMATTR_ENDPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_LPMATTR_ENDPT` writer - Endpoint"]
pub type USB_LPMATTR_ENDPT_W<'a> = crate::FieldWriter<'a, u16, LPMATTR_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - Link State"]
    #[inline(always)]
    pub fn usb_lpmattr_ls(&self) -> USB_LPMATTR_LS_R {
        USB_LPMATTR_LS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline(always)]
    pub fn usb_lpmattr_hird(&self) -> USB_LPMATTR_HIRD_R {
        USB_LPMATTR_HIRD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline(always)]
    pub fn usb_lpmattr_rmtwak(&self) -> USB_LPMATTR_RMTWAK_R {
        USB_LPMATTR_RMTWAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline(always)]
    pub fn usb_lpmattr_endpt(&self) -> USB_LPMATTR_ENDPT_R {
        USB_LPMATTR_ENDPT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Link State"]
    #[inline(always)]
    pub fn usb_lpmattr_ls(&mut self) -> USB_LPMATTR_LS_W {
        USB_LPMATTR_LS_W::new(self)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline(always)]
    pub fn usb_lpmattr_hird(&mut self) -> USB_LPMATTR_HIRD_W {
        USB_LPMATTR_HIRD_W::new(self)
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline(always)]
    pub fn usb_lpmattr_rmtwak(&mut self) -> USB_LPMATTR_RMTWAK_W {
        USB_LPMATTR_RMTWAK_W::new(self)
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline(always)]
    pub fn usb_lpmattr_endpt(&mut self) -> USB_LPMATTR_ENDPT_W {
        USB_LPMATTR_ENDPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB LPM Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmattr](index.html) module"]
pub struct LPMATTR_SPEC;
impl crate::RegisterSpec for LPMATTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lpmattr::R](R) reader structure"]
impl crate::Readable for LPMATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmattr::W](W) writer structure"]
impl crate::Writable for LPMATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPMATTR to value 0"]
impl crate::Resettable for LPMATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
