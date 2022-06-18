#[doc = "Register `MCS` reader"]
pub struct R(crate::R<MCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCS` writer"]
pub struct W(crate::W<MCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCS_SPEC>;
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
impl From<crate::W<MCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MCS_RUN` reader - I2C Master Enable"]
pub type I2C_MCS_RUN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_RUN` writer - I2C Master Enable"]
pub type I2C_MCS_RUN_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 0>;
#[doc = "Field `I2C_MCS_START` reader - Generate START"]
pub type I2C_MCS_START_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_START` writer - Generate START"]
pub type I2C_MCS_START_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 1>;
#[doc = "Field `I2C_MCS_ADRACK` reader - Acknowledge Address"]
pub type I2C_MCS_ADRACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ADRACK` writer - Acknowledge Address"]
pub type I2C_MCS_ADRACK_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 2>;
#[doc = "Field `I2C_MCS_ACK` reader - Data Acknowledge Enable"]
pub type I2C_MCS_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ACK` writer - Data Acknowledge Enable"]
pub type I2C_MCS_ACK_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 3>;
#[doc = "Field `I2C_MCS_ARBLST` reader - Arbitration Lost"]
pub type I2C_MCS_ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ARBLST` writer - Arbitration Lost"]
pub type I2C_MCS_ARBLST_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 4>;
#[doc = "Field `I2C_MCS_IDLE` reader - I2C Idle"]
pub type I2C_MCS_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_IDLE` writer - I2C Idle"]
pub type I2C_MCS_IDLE_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 5>;
#[doc = "Field `I2C_MCS_BURST` reader - Burst Enable"]
pub type I2C_MCS_BURST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_BURST` writer - Burst Enable"]
pub type I2C_MCS_BURST_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 6>;
#[doc = "Field `I2C_MCS_CLKTO` reader - Clock Timeout Error"]
pub type I2C_MCS_CLKTO_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_CLKTO` writer - Clock Timeout Error"]
pub type I2C_MCS_CLKTO_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 7>;
#[doc = "Field `I2C_MCS_ACTDMATX` reader - DMA TX Active Status"]
pub type I2C_MCS_ACTDMATX_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ACTDMATX` writer - DMA TX Active Status"]
pub type I2C_MCS_ACTDMATX_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 30>;
#[doc = "Field `I2C_MCS_ACTDMARX` reader - DMA RX Active Status"]
pub type I2C_MCS_ACTDMARX_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ACTDMARX` writer - DMA RX Active Status"]
pub type I2C_MCS_ACTDMARX_W<'a> = crate::BitWriter<'a, u32, MCS_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - I2C Master Enable"]
    #[inline(always)]
    pub fn i2c_mcs_run(&self) -> I2C_MCS_RUN_R {
        I2C_MCS_RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn i2c_mcs_start(&self) -> I2C_MCS_START_R {
        I2C_MCS_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn i2c_mcs_adrack(&self) -> I2C_MCS_ADRACK_R {
        I2C_MCS_ADRACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn i2c_mcs_ack(&self) -> I2C_MCS_ACK_R {
        I2C_MCS_ACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn i2c_mcs_arblst(&self) -> I2C_MCS_ARBLST_R {
        I2C_MCS_ARBLST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn i2c_mcs_idle(&self) -> I2C_MCS_IDLE_R {
        I2C_MCS_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Burst Enable"]
    #[inline(always)]
    pub fn i2c_mcs_burst(&self) -> I2C_MCS_BURST_R {
        I2C_MCS_BURST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Timeout Error"]
    #[inline(always)]
    pub fn i2c_mcs_clkto(&self) -> I2C_MCS_CLKTO_R {
        I2C_MCS_CLKTO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmatx(&self) -> I2C_MCS_ACTDMATX_R {
        I2C_MCS_ACTDMATX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmarx(&self) -> I2C_MCS_ACTDMARX_R {
        I2C_MCS_ACTDMARX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Enable"]
    #[inline(always)]
    pub fn i2c_mcs_run(&mut self) -> I2C_MCS_RUN_W {
        I2C_MCS_RUN_W::new(self)
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn i2c_mcs_start(&mut self) -> I2C_MCS_START_W {
        I2C_MCS_START_W::new(self)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn i2c_mcs_adrack(&mut self) -> I2C_MCS_ADRACK_W {
        I2C_MCS_ADRACK_W::new(self)
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn i2c_mcs_ack(&mut self) -> I2C_MCS_ACK_W {
        I2C_MCS_ACK_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn i2c_mcs_arblst(&mut self) -> I2C_MCS_ARBLST_W {
        I2C_MCS_ARBLST_W::new(self)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn i2c_mcs_idle(&mut self) -> I2C_MCS_IDLE_W {
        I2C_MCS_IDLE_W::new(self)
    }
    #[doc = "Bit 6 - Burst Enable"]
    #[inline(always)]
    pub fn i2c_mcs_burst(&mut self) -> I2C_MCS_BURST_W {
        I2C_MCS_BURST_W::new(self)
    }
    #[doc = "Bit 7 - Clock Timeout Error"]
    #[inline(always)]
    pub fn i2c_mcs_clkto(&mut self) -> I2C_MCS_CLKTO_W {
        I2C_MCS_CLKTO_W::new(self)
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmatx(&mut self) -> I2C_MCS_ACTDMATX_W {
        I2C_MCS_ACTDMATX_W::new(self)
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmarx(&mut self) -> I2C_MCS_ACTDMARX_W {
        I2C_MCS_ACTDMARX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcs](index.html) module"]
pub struct MCS_SPEC;
impl crate::RegisterSpec for MCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcs::R](R) reader structure"]
impl crate::Readable for MCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcs::W](W) writer structure"]
impl crate::Writable for MCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCS to value 0"]
impl crate::Resettable for MCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
