#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSEXC_IC_FPIDCIC` writer - Floating-Point Input Denormal Exception Interrupt Clear"]
pub type SYSEXC_IC_FPIDCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 0>;
#[doc = "Field `SYSEXC_IC_FPDZCIC` writer - Floating-Point Divide By 0 Exception Interrupt Clear"]
pub type SYSEXC_IC_FPDZCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 1>;
#[doc = "Field `SYSEXC_IC_FPIOCIC` writer - Floating-Point Invalid Operation Interrupt Clear"]
pub type SYSEXC_IC_FPIOCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 2>;
#[doc = "Field `SYSEXC_IC_FPUFCIC` writer - Floating-Point Underflow Exception Interrupt Clear"]
pub type SYSEXC_IC_FPUFCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 3>;
#[doc = "Field `SYSEXC_IC_FPOFCIC` writer - Floating-Point Overflow Exception Interrupt Clear"]
pub type SYSEXC_IC_FPOFCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 4>;
#[doc = "Field `SYSEXC_IC_FPIXCIC` writer - Floating-Point Inexact Exception Interrupt Clear"]
pub type SYSEXC_IC_FPIXCIC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpidcic(&mut self) -> SYSEXC_IC_FPIDCIC_W {
        SYSEXC_IC_FPIDCIC_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpdzcic(&mut self) -> SYSEXC_IC_FPDZCIC_W {
        SYSEXC_IC_FPDZCIC_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpiocic(&mut self) -> SYSEXC_IC_FPIOCIC_W {
        SYSEXC_IC_FPIOCIC_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpufcic(&mut self) -> SYSEXC_IC_FPUFCIC_W {
        SYSEXC_IC_FPUFCIC_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpofcic(&mut self) -> SYSEXC_IC_FPOFCIC_W {
        SYSEXC_IC_FPOFCIC_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Clear"]
    #[inline(always)]
    pub fn sysexc_ic_fpixcic(&mut self) -> SYSEXC_IC_FPIXCIC_W {
        SYSEXC_IC_FPIXCIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Exception Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
