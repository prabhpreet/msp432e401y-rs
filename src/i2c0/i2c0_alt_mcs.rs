#[doc = "Register `MCS` reader"]
pub struct R(crate::R<I2C0_ALT_MCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_ALT_MCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_ALT_MCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_ALT_MCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCS` writer"]
pub struct W(crate::W<I2C0_ALT_MCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_ALT_MCS_SPEC>;
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
impl From<crate::W<I2C0_ALT_MCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_ALT_MCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MCS_BUSY` reader - I2C Busy"]
pub type I2C_MCS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_BUSY` writer - I2C Busy"]
pub type I2C_MCS_BUSY_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 0>;
#[doc = "Field `I2C_MCS_ERROR` reader - Error"]
pub type I2C_MCS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_ERROR` writer - Error"]
pub type I2C_MCS_ERROR_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 1>;
#[doc = "Field `I2C_MCS_STOP` reader - Generate STOP"]
pub type I2C_MCS_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_STOP` writer - Generate STOP"]
pub type I2C_MCS_STOP_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 2>;
#[doc = "Field `I2C_MCS_DATACK` reader - Acknowledge Data"]
pub type I2C_MCS_DATACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_DATACK` writer - Acknowledge Data"]
pub type I2C_MCS_DATACK_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 3>;
#[doc = "Field `I2C_MCS_HS` reader - High-Speed Enable"]
pub type I2C_MCS_HS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_HS` writer - High-Speed Enable"]
pub type I2C_MCS_HS_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 4>;
#[doc = "Field `I2C_MCS_QCMD` reader - Quick Command"]
pub type I2C_MCS_QCMD_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_QCMD` writer - Quick Command"]
pub type I2C_MCS_QCMD_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 5>;
#[doc = "Field `I2C_MCS_BUSBSY` reader - Bus Busy"]
pub type I2C_MCS_BUSBSY_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MCS_BUSBSY` writer - Bus Busy"]
pub type I2C_MCS_BUSBSY_W<'a> = crate::BitWriter<'a, u32, I2C0_ALT_MCS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&self) -> I2C_MCS_BUSY_R {
        I2C_MCS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&self) -> I2C_MCS_ERROR_R {
        I2C_MCS_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&self) -> I2C_MCS_STOP_R {
        I2C_MCS_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&self) -> I2C_MCS_DATACK_R {
        I2C_MCS_DATACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mcs_hs(&self) -> I2C_MCS_HS_R {
        I2C_MCS_HS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Command"]
    #[inline(always)]
    pub fn i2c_mcs_qcmd(&self) -> I2C_MCS_QCMD_R {
        I2C_MCS_QCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bus Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busbsy(&self) -> I2C_MCS_BUSBSY_R {
        I2C_MCS_BUSBSY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&mut self) -> I2C_MCS_BUSY_W {
        I2C_MCS_BUSY_W::new(self)
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&mut self) -> I2C_MCS_ERROR_W {
        I2C_MCS_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&mut self) -> I2C_MCS_STOP_W {
        I2C_MCS_STOP_W::new(self)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&mut self) -> I2C_MCS_DATACK_W {
        I2C_MCS_DATACK_W::new(self)
    }
    #[doc = "Bit 4 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mcs_hs(&mut self) -> I2C_MCS_HS_W {
        I2C_MCS_HS_W::new(self)
    }
    #[doc = "Bit 5 - Quick Command"]
    #[inline(always)]
    pub fn i2c_mcs_qcmd(&mut self) -> I2C_MCS_QCMD_W {
        I2C_MCS_QCMD_W::new(self)
    }
    #[doc = "Bit 6 - Bus Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busbsy(&mut self) -> I2C_MCS_BUSBSY_W {
        I2C_MCS_BUSBSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_alt_mcs](index.html) module"]
pub struct I2C0_ALT_MCS_SPEC;
impl crate::RegisterSpec for I2C0_ALT_MCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_alt_mcs::R](R) reader structure"]
impl crate::Readable for I2C0_ALT_MCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_alt_mcs::W](W) writer structure"]
impl crate::Writable for I2C0_ALT_MCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCS to value 0"]
impl crate::Resettable for I2C0_ALT_MCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
