#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_RIS_RIRIS` reader - UART Ring Indicator Modem Raw Interrupt Status"]
pub type UART_RIS_RIRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_RIRIS` writer - UART Ring Indicator Modem Raw Interrupt Status"]
pub type UART_RIS_RIRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `UART_RIS_CTSRIS` reader - UART Clear to Send Modem Raw Interrupt Status"]
pub type UART_RIS_CTSRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_CTSRIS` writer - UART Clear to Send Modem Raw Interrupt Status"]
pub type UART_RIS_CTSRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `UART_RIS_DCDRIS` reader - UART Data Carrier Detect Modem Raw Interrupt Status"]
pub type UART_RIS_DCDRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_DCDRIS` writer - UART Data Carrier Detect Modem Raw Interrupt Status"]
pub type UART_RIS_DCDRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `UART_RIS_DSRRIS` reader - UART Data Set Ready Modem Raw Interrupt Status"]
pub type UART_RIS_DSRRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_DSRRIS` writer - UART Data Set Ready Modem Raw Interrupt Status"]
pub type UART_RIS_DSRRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `UART_RIS_RXRIS` reader - UART Receive Raw Interrupt Status"]
pub type UART_RIS_RXRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_RXRIS` writer - UART Receive Raw Interrupt Status"]
pub type UART_RIS_RXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `UART_RIS_TXRIS` reader - UART Transmit Raw Interrupt Status"]
pub type UART_RIS_TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_TXRIS` writer - UART Transmit Raw Interrupt Status"]
pub type UART_RIS_TXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
#[doc = "Field `UART_RIS_RTRIS` reader - UART Receive Time-Out Raw Interrupt Status"]
pub type UART_RIS_RTRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_RTRIS` writer - UART Receive Time-Out Raw Interrupt Status"]
pub type UART_RIS_RTRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 6>;
#[doc = "Field `UART_RIS_FERIS` reader - UART Framing Error Raw Interrupt Status"]
pub type UART_RIS_FERIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_FERIS` writer - UART Framing Error Raw Interrupt Status"]
pub type UART_RIS_FERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 7>;
#[doc = "Field `UART_RIS_PERIS` reader - UART Parity Error Raw Interrupt Status"]
pub type UART_RIS_PERIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_PERIS` writer - UART Parity Error Raw Interrupt Status"]
pub type UART_RIS_PERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 8>;
#[doc = "Field `UART_RIS_BERIS` reader - UART Break Error Raw Interrupt Status"]
pub type UART_RIS_BERIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_BERIS` writer - UART Break Error Raw Interrupt Status"]
pub type UART_RIS_BERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 9>;
#[doc = "Field `UART_RIS_OERIS` reader - UART Overrun Error Raw Interrupt Status"]
pub type UART_RIS_OERIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_OERIS` writer - UART Overrun Error Raw Interrupt Status"]
pub type UART_RIS_OERIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 10>;
#[doc = "Field `UART_RIS_EOTRIS` reader - End of Transmission Raw Interrupt Status"]
pub type UART_RIS_EOTRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_EOTRIS` writer - End of Transmission Raw Interrupt Status"]
pub type UART_RIS_EOTRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 11>;
#[doc = "Field `UART_RIS_9BITRIS` reader - 9-Bit Mode Raw Interrupt Status"]
pub type UART_RIS_9BITRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_9BITRIS` writer - 9-Bit Mode Raw Interrupt Status"]
pub type UART_RIS_9BITRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 12>;
#[doc = "Field `UART_RIS_DMARXRIS` reader - Receive DMA Raw Interrupt Status"]
pub type UART_RIS_DMARXRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_DMARXRIS` writer - Receive DMA Raw Interrupt Status"]
pub type UART_RIS_DMARXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 16>;
#[doc = "Field `UART_RIS_DMATXRIS` reader - Transmit DMA Raw Interrupt Status"]
pub type UART_RIS_DMATXRIS_R = crate::BitReader<bool>;
#[doc = "Field `UART_RIS_DMATXRIS` writer - Transmit DMA Raw Interrupt Status"]
pub type UART_RIS_DMATXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_riris(&self) -> UART_RIS_RIRIS_R {
        UART_RIS_RIRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_ctsris(&self) -> UART_RIS_CTSRIS_R {
        UART_RIS_CTSRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dcdris(&self) -> UART_RIS_DCDRIS_R {
        UART_RIS_DCDRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dsrris(&self) -> UART_RIS_DSRRIS_R {
        UART_RIS_DSRRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&self) -> UART_RIS_RXRIS_R {
        UART_RIS_RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&self) -> UART_RIS_TXRIS_R {
        UART_RIS_TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&self) -> UART_RIS_RTRIS_R {
        UART_RIS_RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&self) -> UART_RIS_FERIS_R {
        UART_RIS_FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&self) -> UART_RIS_PERIS_R {
        UART_RIS_PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&self) -> UART_RIS_BERIS_R {
        UART_RIS_BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&self) -> UART_RIS_OERIS_R {
        UART_RIS_OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Transmission Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_eotris(&self) -> UART_RIS_EOTRIS_R {
        UART_RIS_EOTRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_9bitris(&self) -> UART_RIS_9BITRIS_R {
        UART_RIS_9BITRIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmarxris(&self) -> UART_RIS_DMARXRIS_R {
        UART_RIS_DMARXRIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmatxris(&self) -> UART_RIS_DMATXRIS_R {
        UART_RIS_DMATXRIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_riris(&mut self) -> UART_RIS_RIRIS_W {
        UART_RIS_RIRIS_W::new(self)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_ctsris(&mut self) -> UART_RIS_CTSRIS_W {
        UART_RIS_CTSRIS_W::new(self)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dcdris(&mut self) -> UART_RIS_DCDRIS_W {
        UART_RIS_DCDRIS_W::new(self)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dsrris(&mut self) -> UART_RIS_DSRRIS_W {
        UART_RIS_DSRRIS_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&mut self) -> UART_RIS_RXRIS_W {
        UART_RIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&mut self) -> UART_RIS_TXRIS_W {
        UART_RIS_TXRIS_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&mut self) -> UART_RIS_RTRIS_W {
        UART_RIS_RTRIS_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&mut self) -> UART_RIS_FERIS_W {
        UART_RIS_FERIS_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&mut self) -> UART_RIS_PERIS_W {
        UART_RIS_PERIS_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&mut self) -> UART_RIS_BERIS_W {
        UART_RIS_BERIS_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&mut self) -> UART_RIS_OERIS_W {
        UART_RIS_OERIS_W::new(self)
    }
    #[doc = "Bit 11 - End of Transmission Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_eotris(&mut self) -> UART_RIS_EOTRIS_W {
        UART_RIS_EOTRIS_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_9bitris(&mut self) -> UART_RIS_9BITRIS_W {
        UART_RIS_9BITRIS_W::new(self)
    }
    #[doc = "Bit 16 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmarxris(&mut self) -> UART_RIS_DMARXRIS_W {
        UART_RIS_DMARXRIS_W::new(self)
    }
    #[doc = "Bit 17 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmatxris(&mut self) -> UART_RIS_DMATXRIS_W {
        UART_RIS_DMATXRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
