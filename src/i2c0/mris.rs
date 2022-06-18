#[doc = "Register `MRIS` reader"]
pub struct R(crate::R<MRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRIS` writer"]
pub struct W(crate::W<MRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRIS_SPEC>;
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
impl From<crate::W<MRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MRIS_RIS` reader - Master Raw Interrupt Status"]
pub type I2C_MRIS_RIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_RIS` writer - Master Raw Interrupt Status"]
pub type I2C_MRIS_RIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 0>;
#[doc = "Field `I2C_MRIS_CLKRIS` reader - Clock Timeout Raw Interrupt Status"]
pub type I2C_MRIS_CLKRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_CLKRIS` writer - Clock Timeout Raw Interrupt Status"]
pub type I2C_MRIS_CLKRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 1>;
#[doc = "Field `I2C_MRIS_DMARXRIS` reader - Receive DMA Raw Interrupt Status"]
pub type I2C_MRIS_DMARXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_DMARXRIS` writer - Receive DMA Raw Interrupt Status"]
pub type I2C_MRIS_DMARXRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 2>;
#[doc = "Field `I2C_MRIS_DMATXRIS` reader - Transmit DMA Raw Interrupt Status"]
pub type I2C_MRIS_DMATXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_DMATXRIS` writer - Transmit DMA Raw Interrupt Status"]
pub type I2C_MRIS_DMATXRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 3>;
#[doc = "Field `I2C_MRIS_NACKRIS` reader - Address/Data NACK Raw Interrupt Status"]
pub type I2C_MRIS_NACKRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_NACKRIS` writer - Address/Data NACK Raw Interrupt Status"]
pub type I2C_MRIS_NACKRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 4>;
#[doc = "Field `I2C_MRIS_STARTRIS` reader - START Detection Raw Interrupt Status"]
pub type I2C_MRIS_STARTRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_STARTRIS` writer - START Detection Raw Interrupt Status"]
pub type I2C_MRIS_STARTRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 5>;
#[doc = "Field `I2C_MRIS_STOPRIS` reader - STOP Detection Raw Interrupt Status"]
pub type I2C_MRIS_STOPRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_STOPRIS` writer - STOP Detection Raw Interrupt Status"]
pub type I2C_MRIS_STOPRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 6>;
#[doc = "Field `I2C_MRIS_ARBLOSTRIS` reader - Arbitration Lost Raw Interrupt Status"]
pub type I2C_MRIS_ARBLOSTRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_ARBLOSTRIS` writer - Arbitration Lost Raw Interrupt Status"]
pub type I2C_MRIS_ARBLOSTRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 7>;
#[doc = "Field `I2C_MRIS_TXRIS` reader - Transmit Request Raw Interrupt Status"]
pub type I2C_MRIS_TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_TXRIS` writer - Transmit Request Raw Interrupt Status"]
pub type I2C_MRIS_TXRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 8>;
#[doc = "Field `I2C_MRIS_RXRIS` reader - Receive FIFO Request Raw Interrupt Status"]
pub type I2C_MRIS_RXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_RXRIS` writer - Receive FIFO Request Raw Interrupt Status"]
pub type I2C_MRIS_RXRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 9>;
#[doc = "Field `I2C_MRIS_TXFERIS` reader - Transmit FIFO Empty Raw Interrupt Status"]
pub type I2C_MRIS_TXFERIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_TXFERIS` writer - Transmit FIFO Empty Raw Interrupt Status"]
pub type I2C_MRIS_TXFERIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 10>;
#[doc = "Field `I2C_MRIS_RXFFRIS` reader - Receive FIFO Full Raw Interrupt Status"]
pub type I2C_MRIS_RXFFRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MRIS_RXFFRIS` writer - Receive FIFO Full Raw Interrupt Status"]
pub type I2C_MRIS_RXFFRIS_W<'a> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_ris(&self) -> I2C_MRIS_RIS_R {
        I2C_MRIS_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_clkris(&self) -> I2C_MRIS_CLKRIS_R {
        I2C_MRIS_CLKRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmarxris(&self) -> I2C_MRIS_DMARXRIS_R {
        I2C_MRIS_DMARXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmatxris(&self) -> I2C_MRIS_DMATXRIS_R {
        I2C_MRIS_DMATXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address/Data NACK Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_nackris(&self) -> I2C_MRIS_NACKRIS_R {
        I2C_MRIS_NACKRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_startris(&self) -> I2C_MRIS_STARTRIS_R {
        I2C_MRIS_STARTRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_stopris(&self) -> I2C_MRIS_STOPRIS_R {
        I2C_MRIS_STOPRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_arblostris(&self) -> I2C_MRIS_ARBLOSTRIS_R {
        I2C_MRIS_ARBLOSTRIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txris(&self) -> I2C_MRIS_TXRIS_R {
        I2C_MRIS_TXRIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxris(&self) -> I2C_MRIS_RXRIS_R {
        I2C_MRIS_RXRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txferis(&self) -> I2C_MRIS_TXFERIS_R {
        I2C_MRIS_TXFERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxffris(&self) -> I2C_MRIS_RXFFRIS_R {
        I2C_MRIS_RXFFRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_ris(&mut self) -> I2C_MRIS_RIS_W {
        I2C_MRIS_RIS_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_clkris(&mut self) -> I2C_MRIS_CLKRIS_W {
        I2C_MRIS_CLKRIS_W::new(self)
    }
    #[doc = "Bit 2 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmarxris(&mut self) -> I2C_MRIS_DMARXRIS_W {
        I2C_MRIS_DMARXRIS_W::new(self)
    }
    #[doc = "Bit 3 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_dmatxris(&mut self) -> I2C_MRIS_DMATXRIS_W {
        I2C_MRIS_DMATXRIS_W::new(self)
    }
    #[doc = "Bit 4 - Address/Data NACK Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_nackris(&mut self) -> I2C_MRIS_NACKRIS_W {
        I2C_MRIS_NACKRIS_W::new(self)
    }
    #[doc = "Bit 5 - START Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_startris(&mut self) -> I2C_MRIS_STARTRIS_W {
        I2C_MRIS_STARTRIS_W::new(self)
    }
    #[doc = "Bit 6 - STOP Detection Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_stopris(&mut self) -> I2C_MRIS_STOPRIS_W {
        I2C_MRIS_STOPRIS_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_arblostris(&mut self) -> I2C_MRIS_ARBLOSTRIS_W {
        I2C_MRIS_ARBLOSTRIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txris(&mut self) -> I2C_MRIS_TXRIS_W {
        I2C_MRIS_TXRIS_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxris(&mut self) -> I2C_MRIS_RXRIS_W {
        I2C_MRIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_txferis(&mut self) -> I2C_MRIS_TXFERIS_W {
        I2C_MRIS_TXFERIS_W::new(self)
    }
    #[doc = "Bit 11 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_rxffris(&mut self) -> I2C_MRIS_RXFFRIS_W {
        I2C_MRIS_RXFFRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mris](index.html) module"]
pub struct MRIS_SPEC;
impl crate::RegisterSpec for MRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mris::R](R) reader structure"]
impl crate::Readable for MRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mris::W](W) writer structure"]
impl crate::Writable for MRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MRIS to value 0"]
impl crate::Resettable for MRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
