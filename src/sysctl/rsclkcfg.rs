#[doc = "Register `RSCLKCFG` reader"]
pub struct R(crate::R<RSCLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSCLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSCLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSCLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSCLKCFG` writer"]
pub struct W(crate::W<RSCLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSCLKCFG_SPEC>;
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
impl From<crate::W<RSCLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSCLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_PSYSDIV` reader - PLL System Clock Divisor"]
pub type SYSCTL_RSCLKCFG_PSYSDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTL_RSCLKCFG_PSYSDIV` writer - PLL System Clock Divisor"]
pub type SYSCTL_RSCLKCFG_PSYSDIV_W<'a> =
    crate::FieldWriter<'a, u32, RSCLKCFG_SPEC, u16, u16, 10, 0>;
#[doc = "Field `SYSCTL_RSCLKCFG_OSYSDIV` reader - Oscillator System Clock Divisor"]
pub type SYSCTL_RSCLKCFG_OSYSDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTL_RSCLKCFG_OSYSDIV` writer - Oscillator System Clock Divisor"]
pub type SYSCTL_RSCLKCFG_OSYSDIV_W<'a> =
    crate::FieldWriter<'a, u32, RSCLKCFG_SPEC, u16, u16, 10, 10>;
#[doc = "Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RSCLKCFG_OSCSRC_A {
    #[doc = "0: PIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_PIOSC = 0,
    #[doc = "2: LFIOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_LFIOSC = 2,
    #[doc = "3: MOSC is oscillator source"]
    SYSCTL_RSCLKCFG_OSCSRC_MOSC = 3,
    #[doc = "4: Hibernation Module RTC Oscillator (RTCOSC)"]
    SYSCTL_RSCLKCFG_OSCSRC_RTC = 4,
}
impl From<SYSCTL_RSCLKCFG_OSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RSCLKCFG_OSCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_OSCSRC` reader - Oscillator Source"]
pub type SYSCTL_RSCLKCFG_OSCSRC_R = crate::FieldReader<u8, SYSCTL_RSCLKCFG_OSCSRC_A>;
impl SYSCTL_RSCLKCFG_OSCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RSCLKCFG_OSCSRC_A> {
        match self.bits {
            0 => Some(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_PIOSC),
            2 => Some(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC),
            3 => Some(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_MOSC),
            4 => Some(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_RTC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_piosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_lfiosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_mosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_MOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_OSCSRC_RTC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_oscsrc_rtc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_RTC
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_OSCSRC` writer - Oscillator Source"]
pub type SYSCTL_RSCLKCFG_OSCSRC_W<'a> =
    crate::FieldWriter<'a, u32, RSCLKCFG_SPEC, u8, SYSCTL_RSCLKCFG_OSCSRC_A, 4, 20>;
impl<'a> SYSCTL_RSCLKCFG_OSCSRC_W<'a> {
    #[doc = "PIOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_PIOSC)
    }
    #[doc = "LFIOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_LFIOSC)
    }
    #[doc = "MOSC is oscillator source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_MOSC)
    }
    #[doc = "Hibernation Module RTC Oscillator (RTCOSC)"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc_rtc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_OSCSRC_A::SYSCTL_RSCLKCFG_OSCSRC_RTC)
    }
}
#[doc = "PLL Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RSCLKCFG_PLLSRC_A {
    #[doc = "0: PIOSC is PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_PIOSC = 0,
    #[doc = "3: MOSC is the PLL input clock source"]
    SYSCTL_RSCLKCFG_PLLSRC_MOSC = 3,
}
impl From<SYSCTL_RSCLKCFG_PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RSCLKCFG_PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_PLLSRC` reader - PLL Source"]
pub type SYSCTL_RSCLKCFG_PLLSRC_R = crate::FieldReader<u8, SYSCTL_RSCLKCFG_PLLSRC_A>;
impl SYSCTL_RSCLKCFG_PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RSCLKCFG_PLLSRC_A> {
        match self.bits {
            0 => Some(SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_PIOSC),
            3 => Some(SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_MOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_PLLSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_pllsrc_piosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RSCLKCFG_PLLSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_rsclkcfg_pllsrc_mosc(&self) -> bool {
        *self == SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_MOSC
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_PLLSRC` writer - PLL Source"]
pub type SYSCTL_RSCLKCFG_PLLSRC_W<'a> =
    crate::FieldWriter<'a, u32, RSCLKCFG_SPEC, u8, SYSCTL_RSCLKCFG_PLLSRC_A, 4, 24>;
impl<'a> SYSCTL_RSCLKCFG_PLLSRC_W<'a> {
    #[doc = "PIOSC is PLL input clock source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_PIOSC)
    }
    #[doc = "MOSC is the PLL input clock source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_RSCLKCFG_PLLSRC_A::SYSCTL_RSCLKCFG_PLLSRC_MOSC)
    }
}
#[doc = "Field `SYSCTL_RSCLKCFG_USEPLL` reader - Use PLL"]
pub type SYSCTL_RSCLKCFG_USEPLL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RSCLKCFG_USEPLL` writer - Use PLL"]
pub type SYSCTL_RSCLKCFG_USEPLL_W<'a> = crate::BitWriter<'a, u32, RSCLKCFG_SPEC, bool, 28>;
#[doc = "Field `SYSCTL_RSCLKCFG_ACG` reader - Auto Clock Gating"]
pub type SYSCTL_RSCLKCFG_ACG_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RSCLKCFG_ACG` writer - Auto Clock Gating"]
pub type SYSCTL_RSCLKCFG_ACG_W<'a> = crate::BitWriter<'a, u32, RSCLKCFG_SPEC, bool, 29>;
#[doc = "Field `SYSCTL_RSCLKCFG_NEWFREQ` reader - New PLLFREQ Accept"]
pub type SYSCTL_RSCLKCFG_NEWFREQ_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RSCLKCFG_NEWFREQ` writer - New PLLFREQ Accept"]
pub type SYSCTL_RSCLKCFG_NEWFREQ_W<'a> = crate::BitWriter<'a, u32, RSCLKCFG_SPEC, bool, 30>;
#[doc = "Field `SYSCTL_RSCLKCFG_MEMTIMU` reader - Memory Timing Register Update"]
pub type SYSCTL_RSCLKCFG_MEMTIMU_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RSCLKCFG_MEMTIMU` writer - Memory Timing Register Update"]
pub type SYSCTL_RSCLKCFG_MEMTIMU_W<'a> = crate::BitWriter<'a, u32, RSCLKCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_psysdiv(&self) -> SYSCTL_RSCLKCFG_PSYSDIV_R {
        SYSCTL_RSCLKCFG_PSYSDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_osysdiv(&self) -> SYSCTL_RSCLKCFG_OSYSDIV_R {
        SYSCTL_RSCLKCFG_OSYSDIV_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc(&self) -> SYSCTL_RSCLKCFG_OSCSRC_R {
        SYSCTL_RSCLKCFG_OSCSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc(&self) -> SYSCTL_RSCLKCFG_PLLSRC_R {
        SYSCTL_RSCLKCFG_PLLSRC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_usepll(&self) -> SYSCTL_RSCLKCFG_USEPLL_R {
        SYSCTL_RSCLKCFG_USEPLL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_acg(&self) -> SYSCTL_RSCLKCFG_ACG_R {
        SYSCTL_RSCLKCFG_ACG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_newfreq(&self) -> SYSCTL_RSCLKCFG_NEWFREQ_R {
        SYSCTL_RSCLKCFG_NEWFREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_memtimu(&self) -> SYSCTL_RSCLKCFG_MEMTIMU_R {
        SYSCTL_RSCLKCFG_MEMTIMU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_psysdiv(&mut self) -> SYSCTL_RSCLKCFG_PSYSDIV_W {
        SYSCTL_RSCLKCFG_PSYSDIV_W::new(self)
    }
    #[doc = "Bits 10:19 - Oscillator System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_osysdiv(&mut self) -> SYSCTL_RSCLKCFG_OSYSDIV_W {
        SYSCTL_RSCLKCFG_OSYSDIV_W::new(self)
    }
    #[doc = "Bits 20:23 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_oscsrc(&mut self) -> SYSCTL_RSCLKCFG_OSCSRC_W {
        SYSCTL_RSCLKCFG_OSCSRC_W::new(self)
    }
    #[doc = "Bits 24:27 - PLL Source"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_pllsrc(&mut self) -> SYSCTL_RSCLKCFG_PLLSRC_W {
        SYSCTL_RSCLKCFG_PLLSRC_W::new(self)
    }
    #[doc = "Bit 28 - Use PLL"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_usepll(&mut self) -> SYSCTL_RSCLKCFG_USEPLL_W {
        SYSCTL_RSCLKCFG_USEPLL_W::new(self)
    }
    #[doc = "Bit 29 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_acg(&mut self) -> SYSCTL_RSCLKCFG_ACG_W {
        SYSCTL_RSCLKCFG_ACG_W::new(self)
    }
    #[doc = "Bit 30 - New PLLFREQ Accept"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_newfreq(&mut self) -> SYSCTL_RSCLKCFG_NEWFREQ_W {
        SYSCTL_RSCLKCFG_NEWFREQ_W::new(self)
    }
    #[doc = "Bit 31 - Memory Timing Register Update"]
    #[inline(always)]
    pub fn sysctl_rsclkcfg_memtimu(&mut self) -> SYSCTL_RSCLKCFG_MEMTIMU_W {
        SYSCTL_RSCLKCFG_MEMTIMU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Run and Sleep Mode Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsclkcfg](index.html) module"]
pub struct RSCLKCFG_SPEC;
impl crate::RegisterSpec for RSCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsclkcfg::R](R) reader structure"]
impl crate::Readable for RSCLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsclkcfg::W](W) writer structure"]
impl crate::Writable for RSCLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSCLKCFG to value 0"]
impl crate::Resettable for RSCLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
