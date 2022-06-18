#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_STATUS_RPE` reader - MAC MII Receive Protocol Engine Status"]
pub type EMAC_STATUS_RPE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_RPE` writer - MAC MII Receive Protocol Engine Status"]
pub type EMAC_STATUS_RPE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 0>;
#[doc = "Field `EMAC_STATUS_RFCFC` reader - MAC Receive Frame Controller FIFO Status"]
pub type EMAC_STATUS_RFCFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_STATUS_RFCFC` writer - MAC Receive Frame Controller FIFO Status"]
pub type EMAC_STATUS_RFCFC_W<'a> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 2, 1>;
#[doc = "Field `EMAC_STATUS_RWC` reader - TX/RX Controller RX FIFO Write Controller Active Status"]
pub type EMAC_STATUS_RWC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_RWC` writer - TX/RX Controller RX FIFO Write Controller Active Status"]
pub type EMAC_STATUS_RWC_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 4>;
#[doc = "TX/RX Controller Read Controller State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_STATUS_RRC_A {
    #[doc = "0: IDLE state"]
    EMAC_STATUS_RRC_IDLE = 0,
    #[doc = "1: Reading frame data"]
    EMAC_STATUS_RRC_STATUS = 1,
    #[doc = "2: Reading frame status (or timestamp)"]
    EMAC_STATUS_RRC_DATA = 2,
    #[doc = "3: Flushing the frame data and status"]
    EMAC_STATUS_RRC_FLUSH = 3,
}
impl From<EMAC_STATUS_RRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_STATUS_RRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_STATUS_RRC` reader - TX/RX Controller Read Controller State"]
pub type EMAC_STATUS_RRC_R = crate::FieldReader<u8, EMAC_STATUS_RRC_A>;
impl EMAC_STATUS_RRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_STATUS_RRC_A {
        match self.bits {
            0 => EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_IDLE,
            1 => EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_STATUS,
            2 => EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_DATA,
            3 => EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_FLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_idle(&self) -> bool {
        *self == EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_STATUS`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_status(&self) -> bool {
        *self == EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_DATA`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_data(&self) -> bool {
        *self == EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_DATA
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_FLUSH`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_flush(&self) -> bool {
        *self == EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_FLUSH
    }
}
#[doc = "Field `EMAC_STATUS_RRC` writer - TX/RX Controller Read Controller State"]
pub type EMAC_STATUS_RRC_W<'a> =
    crate::FieldWriterSafe<'a, u32, STATUS_SPEC, u8, EMAC_STATUS_RRC_A, 2, 5>;
impl<'a> EMAC_STATUS_RRC_W<'a> {
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_rrc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_IDLE)
    }
    #[doc = "Reading frame data"]
    #[inline(always)]
    pub fn emac_status_rrc_status(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_STATUS)
    }
    #[doc = "Reading frame status (or timestamp)"]
    #[inline(always)]
    pub fn emac_status_rrc_data(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_DATA)
    }
    #[doc = "Flushing the frame data and status"]
    #[inline(always)]
    pub fn emac_status_rrc_flush(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRC_A::EMAC_STATUS_RRC_FLUSH)
    }
}
#[doc = "TX/RX Controller RX FIFO Fill-level Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_STATUS_RXF_A {
    #[doc = "0: RX FIFO Empty"]
    EMAC_STATUS_RXF_EMPTY = 0,
    #[doc = "1: RX FIFO fill level is below the flow-control deactivate threshold"]
    EMAC_STATUS_RXF_BELOW = 1,
    #[doc = "2: RX FIFO fill level is above the flow-control activate threshold"]
    EMAC_STATUS_RXF_ABOVE = 2,
    #[doc = "3: RX FIFO Full"]
    EMAC_STATUS_RXF_FULL = 3,
}
impl From<EMAC_STATUS_RXF_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_STATUS_RXF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_STATUS_RXF` reader - TX/RX Controller RX FIFO Fill-level Status"]
pub type EMAC_STATUS_RXF_R = crate::FieldReader<u8, EMAC_STATUS_RXF_A>;
impl EMAC_STATUS_RXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_STATUS_RXF_A {
        match self.bits {
            0 => EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_EMPTY,
            1 => EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_BELOW,
            2 => EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_ABOVE,
            3 => EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_EMPTY`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_empty(&self) -> bool {
        *self == EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_BELOW`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_below(&self) -> bool {
        *self == EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_BELOW
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_ABOVE`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_above(&self) -> bool {
        *self == EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_ABOVE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_FULL`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_full(&self) -> bool {
        *self == EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_FULL
    }
}
#[doc = "Field `EMAC_STATUS_RXF` writer - TX/RX Controller RX FIFO Fill-level Status"]
pub type EMAC_STATUS_RXF_W<'a> =
    crate::FieldWriterSafe<'a, u32, STATUS_SPEC, u8, EMAC_STATUS_RXF_A, 2, 8>;
impl<'a> EMAC_STATUS_RXF_W<'a> {
    #[doc = "RX FIFO Empty"]
    #[inline(always)]
    pub fn emac_status_rxf_empty(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_EMPTY)
    }
    #[doc = "RX FIFO fill level is below the flow-control deactivate threshold"]
    #[inline(always)]
    pub fn emac_status_rxf_below(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_BELOW)
    }
    #[doc = "RX FIFO fill level is above the flow-control activate threshold"]
    #[inline(always)]
    pub fn emac_status_rxf_above(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_ABOVE)
    }
    #[doc = "RX FIFO Full"]
    #[inline(always)]
    pub fn emac_status_rxf_full(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXF_A::EMAC_STATUS_RXF_FULL)
    }
}
#[doc = "Field `EMAC_STATUS_TPE` reader - MAC MII Transmit Protocol Engine Status"]
pub type EMAC_STATUS_TPE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_TPE` writer - MAC MII Transmit Protocol Engine Status"]
pub type EMAC_STATUS_TPE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 16>;
#[doc = "MAC Transmit Frame Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_STATUS_TFC_A {
    #[doc = "0: IDLE state"]
    EMAC_STATUS_TFC_IDLE = 0,
    #[doc = "1: Waiting for status of previous frame or IFG or backoff period to be over"]
    EMAC_STATUS_TFC_STATUS = 1,
    #[doc = "2: Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    EMAC_STATUS_TFC_PAUSE = 2,
    #[doc = "3: Transferring input frame for transmission"]
    EMAC_STATUS_TFC_INPUT = 3,
}
impl From<EMAC_STATUS_TFC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_STATUS_TFC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_STATUS_TFC` reader - MAC Transmit Frame Controller Status"]
pub type EMAC_STATUS_TFC_R = crate::FieldReader<u8, EMAC_STATUS_TFC_A>;
impl EMAC_STATUS_TFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_STATUS_TFC_A {
        match self.bits {
            0 => EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_IDLE,
            1 => EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_STATUS,
            2 => EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_PAUSE,
            3 => EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_idle(&self) -> bool {
        *self == EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_STATUS`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_status(&self) -> bool {
        *self == EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_PAUSE`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_pause(&self) -> bool {
        *self == EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_PAUSE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_INPUT`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_input(&self) -> bool {
        *self == EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_INPUT
    }
}
#[doc = "Field `EMAC_STATUS_TFC` writer - MAC Transmit Frame Controller Status"]
pub type EMAC_STATUS_TFC_W<'a> =
    crate::FieldWriterSafe<'a, u32, STATUS_SPEC, u8, EMAC_STATUS_TFC_A, 2, 17>;
impl<'a> EMAC_STATUS_TFC_W<'a> {
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_tfc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_IDLE)
    }
    #[doc = "Waiting for status of previous frame or IFG or backoff period to be over"]
    #[inline(always)]
    pub fn emac_status_tfc_status(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_STATUS)
    }
    #[doc = "Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    #[inline(always)]
    pub fn emac_status_tfc_pause(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_PAUSE)
    }
    #[doc = "Transferring input frame for transmission"]
    #[inline(always)]
    pub fn emac_status_tfc_input(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFC_A::EMAC_STATUS_TFC_INPUT)
    }
}
#[doc = "Field `EMAC_STATUS_TXPAUSED` reader - MAC Transmitter PAUSE"]
pub type EMAC_STATUS_TXPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_TXPAUSED` writer - MAC Transmitter PAUSE"]
pub type EMAC_STATUS_TXPAUSED_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 19>;
#[doc = "TX/RX Controller's TX FIFO Read Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_STATUS_TRC_A {
    #[doc = "0: IDLE state"]
    EMAC_STATUS_TRC_IDLE = 0,
    #[doc = "1: READ state (transferring data to MAC transmitter)"]
    EMAC_STATUS_TRC_READ = 1,
    #[doc = "2: Waiting for TX Status from MAC transmitter"]
    EMAC_STATUS_TRC_WAIT = 2,
    #[doc = "3: Writing the received TX Status or flushing the TX FIFO"]
    EMAC_STATUS_TRC_WRFLUSH = 3,
}
impl From<EMAC_STATUS_TRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_STATUS_TRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_STATUS_TRC` reader - TX/RX Controller's TX FIFO Read Controller Status"]
pub type EMAC_STATUS_TRC_R = crate::FieldReader<u8, EMAC_STATUS_TRC_A>;
impl EMAC_STATUS_TRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_STATUS_TRC_A {
        match self.bits {
            0 => EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_IDLE,
            1 => EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_READ,
            2 => EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WAIT,
            3 => EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WRFLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_trc_idle(&self) -> bool {
        *self == EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_READ`"]
    #[inline(always)]
    pub fn is_emac_status_trc_read(&self) -> bool {
        *self == EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_READ
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_WAIT`"]
    #[inline(always)]
    pub fn is_emac_status_trc_wait(&self) -> bool {
        *self == EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WAIT
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_WRFLUSH`"]
    #[inline(always)]
    pub fn is_emac_status_trc_wrflush(&self) -> bool {
        *self == EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WRFLUSH
    }
}
#[doc = "Field `EMAC_STATUS_TRC` writer - TX/RX Controller's TX FIFO Read Controller Status"]
pub type EMAC_STATUS_TRC_W<'a> =
    crate::FieldWriterSafe<'a, u32, STATUS_SPEC, u8, EMAC_STATUS_TRC_A, 2, 20>;
impl<'a> EMAC_STATUS_TRC_W<'a> {
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_trc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_IDLE)
    }
    #[doc = "READ state (transferring data to MAC transmitter)"]
    #[inline(always)]
    pub fn emac_status_trc_read(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_READ)
    }
    #[doc = "Waiting for TX Status from MAC transmitter"]
    #[inline(always)]
    pub fn emac_status_trc_wait(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WAIT)
    }
    #[doc = "Writing the received TX Status or flushing the TX FIFO"]
    #[inline(always)]
    pub fn emac_status_trc_wrflush(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRC_A::EMAC_STATUS_TRC_WRFLUSH)
    }
}
#[doc = "Field `EMAC_STATUS_TWC` reader - TX/RX Controller TX FIFO Write Controller Active Status"]
pub type EMAC_STATUS_TWC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_TWC` writer - TX/RX Controller TX FIFO Write Controller Active Status"]
pub type EMAC_STATUS_TWC_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 22>;
#[doc = "Field `EMAC_STATUS_TXFE` reader - TX/RX Controller TX FIFO Not Empty Status"]
pub type EMAC_STATUS_TXFE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_TXFE` writer - TX/RX Controller TX FIFO Not Empty Status"]
pub type EMAC_STATUS_TXFE_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 24>;
#[doc = "Field `EMAC_STATUS_TXFF` reader - TX/RX Controller TX FIFO Full Status"]
pub type EMAC_STATUS_TXFF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_STATUS_TXFF` writer - TX/RX Controller TX FIFO Full Status"]
pub type EMAC_STATUS_TXFF_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_rpe(&self) -> EMAC_STATUS_RPE_R {
        EMAC_STATUS_RPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn emac_status_rfcfc(&self) -> EMAC_STATUS_RFCFC_R {
        EMAC_STATUS_RFCFC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_rwc(&self) -> EMAC_STATUS_RWC_R {
        EMAC_STATUS_RWC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline(always)]
    pub fn emac_status_rrc(&self) -> EMAC_STATUS_RRC_R {
        EMAC_STATUS_RRC_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline(always)]
    pub fn emac_status_rxf(&self) -> EMAC_STATUS_RXF_R {
        EMAC_STATUS_RXF_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_tpe(&self) -> EMAC_STATUS_TPE_R {
        EMAC_STATUS_TPE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn emac_status_tfc(&self) -> EMAC_STATUS_TFC_R {
        EMAC_STATUS_TFC_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline(always)]
    pub fn emac_status_txpaused(&self) -> EMAC_STATUS_TXPAUSED_R {
        EMAC_STATUS_TXPAUSED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn emac_status_trc(&self) -> EMAC_STATUS_TRC_R {
        EMAC_STATUS_TRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_twc(&self) -> EMAC_STATUS_TWC_R {
        EMAC_STATUS_TWC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline(always)]
    pub fn emac_status_txfe(&self) -> EMAC_STATUS_TXFE_R {
        EMAC_STATUS_TXFE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline(always)]
    pub fn emac_status_txff(&self) -> EMAC_STATUS_TXFF_R {
        EMAC_STATUS_TXFF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_rpe(&mut self) -> EMAC_STATUS_RPE_W {
        EMAC_STATUS_RPE_W::new(self)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn emac_status_rfcfc(&mut self) -> EMAC_STATUS_RFCFC_W {
        EMAC_STATUS_RFCFC_W::new(self)
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_rwc(&mut self) -> EMAC_STATUS_RWC_W {
        EMAC_STATUS_RWC_W::new(self)
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline(always)]
    pub fn emac_status_rrc(&mut self) -> EMAC_STATUS_RRC_W {
        EMAC_STATUS_RRC_W::new(self)
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline(always)]
    pub fn emac_status_rxf(&mut self) -> EMAC_STATUS_RXF_W {
        EMAC_STATUS_RXF_W::new(self)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_tpe(&mut self) -> EMAC_STATUS_TPE_W {
        EMAC_STATUS_TPE_W::new(self)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn emac_status_tfc(&mut self) -> EMAC_STATUS_TFC_W {
        EMAC_STATUS_TFC_W::new(self)
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline(always)]
    pub fn emac_status_txpaused(&mut self) -> EMAC_STATUS_TXPAUSED_W {
        EMAC_STATUS_TXPAUSED_W::new(self)
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn emac_status_trc(&mut self) -> EMAC_STATUS_TRC_W {
        EMAC_STATUS_TRC_W::new(self)
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_twc(&mut self) -> EMAC_STATUS_TWC_W {
        EMAC_STATUS_TWC_W::new(self)
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline(always)]
    pub fn emac_status_txfe(&mut self) -> EMAC_STATUS_TXFE_W {
        EMAC_STATUS_TXFE_W::new(self)
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline(always)]
    pub fn emac_status_txff(&mut self) -> EMAC_STATUS_TXFF_W {
        EMAC_STATUS_TXFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
