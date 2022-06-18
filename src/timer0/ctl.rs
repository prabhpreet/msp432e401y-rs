#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_CTL_TAEN` reader - GPTM Timer A Enable"]
pub type TIMER_CTL_TAEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TAEN` writer - GPTM Timer A Enable"]
pub type TIMER_CTL_TAEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `TIMER_CTL_TASTALL` reader - GPTM Timer A Stall Enable"]
pub type TIMER_CTL_TASTALL_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TASTALL` writer - GPTM Timer A Stall Enable"]
pub type TIMER_CTL_TASTALL_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "GPTM Timer A Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CTL_TAEVENT_A {
    #[doc = "0: Positive edge"]
    TIMER_CTL_TAEVENT_POS = 0,
    #[doc = "1: Negative edge"]
    TIMER_CTL_TAEVENT_NEG = 1,
    #[doc = "3: Both edges"]
    TIMER_CTL_TAEVENT_BOTH = 3,
}
impl From<TIMER_CTL_TAEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CTL_TAEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_CTL_TAEVENT` reader - GPTM Timer A Event Mode"]
pub type TIMER_CTL_TAEVENT_R = crate::FieldReader<u8, TIMER_CTL_TAEVENT_A>;
impl TIMER_CTL_TAEVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER_CTL_TAEVENT_A> {
        match self.bits {
            0 => Some(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS),
            1 => Some(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG),
            3 => Some(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_pos(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_neg(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TAEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_taevent_both(&self) -> bool {
        *self == TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH
    }
}
#[doc = "Field `TIMER_CTL_TAEVENT` writer - GPTM Timer A Event Mode"]
pub type TIMER_CTL_TAEVENT_W<'a> =
    crate::FieldWriter<'a, u32, CTL_SPEC, u8, TIMER_CTL_TAEVENT_A, 2, 2>;
impl<'a> TIMER_CTL_TAEVENT_W<'a> {
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_taevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_taevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TAEVENT_A::TIMER_CTL_TAEVENT_BOTH)
    }
}
#[doc = "Field `TIMER_CTL_RTCEN` reader - GPTM RTC Stall Enable"]
pub type TIMER_CTL_RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_RTCEN` writer - GPTM RTC Stall Enable"]
pub type TIMER_CTL_RTCEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `TIMER_CTL_TAOTE` reader - GPTM Timer A Output Trigger Enable"]
pub type TIMER_CTL_TAOTE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TAOTE` writer - GPTM Timer A Output Trigger Enable"]
pub type TIMER_CTL_TAOTE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Field `TIMER_CTL_TAPWML` reader - GPTM Timer A PWM Output Level"]
pub type TIMER_CTL_TAPWML_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TAPWML` writer - GPTM Timer A PWM Output Level"]
pub type TIMER_CTL_TAPWML_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `TIMER_CTL_TBEN` reader - GPTM Timer B Enable"]
pub type TIMER_CTL_TBEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TBEN` writer - GPTM Timer B Enable"]
pub type TIMER_CTL_TBEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 8>;
#[doc = "Field `TIMER_CTL_TBSTALL` reader - GPTM Timer B Stall Enable"]
pub type TIMER_CTL_TBSTALL_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TBSTALL` writer - GPTM Timer B Stall Enable"]
pub type TIMER_CTL_TBSTALL_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 9>;
#[doc = "GPTM Timer B Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CTL_TBEVENT_A {
    #[doc = "0: Positive edge"]
    TIMER_CTL_TBEVENT_POS = 0,
    #[doc = "1: Negative edge"]
    TIMER_CTL_TBEVENT_NEG = 1,
    #[doc = "3: Both edges"]
    TIMER_CTL_TBEVENT_BOTH = 3,
}
impl From<TIMER_CTL_TBEVENT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CTL_TBEVENT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_CTL_TBEVENT` reader - GPTM Timer B Event Mode"]
pub type TIMER_CTL_TBEVENT_R = crate::FieldReader<u8, TIMER_CTL_TBEVENT_A>;
impl TIMER_CTL_TBEVENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER_CTL_TBEVENT_A> {
        match self.bits {
            0 => Some(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS),
            1 => Some(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG),
            3 => Some(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_POS`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_pos(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_NEG`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_neg(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG
    }
    #[doc = "Checks if the value of the field is `TIMER_CTL_TBEVENT_BOTH`"]
    #[inline(always)]
    pub fn is_timer_ctl_tbevent_both(&self) -> bool {
        *self == TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH
    }
}
#[doc = "Field `TIMER_CTL_TBEVENT` writer - GPTM Timer B Event Mode"]
pub type TIMER_CTL_TBEVENT_W<'a> =
    crate::FieldWriter<'a, u32, CTL_SPEC, u8, TIMER_CTL_TBEVENT_A, 2, 10>;
impl<'a> TIMER_CTL_TBEVENT_W<'a> {
    #[doc = "Positive edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_pos(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_POS)
    }
    #[doc = "Negative edge"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_neg(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_NEG)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn timer_ctl_tbevent_both(self) -> &'a mut W {
        self.variant(TIMER_CTL_TBEVENT_A::TIMER_CTL_TBEVENT_BOTH)
    }
}
#[doc = "Field `TIMER_CTL_TBOTE` reader - GPTM Timer B Output Trigger Enable"]
pub type TIMER_CTL_TBOTE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TBOTE` writer - GPTM Timer B Output Trigger Enable"]
pub type TIMER_CTL_TBOTE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 13>;
#[doc = "Field `TIMER_CTL_TBPWML` reader - GPTM Timer B PWM Output Level"]
pub type TIMER_CTL_TBPWML_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CTL_TBPWML` writer - GPTM Timer B PWM Output Level"]
pub type TIMER_CTL_TBPWML_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&self) -> TIMER_CTL_TAEN_R {
        TIMER_CTL_TAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&self) -> TIMER_CTL_TASTALL_R {
        TIMER_CTL_TASTALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&self) -> TIMER_CTL_TAEVENT_R {
        TIMER_CTL_TAEVENT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&self) -> TIMER_CTL_RTCEN_R {
        TIMER_CTL_RTCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&self) -> TIMER_CTL_TAOTE_R {
        TIMER_CTL_TAOTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&self) -> TIMER_CTL_TAPWML_R {
        TIMER_CTL_TAPWML_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&self) -> TIMER_CTL_TBEN_R {
        TIMER_CTL_TBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&self) -> TIMER_CTL_TBSTALL_R {
        TIMER_CTL_TBSTALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&self) -> TIMER_CTL_TBEVENT_R {
        TIMER_CTL_TBEVENT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&self) -> TIMER_CTL_TBOTE_R {
        TIMER_CTL_TBOTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&self) -> TIMER_CTL_TBPWML_R {
        TIMER_CTL_TBPWML_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline(always)]
    pub fn timer_ctl_taen(&mut self) -> TIMER_CTL_TAEN_W {
        TIMER_CTL_TAEN_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tastall(&mut self) -> TIMER_CTL_TASTALL_W {
        TIMER_CTL_TASTALL_W::new(self)
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_taevent(&mut self) -> TIMER_CTL_TAEVENT_W {
        TIMER_CTL_TAEVENT_W::new(self)
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_rtcen(&mut self) -> TIMER_CTL_RTCEN_W {
        TIMER_CTL_RTCEN_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_taote(&mut self) -> TIMER_CTL_TAOTE_W {
        TIMER_CTL_TAOTE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tapwml(&mut self) -> TIMER_CTL_TAPWML_W {
        TIMER_CTL_TAPWML_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline(always)]
    pub fn timer_ctl_tben(&mut self) -> TIMER_CTL_TBEN_W {
        TIMER_CTL_TBEN_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbstall(&mut self) -> TIMER_CTL_TBSTALL_W {
        TIMER_CTL_TBSTALL_W::new(self)
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline(always)]
    pub fn timer_ctl_tbevent(&mut self) -> TIMER_CTL_TBEVENT_W {
        TIMER_CTL_TBEVENT_W::new(self)
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline(always)]
    pub fn timer_ctl_tbote(&mut self) -> TIMER_CTL_TBOTE_W {
        TIMER_CTL_TBOTE_W::new(self)
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline(always)]
    pub fn timer_ctl_tbpwml(&mut self) -> TIMER_CTL_TBPWML_W {
        TIMER_CTL_TBPWML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
