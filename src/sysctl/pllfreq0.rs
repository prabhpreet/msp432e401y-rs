#[doc = "Register `PLLFREQ0` reader"]
pub struct R(crate::R<PLLFREQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLFREQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLFREQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLFREQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLFREQ0` writer"]
pub struct W(crate::W<PLLFREQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLFREQ0_SPEC>;
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
impl From<crate::W<PLLFREQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLFREQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PLLFREQ0_MINT` reader - PLL M Integer Value"]
pub type SYSCTL_PLLFREQ0_MINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTL_PLLFREQ0_MINT` writer - PLL M Integer Value"]
pub type SYSCTL_PLLFREQ0_MINT_W<'a> = crate::FieldWriter<'a, u32, PLLFREQ0_SPEC, u16, u16, 10, 0>;
#[doc = "Field `SYSCTL_PLLFREQ0_MFRAC` reader - PLL M Fractional Value"]
pub type SYSCTL_PLLFREQ0_MFRAC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTL_PLLFREQ0_MFRAC` writer - PLL M Fractional Value"]
pub type SYSCTL_PLLFREQ0_MFRAC_W<'a> = crate::FieldWriter<'a, u32, PLLFREQ0_SPEC, u16, u16, 10, 10>;
#[doc = "Field `SYSCTL_PLLFREQ0_PLLPWR` reader - PLL Power"]
pub type SYSCTL_PLLFREQ0_PLLPWR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PLLFREQ0_PLLPWR` writer - PLL Power"]
pub type SYSCTL_PLLFREQ0_PLLPWR_W<'a> = crate::BitWriter<'a, u32, PLLFREQ0_SPEC, bool, 23>;
impl R {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mint(&self) -> SYSCTL_PLLFREQ0_MINT_R {
        SYSCTL_PLLFREQ0_MINT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mfrac(&self) -> SYSCTL_PLLFREQ0_MFRAC_R {
        SYSCTL_PLLFREQ0_MFRAC_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_pllpwr(&self) -> SYSCTL_PLLFREQ0_PLLPWR_R {
        SYSCTL_PLLFREQ0_PLLPWR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mint(&mut self) -> SYSCTL_PLLFREQ0_MINT_W {
        SYSCTL_PLLFREQ0_MINT_W::new(self)
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mfrac(&mut self) -> SYSCTL_PLLFREQ0_MFRAC_W {
        SYSCTL_PLLFREQ0_MFRAC_W::new(self)
    }
    #[doc = "Bit 23 - PLL Power"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_pllpwr(&mut self) -> SYSCTL_PLLFREQ0_PLLPWR_W {
        SYSCTL_PLLFREQ0_PLLPWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Frequency 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllfreq0](index.html) module"]
pub struct PLLFREQ0_SPEC;
impl crate::RegisterSpec for PLLFREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllfreq0::R](R) reader structure"]
impl crate::Readable for PLLFREQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllfreq0::W](W) writer structure"]
impl crate::Writable for PLLFREQ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLFREQ0 to value 0"]
impl crate::Resettable for PLLFREQ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
