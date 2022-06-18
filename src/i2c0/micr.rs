#[doc = "Register `MICR` writer"]
pub struct W(crate::W<MICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MICR_SPEC>;
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
impl From<crate::W<MICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MICR_IC` writer - Master Interrupt Clear"]
pub type I2C_MICR_IC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 0>;
#[doc = "Field `I2C_MICR_CLKIC` writer - Clock Timeout Interrupt Clear"]
pub type I2C_MICR_CLKIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 1>;
#[doc = "Field `I2C_MICR_DMARXIC` writer - Receive DMA Interrupt Clear"]
pub type I2C_MICR_DMARXIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 2>;
#[doc = "Field `I2C_MICR_DMATXIC` writer - Transmit DMA Interrupt Clear"]
pub type I2C_MICR_DMATXIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 3>;
#[doc = "Field `I2C_MICR_NACKIC` writer - Address/Data NACK Interrupt Clear"]
pub type I2C_MICR_NACKIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 4>;
#[doc = "Field `I2C_MICR_STARTIC` writer - START Detection Interrupt Clear"]
pub type I2C_MICR_STARTIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 5>;
#[doc = "Field `I2C_MICR_STOPIC` writer - STOP Detection Interrupt Clear"]
pub type I2C_MICR_STOPIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 6>;
#[doc = "Field `I2C_MICR_ARBLOSTIC` writer - Arbitration Lost Interrupt Clear"]
pub type I2C_MICR_ARBLOSTIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 7>;
#[doc = "Field `I2C_MICR_TXIC` writer - Transmit FIFO Request Interrupt Clear"]
pub type I2C_MICR_TXIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 8>;
#[doc = "Field `I2C_MICR_RXIC` writer - Receive FIFO Request Interrupt Clear"]
pub type I2C_MICR_RXIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 9>;
#[doc = "Field `I2C_MICR_TXFEIC` writer - Transmit FIFO Empty Interrupt Clear"]
pub type I2C_MICR_TXFEIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 10>;
#[doc = "Field `I2C_MICR_RXFFIC` writer - Receive FIFO Full Interrupt Clear"]
pub type I2C_MICR_RXFFIC_W<'a> = crate::BitWriter<'a, u32, MICR_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - Master Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_ic(&mut self) -> I2C_MICR_IC_W {
        I2C_MICR_IC_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_clkic(&mut self) -> I2C_MICR_CLKIC_W {
        I2C_MICR_CLKIC_W::new(self)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_dmarxic(&mut self) -> I2C_MICR_DMARXIC_W {
        I2C_MICR_DMARXIC_W::new(self)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_dmatxic(&mut self) -> I2C_MICR_DMATXIC_W {
        I2C_MICR_DMATXIC_W::new(self)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_nackic(&mut self) -> I2C_MICR_NACKIC_W {
        I2C_MICR_NACKIC_W::new(self)
    }
    #[doc = "Bit 5 - START Detection Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_startic(&mut self) -> I2C_MICR_STARTIC_W {
        I2C_MICR_STARTIC_W::new(self)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_stopic(&mut self) -> I2C_MICR_STOPIC_W {
        I2C_MICR_STOPIC_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_arblostic(&mut self) -> I2C_MICR_ARBLOSTIC_W {
        I2C_MICR_ARBLOSTIC_W::new(self)
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_txic(&mut self) -> I2C_MICR_TXIC_W {
        I2C_MICR_TXIC_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_rxic(&mut self) -> I2C_MICR_RXIC_W {
        I2C_MICR_RXIC_W::new(self)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_txfeic(&mut self) -> I2C_MICR_TXFEIC_W {
        I2C_MICR_TXFEIC_W::new(self)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_micr_rxffic(&mut self) -> I2C_MICR_RXFFIC_W {
        I2C_MICR_RXFFIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](index.html) module"]
pub struct MICR_SPEC;
impl crate::RegisterSpec for MICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [micr::W](W) writer structure"]
impl crate::Writable for MICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
