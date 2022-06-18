#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSEXC_MIS_FPIDCMIS` reader - Floating-Point Input Denormal Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIDCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPIDCMIS` writer - Floating-Point Input Denormal Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIDCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `SYSEXC_MIS_FPDZCMIS` reader - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPDZCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPDZCMIS` writer - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPDZCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 1>;
#[doc = "Field `SYSEXC_MIS_FPIOCMIS` reader - Floating-Point Invalid Operation Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIOCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPIOCMIS` writer - Floating-Point Invalid Operation Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIOCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `SYSEXC_MIS_FPUFCMIS` reader - Floating-Point Underflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPUFCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPUFCMIS` writer - Floating-Point Underflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPUFCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `SYSEXC_MIS_FPOFCMIS` reader - Floating-Point Overflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPOFCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPOFCMIS` writer - Floating-Point Overflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPOFCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
#[doc = "Field `SYSEXC_MIS_FPIXCMIS` reader - Floating-Point Inexact Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIXCMIS_R = crate::BitReader<bool>;
#[doc = "Field `SYSEXC_MIS_FPIXCMIS` writer - Floating-Point Inexact Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIXCMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpidcmis(&self) -> SYSEXC_MIS_FPIDCMIS_R {
        SYSEXC_MIS_FPIDCMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpdzcmis(&self) -> SYSEXC_MIS_FPDZCMIS_R {
        SYSEXC_MIS_FPDZCMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpiocmis(&self) -> SYSEXC_MIS_FPIOCMIS_R {
        SYSEXC_MIS_FPIOCMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpufcmis(&self) -> SYSEXC_MIS_FPUFCMIS_R {
        SYSEXC_MIS_FPUFCMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpofcmis(&self) -> SYSEXC_MIS_FPOFCMIS_R {
        SYSEXC_MIS_FPOFCMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpixcmis(&self) -> SYSEXC_MIS_FPIXCMIS_R {
        SYSEXC_MIS_FPIXCMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpidcmis(&mut self) -> SYSEXC_MIS_FPIDCMIS_W {
        SYSEXC_MIS_FPIDCMIS_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpdzcmis(&mut self) -> SYSEXC_MIS_FPDZCMIS_W {
        SYSEXC_MIS_FPDZCMIS_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpiocmis(&mut self) -> SYSEXC_MIS_FPIOCMIS_W {
        SYSEXC_MIS_FPIOCMIS_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpufcmis(&mut self) -> SYSEXC_MIS_FPUFCMIS_W {
        SYSEXC_MIS_FPUFCMIS_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpofcmis(&mut self) -> SYSEXC_MIS_FPOFCMIS_W {
        SYSEXC_MIS_FPOFCMIS_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpixcmis(&mut self) -> SYSEXC_MIS_FPIXCMIS_W {
        SYSEXC_MIS_FPIXCMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Exception Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
