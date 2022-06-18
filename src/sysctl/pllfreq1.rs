#[doc = "Register `PLLFREQ1` reader"]
pub struct R(crate::R<PLLFREQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLFREQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLFREQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLFREQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLFREQ1` writer"]
pub struct W(crate::W<PLLFREQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLFREQ1_SPEC>;
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
impl From<crate::W<PLLFREQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLFREQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PLLFREQ1_N` reader - PLL N Value"]
pub type SYSCTL_PLLFREQ1_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_PLLFREQ1_N` writer - PLL N Value"]
pub type SYSCTL_PLLFREQ1_N_W<'a> = crate::FieldWriter<'a, u32, PLLFREQ1_SPEC, u8, u8, 5, 0>;
#[doc = "Field `SYSCTL_PLLFREQ1_Q` reader - PLL Q Value"]
pub type SYSCTL_PLLFREQ1_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_PLLFREQ1_Q` writer - PLL Q Value"]
pub type SYSCTL_PLLFREQ1_Q_W<'a> = crate::FieldWriter<'a, u32, PLLFREQ1_SPEC, u8, u8, 5, 8>;
impl R {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_n(&self) -> SYSCTL_PLLFREQ1_N_R {
        SYSCTL_PLLFREQ1_N_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_q(&self) -> SYSCTL_PLLFREQ1_Q_R {
        SYSCTL_PLLFREQ1_Q_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_n(&mut self) -> SYSCTL_PLLFREQ1_N_W {
        SYSCTL_PLLFREQ1_N_W::new(self)
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_q(&mut self) -> SYSCTL_PLLFREQ1_Q_W {
        SYSCTL_PLLFREQ1_Q_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Frequency 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfreq1](index.html) module"]
pub struct PLLFREQ1_SPEC;
impl crate::RegisterSpec for PLLFREQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllfreq1::R](R) reader structure"]
impl crate::Readable for PLLFREQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllfreq1::W](W) writer structure"]
impl crate::Writable for PLLFREQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLFREQ1 to value 0"]
impl crate::Resettable for PLLFREQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
