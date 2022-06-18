#[doc = "Register `DMACTL0` reader"]
pub struct R(crate::R<DMACTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL0` writer"]
pub struct W(crate::W<DMACTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL0_SPEC>;
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
impl From<crate::W<DMACTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DMACTL0_ENABLE` reader - DMA Transfer Enable"]
pub type USB_DMACTL0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMACTL0_ENABLE` writer - DMA Transfer Enable"]
pub type USB_DMACTL0_ENABLE_W<'a> = crate::BitWriter<'a, u16, DMACTL0_SPEC, bool, 0>;
#[doc = "Field `USB_DMACTL0_DIR` reader - DMA Direction"]
pub type USB_DMACTL0_DIR_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMACTL0_DIR` writer - DMA Direction"]
pub type USB_DMACTL0_DIR_W<'a> = crate::BitWriter<'a, u16, DMACTL0_SPEC, bool, 1>;
#[doc = "Field `USB_DMACTL0_MODE` reader - DMA Transfer Mode"]
pub type USB_DMACTL0_MODE_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMACTL0_MODE` writer - DMA Transfer Mode"]
pub type USB_DMACTL0_MODE_W<'a> = crate::BitWriter<'a, u16, DMACTL0_SPEC, bool, 2>;
#[doc = "Field `USB_DMACTL0_IE` reader - DMA Interrupt Enable"]
pub type USB_DMACTL0_IE_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMACTL0_IE` writer - DMA Interrupt Enable"]
pub type USB_DMACTL0_IE_W<'a> = crate::BitWriter<'a, u16, DMACTL0_SPEC, bool, 3>;
#[doc = "Field `USB_DMACTL0_EP` reader - Endpoint number"]
pub type USB_DMACTL0_EP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_DMACTL0_EP` writer - Endpoint number"]
pub type USB_DMACTL0_EP_W<'a> = crate::FieldWriter<'a, u16, DMACTL0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `USB_DMACTL0_ERR` reader - Bus Error Bit"]
pub type USB_DMACTL0_ERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_DMACTL0_ERR` writer - Bus Error Bit"]
pub type USB_DMACTL0_ERR_W<'a> = crate::BitWriter<'a, u16, DMACTL0_SPEC, bool, 8>;
#[doc = "Burst Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_DMACTL0_BRSTM_A {
    #[doc = "0: Bursts of unspecified length"]
    USB_DMACTL0_BRSTM_ANY = 0,
    #[doc = "1: INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC4 = 1,
    #[doc = "2: INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC8 = 2,
    #[doc = "3: INCR16, INCR8, INCR4 or unspecified length"]
    USB_DMACTL0_BRSTM_INC16 = 3,
}
impl From<USB_DMACTL0_BRSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_DMACTL0_BRSTM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USB_DMACTL0_BRSTM` reader - Burst Mode"]
pub type USB_DMACTL0_BRSTM_R = crate::FieldReader<u8, USB_DMACTL0_BRSTM_A>;
impl USB_DMACTL0_BRSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_DMACTL0_BRSTM_A {
        match self.bits {
            0 => USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_ANY,
            1 => USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC4,
            2 => USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC8,
            3 => USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_ANY`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_any(&self) -> bool {
        *self == USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_ANY
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC4`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc4(&self) -> bool {
        *self == USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC4
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC8`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc8(&self) -> bool {
        *self == USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC8
    }
    #[doc = "Checks if the value of the field is `USB_DMACTL0_BRSTM_INC16`"]
    #[inline(always)]
    pub fn is_usb_dmactl0_brstm_inc16(&self) -> bool {
        *self == USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC16
    }
}
#[doc = "Field `USB_DMACTL0_BRSTM` writer - Burst Mode"]
pub type USB_DMACTL0_BRSTM_W<'a> =
    crate::FieldWriterSafe<'a, u16, DMACTL0_SPEC, u8, USB_DMACTL0_BRSTM_A, 2, 9>;
impl<'a> USB_DMACTL0_BRSTM_W<'a> {
    #[doc = "Bursts of unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_any(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_ANY)
    }
    #[doc = "INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc4(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC4)
    }
    #[doc = "INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc8(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC8)
    }
    #[doc = "INCR16, INCR8, INCR4 or unspecified length"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm_inc16(self) -> &'a mut W {
        self.variant(USB_DMACTL0_BRSTM_A::USB_DMACTL0_BRSTM_INC16)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_enable(&self) -> USB_DMACTL0_ENABLE_R {
        USB_DMACTL0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn usb_dmactl0_dir(&self) -> USB_DMACTL0_DIR_R {
        USB_DMACTL0_DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_mode(&self) -> USB_DMACTL0_MODE_R {
        USB_DMACTL0_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_ie(&self) -> USB_DMACTL0_IE_R {
        USB_DMACTL0_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn usb_dmactl0_ep(&self) -> USB_DMACTL0_EP_R {
        USB_DMACTL0_EP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn usb_dmactl0_err(&self) -> USB_DMACTL0_ERR_R {
        USB_DMACTL0_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm(&self) -> USB_DMACTL0_BRSTM_R {
        USB_DMACTL0_BRSTM_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Transfer Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_enable(&mut self) -> USB_DMACTL0_ENABLE_W {
        USB_DMACTL0_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Direction"]
    #[inline(always)]
    pub fn usb_dmactl0_dir(&mut self) -> USB_DMACTL0_DIR_W {
        USB_DMACTL0_DIR_W::new(self)
    }
    #[doc = "Bit 2 - DMA Transfer Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_mode(&mut self) -> USB_DMACTL0_MODE_W {
        USB_DMACTL0_MODE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn usb_dmactl0_ie(&mut self) -> USB_DMACTL0_IE_W {
        USB_DMACTL0_IE_W::new(self)
    }
    #[doc = "Bits 4:7 - Endpoint number"]
    #[inline(always)]
    pub fn usb_dmactl0_ep(&mut self) -> USB_DMACTL0_EP_W {
        USB_DMACTL0_EP_W::new(self)
    }
    #[doc = "Bit 8 - Bus Error Bit"]
    #[inline(always)]
    pub fn usb_dmactl0_err(&mut self) -> USB_DMACTL0_ERR_W {
        USB_DMACTL0_ERR_W::new(self)
    }
    #[doc = "Bits 9:10 - Burst Mode"]
    #[inline(always)]
    pub fn usb_dmactl0_brstm(&mut self) -> USB_DMACTL0_BRSTM_W {
        USB_DMACTL0_BRSTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](index.html) module"]
pub struct DMACTL0_SPEC;
impl crate::RegisterSpec for DMACTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmactl0::R](R) reader structure"]
impl crate::Readable for DMACTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](W) writer structure"]
impl crate::Writable for DMACTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL0 to value 0"]
impl crate::Resettable for DMACTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
