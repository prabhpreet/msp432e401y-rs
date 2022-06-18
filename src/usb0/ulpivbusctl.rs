#[doc = "Register `ULPIVBUSCTL` reader"]
pub struct R(crate::R<ULPIVBUSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPIVBUSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPIVBUSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPIVBUSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULPIVBUSCTL` writer"]
pub struct W(crate::W<ULPIVBUSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULPIVBUSCTL_SPEC>;
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
impl From<crate::W<ULPIVBUSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULPIVBUSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ULPIVBUSCTL_USEEXTVBUS` reader - Use External VBUS"]
pub type USB_ULPIVBUSCTL_USEEXTVBUS_R = crate::BitReader<bool>;
#[doc = "Field `USB_ULPIVBUSCTL_USEEXTVBUS` writer - Use External VBUS"]
pub type USB_ULPIVBUSCTL_USEEXTVBUS_W<'a> = crate::BitWriter<'a, u8, ULPIVBUSCTL_SPEC, bool, 0>;
#[doc = "Field `USB_ULPIVBUSCTL_USEEXTVBUSIND` reader - Use External VBUS Indicator"]
pub type USB_ULPIVBUSCTL_USEEXTVBUSIND_R = crate::BitReader<bool>;
#[doc = "Field `USB_ULPIVBUSCTL_USEEXTVBUSIND` writer - Use External VBUS Indicator"]
pub type USB_ULPIVBUSCTL_USEEXTVBUSIND_W<'a> = crate::BitWriter<'a, u8, ULPIVBUSCTL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbus(&self) -> USB_ULPIVBUSCTL_USEEXTVBUS_R {
        USB_ULPIVBUSCTL_USEEXTVBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbusind(&self) -> USB_ULPIVBUSCTL_USEEXTVBUSIND_R {
        USB_ULPIVBUSCTL_USEEXTVBUSIND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbus(&mut self) -> USB_ULPIVBUSCTL_USEEXTVBUS_W {
        USB_ULPIVBUSCTL_USEEXTVBUS_W::new(self)
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbusind(&mut self) -> USB_ULPIVBUSCTL_USEEXTVBUSIND_W {
        USB_ULPIVBUSCTL_USEEXTVBUSIND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB ULPI VBUS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpivbusctl](index.html) module"]
pub struct ULPIVBUSCTL_SPEC;
impl crate::RegisterSpec for ULPIVBUSCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ulpivbusctl::R](R) reader structure"]
impl crate::Readable for ULPIVBUSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulpivbusctl::W](W) writer structure"]
impl crate::Writable for ULPIVBUSCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULPIVBUSCTL to value 0"]
impl crate::Resettable for ULPIVBUSCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
