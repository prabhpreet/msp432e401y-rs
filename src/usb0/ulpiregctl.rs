#[doc = "Register `ULPIREGCTL` reader"]
pub struct R(crate::R<ULPIREGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULPIREGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULPIREGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULPIREGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULPIREGCTL` writer"]
pub struct W(crate::W<ULPIREGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULPIREGCTL_SPEC>;
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
impl From<crate::W<ULPIREGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULPIREGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_ULPIREGCTL_REGACC` reader - Initiate Register Access"]
pub type USB_ULPIREGCTL_REGACC_R = crate::BitReader<bool>;
#[doc = "Field `USB_ULPIREGCTL_REGACC` writer - Initiate Register Access"]
pub type USB_ULPIREGCTL_REGACC_W<'a> = crate::BitWriter<'a, u8, ULPIREGCTL_SPEC, bool, 0>;
#[doc = "Field `USB_ULPIREGCTL_REGCMPLT` reader - Register Access Complete"]
pub type USB_ULPIREGCTL_REGCMPLT_R = crate::BitReader<bool>;
#[doc = "Field `USB_ULPIREGCTL_REGCMPLT` writer - Register Access Complete"]
pub type USB_ULPIREGCTL_REGCMPLT_W<'a> = crate::BitWriter<'a, u8, ULPIREGCTL_SPEC, bool, 1>;
#[doc = "Field `USB_ULPIREGCTL_RDWR` reader - Read/Write Control"]
pub type USB_ULPIREGCTL_RDWR_R = crate::BitReader<bool>;
#[doc = "Field `USB_ULPIREGCTL_RDWR` writer - Read/Write Control"]
pub type USB_ULPIREGCTL_RDWR_W<'a> = crate::BitWriter<'a, u8, ULPIREGCTL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regacc(&self) -> USB_ULPIREGCTL_REGACC_R {
        USB_ULPIREGCTL_REGACC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regcmplt(&self) -> USB_ULPIREGCTL_REGCMPLT_R {
        USB_ULPIREGCTL_REGCMPLT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn usb_ulpiregctl_rdwr(&self) -> USB_ULPIREGCTL_RDWR_R {
        USB_ULPIREGCTL_RDWR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regacc(&mut self) -> USB_ULPIREGCTL_REGACC_W {
        USB_ULPIREGCTL_REGACC_W::new(self)
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regcmplt(&mut self) -> USB_ULPIREGCTL_REGCMPLT_W {
        USB_ULPIREGCTL_REGCMPLT_W::new(self)
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn usb_ulpiregctl_rdwr(&mut self) -> USB_ULPIREGCTL_RDWR_W {
        USB_ULPIREGCTL_RDWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB ULPI Register Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregctl](index.html) module"]
pub struct ULPIREGCTL_SPEC;
impl crate::RegisterSpec for ULPIREGCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ulpiregctl::R](R) reader structure"]
impl crate::Readable for ULPIREGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulpiregctl::W](W) writer structure"]
impl crate::Writable for ULPIREGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ULPIREGCTL to value 0"]
impl crate::Resettable for ULPIREGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
