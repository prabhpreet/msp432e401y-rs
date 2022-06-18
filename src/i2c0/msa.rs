#[doc = "Register `MSA` reader"]
pub struct R(crate::R<MSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSA` writer"]
pub struct W(crate::W<MSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSA_SPEC>;
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
impl From<crate::W<MSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MSA_RS` reader - Receive not send"]
pub type I2C_MSA_RS_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MSA_RS` writer - Receive not send"]
pub type I2C_MSA_RS_W<'a> = crate::BitWriter<'a, u32, MSA_SPEC, bool, 0>;
#[doc = "Field `I2C_MSA_SA` reader - I2C Slave Address"]
pub type I2C_MSA_SA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_MSA_SA` writer - I2C Slave Address"]
pub type I2C_MSA_SA_W<'a> = crate::FieldWriter<'a, u32, MSA_SPEC, u8, u8, 7, 1>;
impl R {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    pub fn i2c_msa_rs(&self) -> I2C_MSA_RS_R {
        I2C_MSA_RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn i2c_msa_sa(&self) -> I2C_MSA_SA_R {
        I2C_MSA_SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive not send"]
    #[inline(always)]
    pub fn i2c_msa_rs(&mut self) -> I2C_MSA_RS_W {
        I2C_MSA_RS_W::new(self)
    }
    #[doc = "Bits 1:7 - I2C Slave Address"]
    #[inline(always)]
    pub fn i2c_msa_sa(&mut self) -> I2C_MSA_SA_W {
        I2C_MSA_SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msa](index.html) module"]
pub struct MSA_SPEC;
impl crate::RegisterSpec for MSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msa::R](R) reader structure"]
impl crate::Readable for MSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msa::W](W) writer structure"]
impl crate::Writable for MSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSA to value 0"]
impl crate::Resettable for MSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
