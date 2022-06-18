#[doc = "Register `CONTIM` reader"]
pub struct R(crate::R<CONTIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTIM` writer"]
pub struct W(crate::W<CONTIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTIM_SPEC>;
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
impl From<crate::W<CONTIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_CONTIM_WTID` reader - Wait ID"]
pub type USB_CONTIM_WTID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_CONTIM_WTID` writer - Wait ID"]
pub type USB_CONTIM_WTID_W<'a> = crate::FieldWriter<'a, u8, CONTIM_SPEC, u8, u8, 4, 0>;
#[doc = "Field `USB_CONTIM_WTCON` reader - Connect Wait"]
pub type USB_CONTIM_WTCON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_CONTIM_WTCON` writer - Connect Wait"]
pub type USB_CONTIM_WTCON_W<'a> = crate::FieldWriter<'a, u8, CONTIM_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    pub fn usb_contim_wtid(&self) -> USB_CONTIM_WTID_R {
        USB_CONTIM_WTID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    pub fn usb_contim_wtcon(&self) -> USB_CONTIM_WTCON_R {
        USB_CONTIM_WTCON_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait ID"]
    #[inline(always)]
    pub fn usb_contim_wtid(&mut self) -> USB_CONTIM_WTID_W {
        USB_CONTIM_WTID_W::new(self)
    }
    #[doc = "Bits 4:7 - Connect Wait"]
    #[inline(always)]
    pub fn usb_contim_wtcon(&mut self) -> USB_CONTIM_WTCON_W {
        USB_CONTIM_WTCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Connect Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [contim](index.html) module"]
pub struct CONTIM_SPEC;
impl crate::RegisterSpec for CONTIM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [contim::R](R) reader structure"]
impl crate::Readable for CONTIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [contim::W](W) writer structure"]
impl crate::Writable for CONTIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTIM to value 0"]
impl crate::Resettable for CONTIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
