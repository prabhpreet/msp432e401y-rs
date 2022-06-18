#[doc = "Register `MBCNT` reader"]
pub struct R(crate::R<MBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBCNT` writer"]
pub struct W(crate::W<MBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBCNT_SPEC>;
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
impl From<crate::W<MBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MBCNT_CNTL` reader - I2C Master Burst Count"]
pub type I2C_MBCNT_CNTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C_MBCNT_CNTL` writer - I2C Master Burst Count"]
pub type I2C_MBCNT_CNTL_W<'a> = crate::FieldWriter<'a, u32, MBCNT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - I2C Master Burst Count"]
    #[inline(always)]
    pub fn i2c_mbcnt_cntl(&self) -> I2C_MBCNT_CNTL_R {
        I2C_MBCNT_CNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Master Burst Count"]
    #[inline(always)]
    pub fn i2c_mbcnt_cntl(&mut self) -> I2C_MBCNT_CNTL_W {
        I2C_MBCNT_CNTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Burst Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbcnt](index.html) module"]
pub struct MBCNT_SPEC;
impl crate::RegisterSpec for MBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbcnt::R](R) reader structure"]
impl crate::Readable for MBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbcnt::W](W) writer structure"]
impl crate::Writable for MBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBCNT to value 0"]
impl crate::Resettable for MBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
