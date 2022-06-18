#[doc = "Register `DMARIS` reader"]
pub struct R(crate::R<DMARIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARIS` writer"]
pub struct W(crate::W<DMARIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARIS_SPEC>;
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
impl From<crate::W<DMARIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_DMARIS_TI` reader - Transmit Interrupt"]
pub type EMAC_DMARIS_TI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_TI` writer - Transmit Interrupt"]
pub type EMAC_DMARIS_TI_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 0>;
#[doc = "Field `EMAC_DMARIS_TPS` reader - Transmit Process Stopped"]
pub type EMAC_DMARIS_TPS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_TPS` writer - Transmit Process Stopped"]
pub type EMAC_DMARIS_TPS_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 1>;
#[doc = "Field `EMAC_DMARIS_TU` reader - Transmit Buffer Unavailable"]
pub type EMAC_DMARIS_TU_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_TU` writer - Transmit Buffer Unavailable"]
pub type EMAC_DMARIS_TU_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 2>;
#[doc = "Field `EMAC_DMARIS_TJT` reader - Transmit Jabber Timeout"]
pub type EMAC_DMARIS_TJT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_TJT` writer - Transmit Jabber Timeout"]
pub type EMAC_DMARIS_TJT_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 3>;
#[doc = "Field `EMAC_DMARIS_OVF` reader - Receive Overflow"]
pub type EMAC_DMARIS_OVF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_OVF` writer - Receive Overflow"]
pub type EMAC_DMARIS_OVF_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 4>;
#[doc = "Field `EMAC_DMARIS_UNF` reader - Transmit Underflow"]
pub type EMAC_DMARIS_UNF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_UNF` writer - Transmit Underflow"]
pub type EMAC_DMARIS_UNF_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 5>;
#[doc = "Field `EMAC_DMARIS_RI` reader - Receive Interrupt"]
pub type EMAC_DMARIS_RI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_RI` writer - Receive Interrupt"]
pub type EMAC_DMARIS_RI_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 6>;
#[doc = "Field `EMAC_DMARIS_RU` reader - Receive Buffer Unavailable"]
pub type EMAC_DMARIS_RU_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_RU` writer - Receive Buffer Unavailable"]
pub type EMAC_DMARIS_RU_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 7>;
#[doc = "Field `EMAC_DMARIS_RPS` reader - Receive Process Stopped"]
pub type EMAC_DMARIS_RPS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_RPS` writer - Receive Process Stopped"]
pub type EMAC_DMARIS_RPS_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 8>;
#[doc = "Field `EMAC_DMARIS_RWT` reader - Receive Watchdog Timeout"]
pub type EMAC_DMARIS_RWT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_RWT` writer - Receive Watchdog Timeout"]
pub type EMAC_DMARIS_RWT_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 9>;
#[doc = "Field `EMAC_DMARIS_ETI` reader - Early Transmit Interrupt"]
pub type EMAC_DMARIS_ETI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_ETI` writer - Early Transmit Interrupt"]
pub type EMAC_DMARIS_ETI_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 10>;
#[doc = "Field `EMAC_DMARIS_FBI` reader - Fatal Bus Error Interrupt"]
pub type EMAC_DMARIS_FBI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_FBI` writer - Fatal Bus Error Interrupt"]
pub type EMAC_DMARIS_FBI_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 13>;
#[doc = "Field `EMAC_DMARIS_ERI` reader - Early Receive Interrupt"]
pub type EMAC_DMARIS_ERI_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_ERI` writer - Early Receive Interrupt"]
pub type EMAC_DMARIS_ERI_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 14>;
#[doc = "Field `EMAC_DMARIS_AIS` reader - Abnormal Interrupt Summary"]
pub type EMAC_DMARIS_AIS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_AIS` writer - Abnormal Interrupt Summary"]
pub type EMAC_DMARIS_AIS_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 15>;
#[doc = "Field `EMAC_DMARIS_NIS` reader - Normal Interrupt Summary"]
pub type EMAC_DMARIS_NIS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_NIS` writer - Normal Interrupt Summary"]
pub type EMAC_DMARIS_NIS_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 16>;
#[doc = "Received Process State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_DMARIS_RS_A {
    #[doc = "0: Stopped: Reset or stop receive command issued"]
    EMAC_DMARIS_RS_STOP = 0,
    #[doc = "1: Running: Fetching receive transfer descriptor"]
    EMAC_DMARIS_RS_RUNRXTD = 1,
    #[doc = "3: Running: Waiting for receive packet"]
    EMAC_DMARIS_RS_RUNRXD = 3,
    #[doc = "4: Suspended: Receive descriptor unavailable"]
    EMAC_DMARIS_RS_SUSPEND = 4,
    #[doc = "5: Running: Closing receive descriptor"]
    EMAC_DMARIS_RS_RUNCRD = 5,
    #[doc = "6: Writing Timestamp"]
    EMAC_DMARIS_RS_TSWS = 6,
    #[doc = "7: Running: Transferring the receive packet data from receive buffer to host memory"]
    EMAC_DMARIS_RS_RUNTXD = 7,
}
impl From<EMAC_DMARIS_RS_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_DMARIS_RS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_DMARIS_RS` reader - Received Process State"]
pub type EMAC_DMARIS_RS_R = crate::FieldReader<u8, EMAC_DMARIS_RS_A>;
impl EMAC_DMARIS_RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_DMARIS_RS_A> {
        match self.bits {
            0 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_STOP),
            1 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXTD),
            3 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXD),
            4 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_SUSPEND),
            5 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNCRD),
            6 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_TSWS),
            7 => Some(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNTXD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_STOP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_stop(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_STOP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNRXTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runrxtd(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXTD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNRXD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runrxd(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_SUSPEND`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_suspend(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_SUSPEND
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNCRD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runcrd(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNCRD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_TSWS`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_tsws(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_TSWS
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_RS_RUNTXD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_rs_runtxd(&self) -> bool {
        *self == EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNTXD
    }
}
#[doc = "Field `EMAC_DMARIS_RS` writer - Received Process State"]
pub type EMAC_DMARIS_RS_W<'a> =
    crate::FieldWriter<'a, u32, DMARIS_SPEC, u8, EMAC_DMARIS_RS_A, 3, 17>;
impl<'a> EMAC_DMARIS_RS_W<'a> {
    #[doc = "Stopped: Reset or stop receive command issued"]
    #[inline(always)]
    pub fn emac_dmaris_rs_stop(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_STOP)
    }
    #[doc = "Running: Fetching receive transfer descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runrxtd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXTD)
    }
    #[doc = "Running: Waiting for receive packet"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runrxd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNRXD)
    }
    #[doc = "Suspended: Receive descriptor unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_rs_suspend(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_SUSPEND)
    }
    #[doc = "Running: Closing receive descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runcrd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNCRD)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn emac_dmaris_rs_tsws(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_TSWS)
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn emac_dmaris_rs_runtxd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_RS_A::EMAC_DMARIS_RS_RUNTXD)
    }
}
#[doc = "Transmit Process State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_DMARIS_TS_A {
    #[doc = "0: Stopped; Reset or Stop transmit command processed"]
    EMAC_DMARIS_TS_STOP = 0,
    #[doc = "1: Running; Fetching transmit transfer descriptor"]
    EMAC_DMARIS_TS_RUNTXTD = 1,
    #[doc = "2: Running; Waiting for status"]
    EMAC_DMARIS_TS_STATUS = 2,
    #[doc = "3: Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    EMAC_DMARIS_TS_RUNTX = 3,
    #[doc = "4: Writing Timestamp"]
    EMAC_DMARIS_TS_TSTAMP = 4,
    #[doc = "6: Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    EMAC_DMARIS_TS_SUSPEND = 6,
    #[doc = "7: Running; Closing transmit descriptor"]
    EMAC_DMARIS_TS_RUNCTD = 7,
}
impl From<EMAC_DMARIS_TS_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_DMARIS_TS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_DMARIS_TS` reader - Transmit Process State"]
pub type EMAC_DMARIS_TS_R = crate::FieldReader<u8, EMAC_DMARIS_TS_A>;
impl EMAC_DMARIS_TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_DMARIS_TS_A> {
        match self.bits {
            0 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STOP),
            1 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTXTD),
            2 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STATUS),
            3 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTX),
            4 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_TSTAMP),
            6 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_SUSPEND),
            7 => Some(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNCTD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_STOP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_stop(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STOP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNTXTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runtxtd(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTXTD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_STATUS`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_status(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNTX`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runtx(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTX
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_TSTAMP`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_tstamp(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_TSTAMP
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_SUSPEND`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_suspend(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_SUSPEND
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_TS_RUNCTD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ts_runctd(&self) -> bool {
        *self == EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNCTD
    }
}
#[doc = "Field `EMAC_DMARIS_TS` writer - Transmit Process State"]
pub type EMAC_DMARIS_TS_W<'a> =
    crate::FieldWriter<'a, u32, DMARIS_SPEC, u8, EMAC_DMARIS_TS_A, 3, 20>;
impl<'a> EMAC_DMARIS_TS_W<'a> {
    #[doc = "Stopped; Reset or Stop transmit command processed"]
    #[inline(always)]
    pub fn emac_dmaris_ts_stop(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STOP)
    }
    #[doc = "Running; Fetching transmit transfer descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runtxtd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTXTD)
    }
    #[doc = "Running; Waiting for status"]
    #[inline(always)]
    pub fn emac_dmaris_ts_status(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_STATUS)
    }
    #[doc = "Running; Reading data from host memory buffer and queuing it to transmit buffer (TX FIFO)"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runtx(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNTX)
    }
    #[doc = "Writing Timestamp"]
    #[inline(always)]
    pub fn emac_dmaris_ts_tstamp(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_TSTAMP)
    }
    #[doc = "Suspended; Transmit descriptor unavailable or transmit buffer underflow"]
    #[inline(always)]
    pub fn emac_dmaris_ts_suspend(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_SUSPEND)
    }
    #[doc = "Running; Closing transmit descriptor"]
    #[inline(always)]
    pub fn emac_dmaris_ts_runctd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_TS_A::EMAC_DMARIS_TS_RUNCTD)
    }
}
#[doc = "Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_DMARIS_AE_A {
    #[doc = "0: Error during RX DMA Write Data Transfer"]
    EMAC_DMARIS_AE_RXDMAWD = 0,
    #[doc = "3: Error during TX DMA Read Data Transfer"]
    EMAC_DMARIS_AE_TXDMARD = 3,
    #[doc = "4: Error during RX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_RXDMADW = 4,
    #[doc = "5: Error during TX DMA Descriptor Write Access"]
    EMAC_DMARIS_AE_TXDMADW = 5,
    #[doc = "6: Error during RX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_RXDMADR = 6,
    #[doc = "7: Error during TX DMA Descriptor Read Access"]
    EMAC_DMARIS_AE_TXDMADR = 7,
}
impl From<EMAC_DMARIS_AE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_DMARIS_AE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_DMARIS_AE` reader - Access Error"]
pub type EMAC_DMARIS_AE_R = crate::FieldReader<u8, EMAC_DMARIS_AE_A>;
impl EMAC_DMARIS_AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_DMARIS_AE_A> {
        match self.bits {
            0 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMAWD),
            3 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMARD),
            4 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADW),
            5 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADW),
            6 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADR),
            7 => Some(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMAWD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmawd(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMAWD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMARD`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmard(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMARD
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMADW`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmadw(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADW
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMADW`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmadw(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADW
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_RXDMADR`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_rxdmadr(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADR
    }
    #[doc = "Checks if the value of the field is `EMAC_DMARIS_AE_TXDMADR`"]
    #[inline(always)]
    pub fn is_emac_dmaris_ae_txdmadr(&self) -> bool {
        *self == EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADR
    }
}
#[doc = "Field `EMAC_DMARIS_AE` writer - Access Error"]
pub type EMAC_DMARIS_AE_W<'a> =
    crate::FieldWriter<'a, u32, DMARIS_SPEC, u8, EMAC_DMARIS_AE_A, 3, 23>;
impl<'a> EMAC_DMARIS_AE_W<'a> {
    #[doc = "Error during RX DMA Write Data Transfer"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmawd(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMAWD)
    }
    #[doc = "Error during TX DMA Read Data Transfer"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmard(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMARD)
    }
    #[doc = "Error during RX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmadw(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADW)
    }
    #[doc = "Error during TX DMA Descriptor Write Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmadw(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADW)
    }
    #[doc = "Error during RX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_rxdmadr(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_RXDMADR)
    }
    #[doc = "Error during TX DMA Descriptor Read Access"]
    #[inline(always)]
    pub fn emac_dmaris_ae_txdmadr(self) -> &'a mut W {
        self.variant(EMAC_DMARIS_AE_A::EMAC_DMARIS_AE_TXDMADR)
    }
}
#[doc = "Field `EMAC_DMARIS_MMC` reader - MAC MMC Interrupt"]
pub type EMAC_DMARIS_MMC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_MMC` writer - MAC MMC Interrupt"]
pub type EMAC_DMARIS_MMC_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 27>;
#[doc = "Field `EMAC_DMARIS_PMT` reader - MAC PMT Interrupt Status"]
pub type EMAC_DMARIS_PMT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_PMT` writer - MAC PMT Interrupt Status"]
pub type EMAC_DMARIS_PMT_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 28>;
#[doc = "Field `EMAC_DMARIS_TT` reader - Timestamp Trigger Interrupt Status"]
pub type EMAC_DMARIS_TT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMARIS_TT` writer - Timestamp Trigger Interrupt Status"]
pub type EMAC_DMARIS_TT_W<'a> = crate::BitWriter<'a, u32, DMARIS_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ti(&self) -> EMAC_DMARIS_TI_R {
        EMAC_DMARIS_TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_tps(&self) -> EMAC_DMARIS_TPS_R {
        EMAC_DMARIS_TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_tu(&self) -> EMAC_DMARIS_TU_R {
        EMAC_DMARIS_TU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_tjt(&self) -> EMAC_DMARIS_TJT_R {
        EMAC_DMARIS_TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn emac_dmaris_ovf(&self) -> EMAC_DMARIS_OVF_R {
        EMAC_DMARIS_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn emac_dmaris_unf(&self) -> EMAC_DMARIS_UNF_R {
        EMAC_DMARIS_UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ri(&self) -> EMAC_DMARIS_RI_R {
        EMAC_DMARIS_RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_ru(&self) -> EMAC_DMARIS_RU_R {
        EMAC_DMARIS_RU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_rps(&self) -> EMAC_DMARIS_RPS_R {
        EMAC_DMARIS_RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_rwt(&self) -> EMAC_DMARIS_RWT_R {
        EMAC_DMARIS_RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eti(&self) -> EMAC_DMARIS_ETI_R {
        EMAC_DMARIS_ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_fbi(&self) -> EMAC_DMARIS_FBI_R {
        EMAC_DMARIS_FBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eri(&self) -> EMAC_DMARIS_ERI_R {
        EMAC_DMARIS_ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_ais(&self) -> EMAC_DMARIS_AIS_R {
        EMAC_DMARIS_AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_nis(&self) -> EMAC_DMARIS_NIS_R {
        EMAC_DMARIS_NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn emac_dmaris_rs(&self) -> EMAC_DMARIS_RS_R {
        EMAC_DMARIS_RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn emac_dmaris_ts(&self) -> EMAC_DMARIS_TS_R {
        EMAC_DMARIS_TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn emac_dmaris_ae(&self) -> EMAC_DMARIS_AE_R {
        EMAC_DMARIS_AE_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_mmc(&self) -> EMAC_DMARIS_MMC_R {
        EMAC_DMARIS_MMC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_pmt(&self) -> EMAC_DMARIS_PMT_R {
        EMAC_DMARIS_PMT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_tt(&self) -> EMAC_DMARIS_TT_R {
        EMAC_DMARIS_TT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ti(&mut self) -> EMAC_DMARIS_TI_W {
        EMAC_DMARIS_TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_tps(&mut self) -> EMAC_DMARIS_TPS_W {
        EMAC_DMARIS_TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_tu(&mut self) -> EMAC_DMARIS_TU_W {
        EMAC_DMARIS_TU_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_tjt(&mut self) -> EMAC_DMARIS_TJT_W {
        EMAC_DMARIS_TJT_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn emac_dmaris_ovf(&mut self) -> EMAC_DMARIS_OVF_W {
        EMAC_DMARIS_OVF_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn emac_dmaris_unf(&mut self) -> EMAC_DMARIS_UNF_W {
        EMAC_DMARIS_UNF_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_ri(&mut self) -> EMAC_DMARIS_RI_W {
        EMAC_DMARIS_RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn emac_dmaris_ru(&mut self) -> EMAC_DMARIS_RU_W {
        EMAC_DMARIS_RU_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn emac_dmaris_rps(&mut self) -> EMAC_DMARIS_RPS_W {
        EMAC_DMARIS_RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_dmaris_rwt(&mut self) -> EMAC_DMARIS_RWT_W {
        EMAC_DMARIS_RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eti(&mut self) -> EMAC_DMARIS_ETI_W {
        EMAC_DMARIS_ETI_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_fbi(&mut self) -> EMAC_DMARIS_FBI_W {
        EMAC_DMARIS_FBI_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_eri(&mut self) -> EMAC_DMARIS_ERI_W {
        EMAC_DMARIS_ERI_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_ais(&mut self) -> EMAC_DMARIS_AIS_W {
        EMAC_DMARIS_AIS_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn emac_dmaris_nis(&mut self) -> EMAC_DMARIS_NIS_W {
        EMAC_DMARIS_NIS_W::new(self)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn emac_dmaris_rs(&mut self) -> EMAC_DMARIS_RS_W {
        EMAC_DMARIS_RS_W::new(self)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn emac_dmaris_ts(&mut self) -> EMAC_DMARIS_TS_W {
        EMAC_DMARIS_TS_W::new(self)
    }
    #[doc = "Bits 23:25 - Access Error"]
    #[inline(always)]
    pub fn emac_dmaris_ae(&mut self) -> EMAC_DMARIS_AE_W {
        EMAC_DMARIS_AE_W::new(self)
    }
    #[doc = "Bit 27 - MAC MMC Interrupt"]
    #[inline(always)]
    pub fn emac_dmaris_mmc(&mut self) -> EMAC_DMARIS_MMC_W {
        EMAC_DMARIS_MMC_W::new(self)
    }
    #[doc = "Bit 28 - MAC PMT Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_pmt(&mut self) -> EMAC_DMARIS_PMT_W {
        EMAC_DMARIS_PMT_W::new(self)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt Status"]
    #[inline(always)]
    pub fn emac_dmaris_tt(&mut self) -> EMAC_DMARIS_TT_W {
        EMAC_DMARIS_TT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC DMA Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaris](index.html) module"]
pub struct DMARIS_SPEC;
impl crate::RegisterSpec for DMARIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaris::R](R) reader structure"]
impl crate::Readable for DMARIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaris::W](W) writer structure"]
impl crate::Writable for DMARIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARIS to value 0"]
impl crate::Resettable for DMARIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
