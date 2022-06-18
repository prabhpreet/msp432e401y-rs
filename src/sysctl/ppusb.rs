#[doc = "Register `PPUSB` reader"]
pub struct R(crate::R<PPUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUSB` writer"]
pub struct W(crate::W<PPUSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUSB_SPEC>;
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
impl From<crate::W<PPUSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPUSB_P0` reader - USB Module Present"]
pub type SYSCTL_PPUSB_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPUSB_P0` writer - USB Module Present"]
pub type SYSCTL_PPUSB_P0_W<'a> = crate::BitWriter<'a, u32, PPUSB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Module Present"]
    #[inline(always)]
    pub fn sysctl_ppusb_p0(&self) -> SYSCTL_PPUSB_P0_R {
        SYSCTL_PPUSB_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Present"]
    #[inline(always)]
    pub fn sysctl_ppusb_p0(&mut self) -> SYSCTL_PPUSB_P0_W {
        SYSCTL_PPUSB_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Serial Bus Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppusb](index.html) module"]
pub struct PPUSB_SPEC;
impl crate::RegisterSpec for PPUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppusb::R](R) reader structure"]
impl crate::Readable for PPUSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppusb::W](W) writer structure"]
impl crate::Writable for PPUSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPUSB to value 0"]
impl crate::Resettable for PPUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
