#[doc = "Register `FIFOSTATUS` reader"]
pub struct R(crate::R<FIFOSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOSTATUS` writer"]
pub struct W(crate::W<FIFOSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOSTATUS_SPEC>;
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
impl From<crate::W<FIFOSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_FIFOSTATUS_TXFE` reader - TX FIFO Empty"]
pub type I2C_FIFOSTATUS_TXFE_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_TXFE` writer - TX FIFO Empty"]
pub type I2C_FIFOSTATUS_TXFE_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 0>;
#[doc = "Field `I2C_FIFOSTATUS_TXFF` reader - TX FIFO Full"]
pub type I2C_FIFOSTATUS_TXFF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_TXFF` writer - TX FIFO Full"]
pub type I2C_FIFOSTATUS_TXFF_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 1>;
#[doc = "Field `I2C_FIFOSTATUS_TXBLWTRIG` reader - TX FIFO Below Trigger Level"]
pub type I2C_FIFOSTATUS_TXBLWTRIG_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_TXBLWTRIG` writer - TX FIFO Below Trigger Level"]
pub type I2C_FIFOSTATUS_TXBLWTRIG_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 2>;
#[doc = "Field `I2C_FIFOSTATUS_RXFE` reader - RX FIFO Empty"]
pub type I2C_FIFOSTATUS_RXFE_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_RXFE` writer - RX FIFO Empty"]
pub type I2C_FIFOSTATUS_RXFE_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 16>;
#[doc = "Field `I2C_FIFOSTATUS_RXFF` reader - RX FIFO Full"]
pub type I2C_FIFOSTATUS_RXFF_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_RXFF` writer - RX FIFO Full"]
pub type I2C_FIFOSTATUS_RXFF_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 17>;
#[doc = "Field `I2C_FIFOSTATUS_RXABVTRIG` reader - RX FIFO Above Trigger Level"]
pub type I2C_FIFOSTATUS_RXABVTRIG_R = crate::BitReader<bool>;
#[doc = "Field `I2C_FIFOSTATUS_RXABVTRIG` writer - RX FIFO Above Trigger Level"]
pub type I2C_FIFOSTATUS_RXABVTRIG_W<'a> = crate::BitWriter<'a, u32, FIFOSTATUS_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - TX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_txfe(&self) -> I2C_FIFOSTATUS_TXFE_R {
        I2C_FIFOSTATUS_TXFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_txff(&self) -> I2C_FIFOSTATUS_TXFF_R {
        I2C_FIFOSTATUS_TXFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_txblwtrig(&self) -> I2C_FIFOSTATUS_TXBLWTRIG_R {
        I2C_FIFOSTATUS_TXBLWTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - RX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxfe(&self) -> I2C_FIFOSTATUS_RXFE_R {
        I2C_FIFOSTATUS_RXFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxff(&self) -> I2C_FIFOSTATUS_RXFF_R {
        I2C_FIFOSTATUS_RXFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxabvtrig(&self) -> I2C_FIFOSTATUS_RXABVTRIG_R {
        I2C_FIFOSTATUS_RXABVTRIG_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_txfe(&mut self) -> I2C_FIFOSTATUS_TXFE_W {
        I2C_FIFOSTATUS_TXFE_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_txff(&mut self) -> I2C_FIFOSTATUS_TXFF_W {
        I2C_FIFOSTATUS_TXFF_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_txblwtrig(&mut self) -> I2C_FIFOSTATUS_TXBLWTRIG_W {
        I2C_FIFOSTATUS_TXBLWTRIG_W::new(self)
    }
    #[doc = "Bit 16 - RX FIFO Empty"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxfe(&mut self) -> I2C_FIFOSTATUS_RXFE_W {
        I2C_FIFOSTATUS_RXFE_W::new(self)
    }
    #[doc = "Bit 17 - RX FIFO Full"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxff(&mut self) -> I2C_FIFOSTATUS_RXFF_W {
        I2C_FIFOSTATUS_RXFF_W::new(self)
    }
    #[doc = "Bit 18 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn i2c_fifostatus_rxabvtrig(&mut self) -> I2C_FIFOSTATUS_RXABVTRIG_W {
        I2C_FIFOSTATUS_RXABVTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostatus](index.html) module"]
pub struct FIFOSTATUS_SPEC;
impl crate::RegisterSpec for FIFOSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifostatus::R](R) reader structure"]
impl crate::Readable for FIFOSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifostatus::W](W) writer structure"]
impl crate::Writable for FIFOSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOSTATUS to value 0"]
impl crate::Resettable for FIFOSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
