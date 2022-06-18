#[doc = "Register `MBLEN` reader"]
pub struct R(crate::R<MBLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBLEN` writer"]
pub struct W(crate::W<MBLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBLEN_SPEC>;
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
impl From<crate::W<MBLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MBLEN_CNTL` reader - I2C Burst Length"]
pub type I2C_MBLEN_CNTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_MBLEN_CNTL` writer - I2C Burst Length"]
pub type I2C_MBLEN_CNTL_W<'a> = crate::FieldWriter<'a, u32, MBLEN_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - I2C Burst Length"]
    #[inline(always)]
    pub fn i2c_mblen_cntl(&self) -> I2C_MBLEN_CNTL_R {
        I2C_MBLEN_CNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Burst Length"]
    #[inline(always)]
    pub fn i2c_mblen_cntl(&mut self) -> I2C_MBLEN_CNTL_W {
        I2C_MBLEN_CNTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Burst Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mblen](index.html) module"]
pub struct MBLEN_SPEC;
impl crate::RegisterSpec for MBLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mblen::R](R) reader structure"]
impl crate::Readable for MBLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mblen::W](W) writer structure"]
impl crate::Writable for MBLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBLEN to value 0"]
impl crate::Resettable for MBLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
