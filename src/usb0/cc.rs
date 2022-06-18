#[doc = "Register `CC` reader"]
pub struct R(crate::R<CC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC` writer"]
pub struct W(crate::W<CC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC_SPEC>;
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
impl From<crate::W<CC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CC_CLKDIV` reader - PLL Clock Divisor"]
pub type USB_CC_CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_CC_CLKDIV` writer - PLL Clock Divisor"]
pub type USB_CC_CLKDIV_W<'a> = crate::FieldWriter<'a, u32, CC_SPEC, u8, u8, 4, 0>;
#[doc = "Field `USB_CC_CSD` reader - Clock Source/Direction"]
pub type USB_CC_CSD_R = crate::BitReader<bool>;
#[doc = "Field `USB_CC_CSD` writer - Clock Source/Direction"]
pub type USB_CC_CSD_W<'a> = crate::BitWriter<'a, u32, CC_SPEC, bool, 8>;
#[doc = "Field `USB_CC_CLKEN` reader - USB Clock Enable"]
pub type USB_CC_CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_CC_CLKEN` writer - USB Clock Enable"]
pub type USB_CC_CLKEN_W<'a> = crate::BitWriter<'a, u32, CC_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:3 - PLL Clock Divisor"]
    #[inline(always)]
    pub fn usb_cc_clkdiv(&self) -> USB_CC_CLKDIV_R {
        USB_CC_CLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Clock Source/Direction"]
    #[inline(always)]
    pub fn usb_cc_csd(&self) -> USB_CC_CSD_R {
        USB_CC_CSD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_cc_clken(&self) -> USB_CC_CLKEN_R {
        USB_CC_CLKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL Clock Divisor"]
    #[inline(always)]
    pub fn usb_cc_clkdiv(&mut self) -> USB_CC_CLKDIV_W {
        USB_CC_CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - Clock Source/Direction"]
    #[inline(always)]
    pub fn usb_cc_csd(&mut self) -> USB_CC_CSD_W {
        USB_CC_CSD_W::new(self)
    }
    #[doc = "Bit 9 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_cc_clken(&mut self) -> USB_CC_CLKEN_W {
        USB_CC_CLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](index.html) module"]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc::R](R) reader structure"]
impl crate::Readable for CC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc::W](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
