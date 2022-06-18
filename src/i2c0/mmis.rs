#[doc = "Register `MMIS` reader"]
pub struct R(crate::R<MMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMIS` writer"]
pub struct W(crate::W<MMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMIS_SPEC>;
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
impl From<crate::W<MMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MMIS_MIS` reader - Masked Interrupt Status"]
pub type I2C_MMIS_MIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_MIS` writer - Masked Interrupt Status"]
pub type I2C_MMIS_MIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 0>;
#[doc = "Field `I2C_MMIS_CLKMIS` reader - Clock Timeout Masked Interrupt Status"]
pub type I2C_MMIS_CLKMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_CLKMIS` writer - Clock Timeout Masked Interrupt Status"]
pub type I2C_MMIS_CLKMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 1>;
#[doc = "Field `I2C_MMIS_DMARXMIS` reader - Receive DMA Interrupt Status"]
pub type I2C_MMIS_DMARXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_DMARXMIS` writer - Receive DMA Interrupt Status"]
pub type I2C_MMIS_DMARXMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 2>;
#[doc = "Field `I2C_MMIS_DMATXMIS` reader - Transmit DMA Interrupt Status"]
pub type I2C_MMIS_DMATXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_DMATXMIS` writer - Transmit DMA Interrupt Status"]
pub type I2C_MMIS_DMATXMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 3>;
#[doc = "Field `I2C_MMIS_NACKMIS` reader - Address/Data NACK Interrupt Mask"]
pub type I2C_MMIS_NACKMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_NACKMIS` writer - Address/Data NACK Interrupt Mask"]
pub type I2C_MMIS_NACKMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 4>;
#[doc = "Field `I2C_MMIS_STARTMIS` reader - START Detection Interrupt Mask"]
pub type I2C_MMIS_STARTMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_STARTMIS` writer - START Detection Interrupt Mask"]
pub type I2C_MMIS_STARTMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 5>;
#[doc = "Field `I2C_MMIS_STOPMIS` reader - STOP Detection Interrupt Mask"]
pub type I2C_MMIS_STOPMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_STOPMIS` writer - STOP Detection Interrupt Mask"]
pub type I2C_MMIS_STOPMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 6>;
#[doc = "Field `I2C_MMIS_ARBLOSTMIS` reader - Arbitration Lost Interrupt Mask"]
pub type I2C_MMIS_ARBLOSTMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_ARBLOSTMIS` writer - Arbitration Lost Interrupt Mask"]
pub type I2C_MMIS_ARBLOSTMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 7>;
#[doc = "Field `I2C_MMIS_TXMIS` reader - Transmit Request Interrupt Mask"]
pub type I2C_MMIS_TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_TXMIS` writer - Transmit Request Interrupt Mask"]
pub type I2C_MMIS_TXMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 8>;
#[doc = "Field `I2C_MMIS_RXMIS` reader - Receive FIFO Request Interrupt Mask"]
pub type I2C_MMIS_RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_RXMIS` writer - Receive FIFO Request Interrupt Mask"]
pub type I2C_MMIS_RXMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 9>;
#[doc = "Field `I2C_MMIS_TXFEMIS` reader - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_MMIS_TXFEMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_TXFEMIS` writer - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_MMIS_TXFEMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 10>;
#[doc = "Field `I2C_MMIS_RXFFMIS` reader - Receive FIFO Full Interrupt Mask"]
pub type I2C_MMIS_RXFFMIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MMIS_RXFFMIS` writer - Receive FIFO Full Interrupt Mask"]
pub type I2C_MMIS_RXFFMIS_W<'a> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_mis(&self) -> I2C_MMIS_MIS_R {
        I2C_MMIS_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_clkmis(&self) -> I2C_MMIS_CLKMIS_R {
        I2C_MMIS_CLKMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmarxmis(&self) -> I2C_MMIS_DMARXMIS_R {
        I2C_MMIS_DMARXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmatxmis(&self) -> I2C_MMIS_DMATXMIS_R {
        I2C_MMIS_DMATXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_nackmis(&self) -> I2C_MMIS_NACKMIS_R {
        I2C_MMIS_NACKMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_startmis(&self) -> I2C_MMIS_STARTMIS_R {
        I2C_MMIS_STARTMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_stopmis(&self) -> I2C_MMIS_STOPMIS_R {
        I2C_MMIS_STOPMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_arblostmis(&self) -> I2C_MMIS_ARBLOSTMIS_R {
        I2C_MMIS_ARBLOSTMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txmis(&self) -> I2C_MMIS_TXMIS_R {
        I2C_MMIS_TXMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxmis(&self) -> I2C_MMIS_RXMIS_R {
        I2C_MMIS_RXMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txfemis(&self) -> I2C_MMIS_TXFEMIS_R {
        I2C_MMIS_TXFEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxffmis(&self) -> I2C_MMIS_RXFFMIS_R {
        I2C_MMIS_RXFFMIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_mis(&mut self) -> I2C_MMIS_MIS_W {
        I2C_MMIS_MIS_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_clkmis(&mut self) -> I2C_MMIS_CLKMIS_W {
        I2C_MMIS_CLKMIS_W::new(self)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmarxmis(&mut self) -> I2C_MMIS_DMARXMIS_W {
        I2C_MMIS_DMARXMIS_W::new(self)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_dmatxmis(&mut self) -> I2C_MMIS_DMATXMIS_W {
        I2C_MMIS_DMATXMIS_W::new(self)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_nackmis(&mut self) -> I2C_MMIS_NACKMIS_W {
        I2C_MMIS_NACKMIS_W::new(self)
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_startmis(&mut self) -> I2C_MMIS_STARTMIS_W {
        I2C_MMIS_STARTMIS_W::new(self)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_stopmis(&mut self) -> I2C_MMIS_STOPMIS_W {
        I2C_MMIS_STOPMIS_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_arblostmis(&mut self) -> I2C_MMIS_ARBLOSTMIS_W {
        I2C_MMIS_ARBLOSTMIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txmis(&mut self) -> I2C_MMIS_TXMIS_W {
        I2C_MMIS_TXMIS_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxmis(&mut self) -> I2C_MMIS_RXMIS_W {
        I2C_MMIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_txfemis(&mut self) -> I2C_MMIS_TXFEMIS_W {
        I2C_MMIS_TXFEMIS_W::new(self)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mmis_rxffmis(&mut self) -> I2C_MMIS_RXFFMIS_W {
        I2C_MMIS_RXFFMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmis](index.html) module"]
pub struct MMIS_SPEC;
impl crate::RegisterSpec for MMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmis::R](R) reader structure"]
impl crate::Readable for MMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmis::W](W) writer structure"]
impl crate::Writable for MMIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMIS to value 0"]
impl crate::Resettable for MMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
