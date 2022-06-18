#[doc = "Register `FR` reader"]
pub struct R(crate::R<FR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR` writer"]
pub struct W(crate::W<FR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR_SPEC>;
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
impl From<crate::W<FR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_FR_CTS` reader - Clear To Send"]
pub type UART_FR_CTS_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_CTS` writer - Clear To Send"]
pub type UART_FR_CTS_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 0>;
#[doc = "Field `UART_FR_DSR` reader - Data Set Ready"]
pub type UART_FR_DSR_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_DSR` writer - Data Set Ready"]
pub type UART_FR_DSR_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 1>;
#[doc = "Field `UART_FR_DCD` reader - Data Carrier Detect"]
pub type UART_FR_DCD_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_DCD` writer - Data Carrier Detect"]
pub type UART_FR_DCD_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 2>;
#[doc = "Field `UART_FR_BUSY` reader - UART Busy"]
pub type UART_FR_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_BUSY` writer - UART Busy"]
pub type UART_FR_BUSY_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 3>;
#[doc = "Field `UART_FR_RXFE` reader - UART Receive FIFO Empty"]
pub type UART_FR_RXFE_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_RXFE` writer - UART Receive FIFO Empty"]
pub type UART_FR_RXFE_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 4>;
#[doc = "Field `UART_FR_TXFF` reader - UART Transmit FIFO Full"]
pub type UART_FR_TXFF_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_TXFF` writer - UART Transmit FIFO Full"]
pub type UART_FR_TXFF_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 5>;
#[doc = "Field `UART_FR_RXFF` reader - UART Receive FIFO Full"]
pub type UART_FR_RXFF_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_RXFF` writer - UART Receive FIFO Full"]
pub type UART_FR_RXFF_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 6>;
#[doc = "Field `UART_FR_TXFE` reader - UART Transmit FIFO Empty"]
pub type UART_FR_TXFE_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_TXFE` writer - UART Transmit FIFO Empty"]
pub type UART_FR_TXFE_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 7>;
#[doc = "Field `UART_FR_RI` reader - Ring Indicator"]
pub type UART_FR_RI_R = crate::BitReader<bool>;
#[doc = "Field `UART_FR_RI` writer - Ring Indicator"]
pub type UART_FR_RI_W<'a> = crate::BitWriter<'a, u32, FR_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn uart_fr_cts(&self) -> UART_FR_CTS_R {
        UART_FR_CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Set Ready"]
    #[inline(always)]
    pub fn uart_fr_dsr(&self) -> UART_FR_DSR_R {
        UART_FR_DSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Carrier Detect"]
    #[inline(always)]
    pub fn uart_fr_dcd(&self) -> UART_FR_DCD_R {
        UART_FR_DCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&self) -> UART_FR_BUSY_R {
        UART_FR_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&self) -> UART_FR_RXFE_R {
        UART_FR_RXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&self) -> UART_FR_TXFF_R {
        UART_FR_TXFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&self) -> UART_FR_RXFF_R {
        UART_FR_RXFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&self) -> UART_FR_TXFE_R {
        UART_FR_TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn uart_fr_ri(&self) -> UART_FR_RI_R {
        UART_FR_RI_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear To Send"]
    #[inline(always)]
    pub fn uart_fr_cts(&mut self) -> UART_FR_CTS_W {
        UART_FR_CTS_W::new(self)
    }
    #[doc = "Bit 1 - Data Set Ready"]
    #[inline(always)]
    pub fn uart_fr_dsr(&mut self) -> UART_FR_DSR_W {
        UART_FR_DSR_W::new(self)
    }
    #[doc = "Bit 2 - Data Carrier Detect"]
    #[inline(always)]
    pub fn uart_fr_dcd(&mut self) -> UART_FR_DCD_W {
        UART_FR_DCD_W::new(self)
    }
    #[doc = "Bit 3 - UART Busy"]
    #[inline(always)]
    pub fn uart_fr_busy(&mut self) -> UART_FR_BUSY_W {
        UART_FR_BUSY_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_rxfe(&mut self) -> UART_FR_RXFE_W {
        UART_FR_RXFE_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_txff(&mut self) -> UART_FR_TXFF_W {
        UART_FR_TXFF_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive FIFO Full"]
    #[inline(always)]
    pub fn uart_fr_rxff(&mut self) -> UART_FR_RXFF_W {
        UART_FR_RXFF_W::new(self)
    }
    #[doc = "Bit 7 - UART Transmit FIFO Empty"]
    #[inline(always)]
    pub fn uart_fr_txfe(&mut self) -> UART_FR_TXFE_W {
        UART_FR_TXFE_W::new(self)
    }
    #[doc = "Bit 8 - Ring Indicator"]
    #[inline(always)]
    pub fn uart_fr_ri(&mut self) -> UART_FR_RI_W {
        UART_FR_RI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](index.html) module"]
pub struct FR_SPEC;
impl crate::RegisterSpec for FR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr::R](R) reader structure"]
impl crate::Readable for FR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr::W](W) writer structure"]
impl crate::Writable for FR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
