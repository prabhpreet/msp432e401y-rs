#[doc = "Register `DRIM` reader"]
pub struct R(crate::R<DRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIM` writer"]
pub struct W(crate::W<DRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIM_SPEC>;
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
impl From<crate::W<DRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_DRIM_RESUME` reader - RESUME Interrupt Mask"]
pub type USB_DRIM_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `USB_DRIM_RESUME` writer - RESUME Interrupt Mask"]
pub type USB_DRIM_RESUME_W<'a> = crate::BitWriter<'a, u32, DRIM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - RESUME Interrupt Mask"]
    #[inline(always)]
    pub fn usb_drim_resume(&self) -> USB_DRIM_RESUME_R {
        USB_DRIM_RESUME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESUME Interrupt Mask"]
    #[inline(always)]
    pub fn usb_drim_resume(&mut self) -> USB_DRIM_RESUME_W {
        USB_DRIM_RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device RESUME Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drim](index.html) module"]
pub struct DRIM_SPEC;
impl crate::RegisterSpec for DRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drim::R](R) reader structure"]
impl crate::Readable for DRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drim::W](W) writer structure"]
impl crate::Writable for DRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIM to value 0"]
impl crate::Resettable for DRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
