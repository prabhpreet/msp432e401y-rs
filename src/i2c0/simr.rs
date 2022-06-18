#[doc = "Register `SIMR` reader"]
pub struct R(crate::R<SIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIMR` writer"]
pub struct W(crate::W<SIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIMR_SPEC>;
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
impl From<crate::W<SIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SIMR_DATAIM` reader - Data Interrupt Mask"]
pub type I2C_SIMR_DATAIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_DATAIM` writer - Data Interrupt Mask"]
pub type I2C_SIMR_DATAIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 0>;
#[doc = "Field `I2C_SIMR_STARTIM` reader - Start Condition Interrupt Mask"]
pub type I2C_SIMR_STARTIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_STARTIM` writer - Start Condition Interrupt Mask"]
pub type I2C_SIMR_STARTIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 1>;
#[doc = "Field `I2C_SIMR_STOPIM` reader - Stop Condition Interrupt Mask"]
pub type I2C_SIMR_STOPIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_STOPIM` writer - Stop Condition Interrupt Mask"]
pub type I2C_SIMR_STOPIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 2>;
#[doc = "Field `I2C_SIMR_DMARXIM` reader - Receive DMA Interrupt Mask"]
pub type I2C_SIMR_DMARXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_DMARXIM` writer - Receive DMA Interrupt Mask"]
pub type I2C_SIMR_DMARXIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 3>;
#[doc = "Field `I2C_SIMR_DMATXIM` reader - Transmit DMA Interrupt Mask"]
pub type I2C_SIMR_DMATXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_DMATXIM` writer - Transmit DMA Interrupt Mask"]
pub type I2C_SIMR_DMATXIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 4>;
#[doc = "Field `I2C_SIMR_TXIM` reader - Transmit FIFO Request Interrupt Mask"]
pub type I2C_SIMR_TXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_TXIM` writer - Transmit FIFO Request Interrupt Mask"]
pub type I2C_SIMR_TXIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 5>;
#[doc = "Field `I2C_SIMR_RXIM` reader - Receive FIFO Request Interrupt Mask"]
pub type I2C_SIMR_RXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_RXIM` writer - Receive FIFO Request Interrupt Mask"]
pub type I2C_SIMR_RXIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 6>;
#[doc = "Field `I2C_SIMR_TXFEIM` reader - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_SIMR_TXFEIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_TXFEIM` writer - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_SIMR_TXFEIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 7>;
#[doc = "Field `I2C_SIMR_RXFFIM` reader - Receive FIFO Full Interrupt Mask"]
pub type I2C_SIMR_RXFFIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SIMR_RXFFIM` writer - Receive FIFO Full Interrupt Mask"]
pub type I2C_SIMR_RXFFIM_W<'a> = crate::BitWriter<'a, u32, SIMR_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&self) -> I2C_SIMR_DATAIM_R {
        I2C_SIMR_DATAIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_startim(&self) -> I2C_SIMR_STARTIM_R {
        I2C_SIMR_STARTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_stopim(&self) -> I2C_SIMR_STOPIM_R {
        I2C_SIMR_STOPIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmarxim(&self) -> I2C_SIMR_DMARXIM_R {
        I2C_SIMR_DMARXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmatxim(&self) -> I2C_SIMR_DMATXIM_R {
        I2C_SIMR_DMATXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txim(&self) -> I2C_SIMR_TXIM_R {
        I2C_SIMR_TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxim(&self) -> I2C_SIMR_RXIM_R {
        I2C_SIMR_RXIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txfeim(&self) -> I2C_SIMR_TXFEIM_R {
        I2C_SIMR_TXFEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxffim(&self) -> I2C_SIMR_RXFFIM_R {
        I2C_SIMR_RXFFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&mut self) -> I2C_SIMR_DATAIM_W {
        I2C_SIMR_DATAIM_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_startim(&mut self) -> I2C_SIMR_STARTIM_W {
        I2C_SIMR_STARTIM_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_stopim(&mut self) -> I2C_SIMR_STOPIM_W {
        I2C_SIMR_STOPIM_W::new(self)
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmarxim(&mut self) -> I2C_SIMR_DMARXIM_W {
        I2C_SIMR_DMARXIM_W::new(self)
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dmatxim(&mut self) -> I2C_SIMR_DMATXIM_W {
        I2C_SIMR_DMATXIM_W::new(self)
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txim(&mut self) -> I2C_SIMR_TXIM_W {
        I2C_SIMR_TXIM_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxim(&mut self) -> I2C_SIMR_RXIM_W {
        I2C_SIMR_RXIM_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_txfeim(&mut self) -> I2C_SIMR_TXFEIM_W {
        I2C_SIMR_TXFEIM_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_rxffim(&mut self) -> I2C_SIMR_RXFFIM_W {
        I2C_SIMR_RXFFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr](index.html) module"]
pub struct SIMR_SPEC;
impl crate::RegisterSpec for SIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [simr::R](R) reader structure"]
impl crate::Readable for SIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [simr::W](W) writer structure"]
impl crate::Writable for SIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIMR to value 0"]
impl crate::Resettable for SIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
