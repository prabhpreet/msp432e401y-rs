#[doc = "Register `ALTCLKCFG` reader"]
pub struct R(crate::R<ALTCLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTCLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTCLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTCLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTCLKCFG` writer"]
pub struct W(crate::W<ALTCLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTCLKCFG_SPEC>;
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
impl From<crate::W<ALTCLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTCLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alternate Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_ALTCLKCFG_ALTCLK_A {
    #[doc = "0: PIOSC"]
    SYSCTL_ALTCLKCFG_ALTCLK_PIOSC = 0,
    #[doc = "3: Hibernation Module Real-time clock output (RTCOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC = 3,
    #[doc = "4: Low-frequency internal oscillator (LFIOSC)"]
    SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC = 4,
}
impl From<SYSCTL_ALTCLKCFG_ALTCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_ALTCLKCFG_ALTCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_ALTCLKCFG_ALTCLK` reader - Alternate Clock Source"]
pub type SYSCTL_ALTCLKCFG_ALTCLK_R = crate::FieldReader<u8, SYSCTL_ALTCLKCFG_ALTCLK_A>;
impl SYSCTL_ALTCLKCFG_ALTCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_ALTCLKCFG_ALTCLK_A> {
        match self.bits {
            0 => Some(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC),
            3 => Some(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC),
            4 => Some(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_PIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_piosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_rtcosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC
    }
    #[doc = "Checks if the value of the field is `SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC`"]
    #[inline(always)]
    pub fn is_sysctl_altclkcfg_altclk_lfiosc(&self) -> bool {
        *self == SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC
    }
}
#[doc = "Field `SYSCTL_ALTCLKCFG_ALTCLK` writer - Alternate Clock Source"]
pub type SYSCTL_ALTCLKCFG_ALTCLK_W<'a> =
    crate::FieldWriter<'a, u32, ALTCLKCFG_SPEC, u8, SYSCTL_ALTCLKCFG_ALTCLK_A, 4, 0>;
impl<'a> SYSCTL_ALTCLKCFG_ALTCLK_W<'a> {
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_piosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_PIOSC)
    }
    #[doc = "Hibernation Module Real-time clock output (RTCOSC)"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_rtcosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_RTCOSC)
    }
    #[doc = "Low-frequency internal oscillator (LFIOSC)"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk_lfiosc(self) -> &'a mut W {
        self.variant(SYSCTL_ALTCLKCFG_ALTCLK_A::SYSCTL_ALTCLKCFG_ALTCLK_LFIOSC)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk(&self) -> SYSCTL_ALTCLKCFG_ALTCLK_R {
        SYSCTL_ALTCLKCFG_ALTCLK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate Clock Source"]
    #[inline(always)]
    pub fn sysctl_altclkcfg_altclk(&mut self) -> SYSCTL_ALTCLKCFG_ALTCLK_W {
        SYSCTL_ALTCLKCFG_ALTCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altclkcfg](index.html) module"]
pub struct ALTCLKCFG_SPEC;
impl crate::RegisterSpec for ALTCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altclkcfg::R](R) reader structure"]
impl crate::Readable for ALTCLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altclkcfg::W](W) writer structure"]
impl crate::Writable for ALTCLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTCLKCFG to value 0"]
impl crate::Resettable for ALTCLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
