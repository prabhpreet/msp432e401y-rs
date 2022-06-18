#[doc = "Register `EPC` reader"]
pub struct R(crate::R<EPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPC` writer"]
pub struct W(crate::W<EPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPC_SPEC>;
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
impl From<crate::W<EPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Power Supply Enable Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_EPC_EPEN_A {
    #[doc = "0: Power Enable Active Low"]
    USB_EPC_EPEN_LOW = 0,
    #[doc = "1: Power Enable Active High"]
    USB_EPC_EPEN_HIGH = 1,
    #[doc = "2: Power Enable High if VBUS Low (OTG only)"]
    USB_EPC_EPEN_VBLOW = 2,
    #[doc = "3: Power Enable High if VBUS High (OTG only)"]
    USB_EPC_EPEN_VBHIGH = 3,
}
impl From<USB_EPC_EPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_EPC_EPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_EPC_EPEN` reader - External Power Supply Enable Configuration"]
pub type USB_EPC_EPEN_R = crate::FieldReader<u8, USB_EPC_EPEN_A>;
impl USB_EPC_EPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_EPC_EPEN_A {
        match self.bits {
            0 => USB_EPC_EPEN_A::USB_EPC_EPEN_LOW,
            1 => USB_EPC_EPEN_A::USB_EPC_EPEN_HIGH,
            2 => USB_EPC_EPEN_A::USB_EPC_EPEN_VBLOW,
            3 => USB_EPC_EPEN_A::USB_EPC_EPEN_VBHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_LOW`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_low(&self) -> bool {
        *self == USB_EPC_EPEN_A::USB_EPC_EPEN_LOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_HIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_high(&self) -> bool {
        *self == USB_EPC_EPEN_A::USB_EPC_EPEN_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_VBLOW`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_vblow(&self) -> bool {
        *self == USB_EPC_EPEN_A::USB_EPC_EPEN_VBLOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_EPEN_VBHIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_epen_vbhigh(&self) -> bool {
        *self == USB_EPC_EPEN_A::USB_EPC_EPEN_VBHIGH
    }
}
#[doc = "Field `USB_EPC_EPEN` writer - External Power Supply Enable Configuration"]
pub type USB_EPC_EPEN_W<'a> = crate::FieldWriterSafe<'a, u32, EPC_SPEC, u8, USB_EPC_EPEN_A, 2, 0>;
impl<'a> USB_EPC_EPEN_W<'a> {
    #[doc = "Power Enable Active Low"]
    #[inline(always)]
    pub fn usb_epc_epen_low(self) -> &'a mut W {
        self.variant(USB_EPC_EPEN_A::USB_EPC_EPEN_LOW)
    }
    #[doc = "Power Enable Active High"]
    #[inline(always)]
    pub fn usb_epc_epen_high(self) -> &'a mut W {
        self.variant(USB_EPC_EPEN_A::USB_EPC_EPEN_HIGH)
    }
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    #[inline(always)]
    pub fn usb_epc_epen_vblow(self) -> &'a mut W {
        self.variant(USB_EPC_EPEN_A::USB_EPC_EPEN_VBLOW)
    }
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    #[inline(always)]
    pub fn usb_epc_epen_vbhigh(self) -> &'a mut W {
        self.variant(USB_EPC_EPEN_A::USB_EPC_EPEN_VBHIGH)
    }
}
#[doc = "Field `USB_EPC_EPENDE` reader - EPEN Drive Enable"]
pub type USB_EPC_EPENDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPC_EPENDE` writer - EPEN Drive Enable"]
pub type USB_EPC_EPENDE_W<'a> = crate::BitWriter<'a, u32, EPC_SPEC, bool, 2>;
#[doc = "Field `USB_EPC_PFLTEN` reader - Power Fault Input Enable"]
pub type USB_EPC_PFLTEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPC_PFLTEN` writer - Power Fault Input Enable"]
pub type USB_EPC_PFLTEN_W<'a> = crate::BitWriter<'a, u32, EPC_SPEC, bool, 4>;
#[doc = "Field `USB_EPC_PFLTSEN_HIGH` reader - Power Fault Sense"]
pub type USB_EPC_PFLTSEN_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPC_PFLTSEN_HIGH` writer - Power Fault Sense"]
pub type USB_EPC_PFLTSEN_HIGH_W<'a> = crate::BitWriter<'a, u32, EPC_SPEC, bool, 5>;
#[doc = "Field `USB_EPC_PFLTAEN` reader - Power Fault Action Enable"]
pub type USB_EPC_PFLTAEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_EPC_PFLTAEN` writer - Power Fault Action Enable"]
pub type USB_EPC_PFLTAEN_W<'a> = crate::BitWriter<'a, u32, EPC_SPEC, bool, 6>;
#[doc = "Power Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_EPC_PFLTACT_A {
    #[doc = "0: Unchanged"]
    USB_EPC_PFLTACT_UNCHG = 0,
    #[doc = "1: Tristate"]
    USB_EPC_PFLTACT_TRIS = 1,
    #[doc = "2: Low"]
    USB_EPC_PFLTACT_LOW = 2,
    #[doc = "3: High"]
    USB_EPC_PFLTACT_HIGH = 3,
}
impl From<USB_EPC_PFLTACT_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_EPC_PFLTACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_EPC_PFLTACT` reader - Power Fault Action"]
pub type USB_EPC_PFLTACT_R = crate::FieldReader<u8, USB_EPC_PFLTACT_A>;
impl USB_EPC_PFLTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_EPC_PFLTACT_A {
        match self.bits {
            0 => USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_UNCHG,
            1 => USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_TRIS,
            2 => USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_LOW,
            3 => USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_UNCHG`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_unchg(&self) -> bool {
        *self == USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_UNCHG
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_TRIS`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_tris(&self) -> bool {
        *self == USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_TRIS
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_LOW`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_low(&self) -> bool {
        *self == USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_LOW
    }
    #[doc = "Checks if the value of the field is `USB_EPC_PFLTACT_HIGH`"]
    #[inline(always)]
    pub fn is_usb_epc_pfltact_high(&self) -> bool {
        *self == USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_HIGH
    }
}
#[doc = "Field `USB_EPC_PFLTACT` writer - Power Fault Action"]
pub type USB_EPC_PFLTACT_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPC_SPEC, u8, USB_EPC_PFLTACT_A, 2, 8>;
impl<'a> USB_EPC_PFLTACT_W<'a> {
    #[doc = "Unchanged"]
    #[inline(always)]
    pub fn usb_epc_pfltact_unchg(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_UNCHG)
    }
    #[doc = "Tristate"]
    #[inline(always)]
    pub fn usb_epc_pfltact_tris(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_TRIS)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_epc_pfltact_low(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_LOW)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_epc_pfltact_high(self) -> &'a mut W {
        self.variant(USB_EPC_PFLTACT_A::USB_EPC_PFLTACT_HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn usb_epc_epen(&self) -> USB_EPC_EPEN_R {
        USB_EPC_EPEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn usb_epc_epende(&self) -> USB_EPC_EPENDE_R {
        USB_EPC_EPENDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn usb_epc_pflten(&self) -> USB_EPC_PFLTEN_R {
        USB_EPC_PFLTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn usb_epc_pfltsen_high(&self) -> USB_EPC_PFLTSEN_HIGH_R {
        USB_EPC_PFLTSEN_HIGH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn usb_epc_pfltaen(&self) -> USB_EPC_PFLTAEN_R {
        USB_EPC_PFLTAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn usb_epc_pfltact(&self) -> USB_EPC_PFLTACT_R {
        USB_EPC_PFLTACT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline(always)]
    pub fn usb_epc_epen(&mut self) -> USB_EPC_EPEN_W {
        USB_EPC_EPEN_W::new(self)
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline(always)]
    pub fn usb_epc_epende(&mut self) -> USB_EPC_EPENDE_W {
        USB_EPC_EPENDE_W::new(self)
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline(always)]
    pub fn usb_epc_pflten(&mut self) -> USB_EPC_PFLTEN_W {
        USB_EPC_PFLTEN_W::new(self)
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline(always)]
    pub fn usb_epc_pfltsen_high(&mut self) -> USB_EPC_PFLTSEN_HIGH_W {
        USB_EPC_PFLTSEN_HIGH_W::new(self)
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline(always)]
    pub fn usb_epc_pfltaen(&mut self) -> USB_EPC_PFLTAEN_W {
        USB_EPC_PFLTAEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline(always)]
    pub fn usb_epc_pfltact(&mut self) -> USB_EPC_PFLTACT_W {
        USB_EPC_PFLTACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB External Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epc](index.html) module"]
pub struct EPC_SPEC;
impl crate::RegisterSpec for EPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epc::R](R) reader structure"]
impl crate::Readable for EPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epc::W](W) writer structure"]
impl crate::Writable for EPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPC to value 0"]
impl crate::Resettable for EPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
