#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_MIS_RIMIS` reader - UART Ring Indicator Modem Masked Interrupt Status"]
pub type UART_MIS_RIMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_RIMIS` writer - UART Ring Indicator Modem Masked Interrupt Status"]
pub type UART_MIS_RIMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `UART_MIS_CTSMIS` reader - UART Clear to Send Modem Masked Interrupt Status"]
pub type UART_MIS_CTSMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_CTSMIS` writer - UART Clear to Send Modem Masked Interrupt Status"]
pub type UART_MIS_CTSMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 1>;
#[doc = "Field `UART_MIS_DCDMIS` reader - UART Data Carrier Detect Modem Masked Interrupt Status"]
pub type UART_MIS_DCDMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_DCDMIS` writer - UART Data Carrier Detect Modem Masked Interrupt Status"]
pub type UART_MIS_DCDMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `UART_MIS_DSRMIS` reader - UART Data Set Ready Modem Masked Interrupt Status"]
pub type UART_MIS_DSRMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_DSRMIS` writer - UART Data Set Ready Modem Masked Interrupt Status"]
pub type UART_MIS_DSRMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `UART_MIS_RXMIS` reader - UART Receive Masked Interrupt Status"]
pub type UART_MIS_RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_RXMIS` writer - UART Receive Masked Interrupt Status"]
pub type UART_MIS_RXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
#[doc = "Field `UART_MIS_TXMIS` reader - UART Transmit Masked Interrupt Status"]
pub type UART_MIS_TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_TXMIS` writer - UART Transmit Masked Interrupt Status"]
pub type UART_MIS_TXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 5>;
#[doc = "Field `UART_MIS_RTMIS` reader - UART Receive Time-Out Masked Interrupt Status"]
pub type UART_MIS_RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_RTMIS` writer - UART Receive Time-Out Masked Interrupt Status"]
pub type UART_MIS_RTMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 6>;
#[doc = "Field `UART_MIS_FEMIS` reader - UART Framing Error Masked Interrupt Status"]
pub type UART_MIS_FEMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_FEMIS` writer - UART Framing Error Masked Interrupt Status"]
pub type UART_MIS_FEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 7>;
#[doc = "Field `UART_MIS_PEMIS` reader - UART Parity Error Masked Interrupt Status"]
pub type UART_MIS_PEMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_PEMIS` writer - UART Parity Error Masked Interrupt Status"]
pub type UART_MIS_PEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 8>;
#[doc = "Field `UART_MIS_BEMIS` reader - UART Break Error Masked Interrupt Status"]
pub type UART_MIS_BEMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_BEMIS` writer - UART Break Error Masked Interrupt Status"]
pub type UART_MIS_BEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 9>;
#[doc = "Field `UART_MIS_OEMIS` reader - UART Overrun Error Masked Interrupt Status"]
pub type UART_MIS_OEMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_OEMIS` writer - UART Overrun Error Masked Interrupt Status"]
pub type UART_MIS_OEMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 10>;
#[doc = "Field `UART_MIS_EOTMIS` reader - End of Transmission Masked Interrupt Status"]
pub type UART_MIS_EOTMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_EOTMIS` writer - End of Transmission Masked Interrupt Status"]
pub type UART_MIS_EOTMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 11>;
#[doc = "Field `UART_MIS_9BITMIS` reader - 9-Bit Mode Masked Interrupt Status"]
pub type UART_MIS_9BITMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_9BITMIS` writer - 9-Bit Mode Masked Interrupt Status"]
pub type UART_MIS_9BITMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 12>;
#[doc = "Field `UART_MIS_DMARXMIS` reader - Receive DMA Masked Interrupt Status"]
pub type UART_MIS_DMARXMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_DMARXMIS` writer - Receive DMA Masked Interrupt Status"]
pub type UART_MIS_DMARXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 16>;
#[doc = "Field `UART_MIS_DMATXMIS` reader - Transmit DMA Masked Interrupt Status"]
pub type UART_MIS_DMATXMIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_MIS_DMATXMIS` writer - Transmit DMA Masked Interrupt Status"]
pub type UART_MIS_DMATXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - UART Ring Indicator Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rimis(&self) -> UART_MIS_RIMIS_R {
        UART_MIS_RIMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_ctsmis(&self) -> UART_MIS_CTSMIS_R {
        UART_MIS_CTSMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dcdmis(&self) -> UART_MIS_DCDMIS_R {
        UART_MIS_DCDMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dsrmis(&self) -> UART_MIS_DSRMIS_R {
        UART_MIS_DSRMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&self) -> UART_MIS_RXMIS_R {
        UART_MIS_RXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&self) -> UART_MIS_TXMIS_R {
        UART_MIS_TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&self) -> UART_MIS_RTMIS_R {
        UART_MIS_RTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&self) -> UART_MIS_FEMIS_R {
        UART_MIS_FEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&self) -> UART_MIS_PEMIS_R {
        UART_MIS_PEMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&self) -> UART_MIS_BEMIS_R {
        UART_MIS_BEMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&self) -> UART_MIS_OEMIS_R {
        UART_MIS_OEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Transmission Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_eotmis(&self) -> UART_MIS_EOTMIS_R {
        UART_MIS_EOTMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_9bitmis(&self) -> UART_MIS_9BITMIS_R {
        UART_MIS_9BITMIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmarxmis(&self) -> UART_MIS_DMARXMIS_R {
        UART_MIS_DMARXMIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmatxmis(&self) -> UART_MIS_DMATXMIS_R {
        UART_MIS_DMATXMIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Ring Indicator Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rimis(&mut self) -> UART_MIS_RIMIS_W {
        UART_MIS_RIMIS_W::new(self)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_ctsmis(&mut self) -> UART_MIS_CTSMIS_W {
        UART_MIS_CTSMIS_W::new(self)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dcdmis(&mut self) -> UART_MIS_DCDMIS_W {
        UART_MIS_DCDMIS_W::new(self)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dsrmis(&mut self) -> UART_MIS_DSRMIS_W {
        UART_MIS_DSRMIS_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&mut self) -> UART_MIS_RXMIS_W {
        UART_MIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&mut self) -> UART_MIS_TXMIS_W {
        UART_MIS_TXMIS_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&mut self) -> UART_MIS_RTMIS_W {
        UART_MIS_RTMIS_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&mut self) -> UART_MIS_FEMIS_W {
        UART_MIS_FEMIS_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&mut self) -> UART_MIS_PEMIS_W {
        UART_MIS_PEMIS_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&mut self) -> UART_MIS_BEMIS_W {
        UART_MIS_BEMIS_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&mut self) -> UART_MIS_OEMIS_W {
        UART_MIS_OEMIS_W::new(self)
    }
    #[doc = "Bit 11 - End of Transmission Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_eotmis(&mut self) -> UART_MIS_EOTMIS_W {
        UART_MIS_EOTMIS_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_9bitmis(&mut self) -> UART_MIS_9BITMIS_W {
        UART_MIS_9BITMIS_W::new(self)
    }
    #[doc = "Bit 16 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmarxmis(&mut self) -> UART_MIS_DMARXMIS_W {
        UART_MIS_DMARXMIS_W::new(self)
    }
    #[doc = "Bit 17 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmatxmis(&mut self) -> UART_MIS_DMATXMIS_W {
        UART_MIS_DMATXMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
