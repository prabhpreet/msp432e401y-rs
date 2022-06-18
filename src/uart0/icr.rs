#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_ICR_RIMIC` writer - UART Ring Indicator Modem Interrupt Clear"]
pub type UART_ICR_RIMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 0>;
#[doc = "Field `UART_ICR_CTSMIC` writer - UART Clear to Send Modem Interrupt Clear"]
pub type UART_ICR_CTSMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 1>;
#[doc = "Field `UART_ICR_DCDMIC` writer - UART Data Carrier Detect Modem Interrupt Clear"]
pub type UART_ICR_DCDMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 2>;
#[doc = "Field `UART_ICR_DSRMIC` writer - UART Data Set Ready Modem Interrupt Clear"]
pub type UART_ICR_DSRMIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 3>;
#[doc = "Field `UART_ICR_RXIC` writer - Receive Interrupt Clear"]
pub type UART_ICR_RXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 4>;
#[doc = "Field `UART_ICR_TXIC` writer - Transmit Interrupt Clear"]
pub type UART_ICR_TXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 5>;
#[doc = "Field `UART_ICR_RTIC` writer - Receive Time-Out Interrupt Clear"]
pub type UART_ICR_RTIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 6>;
#[doc = "Field `UART_ICR_FEIC` writer - Framing Error Interrupt Clear"]
pub type UART_ICR_FEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 7>;
#[doc = "Field `UART_ICR_PEIC` writer - Parity Error Interrupt Clear"]
pub type UART_ICR_PEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 8>;
#[doc = "Field `UART_ICR_BEIC` writer - Break Error Interrupt Clear"]
pub type UART_ICR_BEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 9>;
#[doc = "Field `UART_ICR_OEIC` writer - Overrun Error Interrupt Clear"]
pub type UART_ICR_OEIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 10>;
#[doc = "Field `UART_ICR_EOTIC` writer - End of Transmission Interrupt Clear"]
pub type UART_ICR_EOTIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 11>;
#[doc = "Field `UART_ICR_9BITIC` writer - 9-Bit Mode Interrupt Clear"]
pub type UART_ICR_9BITIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 12>;
#[doc = "Field `UART_ICR_DMARXIC` writer - Receive DMA Interrupt Clear"]
pub type UART_ICR_DMARXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 16>;
#[doc = "Field `UART_ICR_DMATXIC` writer - Transmit DMA Interrupt Clear"]
pub type UART_ICR_DMATXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 17>;
impl W {
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rimic(&mut self) -> UART_ICR_RIMIC_W {
        UART_ICR_RIMIC_W::new(self)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_ctsmic(&mut self) -> UART_ICR_CTSMIC_W {
        UART_ICR_CTSMIC_W::new(self)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dcdmic(&mut self) -> UART_ICR_DCDMIC_W {
        UART_ICR_DCDMIC_W::new(self)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dsrmic(&mut self) -> UART_ICR_DSRMIC_W {
        UART_ICR_DSRMIC_W::new(self)
    }
    #[doc = "Bit 4 - Receive Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rxic(&mut self) -> UART_ICR_RXIC_W {
        UART_ICR_RXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_txic(&mut self) -> UART_ICR_TXIC_W {
        UART_ICR_TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_rtic(&mut self) -> UART_ICR_RTIC_W {
        UART_ICR_RTIC_W::new(self)
    }
    #[doc = "Bit 7 - Framing Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_feic(&mut self) -> UART_ICR_FEIC_W {
        UART_ICR_FEIC_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_peic(&mut self) -> UART_ICR_PEIC_W {
        UART_ICR_PEIC_W::new(self)
    }
    #[doc = "Bit 9 - Break Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_beic(&mut self) -> UART_ICR_BEIC_W {
        UART_ICR_BEIC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun Error Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_oeic(&mut self) -> UART_ICR_OEIC_W {
        UART_ICR_OEIC_W::new(self)
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_eotic(&mut self) -> UART_ICR_EOTIC_W {
        UART_ICR_EOTIC_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_9bitic(&mut self) -> UART_ICR_9BITIC_W {
        UART_ICR_9BITIC_W::new(self)
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dmarxic(&mut self) -> UART_ICR_DMARXIC_W {
        UART_ICR_DMARXIC_W::new(self)
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn uart_icr_dmatxic(&mut self) -> UART_ICR_DMATXIC_W {
        UART_ICR_DMATXIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
