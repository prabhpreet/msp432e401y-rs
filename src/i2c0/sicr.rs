#[doc = "Register `SICR` writer"]
pub struct W(crate::W<SICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SICR_SPEC>;
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
impl From<crate::W<SICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SICR_DATAIC` writer - Data Interrupt Clear"]
pub type I2C_SICR_DATAIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 0>;
#[doc = "Field `I2C_SICR_STARTIC` writer - Start Condition Interrupt Clear"]
pub type I2C_SICR_STARTIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 1>;
#[doc = "Field `I2C_SICR_STOPIC` writer - Stop Condition Interrupt Clear"]
pub type I2C_SICR_STOPIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 2>;
#[doc = "Field `I2C_SICR_DMARXIC` writer - Receive DMA Interrupt Clear"]
pub type I2C_SICR_DMARXIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 3>;
#[doc = "Field `I2C_SICR_DMATXIC` writer - Transmit DMA Interrupt Clear"]
pub type I2C_SICR_DMATXIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 4>;
#[doc = "Field `I2C_SICR_TXIC` writer - Transmit Request Interrupt Mask"]
pub type I2C_SICR_TXIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 5>;
#[doc = "Field `I2C_SICR_RXIC` writer - Receive Request Interrupt Mask"]
pub type I2C_SICR_RXIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 6>;
#[doc = "Field `I2C_SICR_TXFEIC` writer - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_SICR_TXFEIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 7>;
#[doc = "Field `I2C_SICR_RXFFIC` writer - Receive FIFO Full Interrupt Mask"]
pub type I2C_SICR_RXFFIC_W<'a> = crate::BitWriter<'a, u32, SICR_SPEC, bool, 8>;
impl W {
    #[doc = "Bit 0 - Data Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dataic(&mut self) -> I2C_SICR_DATAIC_W {
        I2C_SICR_DATAIC_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_startic(&mut self) -> I2C_SICR_STARTIC_W {
        I2C_SICR_STARTIC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_stopic(&mut self) -> I2C_SICR_STOPIC_W {
        I2C_SICR_STOPIC_W::new(self)
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dmarxic(&mut self) -> I2C_SICR_DMARXIC_W {
        I2C_SICR_DMARXIC_W::new(self)
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dmatxic(&mut self) -> I2C_SICR_DMATXIC_W {
        I2C_SICR_DMATXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_txic(&mut self) -> I2C_SICR_TXIC_W {
        I2C_SICR_TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_rxic(&mut self) -> I2C_SICR_RXIC_W {
        I2C_SICR_RXIC_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_txfeic(&mut self) -> I2C_SICR_TXFEIC_W {
        I2C_SICR_TXFEIC_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_rxffic(&mut self) -> I2C_SICR_RXFFIC_W {
        I2C_SICR_RXFFIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](index.html) module"]
pub struct SICR_SPEC;
impl crate::RegisterSpec for SICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sicr::W](W) writer structure"]
impl crate::Writable for SICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
