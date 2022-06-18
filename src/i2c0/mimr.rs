#[doc = "Register `MIMR` reader"]
pub struct R(crate::R<MIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIMR` writer"]
pub struct W(crate::W<MIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIMR_SPEC>;
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
impl From<crate::W<MIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MIMR_IM` reader - Master Interrupt Mask"]
pub type I2C_MIMR_IM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_IM` writer - Master Interrupt Mask"]
pub type I2C_MIMR_IM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 0>;
#[doc = "Field `I2C_MIMR_CLKIM` reader - Clock Timeout Interrupt Mask"]
pub type I2C_MIMR_CLKIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_CLKIM` writer - Clock Timeout Interrupt Mask"]
pub type I2C_MIMR_CLKIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 1>;
#[doc = "Field `I2C_MIMR_DMARXIM` reader - Receive DMA Interrupt Mask"]
pub type I2C_MIMR_DMARXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_DMARXIM` writer - Receive DMA Interrupt Mask"]
pub type I2C_MIMR_DMARXIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 2>;
#[doc = "Field `I2C_MIMR_DMATXIM` reader - Transmit DMA Interrupt Mask"]
pub type I2C_MIMR_DMATXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_DMATXIM` writer - Transmit DMA Interrupt Mask"]
pub type I2C_MIMR_DMATXIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 3>;
#[doc = "Field `I2C_MIMR_NACKIM` reader - Address/Data NACK Interrupt Mask"]
pub type I2C_MIMR_NACKIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_NACKIM` writer - Address/Data NACK Interrupt Mask"]
pub type I2C_MIMR_NACKIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 4>;
#[doc = "Field `I2C_MIMR_STARTIM` reader - START Detection Interrupt Mask"]
pub type I2C_MIMR_STARTIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_STARTIM` writer - START Detection Interrupt Mask"]
pub type I2C_MIMR_STARTIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 5>;
#[doc = "Field `I2C_MIMR_STOPIM` reader - STOP Detection Interrupt Mask"]
pub type I2C_MIMR_STOPIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_STOPIM` writer - STOP Detection Interrupt Mask"]
pub type I2C_MIMR_STOPIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 6>;
#[doc = "Field `I2C_MIMR_ARBLOSTIM` reader - Arbitration Lost Interrupt Mask"]
pub type I2C_MIMR_ARBLOSTIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_ARBLOSTIM` writer - Arbitration Lost Interrupt Mask"]
pub type I2C_MIMR_ARBLOSTIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 7>;
#[doc = "Field `I2C_MIMR_TXIM` reader - Transmit FIFO Request Interrupt Mask"]
pub type I2C_MIMR_TXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_TXIM` writer - Transmit FIFO Request Interrupt Mask"]
pub type I2C_MIMR_TXIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 8>;
#[doc = "Field `I2C_MIMR_RXIM` reader - Receive FIFO Request Interrupt Mask"]
pub type I2C_MIMR_RXIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_RXIM` writer - Receive FIFO Request Interrupt Mask"]
pub type I2C_MIMR_RXIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 9>;
#[doc = "Field `I2C_MIMR_TXFEIM` reader - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_MIMR_TXFEIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_TXFEIM` writer - Transmit FIFO Empty Interrupt Mask"]
pub type I2C_MIMR_TXFEIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 10>;
#[doc = "Field `I2C_MIMR_RXFFIM` reader - Receive FIFO Full Interrupt Mask"]
pub type I2C_MIMR_RXFFIM_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MIMR_RXFFIM` writer - Receive FIFO Full Interrupt Mask"]
pub type I2C_MIMR_RXFFIM_W<'a> = crate::BitWriter<'a, u32, MIMR_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&self) -> I2C_MIMR_IM_R {
        I2C_MIMR_IM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_clkim(&self) -> I2C_MIMR_CLKIM_R {
        I2C_MIMR_CLKIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmarxim(&self) -> I2C_MIMR_DMARXIM_R {
        I2C_MIMR_DMARXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmatxim(&self) -> I2C_MIMR_DMATXIM_R {
        I2C_MIMR_DMATXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_nackim(&self) -> I2C_MIMR_NACKIM_R {
        I2C_MIMR_NACKIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_startim(&self) -> I2C_MIMR_STARTIM_R {
        I2C_MIMR_STARTIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_stopim(&self) -> I2C_MIMR_STOPIM_R {
        I2C_MIMR_STOPIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_arblostim(&self) -> I2C_MIMR_ARBLOSTIM_R {
        I2C_MIMR_ARBLOSTIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txim(&self) -> I2C_MIMR_TXIM_R {
        I2C_MIMR_TXIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxim(&self) -> I2C_MIMR_RXIM_R {
        I2C_MIMR_RXIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txfeim(&self) -> I2C_MIMR_TXFEIM_R {
        I2C_MIMR_TXFEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxffim(&self) -> I2C_MIMR_RXFFIM_R {
        I2C_MIMR_RXFFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&mut self) -> I2C_MIMR_IM_W {
        I2C_MIMR_IM_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_clkim(&mut self) -> I2C_MIMR_CLKIM_W {
        I2C_MIMR_CLKIM_W::new(self)
    }
    #[doc = "Bit 2 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmarxim(&mut self) -> I2C_MIMR_DMARXIM_W {
        I2C_MIMR_DMARXIM_W::new(self)
    }
    #[doc = "Bit 3 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_dmatxim(&mut self) -> I2C_MIMR_DMATXIM_W {
        I2C_MIMR_DMATXIM_W::new(self)
    }
    #[doc = "Bit 4 - Address/Data NACK Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_nackim(&mut self) -> I2C_MIMR_NACKIM_W {
        I2C_MIMR_NACKIM_W::new(self)
    }
    #[doc = "Bit 5 - START Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_startim(&mut self) -> I2C_MIMR_STARTIM_W {
        I2C_MIMR_STARTIM_W::new(self)
    }
    #[doc = "Bit 6 - STOP Detection Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_stopim(&mut self) -> I2C_MIMR_STOPIM_W {
        I2C_MIMR_STOPIM_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_arblostim(&mut self) -> I2C_MIMR_ARBLOSTIM_W {
        I2C_MIMR_ARBLOSTIM_W::new(self)
    }
    #[doc = "Bit 8 - Transmit FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txim(&mut self) -> I2C_MIMR_TXIM_W {
        I2C_MIMR_TXIM_W::new(self)
    }
    #[doc = "Bit 9 - Receive FIFO Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxim(&mut self) -> I2C_MIMR_RXIM_W {
        I2C_MIMR_RXIM_W::new(self)
    }
    #[doc = "Bit 10 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_txfeim(&mut self) -> I2C_MIMR_TXFEIM_W {
        I2C_MIMR_TXFEIM_W::new(self)
    }
    #[doc = "Bit 11 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_rxffim(&mut self) -> I2C_MIMR_RXFFIM_W {
        I2C_MIMR_RXFFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mimr](index.html) module"]
pub struct MIMR_SPEC;
impl crate::RegisterSpec for MIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mimr::R](R) reader structure"]
impl crate::Readable for MIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mimr::W](W) writer structure"]
impl crate::Writable for MIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIMR to value 0"]
impl crate::Resettable for MIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
