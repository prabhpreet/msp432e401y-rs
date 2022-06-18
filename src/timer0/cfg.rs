#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPTM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMER_CFG_A {
    #[doc = "0: For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    TIMER_CFG_32_BIT_TIMER = 0,
    #[doc = "1: For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    TIMER_CFG_32_BIT_RTC = 1,
    #[doc = "4: For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    TIMER_CFG_16_BIT = 4,
}
impl From<TIMER_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMER_CFG` reader - GPTM Configuration"]
pub type TIMER_CFG_R = crate::FieldReader<u8, TIMER_CFG_A>;
impl TIMER_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMER_CFG_A> {
        match self.bits {
            0 => Some(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER),
            1 => Some(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC),
            4 => Some(TIMER_CFG_A::TIMER_CFG_16_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_TIMER`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_timer(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_32_BIT_RTC`"]
    #[inline(always)]
    pub fn is_timer_cfg_32_bit_rtc(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_32_BIT_RTC
    }
    #[doc = "Checks if the value of the field is `TIMER_CFG_16_BIT`"]
    #[inline(always)]
    pub fn is_timer_cfg_16_bit(&self) -> bool {
        *self == TIMER_CFG_A::TIMER_CFG_16_BIT
    }
}
#[doc = "Field `TIMER_CFG` writer - GPTM Configuration"]
pub type TIMER_CFG_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, TIMER_CFG_A, 3, 0>;
impl<'a> TIMER_CFG_W<'a> {
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_timer(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_TIMER)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 32-bit real-time clock (RTC) counter configuration"]
    #[inline(always)]
    pub fn timer_cfg_32_bit_rtc(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_32_BIT_RTC)
    }
    #[doc = "For a 16/32-bit timer, this value selects the 16-bit timer configuration"]
    #[inline(always)]
    pub fn timer_cfg_16_bit(self) -> &'a mut W {
        self.variant(TIMER_CFG_A::TIMER_CFG_16_BIT)
    }
}
impl R {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&self) -> TIMER_CFG_R {
        TIMER_CFG_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPTM Configuration"]
    #[inline(always)]
    pub fn timer_cfg(&mut self) -> TIMER_CFG_W {
        TIMER_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
