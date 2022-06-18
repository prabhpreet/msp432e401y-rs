#[doc = "Register `TAMR` reader"]
pub struct R(crate::R<TAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMR` writer"]
pub struct W(crate::W<TAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMR_SPEC>;
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
impl From<crate::W<TAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPTM Timer A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TAMR_TAMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TAMR_TAMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TAMR_TAMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TAMR_TAMR_CAP = 3,
}
impl From<TIMER_TAMR_TAMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TAMR_TAMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_TAMR_TAMR` reader - GPTM Timer A Mode"]
pub type TIMER_TAMR_TAMR_R = crate::FieldReader<u8, TIMER_TAMR_TAMR_A>;
impl TIMER_TAMR_TAMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER_TAMR_TAMR_A> {
        match self.bits {
            1 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT),
            2 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD),
            3 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_1_SHOT`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_1_shot(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_PERIOD`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_period(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TAMR_CAP`"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_cap(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP
    }
}
#[doc = "Field `TIMER_TAMR_TAMR` writer - GPTM Timer A Mode"]
pub type TIMER_TAMR_TAMR_W<'a> =
    crate::FieldWriter<'a, u32, TAMR_SPEC, u8, TIMER_TAMR_TAMR_A, 2, 0>;
impl<'a> TIMER_TAMR_TAMR_W<'a> {
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_1_shot(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_period(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_cap(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP)
    }
}
#[doc = "Field `TIMER_TAMR_TACMR` reader - GPTM Timer A Capture Mode"]
pub type TIMER_TAMR_TACMR_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TACMR` writer - GPTM Timer A Capture Mode"]
pub type TIMER_TAMR_TACMR_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 2>;
#[doc = "Field `TIMER_TAMR_TAAMS` reader - GPTM Timer A Alternate Mode Select"]
pub type TIMER_TAMR_TAAMS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAAMS` writer - GPTM Timer A Alternate Mode Select"]
pub type TIMER_TAMR_TAAMS_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 3>;
#[doc = "Field `TIMER_TAMR_TACDIR` reader - GPTM Timer A Count Direction"]
pub type TIMER_TAMR_TACDIR_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TACDIR` writer - GPTM Timer A Count Direction"]
pub type TIMER_TAMR_TACDIR_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 4>;
#[doc = "Field `TIMER_TAMR_TAMIE` reader - GPTM Timer A Match Interrupt Enable"]
pub type TIMER_TAMR_TAMIE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAMIE` writer - GPTM Timer A Match Interrupt Enable"]
pub type TIMER_TAMR_TAMIE_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 5>;
#[doc = "Field `TIMER_TAMR_TAWOT` reader - GPTM Timer A Wait-on-Trigger"]
pub type TIMER_TAMR_TAWOT_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAWOT` writer - GPTM Timer A Wait-on-Trigger"]
pub type TIMER_TAMR_TAWOT_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 6>;
#[doc = "Field `TIMER_TAMR_TASNAPS` reader - GPTM Timer A Snap-Shot Mode"]
pub type TIMER_TAMR_TASNAPS_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TASNAPS` writer - GPTM Timer A Snap-Shot Mode"]
pub type TIMER_TAMR_TASNAPS_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 7>;
#[doc = "Field `TIMER_TAMR_TAILD` reader - GPTM Timer A Interval Load Write"]
pub type TIMER_TAMR_TAILD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAILD` writer - GPTM Timer A Interval Load Write"]
pub type TIMER_TAMR_TAILD_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 8>;
#[doc = "Field `TIMER_TAMR_TAPWMIE` reader - GPTM Timer A PWM Interrupt Enable"]
pub type TIMER_TAMR_TAPWMIE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAPWMIE` writer - GPTM Timer A PWM Interrupt Enable"]
pub type TIMER_TAMR_TAPWMIE_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 9>;
#[doc = "Field `TIMER_TAMR_TAMRSU` reader - GPTM Timer A Match Register Update"]
pub type TIMER_TAMR_TAMRSU_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAMRSU` writer - GPTM Timer A Match Register Update"]
pub type TIMER_TAMR_TAMRSU_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 10>;
#[doc = "Field `TIMER_TAMR_TAPLO` reader - GPTM Timer A PWM Legacy Operation"]
pub type TIMER_TAMR_TAPLO_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TAPLO` writer - GPTM Timer A PWM Legacy Operation"]
pub type TIMER_TAMR_TAPLO_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 11>;
#[doc = "Field `TIMER_TAMR_TACINTD` reader - One-shot/Periodic Interrupt Disable"]
pub type TIMER_TAMR_TACINTD_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_TAMR_TACINTD` writer - One-shot/Periodic Interrupt Disable"]
pub type TIMER_TAMR_TACINTD_W<'a> = crate::BitWriter<'a, u32, TAMR_SPEC, bool, 12>;
#[doc = "Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_TAMR_TCACT_A {
    #[doc = "0: Disable compare operations"]
    TIMER_TAMR_TCACT_NONE = 0,
    #[doc = "1: Toggle State on Time-Out"]
    TIMER_TAMR_TCACT_TOGGLE = 1,
    #[doc = "2: Clear CCP on Time-Out"]
    TIMER_TAMR_TCACT_CLRTO = 2,
    #[doc = "3: Set CCP on Time-Out"]
    TIMER_TAMR_TCACT_SETTO = 3,
    #[doc = "4: Set CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_SETTOGTO = 4,
    #[doc = "5: Clear CCP immediately and toggle on Time-Out"]
    TIMER_TAMR_TCACT_CLRTOGTO = 5,
    #[doc = "6: Set CCP immediately and clear on Time-Out"]
    TIMER_TAMR_TCACT_SETCLRTO = 6,
    #[doc = "7: Clear CCP immediately and set on Time-Out"]
    TIMER_TAMR_TCACT_CLRSETTO = 7,
}
impl From<TIMER_TAMR_TCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TAMR_TCACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_TAMR_TCACT` reader - Timer Compare Action Select"]
pub type TIMER_TAMR_TCACT_R = crate::FieldReader<u8, TIMER_TAMR_TCACT_A>;
impl TIMER_TAMR_TCACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_TAMR_TCACT_A {
        match self.bits {
            0 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_NONE,
            1 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_TOGGLE,
            2 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTO,
            3 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTO,
            4 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTOGTO,
            5 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTOGTO,
            6 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETCLRTO,
            7 => TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_NONE`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_none(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_NONE
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_TOGGLE`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_toggle(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_TOGGLE
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_setto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_settogto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRTOGTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrtogto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_SETCLRTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_setclrto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETCLRTO
    }
    #[doc = "Checks if the value of the field is `TIMER_TAMR_TCACT_CLRSETTO`"]
    #[inline(always)]
    pub fn is_timer_tamr_tcact_clrsetto(&self) -> bool {
        *self == TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRSETTO
    }
}
#[doc = "Field `TIMER_TAMR_TCACT` writer - Timer Compare Action Select"]
pub type TIMER_TAMR_TCACT_W<'a> =
    crate::FieldWriterSafe<'a, u32, TAMR_SPEC, u8, TIMER_TAMR_TCACT_A, 3, 13>;
impl<'a> TIMER_TAMR_TCACT_W<'a> {
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn timer_tamr_tcact_none(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_toggle(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_setto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_settogto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrtogto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_setclrto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline(always)]
    pub fn timer_tamr_tcact_clrsetto(self) -> &'a mut W {
        self.variant(TIMER_TAMR_TCACT_A::TIMER_TAMR_TCACT_CLRSETTO)
    }
}
impl R {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&self) -> TIMER_TAMR_TAMR_R {
        TIMER_TAMR_TAMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&self) -> TIMER_TAMR_TACMR_R {
        TIMER_TAMR_TACMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&self) -> TIMER_TAMR_TAAMS_R {
        TIMER_TAMR_TAAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn timer_tamr_tacdir(&self) -> TIMER_TAMR_TACDIR_R {
        TIMER_TAMR_TACDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tamie(&self) -> TIMER_TAMR_TAMIE_R {
        TIMER_TAMR_TAMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tamr_tawot(&self) -> TIMER_TAMR_TAWOT_R {
        TIMER_TAMR_TAWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tamr_tasnaps(&self) -> TIMER_TAMR_TASNAPS_R {
        TIMER_TAMR_TASNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn timer_tamr_taild(&self) -> TIMER_TAMR_TAILD_R {
        TIMER_TAMR_TAILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tapwmie(&self) -> TIMER_TAMR_TAPWMIE_R {
        TIMER_TAMR_TAPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn timer_tamr_tamrsu(&self) -> TIMER_TAMR_TAMRSU_R {
        TIMER_TAMR_TAMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tamr_taplo(&self) -> TIMER_TAMR_TAPLO_R {
        TIMER_TAMR_TAPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tamr_tacintd(&self) -> TIMER_TAMR_TACINTD_R {
        TIMER_TAMR_TACINTD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tamr_tcact(&self) -> TIMER_TAMR_TCACT_R {
        TIMER_TAMR_TCACT_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&mut self) -> TIMER_TAMR_TAMR_W {
        TIMER_TAMR_TAMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&mut self) -> TIMER_TAMR_TACMR_W {
        TIMER_TAMR_TACMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&mut self) -> TIMER_TAMR_TAAMS_W {
        TIMER_TAMR_TAAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn timer_tamr_tacdir(&mut self) -> TIMER_TAMR_TACDIR_W {
        TIMER_TAMR_TACDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tamie(&mut self) -> TIMER_TAMR_TAMIE_W {
        TIMER_TAMR_TAMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tamr_tawot(&mut self) -> TIMER_TAMR_TAWOT_W {
        TIMER_TAMR_TAWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tamr_tasnaps(&mut self) -> TIMER_TAMR_TASNAPS_W {
        TIMER_TAMR_TASNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn timer_tamr_taild(&mut self) -> TIMER_TAMR_TAILD_W {
        TIMER_TAMR_TAILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tapwmie(&mut self) -> TIMER_TAMR_TAPWMIE_W {
        TIMER_TAMR_TAPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn timer_tamr_tamrsu(&mut self) -> TIMER_TAMR_TAMRSU_W {
        TIMER_TAMR_TAMRSU_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tamr_taplo(&mut self) -> TIMER_TAMR_TAPLO_W {
        TIMER_TAMR_TAPLO_W::new(self)
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn timer_tamr_tacintd(&mut self) -> TIMER_TAMR_TACINTD_W {
        TIMER_TAMR_TACINTD_W::new(self)
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline(always)]
    pub fn timer_tamr_tcact(&mut self) -> TIMER_TAMR_TCACT_W {
        TIMER_TAMR_TCACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer A Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamr](index.html) module"]
pub struct TAMR_SPEC;
impl crate::RegisterSpec for TAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamr::R](R) reader structure"]
impl crate::Readable for TAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamr::W](W) writer structure"]
impl crate::Writable for TAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMR to value 0"]
impl crate::Resettable for TAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
