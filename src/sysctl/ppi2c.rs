#[doc = "Register `PPI2C` reader"]
pub struct R(crate::R<PPI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPI2C` writer"]
pub struct W(crate::W<PPI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPI2C_SPEC>;
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
impl From<crate::W<PPI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPI2C_P0` reader - I2C Module 0 Present"]
pub type SYSCTL_PPI2C_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P0` writer - I2C Module 0 Present"]
pub type SYSCTL_PPI2C_P0_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PPI2C_P1` reader - I2C Module 1 Present"]
pub type SYSCTL_PPI2C_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P1` writer - I2C Module 1 Present"]
pub type SYSCTL_PPI2C_P1_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PPI2C_P2` reader - I2C Module 2 Present"]
pub type SYSCTL_PPI2C_P2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P2` writer - I2C Module 2 Present"]
pub type SYSCTL_PPI2C_P2_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PPI2C_P3` reader - I2C Module 3 Present"]
pub type SYSCTL_PPI2C_P3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P3` writer - I2C Module 3 Present"]
pub type SYSCTL_PPI2C_P3_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_PPI2C_P4` reader - I2C Module 4 Present"]
pub type SYSCTL_PPI2C_P4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P4` writer - I2C Module 4 Present"]
pub type SYSCTL_PPI2C_P4_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_PPI2C_P5` reader - I2C Module 5 Present"]
pub type SYSCTL_PPI2C_P5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P5` writer - I2C Module 5 Present"]
pub type SYSCTL_PPI2C_P5_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_PPI2C_P6` reader - I2C Module 6 Present"]
pub type SYSCTL_PPI2C_P6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P6` writer - I2C Module 6 Present"]
pub type SYSCTL_PPI2C_P6_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_PPI2C_P7` reader - I2C Module 7 Present"]
pub type SYSCTL_PPI2C_P7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P7` writer - I2C Module 7 Present"]
pub type SYSCTL_PPI2C_P7_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_PPI2C_P8` reader - I2C Module 8 Present"]
pub type SYSCTL_PPI2C_P8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P8` writer - I2C Module 8 Present"]
pub type SYSCTL_PPI2C_P8_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_PPI2C_P9` reader - I2C Module 9 Present"]
pub type SYSCTL_PPI2C_P9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPI2C_P9` writer - I2C Module 9 Present"]
pub type SYSCTL_PPI2C_P9_W<'a> = crate::BitWriter<'a, u32, PPI2C_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p0(&self) -> SYSCTL_PPI2C_P0_R {
        SYSCTL_PPI2C_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p1(&self) -> SYSCTL_PPI2C_P1_R {
        SYSCTL_PPI2C_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p2(&self) -> SYSCTL_PPI2C_P2_R {
        SYSCTL_PPI2C_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p3(&self) -> SYSCTL_PPI2C_P3_R {
        SYSCTL_PPI2C_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p4(&self) -> SYSCTL_PPI2C_P4_R {
        SYSCTL_PPI2C_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p5(&self) -> SYSCTL_PPI2C_P5_R {
        SYSCTL_PPI2C_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Module 6 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p6(&self) -> SYSCTL_PPI2C_P6_R {
        SYSCTL_PPI2C_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Module 7 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p7(&self) -> SYSCTL_PPI2C_P7_R {
        SYSCTL_PPI2C_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Module 8 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p8(&self) -> SYSCTL_PPI2C_P8_R {
        SYSCTL_PPI2C_P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Module 9 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p9(&self) -> SYSCTL_PPI2C_P9_R {
        SYSCTL_PPI2C_P9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p0(&mut self) -> SYSCTL_PPI2C_P0_W {
        SYSCTL_PPI2C_P0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p1(&mut self) -> SYSCTL_PPI2C_P1_W {
        SYSCTL_PPI2C_P1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p2(&mut self) -> SYSCTL_PPI2C_P2_W {
        SYSCTL_PPI2C_P2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p3(&mut self) -> SYSCTL_PPI2C_P3_W {
        SYSCTL_PPI2C_P3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p4(&mut self) -> SYSCTL_PPI2C_P4_W {
        SYSCTL_PPI2C_P4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p5(&mut self) -> SYSCTL_PPI2C_P5_W {
        SYSCTL_PPI2C_P5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Module 6 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p6(&mut self) -> SYSCTL_PPI2C_P6_W {
        SYSCTL_PPI2C_P6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Module 7 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p7(&mut self) -> SYSCTL_PPI2C_P7_W {
        SYSCTL_PPI2C_P7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Module 8 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p8(&mut self) -> SYSCTL_PPI2C_P8_W {
        SYSCTL_PPI2C_P8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Module 9 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p9(&mut self) -> SYSCTL_PPI2C_P9_W {
        SYSCTL_PPI2C_P9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Circuit Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppi2c](index.html) module"]
pub struct PPI2C_SPEC;
impl crate::RegisterSpec for PPI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppi2c::R](R) reader structure"]
impl crate::Readable for PPI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppi2c::W](W) writer structure"]
impl crate::Writable for PPI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPI2C to value 0"]
impl crate::Resettable for PPI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
