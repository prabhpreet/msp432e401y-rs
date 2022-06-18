#[doc = "Register `FIFODATA` reader"]
pub struct R(crate::R<FIFODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFODATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFODATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFODATA` writer"]
pub struct W(crate::W<FIFODATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFODATA_SPEC>;
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
impl From<crate::W<FIFODATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFODATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_FIFODATA_DATA` reader - I2C TX FIFO Write Data Byte"]
pub type I2C_FIFODATA_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_FIFODATA_DATA` writer - I2C TX FIFO Write Data Byte"]
pub type I2C_FIFODATA_DATA_W<'a> = crate::FieldWriter<'a, u32, FIFODATA_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - I2C TX FIFO Write Data Byte"]
    #[inline(always)]
    pub fn i2c_fifodata_data(&self) -> I2C_FIFODATA_DATA_R {
        I2C_FIFODATA_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C TX FIFO Write Data Byte"]
    #[inline(always)]
    pub fn i2c_fifodata_data(&mut self) -> I2C_FIFODATA_DATA_W {
        I2C_FIFODATA_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C FIFO Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifodata](index.html) module"]
pub struct FIFODATA_SPEC;
impl crate::RegisterSpec for FIFODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifodata::R](R) reader structure"]
impl crate::Readable for FIFODATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifodata::W](W) writer structure"]
impl crate::Writable for FIFODATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFODATA to value 0"]
impl crate::Resettable for FIFODATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
