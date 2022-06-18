#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SCSR_RREQ` reader - Receive Request"]
pub type I2C_SCSR_RREQ_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_RREQ` writer - Receive Request"]
pub type I2C_SCSR_RREQ_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 0>;
#[doc = "Field `I2C_SCSR_TXFIFO` reader - TX FIFO Enable"]
pub type I2C_SCSR_TXFIFO_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_TXFIFO` writer - TX FIFO Enable"]
pub type I2C_SCSR_TXFIFO_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 1>;
#[doc = "Field `I2C_SCSR_FBR` reader - First Byte Received"]
pub type I2C_SCSR_FBR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_FBR` writer - First Byte Received"]
pub type I2C_SCSR_FBR_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 2>;
#[doc = "Field `I2C_SCSR_OAR2SEL` reader - OAR2 Address Matched"]
pub type I2C_SCSR_OAR2SEL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_OAR2SEL` writer - OAR2 Address Matched"]
pub type I2C_SCSR_OAR2SEL_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 3>;
#[doc = "Field `I2C_SCSR_QCMDST` reader - Quick Command Status"]
pub type I2C_SCSR_QCMDST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_QCMDST` writer - Quick Command Status"]
pub type I2C_SCSR_QCMDST_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 4>;
#[doc = "Field `I2C_SCSR_QCMDRW` reader - Quick Command Read / Write"]
pub type I2C_SCSR_QCMDRW_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_QCMDRW` writer - Quick Command Read / Write"]
pub type I2C_SCSR_QCMDRW_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 5>;
#[doc = "Field `I2C_SCSR_ACTDMATX` reader - DMA TX Active Status"]
pub type I2C_SCSR_ACTDMATX_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_ACTDMATX` writer - DMA TX Active Status"]
pub type I2C_SCSR_ACTDMATX_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 30>;
#[doc = "Field `I2C_SCSR_ACTDMARX` reader - DMA RX Active Status"]
pub type I2C_SCSR_ACTDMARX_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SCSR_ACTDMARX` writer - DMA RX Active Status"]
pub type I2C_SCSR_ACTDMARX_W<'a> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn i2c_scsr_rreq(&self) -> I2C_SCSR_RREQ_R {
        I2C_SCSR_RREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_txfifo(&self) -> I2C_SCSR_TXFIFO_R {
        I2C_SCSR_TXFIFO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    pub fn i2c_scsr_fbr(&self) -> I2C_SCSR_FBR_R {
        I2C_SCSR_FBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn i2c_scsr_oar2sel(&self) -> I2C_SCSR_OAR2SEL_R {
        I2C_SCSR_OAR2SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Quick Command Status"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdst(&self) -> I2C_SCSR_QCMDST_R {
        I2C_SCSR_QCMDST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Command Read / Write"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdrw(&self) -> I2C_SCSR_QCMDRW_R {
        I2C_SCSR_QCMDRW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmatx(&self) -> I2C_SCSR_ACTDMATX_R {
        I2C_SCSR_ACTDMATX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmarx(&self) -> I2C_SCSR_ACTDMARX_R {
        I2C_SCSR_ACTDMARX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn i2c_scsr_rreq(&mut self) -> I2C_SCSR_RREQ_W {
        I2C_SCSR_RREQ_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_txfifo(&mut self) -> I2C_SCSR_TXFIFO_W {
        I2C_SCSR_TXFIFO_W::new(self)
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    pub fn i2c_scsr_fbr(&mut self) -> I2C_SCSR_FBR_W {
        I2C_SCSR_FBR_W::new(self)
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn i2c_scsr_oar2sel(&mut self) -> I2C_SCSR_OAR2SEL_W {
        I2C_SCSR_OAR2SEL_W::new(self)
    }
    #[doc = "Bit 4 - Quick Command Status"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdst(&mut self) -> I2C_SCSR_QCMDST_W {
        I2C_SCSR_QCMDST_W::new(self)
    }
    #[doc = "Bit 5 - Quick Command Read / Write"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdrw(&mut self) -> I2C_SCSR_QCMDRW_W {
        I2C_SCSR_QCMDRW_W::new(self)
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmatx(&mut self) -> I2C_SCSR_ACTDMATX_W {
        I2C_SCSR_ACTDMATX_W::new(self)
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmarx(&mut self) -> I2C_SCSR_ACTDMARX_W {
        I2C_SCSR_ACTDMARX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](index.html) module"]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scsr::R](R) reader structure"]
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scsr::W](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
