#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSEXC_RIS_FPIDCRIS` reader - Floating-Point Input Denormal Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIDCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPIDCRIS` writer - Floating-Point Input Denormal Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIDCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `SYSEXC_RIS_FPDZCRIS` reader - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPDZCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPDZCRIS` writer - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPDZCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `SYSEXC_RIS_FPIOCRIS` reader - Floating-Point Invalid Operation Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIOCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPIOCRIS` writer - Floating-Point Invalid Operation Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIOCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `SYSEXC_RIS_FPUFCRIS` reader - Floating-Point Underflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPUFCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPUFCRIS` writer - Floating-Point Underflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPUFCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `SYSEXC_RIS_FPOFCRIS` reader - Floating-Point Overflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPOFCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPOFCRIS` writer - Floating-Point Overflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPOFCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `SYSEXC_RIS_FPIXCRIS` reader - Floating-Point Inexact Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIXCRIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_RIS_FPIXCRIS` writer - Floating-Point Inexact Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIXCRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpidcris(&self) -> SYSEXC_RIS_FPIDCRIS_R {
        SYSEXC_RIS_FPIDCRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpdzcris(&self) -> SYSEXC_RIS_FPDZCRIS_R {
        SYSEXC_RIS_FPDZCRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpiocris(&self) -> SYSEXC_RIS_FPIOCRIS_R {
        SYSEXC_RIS_FPIOCRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpufcris(&self) -> SYSEXC_RIS_FPUFCRIS_R {
        SYSEXC_RIS_FPUFCRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpofcris(&self) -> SYSEXC_RIS_FPOFCRIS_R {
        SYSEXC_RIS_FPOFCRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpixcris(&self) -> SYSEXC_RIS_FPIXCRIS_R {
        SYSEXC_RIS_FPIXCRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpidcris(&mut self) -> SYSEXC_RIS_FPIDCRIS_W {
        SYSEXC_RIS_FPIDCRIS_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpdzcris(&mut self) -> SYSEXC_RIS_FPDZCRIS_W {
        SYSEXC_RIS_FPDZCRIS_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpiocris(&mut self) -> SYSEXC_RIS_FPIOCRIS_W {
        SYSEXC_RIS_FPIOCRIS_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpufcris(&mut self) -> SYSEXC_RIS_FPUFCRIS_W {
        SYSEXC_RIS_FPUFCRIS_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpofcris(&mut self) -> SYSEXC_RIS_FPOFCRIS_W {
        SYSEXC_RIS_FPOFCRIS_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpixcris(&mut self) -> SYSEXC_RIS_FPIXCRIS_W {
        SYSEXC_RIS_FPIXCRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Exception Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
