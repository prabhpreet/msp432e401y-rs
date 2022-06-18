#[doc = "Register `DCGCI2C` reader"]
pub struct R(crate::R<DCGCI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCI2C` writer"]
pub struct W(crate::W<DCGCI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCI2C_SPEC>;
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
impl From<crate::W<DCGCI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCI2C_D0` reader - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D0` writer - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D0_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCI2C_D1` reader - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D1` writer - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D1_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_DCGCI2C_D2` reader - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D2` writer - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D2_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_DCGCI2C_D3` reader - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D3` writer - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D3_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_DCGCI2C_D4` reader - I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D4` writer - I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D4_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_DCGCI2C_D5` reader - I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D5` writer - I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D5_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_DCGCI2C_D6` reader - I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D6` writer - I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D6_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_DCGCI2C_D7` reader - I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D7` writer - I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D7_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_DCGCI2C_D8` reader - I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D8` writer - I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D8_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_DCGCI2C_D9` reader - I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCI2C_D9` writer - I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCI2C_D9_W<'a> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d0(&self) -> SYSCTL_DCGCI2C_D0_R {
        SYSCTL_DCGCI2C_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d1(&self) -> SYSCTL_DCGCI2C_D1_R {
        SYSCTL_DCGCI2C_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d2(&self) -> SYSCTL_DCGCI2C_D2_R {
        SYSCTL_DCGCI2C_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d3(&self) -> SYSCTL_DCGCI2C_D3_R {
        SYSCTL_DCGCI2C_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d4(&self) -> SYSCTL_DCGCI2C_D4_R {
        SYSCTL_DCGCI2C_D4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d5(&self) -> SYSCTL_DCGCI2C_D5_R {
        SYSCTL_DCGCI2C_D5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d6(&self) -> SYSCTL_DCGCI2C_D6_R {
        SYSCTL_DCGCI2C_D6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d7(&self) -> SYSCTL_DCGCI2C_D7_R {
        SYSCTL_DCGCI2C_D7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d8(&self) -> SYSCTL_DCGCI2C_D8_R {
        SYSCTL_DCGCI2C_D8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d9(&self) -> SYSCTL_DCGCI2C_D9_R {
        SYSCTL_DCGCI2C_D9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d0(&mut self) -> SYSCTL_DCGCI2C_D0_W {
        SYSCTL_DCGCI2C_D0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d1(&mut self) -> SYSCTL_DCGCI2C_D1_W {
        SYSCTL_DCGCI2C_D1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d2(&mut self) -> SYSCTL_DCGCI2C_D2_W {
        SYSCTL_DCGCI2C_D2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d3(&mut self) -> SYSCTL_DCGCI2C_D3_W {
        SYSCTL_DCGCI2C_D3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Module 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d4(&mut self) -> SYSCTL_DCGCI2C_D4_W {
        SYSCTL_DCGCI2C_D4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Module 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d5(&mut self) -> SYSCTL_DCGCI2C_D5_W {
        SYSCTL_DCGCI2C_D5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Module 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d6(&mut self) -> SYSCTL_DCGCI2C_D6_W {
        SYSCTL_DCGCI2C_D6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Module 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d7(&mut self) -> SYSCTL_DCGCI2C_D7_W {
        SYSCTL_DCGCI2C_D7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Module 8 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d8(&mut self) -> SYSCTL_DCGCI2C_D8_W {
        SYSCTL_DCGCI2C_D8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Module 9 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgci2c_d9(&mut self) -> SYSCTL_DCGCI2C_D9_W {
        SYSCTL_DCGCI2C_D9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgci2c](index.html) module"]
pub struct DCGCI2C_SPEC;
impl crate::RegisterSpec for DCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgci2c::R](R) reader structure"]
impl crate::Readable for DCGCI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgci2c::W](W) writer structure"]
impl crate::Writable for DCGCI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCI2C to value 0"]
impl crate::Resettable for DCGCI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
