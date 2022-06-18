#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MDR_DATA` reader - This byte contains the data transferred during a transaction"]
pub type I2C_MDR_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_MDR_DATA` writer - This byte contains the data transferred during a transaction"]
pub type I2C_MDR_DATA_W<'a> = crate::FieldWriter<'a, u32, MDR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - This byte contains the data transferred during a transaction"]
    #[inline(always)]
    pub fn i2c_mdr_data(&self) -> I2C_MDR_DATA_R {
        I2C_MDR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This byte contains the data transferred during a transaction"]
    #[inline(always)]
    pub fn i2c_mdr_data(&mut self) -> I2C_MDR_DATA_W {
        I2C_MDR_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
