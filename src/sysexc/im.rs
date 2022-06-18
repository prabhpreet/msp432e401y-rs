#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSEXC_IM_FPIDCIM` reader - Floating-Point Input Denormal Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIDCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPIDCIM` writer - Floating-Point Input Denormal Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIDCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `SYSEXC_IM_FPDZCIM` reader - Floating-Point Divide By 0 Exception Interrupt Mask"]
pub type SYSEXC_IM_FPDZCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPDZCIM` writer - Floating-Point Divide By 0 Exception Interrupt Mask"]
pub type SYSEXC_IM_FPDZCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 1>;
#[doc = "Field `SYSEXC_IM_FPIOCIM` reader - Floating-Point Invalid Operation Interrupt Mask"]
pub type SYSEXC_IM_FPIOCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPIOCIM` writer - Floating-Point Invalid Operation Interrupt Mask"]
pub type SYSEXC_IM_FPIOCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `SYSEXC_IM_FPUFCIM` reader - Floating-Point Underflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPUFCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPUFCIM` writer - Floating-Point Underflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPUFCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `SYSEXC_IM_FPOFCIM` reader - Floating-Point Overflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPOFCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPOFCIM` writer - Floating-Point Overflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPOFCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 4>;
#[doc = "Field `SYSEXC_IM_FPIXCIM` reader - Floating-Point Inexact Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIXCIM_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_IM_FPIXCIM` writer - Floating-Point Inexact Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIXCIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpidcim(&self) -> SYSEXC_IM_FPIDCIM_R {
        SYSEXC_IM_FPIDCIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpdzcim(&self) -> SYSEXC_IM_FPDZCIM_R {
        SYSEXC_IM_FPDZCIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpiocim(&self) -> SYSEXC_IM_FPIOCIM_R {
        SYSEXC_IM_FPIOCIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpufcim(&self) -> SYSEXC_IM_FPUFCIM_R {
        SYSEXC_IM_FPUFCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpofcim(&self) -> SYSEXC_IM_FPOFCIM_R {
        SYSEXC_IM_FPOFCIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpixcim(&self) -> SYSEXC_IM_FPIXCIM_R {
        SYSEXC_IM_FPIXCIM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpidcim(&mut self) -> SYSEXC_IM_FPIDCIM_W {
        SYSEXC_IM_FPIDCIM_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpdzcim(&mut self) -> SYSEXC_IM_FPDZCIM_W {
        SYSEXC_IM_FPDZCIM_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpiocim(&mut self) -> SYSEXC_IM_FPIOCIM_W {
        SYSEXC_IM_FPIOCIM_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpufcim(&mut self) -> SYSEXC_IM_FPUFCIM_W {
        SYSEXC_IM_FPUFCIM_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpofcim(&mut self) -> SYSEXC_IM_FPOFCIM_W {
        SYSEXC_IM_FPOFCIM_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpixcim(&mut self) -> SYSEXC_IM_FPIXCIM_W {
        SYSEXC_IM_FPIXCIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Exception Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
