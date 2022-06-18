#[doc = "Register `FLOWCTL` reader"]
pub struct R(crate::R<FLOWCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOWCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOWCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOWCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOWCTL` writer"]
pub struct W(crate::W<FLOWCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOWCTL_SPEC>;
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
impl From<crate::W<FLOWCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOWCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_FLOWCTL_FCBBPA` reader - Flow Control Busy or Back-pressure Activate"]
pub type EMAC_FLOWCTL_FCBBPA_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FLOWCTL_FCBBPA` writer - Flow Control Busy or Back-pressure Activate"]
pub type EMAC_FLOWCTL_FCBBPA_W<'a> = crate::BitWriter<'a, u32, FLOWCTL_SPEC, bool, 0>;
#[doc = "Field `EMAC_FLOWCTL_TFE` reader - Transmit Flow Control Enable"]
pub type EMAC_FLOWCTL_TFE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FLOWCTL_TFE` writer - Transmit Flow Control Enable"]
pub type EMAC_FLOWCTL_TFE_W<'a> = crate::BitWriter<'a, u32, FLOWCTL_SPEC, bool, 1>;
#[doc = "Field `EMAC_FLOWCTL_RFE` reader - Receive Flow Control Enable"]
pub type EMAC_FLOWCTL_RFE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FLOWCTL_RFE` writer - Receive Flow Control Enable"]
pub type EMAC_FLOWCTL_RFE_W<'a> = crate::BitWriter<'a, u32, FLOWCTL_SPEC, bool, 2>;
#[doc = "Field `EMAC_FLOWCTL_UP` reader - Unicast Pause Frame Detect"]
pub type EMAC_FLOWCTL_UP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FLOWCTL_UP` writer - Unicast Pause Frame Detect"]
pub type EMAC_FLOWCTL_UP_W<'a> = crate::BitWriter<'a, u32, FLOWCTL_SPEC, bool, 3>;
#[doc = "Pause Low Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_FLOWCTL_PLT_A {
    #[doc = "0: The threshold is Pause time minus 4 slot times (PT - 4 slot times)"]
    EMAC_FLOWCTL_PLT_4 = 0,
    #[doc = "1: The threshold is Pause time minus 28 slot times (PT - 28 slot times)"]
    EMAC_FLOWCTL_PLT_28 = 1,
    #[doc = "2: The threshold is Pause time minus 144 slot times (PT - 144 slot times)"]
    EMAC_FLOWCTL_PLT_144 = 2,
    #[doc = "3: The threshold is Pause time minus 256 slot times (PT - 256 slot times)"]
    EMAC_FLOWCTL_PLT_156 = 3,
}
impl From<EMAC_FLOWCTL_PLT_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_FLOWCTL_PLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_FLOWCTL_PLT` reader - Pause Low Threshold"]
pub type EMAC_FLOWCTL_PLT_R = crate::FieldReader<u8, EMAC_FLOWCTL_PLT_A>;
impl EMAC_FLOWCTL_PLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_FLOWCTL_PLT_A {
        match self.bits {
            0 => EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_4,
            1 => EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_28,
            2 => EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_144,
            3 => EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_156,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_4`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_4(&self) -> bool {
        *self == EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_4
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_28`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_28(&self) -> bool {
        *self == EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_28
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_144`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_144(&self) -> bool {
        *self == EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_144
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_156`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_156(&self) -> bool {
        *self == EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_156
    }
}
#[doc = "Field `EMAC_FLOWCTL_PLT` writer - Pause Low Threshold"]
pub type EMAC_FLOWCTL_PLT_W<'a> =
    crate::FieldWriterSafe<'a, u32, FLOWCTL_SPEC, u8, EMAC_FLOWCTL_PLT_A, 2, 4>;
impl<'a> EMAC_FLOWCTL_PLT_W<'a> {
    #[doc = "The threshold is Pause time minus 4 slot times (PT - 4 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_4(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_4)
    }
    #[doc = "The threshold is Pause time minus 28 slot times (PT - 28 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_28(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_28)
    }
    #[doc = "The threshold is Pause time minus 144 slot times (PT - 144 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_144(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_144)
    }
    #[doc = "The threshold is Pause time minus 256 slot times (PT - 256 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_156(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLT_A::EMAC_FLOWCTL_PLT_156)
    }
}
#[doc = "Field `EMAC_FLOWCTL_DZQP` reader - Disable Zero-Quanta Pause"]
pub type EMAC_FLOWCTL_DZQP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FLOWCTL_DZQP` writer - Disable Zero-Quanta Pause"]
pub type EMAC_FLOWCTL_DZQP_W<'a> = crate::BitWriter<'a, u32, FLOWCTL_SPEC, bool, 7>;
#[doc = "Field `EMAC_FLOWCTL_PT` reader - Pause Time"]
pub type EMAC_FLOWCTL_PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_FLOWCTL_PT` writer - Pause Time"]
pub type EMAC_FLOWCTL_PT_W<'a> = crate::FieldWriter<'a, u32, FLOWCTL_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Back-pressure Activate"]
    #[inline(always)]
    pub fn emac_flowctl_fcbbpa(&self) -> EMAC_FLOWCTL_FCBBPA_R {
        EMAC_FLOWCTL_FCBBPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_tfe(&self) -> EMAC_FLOWCTL_TFE_R {
        EMAC_FLOWCTL_TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_rfe(&self) -> EMAC_FLOWCTL_RFE_R {
        EMAC_FLOWCTL_RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn emac_flowctl_up(&self) -> EMAC_FLOWCTL_UP_R {
        EMAC_FLOWCTL_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn emac_flowctl_plt(&self) -> EMAC_FLOWCTL_PLT_R {
        EMAC_FLOWCTL_PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn emac_flowctl_dzqp(&self) -> EMAC_FLOWCTL_DZQP_R {
        EMAC_FLOWCTL_DZQP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn emac_flowctl_pt(&self) -> EMAC_FLOWCTL_PT_R {
        EMAC_FLOWCTL_PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Back-pressure Activate"]
    #[inline(always)]
    pub fn emac_flowctl_fcbbpa(&mut self) -> EMAC_FLOWCTL_FCBBPA_W {
        EMAC_FLOWCTL_FCBBPA_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_tfe(&mut self) -> EMAC_FLOWCTL_TFE_W {
        EMAC_FLOWCTL_TFE_W::new(self)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_rfe(&mut self) -> EMAC_FLOWCTL_RFE_W {
        EMAC_FLOWCTL_RFE_W::new(self)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn emac_flowctl_up(&mut self) -> EMAC_FLOWCTL_UP_W {
        EMAC_FLOWCTL_UP_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn emac_flowctl_plt(&mut self) -> EMAC_FLOWCTL_PLT_W {
        EMAC_FLOWCTL_PLT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn emac_flowctl_dzqp(&mut self) -> EMAC_FLOWCTL_DZQP_W {
        EMAC_FLOWCTL_DZQP_W::new(self)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn emac_flowctl_pt(&mut self) -> EMAC_FLOWCTL_PT_W {
        EMAC_FLOWCTL_PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Flow Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowctl](index.html) module"]
pub struct FLOWCTL_SPEC;
impl crate::RegisterSpec for FLOWCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flowctl::R](R) reader structure"]
impl crate::Readable for FLOWCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flowctl::W](W) writer structure"]
impl crate::Writable for FLOWCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOWCTL to value 0"]
impl crate::Resettable for FLOWCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
