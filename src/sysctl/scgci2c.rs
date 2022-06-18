#[doc = "Register `SCGCI2C` reader"]
pub struct R(crate::R<SCGCI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCI2C` writer"]
pub struct W(crate::W<SCGCI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCI2C_SPEC>;
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
impl From<crate::W<SCGCI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCI2C_S0` reader - I2C Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S0` writer - I2C Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S0_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCI2C_S1` reader - I2C Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S1` writer - I2C Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S1_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SCGCI2C_S2` reader - I2C Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S2` writer - I2C Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S2_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SCGCI2C_S3` reader - I2C Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S3` writer - I2C Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S3_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_SCGCI2C_S4` reader - I2C Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S4` writer - I2C Module 4 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S4_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_SCGCI2C_S5` reader - I2C Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S5` writer - I2C Module 5 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S5_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_SCGCI2C_S6` reader - I2C Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S6` writer - I2C Module 6 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S6_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_SCGCI2C_S7` reader - I2C Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S7` writer - I2C Module 7 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S7_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_SCGCI2C_S8` reader - I2C Module 8 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S8` writer - I2C Module 8 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S8_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_SCGCI2C_S9` reader - I2C Module 9 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCI2C_S9` writer - I2C Module 9 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCI2C_S9_W<'a> = crate::BitWriter<'a, u32, SCGCI2C_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s0(&self) -> SYSCTL_SCGCI2C_S0_R {
        SYSCTL_SCGCI2C_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s1(&self) -> SYSCTL_SCGCI2C_S1_R {
        SYSCTL_SCGCI2C_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s2(&self) -> SYSCTL_SCGCI2C_S2_R {
        SYSCTL_SCGCI2C_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s3(&self) -> SYSCTL_SCGCI2C_S3_R {
        SYSCTL_SCGCI2C_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s4(&self) -> SYSCTL_SCGCI2C_S4_R {
        SYSCTL_SCGCI2C_S4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s5(&self) -> SYSCTL_SCGCI2C_S5_R {
        SYSCTL_SCGCI2C_S5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s6(&self) -> SYSCTL_SCGCI2C_S6_R {
        SYSCTL_SCGCI2C_S6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s7(&self) -> SYSCTL_SCGCI2C_S7_R {
        SYSCTL_SCGCI2C_S7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s8(&self) -> SYSCTL_SCGCI2C_S8_R {
        SYSCTL_SCGCI2C_S8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s9(&self) -> SYSCTL_SCGCI2C_S9_R {
        SYSCTL_SCGCI2C_S9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s0(&mut self) -> SYSCTL_SCGCI2C_S0_W {
        SYSCTL_SCGCI2C_S0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s1(&mut self) -> SYSCTL_SCGCI2C_S1_W {
        SYSCTL_SCGCI2C_S1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s2(&mut self) -> SYSCTL_SCGCI2C_S2_W {
        SYSCTL_SCGCI2C_S2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s3(&mut self) -> SYSCTL_SCGCI2C_S3_W {
        SYSCTL_SCGCI2C_S3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s4(&mut self) -> SYSCTL_SCGCI2C_S4_W {
        SYSCTL_SCGCI2C_S4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s5(&mut self) -> SYSCTL_SCGCI2C_S5_W {
        SYSCTL_SCGCI2C_S5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s6(&mut self) -> SYSCTL_SCGCI2C_S6_W {
        SYSCTL_SCGCI2C_S6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s7(&mut self) -> SYSCTL_SCGCI2C_S7_W {
        SYSCTL_SCGCI2C_S7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s8(&mut self) -> SYSCTL_SCGCI2C_S8_W {
        SYSCTL_SCGCI2C_S8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s9(&mut self) -> SYSCTL_SCGCI2C_S9_W {
        SYSCTL_SCGCI2C_S9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgci2c](index.html) module"]
pub struct SCGCI2C_SPEC;
impl crate::RegisterSpec for SCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgci2c::R](R) reader structure"]
impl crate::Readable for SCGCI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgci2c::W](W) writer structure"]
impl crate::Writable for SCGCI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCI2C to value 0"]
impl crate::Resettable for SCGCI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
