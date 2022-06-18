#[doc = "Register `DSCLKCFG` reader"]
pub struct R(crate::R<DSCLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCLKCFG` writer"]
pub struct W(crate::W<DSCLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCLKCFG_SPEC>;
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
impl From<crate::W<DSCLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DSCLKCFG_DSSYSDIV` reader - Deep Sleep Clock Divisor"]
pub type SYSCTL_DSCLKCFG_DSSYSDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCTL_DSCLKCFG_DSSYSDIV` writer - Deep Sleep Clock Divisor"]
pub type SYSCTL_DSCLKCFG_DSSYSDIV_W<'a> =
    crate::FieldWriter<'a, u32, DSCLKCFG_SPEC, u16, u16, 10, 0>;
#[doc = "Deep Sleep Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DSCLKCFG_DSOSCSRC_A {
    #[doc = "0: PIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC = 0,
    #[doc = "2: LFIOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC = 2,
    #[doc = "3: MOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_MOSC = 3,
    #[doc = "4: Hibernation Module RTCOSC"]
    SYSCTL_DSCLKCFG_DSOSCSRC_RTC = 4,
}
impl From<SYSCTL_DSCLKCFG_DSOSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSCLKCFG_DSOSCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DSCLKCFG_DSOSCSRC` reader - Deep Sleep Oscillator Source"]
pub type SYSCTL_DSCLKCFG_DSOSCSRC_R = crate::FieldReader<u8, SYSCTL_DSCLKCFG_DSOSCSRC_A>;
impl SYSCTL_DSCLKCFG_DSOSCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DSCLKCFG_DSOSCSRC_A> {
        match self.bits {
            0 => Some(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC),
            2 => Some(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC),
            3 => Some(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC),
            4 => Some(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_RTC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_piosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_lfiosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_MOSC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_mosc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSCLKCFG_DSOSCSRC_RTC`"]
    #[inline(always)]
    pub fn is_sysctl_dsclkcfg_dsoscsrc_rtc(&self) -> bool {
        *self == SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_RTC
    }
}
#[doc = "Field `SYSCTL_DSCLKCFG_DSOSCSRC` writer - Deep Sleep Oscillator Source"]
pub type SYSCTL_DSCLKCFG_DSOSCSRC_W<'a> =
    crate::FieldWriter<'a, u32, DSCLKCFG_SPEC, u8, SYSCTL_DSCLKCFG_DSOSCSRC_A, 4, 20>;
impl<'a> SYSCTL_DSCLKCFG_DSOSCSRC_W<'a> {
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_PIOSC)
    }
    #[doc = "LFIOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_LFIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_mosc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_MOSC)
    }
    #[doc = "Hibernation Module RTCOSC"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc_rtc(self) -> &'a mut W {
        self.variant(SYSCTL_DSCLKCFG_DSOSCSRC_A::SYSCTL_DSCLKCFG_DSOSCSRC_RTC)
    }
}
#[doc = "Field `SYSCTL_DSCLKCFG_MOSCDPD` reader - MOSC Disable Power Down"]
pub type SYSCTL_DSCLKCFG_MOSCDPD_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DSCLKCFG_MOSCDPD` writer - MOSC Disable Power Down"]
pub type SYSCTL_DSCLKCFG_MOSCDPD_W<'a> = crate::BitWriter<'a, u32, DSCLKCFG_SPEC, bool, 30>;
#[doc = "Field `SYSCTL_DSCLKCFG_PIOSCPD` reader - PIOSC Power Down"]
pub type SYSCTL_DSCLKCFG_PIOSCPD_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DSCLKCFG_PIOSCPD` writer - PIOSC Power Down"]
pub type SYSCTL_DSCLKCFG_PIOSCPD_W<'a> = crate::BitWriter<'a, u32, DSCLKCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dssysdiv(&self) -> SYSCTL_DSCLKCFG_DSSYSDIV_R {
        SYSCTL_DSCLKCFG_DSSYSDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc(&self) -> SYSCTL_DSCLKCFG_DSOSCSRC_R {
        SYSCTL_DSCLKCFG_DSOSCSRC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_moscdpd(&self) -> SYSCTL_DSCLKCFG_MOSCDPD_R {
        SYSCTL_DSCLKCFG_MOSCDPD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_pioscpd(&self) -> SYSCTL_DSCLKCFG_PIOSCPD_R {
        SYSCTL_DSCLKCFG_PIOSCPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Deep Sleep Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dssysdiv(&mut self) -> SYSCTL_DSCLKCFG_DSSYSDIV_W {
        SYSCTL_DSCLKCFG_DSSYSDIV_W::new(self)
    }
    #[doc = "Bits 20:23 - Deep Sleep Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_dsoscsrc(&mut self) -> SYSCTL_DSCLKCFG_DSOSCSRC_W {
        SYSCTL_DSCLKCFG_DSOSCSRC_W::new(self)
    }
    #[doc = "Bit 30 - MOSC Disable Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_moscdpd(&mut self) -> SYSCTL_DSCLKCFG_MOSCDPD_W {
        SYSCTL_DSCLKCFG_MOSCDPD_W::new(self)
    }
    #[doc = "Bit 31 - PIOSC Power Down"]
    #[inline(always)]
    pub fn sysctl_dsclkcfg_pioscpd(&mut self) -> SYSCTL_DSCLKCFG_PIOSCPD_W {
        SYSCTL_DSCLKCFG_PIOSCPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Sleep Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsclkcfg](index.html) module"]
pub struct DSCLKCFG_SPEC;
impl crate::RegisterSpec for DSCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsclkcfg::R](R) reader structure"]
impl crate::Readable for DSCLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsclkcfg::W](W) writer structure"]
impl crate::Writable for DSCLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCLKCFG to value 0"]
impl crate::Resettable for DSCLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
