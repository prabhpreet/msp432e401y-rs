#[doc = "Register `SRIS` reader"]
pub struct R(crate::R<SRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRIS` writer"]
pub struct W(crate::W<SRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRIS_SPEC>;
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
impl From<crate::W<SRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SRIS_DATARIS` reader - Data Raw Interrupt Status"]
pub type I2C_SRIS_DATARIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_DATARIS` writer - Data Raw Interrupt Status"]
pub type I2C_SRIS_DATARIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 0>;
#[doc = "Field `I2C_SRIS_STARTRIS` reader - Start Condition Raw Interrupt Status"]
pub type I2C_SRIS_STARTRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_STARTRIS` writer - Start Condition Raw Interrupt Status"]
pub type I2C_SRIS_STARTRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 1>;
#[doc = "Field `I2C_SRIS_STOPRIS` reader - Stop Condition Raw Interrupt Status"]
pub type I2C_SRIS_STOPRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_STOPRIS` writer - Stop Condition Raw Interrupt Status"]
pub type I2C_SRIS_STOPRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 2>;
#[doc = "Field `I2C_SRIS_DMARXRIS` reader - Receive DMA Raw Interrupt Status"]
pub type I2C_SRIS_DMARXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_DMARXRIS` writer - Receive DMA Raw Interrupt Status"]
pub type I2C_SRIS_DMARXRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 3>;
#[doc = "Field `I2C_SRIS_DMATXRIS` reader - Transmit DMA Raw Interrupt Status"]
pub type I2C_SRIS_DMATXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_DMATXRIS` writer - Transmit DMA Raw Interrupt Status"]
pub type I2C_SRIS_DMATXRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 4>;
#[doc = "Field `I2C_SRIS_TXRIS` reader - Transmit Request Raw Interrupt Status"]
pub type I2C_SRIS_TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_TXRIS` writer - Transmit Request Raw Interrupt Status"]
pub type I2C_SRIS_TXRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 5>;
#[doc = "Field `I2C_SRIS_RXRIS` reader - Receive FIFO Request Raw Interrupt Status"]
pub type I2C_SRIS_RXRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_RXRIS` writer - Receive FIFO Request Raw Interrupt Status"]
pub type I2C_SRIS_RXRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 6>;
#[doc = "Field `I2C_SRIS_TXFERIS` reader - Transmit FIFO Empty Raw Interrupt Status"]
pub type I2C_SRIS_TXFERIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_TXFERIS` writer - Transmit FIFO Empty Raw Interrupt Status"]
pub type I2C_SRIS_TXFERIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 7>;
#[doc = "Field `I2C_SRIS_RXFFRIS` reader - Receive FIFO Full Raw Interrupt Status"]
pub type I2C_SRIS_RXFFRIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SRIS_RXFFRIS` writer - Receive FIFO Full Raw Interrupt Status"]
pub type I2C_SRIS_RXFFRIS_W<'a> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dataris(&self) -> I2C_SRIS_DATARIS_R {
        I2C_SRIS_DATARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_startris(&self) -> I2C_SRIS_STARTRIS_R {
        I2C_SRIS_STARTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_stopris(&self) -> I2C_SRIS_STOPRIS_R {
        I2C_SRIS_STOPRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmarxris(&self) -> I2C_SRIS_DMARXRIS_R {
        I2C_SRIS_DMARXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmatxris(&self) -> I2C_SRIS_DMATXRIS_R {
        I2C_SRIS_DMATXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txris(&self) -> I2C_SRIS_TXRIS_R {
        I2C_SRIS_TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxris(&self) -> I2C_SRIS_RXRIS_R {
        I2C_SRIS_RXRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txferis(&self) -> I2C_SRIS_TXFERIS_R {
        I2C_SRIS_TXFERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxffris(&self) -> I2C_SRIS_RXFFRIS_R {
        I2C_SRIS_RXFFRIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dataris(&mut self) -> I2C_SRIS_DATARIS_W {
        I2C_SRIS_DATARIS_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_startris(&mut self) -> I2C_SRIS_STARTRIS_W {
        I2C_SRIS_STARTRIS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_stopris(&mut self) -> I2C_SRIS_STOPRIS_W {
        I2C_SRIS_STOPRIS_W::new(self)
    }
    #[doc = "Bit 3 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmarxris(&mut self) -> I2C_SRIS_DMARXRIS_W {
        I2C_SRIS_DMARXRIS_W::new(self)
    }
    #[doc = "Bit 4 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dmatxris(&mut self) -> I2C_SRIS_DMATXRIS_W {
        I2C_SRIS_DMATXRIS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txris(&mut self) -> I2C_SRIS_TXRIS_W {
        I2C_SRIS_TXRIS_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Request Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxris(&mut self) -> I2C_SRIS_RXRIS_W {
        I2C_SRIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_txferis(&mut self) -> I2C_SRIS_TXFERIS_W {
        I2C_SRIS_TXFERIS_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Full Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_rxffris(&mut self) -> I2C_SRIS_RXFFRIS_W {
        I2C_SRIS_RXFFRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sris](index.html) module"]
pub struct SRIS_SPEC;
impl crate::RegisterSpec for SRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sris::R](R) reader structure"]
impl crate::Readable for SRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sris::W](W) writer structure"]
impl crate::Writable for SRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRIS to value 0"]
impl crate::Resettable for SRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
