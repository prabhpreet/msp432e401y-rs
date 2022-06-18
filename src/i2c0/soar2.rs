#[doc = "Register `SOAR2` reader"]
pub struct R(crate::R<SOAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOAR2` writer"]
pub struct W(crate::W<SOAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOAR2_SPEC>;
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
impl From<crate::W<SOAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SOAR2_OAR2` reader - I2C Slave Own Address 2"]
pub type I2C_SOAR2_OAR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_SOAR2_OAR2` writer - I2C Slave Own Address 2"]
pub type I2C_SOAR2_OAR2_W<'a> = crate::FieldWriter<'a, u32, SOAR2_SPEC, u8, u8, 7, 0>;
#[doc = "Field `I2C_SOAR2_OAR2EN` reader - I2C Slave Own Address 2 Enable"]
pub type I2C_SOAR2_OAR2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C_SOAR2_OAR2EN` writer - I2C Slave Own Address 2 Enable"]
pub type I2C_SOAR2_OAR2EN_W<'a> = crate::BitWriter<'a, u32, SOAR2_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub fn i2c_soar2_oar2(&self) -> I2C_SOAR2_OAR2_R {
        I2C_SOAR2_OAR2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn i2c_soar2_oar2en(&self) -> I2C_SOAR2_OAR2EN_R {
        I2C_SOAR2_OAR2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address 2"]
    #[inline(always)]
    pub fn i2c_soar2_oar2(&mut self) -> I2C_SOAR2_OAR2_W {
        I2C_SOAR2_OAR2_W::new(self)
    }
    #[doc = "Bit 7 - I2C Slave Own Address 2 Enable"]
    #[inline(always)]
    pub fn i2c_soar2_oar2en(&mut self) -> I2C_SOAR2_OAR2EN_W {
        I2C_SOAR2_OAR2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Own Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar2](index.html) module"]
pub struct SOAR2_SPEC;
impl crate::RegisterSpec for SOAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soar2::R](R) reader structure"]
impl crate::Readable for SOAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soar2::W](W) writer structure"]
impl crate::Writable for SOAR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOAR2 to value 0"]
impl crate::Resettable for SOAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
