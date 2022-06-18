#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_PC_ULPIEN` reader - ULPI Enable"]
pub type USB_PC_ULPIEN_R = crate::BitReader<bool>;
#[doc = "Field `USB_PC_ULPIEN` writer - ULPI Enable"]
pub type USB_PC_ULPIEN_W<'a> = crate::BitWriter<'a, u32, PC_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 16 - ULPI Enable"]
    #[inline(always)]
    pub fn usb_pc_ulpien(&self) -> USB_PC_ULPIEN_R {
        USB_PC_ULPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ULPI Enable"]
    #[inline(always)]
    pub fn usb_pc_ulpien(&mut self) -> USB_PC_ULPIEN_W {
        USB_PC_ULPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
