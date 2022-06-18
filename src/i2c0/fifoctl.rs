#[doc = "Register `FIFOCTL` reader"]
pub struct R(crate::R<FIFOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCTL` writer"]
pub struct W(crate::W<FIFOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCTL_SPEC>;
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
impl From<crate::W<FIFOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_FIFOCTL_TXTRIG` reader - TX FIFO Trigger"]
pub type I2C_FIFOCTL_TXTRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_FIFOCTL_TXTRIG` writer - TX FIFO Trigger"]
pub type I2C_FIFOCTL_TXTRIG_W<'a> = crate::FieldWriter<'a, u32, FIFOCTL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `I2C_FIFOCTL_DMATXENA` reader - DMA TX Channel Enable"]
pub type I2C_FIFOCTL_DMATXENA_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_DMATXENA` writer - DMA TX Channel Enable"]
pub type I2C_FIFOCTL_DMATXENA_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 13>;
#[doc = "Field `I2C_FIFOCTL_TXFLUSH` reader - TX FIFO Flush"]
pub type I2C_FIFOCTL_TXFLUSH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_TXFLUSH` writer - TX FIFO Flush"]
pub type I2C_FIFOCTL_TXFLUSH_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 14>;
#[doc = "Field `I2C_FIFOCTL_TXASGNMT` reader - TX Control Assignment"]
pub type I2C_FIFOCTL_TXASGNMT_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_TXASGNMT` writer - TX Control Assignment"]
pub type I2C_FIFOCTL_TXASGNMT_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 15>;
#[doc = "Field `I2C_FIFOCTL_RXTRIG` reader - RX FIFO Trigger"]
pub type I2C_FIFOCTL_RXTRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_FIFOCTL_RXTRIG` writer - RX FIFO Trigger"]
pub type I2C_FIFOCTL_RXTRIG_W<'a> = crate::FieldWriter<'a, u32, FIFOCTL_SPEC, u8, u8, 3, 16>;
#[doc = "Field `I2C_FIFOCTL_DMARXENA` reader - DMA RX Channel Enable"]
pub type I2C_FIFOCTL_DMARXENA_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_DMARXENA` writer - DMA RX Channel Enable"]
pub type I2C_FIFOCTL_DMARXENA_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 29>;
#[doc = "Field `I2C_FIFOCTL_RXFLUSH` reader - RX FIFO Flush"]
pub type I2C_FIFOCTL_RXFLUSH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_RXFLUSH` writer - RX FIFO Flush"]
pub type I2C_FIFOCTL_RXFLUSH_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 30>;
#[doc = "Field `I2C_FIFOCTL_RXASGNMT` reader - RX Control Assignment"]
pub type I2C_FIFOCTL_RXASGNMT_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOCTL_RXASGNMT` writer - RX Control Assignment"]
pub type I2C_FIFOCTL_RXASGNMT_W<'a> = crate::BitWriter<'a, u32, FIFOCTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_txtrig(&self) -> I2C_FIFOCTL_TXTRIG_R {
        I2C_FIFOCTL_TXTRIG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmatxena(&self) -> I2C_FIFOCTL_DMATXENA_R {
        I2C_FIFOCTL_DMATXENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_txflush(&self) -> I2C_FIFOCTL_TXFLUSH_R {
        I2C_FIFOCTL_TXFLUSH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_txasgnmt(&self) -> I2C_FIFOCTL_TXASGNMT_R {
        I2C_FIFOCTL_TXASGNMT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxtrig(&self) -> I2C_FIFOCTL_RXTRIG_R {
        I2C_FIFOCTL_RXTRIG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmarxena(&self) -> I2C_FIFOCTL_DMARXENA_R {
        I2C_FIFOCTL_DMARXENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxflush(&self) -> I2C_FIFOCTL_RXFLUSH_R {
        I2C_FIFOCTL_RXFLUSH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxasgnmt(&self) -> I2C_FIFOCTL_RXASGNMT_R {
        I2C_FIFOCTL_RXASGNMT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_txtrig(&mut self) -> I2C_FIFOCTL_TXTRIG_W {
        I2C_FIFOCTL_TXTRIG_W::new(self)
    }
    #[doc = "Bit 13 - DMA TX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmatxena(&mut self) -> I2C_FIFOCTL_DMATXENA_W {
        I2C_FIFOCTL_DMATXENA_W::new(self)
    }
    #[doc = "Bit 14 - TX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_txflush(&mut self) -> I2C_FIFOCTL_TXFLUSH_W {
        I2C_FIFOCTL_TXFLUSH_W::new(self)
    }
    #[doc = "Bit 15 - TX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_txasgnmt(&mut self) -> I2C_FIFOCTL_TXASGNMT_W {
        I2C_FIFOCTL_TXASGNMT_W::new(self)
    }
    #[doc = "Bits 16:18 - RX FIFO Trigger"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxtrig(&mut self) -> I2C_FIFOCTL_RXTRIG_W {
        I2C_FIFOCTL_RXTRIG_W::new(self)
    }
    #[doc = "Bit 29 - DMA RX Channel Enable"]
    #[inline(always)]
    pub fn i2c_fifoctl_dmarxena(&mut self) -> I2C_FIFOCTL_DMARXENA_W {
        I2C_FIFOCTL_DMARXENA_W::new(self)
    }
    #[doc = "Bit 30 - RX FIFO Flush"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxflush(&mut self) -> I2C_FIFOCTL_RXFLUSH_W {
        I2C_FIFOCTL_RXFLUSH_W::new(self)
    }
    #[doc = "Bit 31 - RX Control Assignment"]
    #[inline(always)]
    pub fn i2c_fifoctl_rxasgnmt(&mut self) -> I2C_FIFOCTL_RXASGNMT_W {
        I2C_FIFOCTL_RXASGNMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C FIFO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctl](index.html) module"]
pub struct FIFOCTL_SPEC;
impl crate::RegisterSpec for FIFOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoctl::R](R) reader structure"]
impl crate::Readable for FIFOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoctl::W](W) writer structure"]
impl crate::Writable for FIFOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOCTL to value 0"]
impl crate::Resettable for FIFOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
