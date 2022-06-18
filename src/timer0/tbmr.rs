#[doc = "Register `TBMR` reader"]
pub struct R(crate::R<TBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBMR` writer"]
pub struct W(crate::W<TBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBMR_SPEC>;
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
impl From<crate::W<TBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPTM Timer B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TBMR_TBMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TBMR_TBMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TBMR_TBMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TBMR_TBMR_CAP = 3,
}
impl From<TIMER_TBMR_TBMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TBMR_TBMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_TBMR_TBMR` reader - GPTM Timer B Mode"]
pub type TIMER_TBMR_TBMR_R = crate::FieldReader<u8, TIMER_TBMR_TBMR_A>;
impl TIMER_TBMR_TBMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER_TBMR_TBMR_A> {
        match self.bits {
            1 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT),
            2 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD),
            3 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_1_shot(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_period(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TBMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_cap(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP
    }
}
#[doc = "Field `TIMER_TBMR_TBMR` writer - GPTM Timer B Mode"]
pub type TIMER_TBMR_TBMR_W<'a> =
    crate::FieldWriter<'a, u32, TBMR_SPEC, u8, TIMER_TBMR_TBMR_A, 2, 0>;
impl<'a> TIMER_TBMR_TBMR_W<'a> {
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_period(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_cap(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP)
    }
}
#[doc = "Field `TIMER_TBMR_TBCMR` reader - GPTM Timer B Capture Mode"]
pub type TIMER_TBMR_TBCMR_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBCMR` writer - GPTM Timer B Capture Mode"]
pub type TIMER_TBMR_TBCMR_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 2>;
#[doc = "Field `TIMER_TBMR_TBAMS` reader - GPTM Timer B Alternate Mode Select"]
pub type TIMER_TBMR_TBAMS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBAMS` writer - GPTM Timer B Alternate Mode Select"]
pub type TIMER_TBMR_TBAMS_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 3>;
#[doc = "Field `TIMER_TBMR_TBCDIR` reader - GPTM Timer B Count Direction"]
pub type TIMER_TBMR_TBCDIR_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBCDIR` writer - GPTM Timer B Count Direction"]
pub type TIMER_TBMR_TBCDIR_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 4>;
#[doc = "Field `TIMER_TBMR_TBMIE` reader - GPTM Timer B Match Interrupt Enable"]
pub type TIMER_TBMR_TBMIE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBMIE` writer - GPTM Timer B Match Interrupt Enable"]
pub type TIMER_TBMR_TBMIE_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 5>;
#[doc = "Field `TIMER_TBMR_TBWOT` reader - GPTM Timer B Wait-on-Trigger"]
pub type TIMER_TBMR_TBWOT_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBWOT` writer - GPTM Timer B Wait-on-Trigger"]
pub type TIMER_TBMR_TBWOT_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 6>;
#[doc = "Field `TIMER_TBMR_TBSNAPS` reader - GPTM Timer B Snap-Shot Mode"]
pub type TIMER_TBMR_TBSNAPS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBSNAPS` writer - GPTM Timer B Snap-Shot Mode"]
pub type TIMER_TBMR_TBSNAPS_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 7>;
#[doc = "Field `TIMER_TBMR_TBILD` reader - GPTM Timer B Interval Load Write"]
pub type TIMER_TBMR_TBILD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBILD` writer - GPTM Timer B Interval Load Write"]
pub type TIMER_TBMR_TBILD_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 8>;
#[doc = "Field `TIMER_TBMR_TBPWMIE` reader - GPTM Timer B PWM Interrupt Enable"]
pub type TIMER_TBMR_TBPWMIE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBPWMIE` writer - GPTM Timer B PWM Interrupt Enable"]
pub type TIMER_TBMR_TBPWMIE_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 9>;
#[doc = "Field `TIMER_TBMR_TBMRSU` reader - GPTM Timer B Match Register Update"]
pub type TIMER_TBMR_TBMRSU_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBMRSU` writer - GPTM Timer B Match Register Update"]
pub type TIMER_TBMR_TBMRSU_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 10>;
#[doc = "Field `TIMER_TBMR_TBPLO` reader - GPTM Timer B PWM Legacy Operation"]
pub type TIMER_TBMR_TBPLO_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBPLO` writer - GPTM Timer B PWM Legacy Operation"]
pub type TIMER_TBMR_TBPLO_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 11>;
#[doc = "Field `TIMER_TBMR_TBCINTD` reader - One-Shot/Periodic Interrupt Disable"]
pub type TIMER_TBMR_TBCINTD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TBMR_TBCINTD` writer - One-Shot/Periodic Interrupt Disable"]
pub type TIMER_TBMR_TBCINTD_W<'a> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, 12>;
#[doc = "Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TBMR_TCACT_A {
    #[doc = "0: Disable compare operations"]
    TIMER_TBMR_TCACT_NONE = 0,
    #[doc = "1: Toggle State on Time-Out"]
    TIMER_TBMR_TCACT_TOGGLE = 1,
    #[doc = "2: Clear CCP on Time-Out"]
    TIMER_TBMR_TCACT_CLRTO = 2,
    #[doc = "3: Set CCP on Time-Out"]
    TIMER_TBMR_TCACT_SETTO = 3,
    #[doc = "4: Set CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_SETTOGTO = 4,
    #[doc = "5: Clear CCP immediately and toggle on Time-Out"]
    TIMER_TBMR_TCACT_CLRTOGTO = 5,
    #[doc = "6: Set CCP immediately and clear on Time-Out"]
    TIMER_TBMR_TCACT_SETCLRTO = 6,
    #[doc = "7: Clear CCP immediately and set on Time-Out"]
    TIMER_TBMR_TCACT_CLRSETTO = 7,
}
impl From<TIMER_TBMR_TCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TBMR_TCACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_TBMR_TCACT` reader - Timer Compare Action Select"]
pub type TIMER_TBMR_TCACT_R = crate::FieldReader<u8, TIMER_TBMR_TCACT_A>;
impl TIMER_TBMR_TCACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_TBMR_TCACT_A {
        match self.bits {
            0 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_NONE,
            1 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_TOGGLE,
            2 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTO,
            3 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTO,
            4 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTOGTO,
            5 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTOGTO,
            6 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETCLRTO,
            7 => TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_NONE`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_none(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_TOGGLE`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_toggle(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_TOGGLE
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_setto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_settogto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrtogto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_SETCLRTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_setclrto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETCLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TBMR_TCACT_CLRSETTO`"]
    #[inline(always)]
    pub fn is_timer_tbmr_tcact_clrsetto(&self) -> bool {
        *self == TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRSETTO
    }
}
#[doc = "Field `TIMER_TBMR_TCACT` writer - Timer Compare Action Select"]
pub type TIMER_TBMR_TCACT_W<'a> =
    crate::FieldWriterSafe<'a, u32, TBMR_SPEC, u8, TIMER_TBMR_TCACT_A, 3, 13>;
impl<'a> TIMER_TBMR_TCACT_W<'a> {
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_none(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_toggle(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_setto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_settogto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrtogto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_setclrto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline(always)]
    pub fn timer_tbmr_tcact_clrsetto(self) -> &'a mut W {
        self.variant(TIMER_TBMR_TCACT_A::TIMER_TBMR_TCACT_CLRSETTO)
    }
}
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&self) -> TIMER_TBMR_TBMR_R {
        TIMER_TBMR_TBMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&self) -> TIMER_TBMR_TBCMR_R {
        TIMER_TBMR_TBCMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&self) -> TIMER_TBMR_TBAMS_R {
        TIMER_TBMR_TBAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn timer_tbmr_tbcdir(&self) -> TIMER_TBMR_TBCDIR_R {
        TIMER_TBMR_TBCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbmie(&self) -> TIMER_TBMR_TBMIE_R {
        TIMER_TBMR_TBMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tbmr_tbwot(&self) -> TIMER_TBMR_TBWOT_R {
        TIMER_TBMR_TBWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbsnaps(&self) -> TIMER_TBMR_TBSNAPS_R {
        TIMER_TBMR_TBSNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn timer_tbmr_tbild(&self) -> TIMER_TBMR_TBILD_R {
        TIMER_TBMR_TBILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbpwmie(&self) -> TIMER_TBMR_TBPWMIE_R {
        TIMER_TBMR_TBPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn timer_tbmr_tbmrsu(&self) -> TIMER_TBMR_TBMRSU_R {
        TIMER_TBMR_TBMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tbmr_tbplo(&self) -> TIMER_TBMR_TBPLO_R {
        TIMER_TBMR_TBPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tbmr_tbcintd(&self) -> TIMER_TBMR_TBCINTD_R {
        TIMER_TBMR_TBCINTD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tbmr_tcact(&self) -> TIMER_TBMR_TCACT_R {
        TIMER_TBMR_TCACT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&mut self) -> TIMER_TBMR_TBMR_W {
        TIMER_TBMR_TBMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&mut self) -> TIMER_TBMR_TBCMR_W {
        TIMER_TBMR_TBCMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&mut self) -> TIMER_TBMR_TBAMS_W {
        TIMER_TBMR_TBAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn timer_tbmr_tbcdir(&mut self) -> TIMER_TBMR_TBCDIR_W {
        TIMER_TBMR_TBCDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbmie(&mut self) -> TIMER_TBMR_TBMIE_W {
        TIMER_TBMR_TBMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tbmr_tbwot(&mut self) -> TIMER_TBMR_TBWOT_W {
        TIMER_TBMR_TBWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbsnaps(&mut self) -> TIMER_TBMR_TBSNAPS_W {
        TIMER_TBMR_TBSNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn timer_tbmr_tbild(&mut self) -> TIMER_TBMR_TBILD_W {
        TIMER_TBMR_TBILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbpwmie(&mut self) -> TIMER_TBMR_TBPWMIE_W {
        TIMER_TBMR_TBPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn timer_tbmr_tbmrsu(&mut self) -> TIMER_TBMR_TBMRSU_W {
        TIMER_TBMR_TBMRSU_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tbmr_tbplo(&mut self) -> TIMER_TBMR_TBPLO_W {
        TIMER_TBMR_TBPLO_W::new(self)
    }
    #[doc = "Bit 12 - One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tbmr_tbcintd(&mut self) -> TIMER_TBMR_TBCINTD_W {
        TIMER_TBMR_TBCINTD_W::new(self)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tbmr_tcact(&mut self) -> TIMER_TBMR_TCACT_W {
        TIMER_TBMR_TCACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmr](index.html) module"]
pub struct TBMR_SPEC;
impl crate::RegisterSpec for TBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbmr::R](R) reader structure"]
impl crate::Readable for TBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbmr::W](W) writer structure"]
impl crate::Writable for TBMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TBMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
