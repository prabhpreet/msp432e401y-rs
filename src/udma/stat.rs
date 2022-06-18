#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_STAT_MASTEN` reader - Master Enable Status"]
pub type UDMA_STAT_MASTEN_R = crate::BitReader<bool>;
#[doc = "Field `UDMA_STAT_MASTEN` writer - Master Enable Status"]
pub type UDMA_STAT_MASTEN_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 0>;
#[doc = "Control State Machine Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UDMA_STAT_STATE_A {
    #[doc = "0: Idle"]
    UDMA_STAT_STATE_IDLE = 0,
    #[doc = "1: Reading channel controller data"]
    UDMA_STAT_STATE_RD_CTRL = 1,
    #[doc = "2: Reading source end pointer"]
    UDMA_STAT_STATE_RD_SRCENDP = 2,
    #[doc = "3: Reading destination end pointer"]
    UDMA_STAT_STATE_RD_DSTENDP = 3,
    #[doc = "4: Reading source data"]
    UDMA_STAT_STATE_RD_SRCDAT = 4,
    #[doc = "5: Writing destination data"]
    UDMA_STAT_STATE_WR_DSTDAT = 5,
    #[doc = "6: Waiting for uDMA request to clear"]
    UDMA_STAT_STATE_WAIT = 6,
    #[doc = "7: Writing channel controller data"]
    UDMA_STAT_STATE_WR_CTRL = 7,
    #[doc = "8: Stalled"]
    UDMA_STAT_STATE_STALL = 8,
    #[doc = "9: Done"]
    UDMA_STAT_STATE_DONE = 9,
    #[doc = "10: Undefined"]
    UDMA_STAT_STATE_UNDEF = 10,
}
impl From<UDMA_STAT_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: UDMA_STAT_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UDMA_STAT_STATE` reader - Control State Machine Status"]
pub type UDMA_STAT_STATE_R = crate::FieldReader<u8, UDMA_STAT_STATE_A>;
impl UDMA_STAT_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UDMA_STAT_STATE_A> {
        match self.bits {
            0 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE),
            1 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL),
            2 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP),
            3 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP),
            4 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT),
            5 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT),
            6 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT),
            7 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL),
            8 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL),
            9 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE),
            10 => Some(UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_IDLE`"]
    #[inline(always)]
    pub fn is_udma_stat_state_idle(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_CTRL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_SRCENDP`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcendp(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_DSTENDP`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_dstendp(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_RD_SRCDAT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_rd_srcdat(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WR_DSTDAT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_dstdat(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WAIT`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wait(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_WR_CTRL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_wr_ctrl(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_STALL`"]
    #[inline(always)]
    pub fn is_udma_stat_state_stall(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_DONE`"]
    #[inline(always)]
    pub fn is_udma_stat_state_done(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE
    }
    #[doc = "Checks if the value of the field is `UDMA_STAT_STATE_UNDEF`"]
    #[inline(always)]
    pub fn is_udma_stat_state_undef(&self) -> bool {
        *self == UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF
    }
}
#[doc = "Field `UDMA_STAT_STATE` writer - Control State Machine Status"]
pub type UDMA_STAT_STATE_W<'a> =
    crate::FieldWriter<'a, u32, STAT_SPEC, u8, UDMA_STAT_STATE_A, 4, 4>;
impl<'a> UDMA_STAT_STATE_W<'a> {
    #[doc = "Idle"]
    #[inline(always)]
    pub fn udma_stat_state_idle(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_IDLE)
    }
    #[doc = "Reading channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_ctrl(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_CTRL)
    }
    #[doc = "Reading source end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcendp(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCENDP)
    }
    #[doc = "Reading destination end pointer"]
    #[inline(always)]
    pub fn udma_stat_state_rd_dstendp(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_DSTENDP)
    }
    #[doc = "Reading source data"]
    #[inline(always)]
    pub fn udma_stat_state_rd_srcdat(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_RD_SRCDAT)
    }
    #[doc = "Writing destination data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_dstdat(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_DSTDAT)
    }
    #[doc = "Waiting for uDMA request to clear"]
    #[inline(always)]
    pub fn udma_stat_state_wait(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WAIT)
    }
    #[doc = "Writing channel controller data"]
    #[inline(always)]
    pub fn udma_stat_state_wr_ctrl(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_WR_CTRL)
    }
    #[doc = "Stalled"]
    #[inline(always)]
    pub fn udma_stat_state_stall(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_STALL)
    }
    #[doc = "Done"]
    #[inline(always)]
    pub fn udma_stat_state_done(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_DONE)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn udma_stat_state_undef(self) -> &'a mut W {
        self.variant(UDMA_STAT_STATE_A::UDMA_STAT_STATE_UNDEF)
    }
}
#[doc = "Field `UDMA_STAT_DMACHANS` reader - Available uDMA Channels Minus 1"]
pub type UDMA_STAT_DMACHANS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UDMA_STAT_DMACHANS` writer - Available uDMA Channels Minus 1"]
pub type UDMA_STAT_DMACHANS_W<'a> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, u8, 5, 16>;
impl R {
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn udma_stat_masten(&self) -> UDMA_STAT_MASTEN_R {
        UDMA_STAT_MASTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn udma_stat_state(&self) -> UDMA_STAT_STATE_R {
        UDMA_STAT_STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn udma_stat_dmachans(&self) -> UDMA_STAT_DMACHANS_R {
        UDMA_STAT_DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable Status"]
    #[inline(always)]
    pub fn udma_stat_masten(&mut self) -> UDMA_STAT_MASTEN_W {
        UDMA_STAT_MASTEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Control State Machine Status"]
    #[inline(always)]
    pub fn udma_stat_state(&mut self) -> UDMA_STAT_STATE_W {
        UDMA_STAT_STATE_W::new(self)
    }
    #[doc = "Bits 16:20 - Available uDMA Channels Minus 1"]
    #[inline(always)]
    pub fn udma_stat_dmachans(&mut self) -> UDMA_STAT_DMACHANS_W {
        UDMA_STAT_DMACHANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
