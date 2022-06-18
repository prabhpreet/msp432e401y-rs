#[doc = "Register `SACKCTL` reader"]
pub struct R(crate::R<SACKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SACKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SACKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SACKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SACKCTL` writer"]
pub struct W(crate::W<SACKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SACKCTL_SPEC>;
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
impl From<crate::W<SACKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SACKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SACKCTL_ACKOEN` reader - I2C Slave ACK Override Enable"]
pub type I2C_SACKCTL_ACKOEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SACKCTL_ACKOEN` writer - I2C Slave ACK Override Enable"]
pub type I2C_SACKCTL_ACKOEN_W<'a> = crate::BitWriter<'a, u32, SACKCTL_SPEC, bool, 0>;
#[doc = "Field `I2C_SACKCTL_ACKOVAL` reader - I2C Slave ACK Override Value"]
pub type I2C_SACKCTL_ACKOVAL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SACKCTL_ACKOVAL` writer - I2C Slave ACK Override Value"]
pub type I2C_SACKCTL_ACKOVAL_W<'a> = crate::BitWriter<'a, u32, SACKCTL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoen(&self) -> I2C_SACKCTL_ACKOEN_R {
        I2C_SACKCTL_ACKOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoval(&self) -> I2C_SACKCTL_ACKOVAL_R {
        I2C_SACKCTL_ACKOVAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Slave ACK Override Enable"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoen(&mut self) -> I2C_SACKCTL_ACKOEN_W {
        I2C_SACKCTL_ACKOEN_W::new(self)
    }
    #[doc = "Bit 1 - I2C Slave ACK Override Value"]
    #[inline(always)]
    pub fn i2c_sackctl_ackoval(&mut self) -> I2C_SACKCTL_ACKOVAL_W {
        I2C_SACKCTL_ACKOVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave ACK Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sackctl](index.html) module"]
pub struct SACKCTL_SPEC;
impl crate::RegisterSpec for SACKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sackctl::R](R) reader structure"]
impl crate::Readable for SACKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sackctl::W](W) writer structure"]
impl crate::Writable for SACKCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SACKCTL to value 0"]
impl crate::Resettable for SACKCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
