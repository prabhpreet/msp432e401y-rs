#[doc = "Register `SOAR` reader"]
pub struct R(crate::R<SOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOAR` writer"]
pub struct W(crate::W<SOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOAR_SPEC>;
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
impl From<crate::W<SOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SOAR_OAR` reader - I2C Slave Own Address"]
pub type I2C_SOAR_OAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_SOAR_OAR` writer - I2C Slave Own Address"]
pub type I2C_SOAR_OAR_W<'a> = crate::FieldWriter<'a, u32, SOAR_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    pub fn i2c_soar_oar(&self) -> I2C_SOAR_OAR_R {
        I2C_SOAR_OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Slave Own Address"]
    #[inline(always)]
    pub fn i2c_soar_oar(&mut self) -> I2C_SOAR_OAR_W {
        I2C_SOAR_OAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar](index.html) module"]
pub struct SOAR_SPEC;
impl crate::RegisterSpec for SOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soar::R](R) reader structure"]
impl crate::Readable for SOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soar::W](W) writer structure"]
impl crate::Writable for SOAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOAR to value 0"]
impl crate::Resettable for SOAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
