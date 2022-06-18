#[doc = "Register `SMIS` reader"]
pub struct R(crate::R<SMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMIS` writer"]
pub struct W(crate::W<SMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMIS_SPEC>;
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
impl From<crate::W<SMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SMIS_DATAMIS` reader - Data Masked Interrupt Status"]
pub type I2C_SMIS_DATAMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_DATAMIS` writer - Data Masked Interrupt Status"]
pub type I2C_SMIS_DATAMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 0>;
#[doc = "Field `I2C_SMIS_STARTMIS` reader - Start Condition Masked Interrupt Status"]
pub type I2C_SMIS_STARTMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_STARTMIS` writer - Start Condition Masked Interrupt Status"]
pub type I2C_SMIS_STARTMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 1>;
#[doc = "Field `I2C_SMIS_STOPMIS` reader - Stop Condition Masked Interrupt Status"]
pub type I2C_SMIS_STOPMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_STOPMIS` writer - Stop Condition Masked Interrupt Status"]
pub type I2C_SMIS_STOPMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 2>;
#[doc = "Field `I2C_SMIS_DMARXMIS` reader - Receive DMA Masked Interrupt Status"]
pub type I2C_SMIS_DMARXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_DMARXMIS` writer - Receive DMA Masked Interrupt Status"]
pub type I2C_SMIS_DMARXMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 3>;
#[doc = "Field `I2C_SMIS_DMATXMIS` reader - Transmit DMA Masked Interrupt Status"]
pub type I2C_SMIS_DMATXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_DMATXMIS` writer - Transmit DMA Masked Interrupt Status"]
pub type I2C_SMIS_DMATXMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 4>;
#[doc = "Field `I2C_SMIS_TXMIS` reader - Transmit FIFO Request Interrupt Mask"]
pub type I2C_SMIS_TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_TXMIS` writer - Transmit FIFO Request Interrupt Mask"]
pub type I2C_SMIS_TXMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 5>;
#[doc = "Field `I2C_SMIS_RXMIS` reader - Receive FIFO Request Interrupt Mask"]
pub type I2C_SMIS_RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_RXMIS` writer - Receive FIFO Request Interrupt Mask"]
pub type I2C_SMIS_RXMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 6>;
#[doc = "Field `I2C_SMIS_TXFEMIS` reader - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_SMIS_TXFEMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_TXFEMIS` writer - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_SMIS_TXFEMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 7>;
#[doc = "Field `I2C_SMIS_RXFFMIS` reader - Receive FIFO Full Interrupt Mask"]
pub type I2C_SMIS_RXFFMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SMIS_RXFFMIS` writer - Receive FIFO Full Interrupt Mask"]
pub type I2C_SMIS_RXFFMIS_W<'a> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&self) -> I2C_SMIS_DATAMIS_R {
        I2C_SMIS_DATAMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_startmis(&self) -> I2C_SMIS_STARTMIS_R {
        I2C_SMIS_STARTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_stopmis(&self) -> I2C_SMIS_STOPMIS_R {
        I2C_SMIS_STOPMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmarxmis(&self) -> I2C_SMIS_DMARXMIS_R {
        I2C_SMIS_DMARXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmatxmis(&self) -> I2C_SMIS_DMATXMIS_R {
        I2C_SMIS_DMATXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txmis(&self) -> I2C_SMIS_TXMIS_R {
        I2C_SMIS_TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxmis(&self) -> I2C_SMIS_RXMIS_R {
        I2C_SMIS_RXMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txfemis(&self) -> I2C_SMIS_TXFEMIS_R {
        I2C_SMIS_TXFEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxffmis(&self) -> I2C_SMIS_RXFFMIS_R {
        I2C_SMIS_RXFFMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&mut self) -> I2C_SMIS_DATAMIS_W {
        I2C_SMIS_DATAMIS_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_startmis(&mut self) -> I2C_SMIS_STARTMIS_W {
        I2C_SMIS_STARTMIS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_stopmis(&mut self) -> I2C_SMIS_STOPMIS_W {
        I2C_SMIS_STOPMIS_W::new(self)
    }
    #[doc = "Bit 3 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmarxmis(&mut self) -> I2C_SMIS_DMARXMIS_W {
        I2C_SMIS_DMARXMIS_W::new(self)
    }
    #[doc = "Bit 4 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_dmatxmis(&mut self) -> I2C_SMIS_DMATXMIS_W {
        I2C_SMIS_DMATXMIS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txmis(&mut self) -> I2C_SMIS_TXMIS_W {
        I2C_SMIS_TXMIS_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxmis(&mut self) -> I2C_SMIS_RXMIS_W {
        I2C_SMIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_txfemis(&mut self) -> I2C_SMIS_TXFEMIS_W {
        I2C_SMIS_TXFEMIS_W::new(self)
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_smis_rxffmis(&mut self) -> I2C_SMIS_RXFFMIS_W {
        I2C_SMIS_RXFFMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](index.html) module"]
pub struct SMIS_SPEC;
impl crate::RegisterSpec for SMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smis::R](R) reader structure"]
impl crate::Readable for SMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smis::W](W) writer structure"]
impl crate::Writable for SMIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMIS to value 0"]
impl crate::Resettable for SMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
