#[doc = "Register `MBMON` reader"]
pub struct R(crate::R<MBMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBMON` writer"]
pub struct W(crate::W<MBMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBMON_SPEC>;
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
impl From<crate::W<MBMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_MBMON_SCL` reader - I2C SCL Status"]
pub type I2C_MBMON_SCL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MBMON_SCL` writer - I2C SCL Status"]
pub type I2C_MBMON_SCL_W<'a> = crate::BitWriter<'a, u32, MBMON_SPEC, bool, 0>;
#[doc = "Field `I2C_MBMON_SDA` reader - I2C SDA Status"]
pub type I2C_MBMON_SDA_R = crate::BitReader<bool>;
#[doc = "Field `I2C_MBMON_SDA` writer - I2C SDA Status"]
pub type I2C_MBMON_SDA_W<'a> = crate::BitWriter<'a, u32, MBMON_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn i2c_mbmon_scl(&self) -> I2C_MBMON_SCL_R {
        I2C_MBMON_SCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn i2c_mbmon_sda(&self) -> I2C_MBMON_SDA_R {
        I2C_MBMON_SDA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn i2c_mbmon_scl(&mut self) -> I2C_MBMON_SCL_W {
        I2C_MBMON_SCL_W::new(self)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn i2c_mbmon_sda(&mut self) -> I2C_MBMON_SDA_W {
        I2C_MBMON_SDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Bus Monitor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbmon](index.html) module"]
pub struct MBMON_SPEC;
impl crate::RegisterSpec for MBMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbmon::R](R) reader structure"]
impl crate::Readable for MBMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbmon::W](W) writer structure"]
impl crate::Writable for MBMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBMON to value 0"]
impl crate::Resettable for MBMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
