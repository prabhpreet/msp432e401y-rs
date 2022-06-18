#[doc = "Register `RCGCI2C` reader"]
pub struct R(crate::R<RCGCI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCI2C` writer"]
pub struct W(crate::W<RCGCI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCI2C_SPEC>;
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
impl From<crate::W<RCGCI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCI2C_R0` reader - I2C Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R0` writer - I2C Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R0_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_RCGCI2C_R1` reader - I2C Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R1` writer - I2C Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R1_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_RCGCI2C_R2` reader - I2C Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R2` writer - I2C Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R2_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_RCGCI2C_R3` reader - I2C Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R3` writer - I2C Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R3_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 3>;
#[doc = "Field `SYSCTL_RCGCI2C_R4` reader - I2C Module 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R4_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R4` writer - I2C Module 4 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R4_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 4>;
#[doc = "Field `SYSCTL_RCGCI2C_R5` reader - I2C Module 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R5_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R5` writer - I2C Module 5 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R5_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 5>;
#[doc = "Field `SYSCTL_RCGCI2C_R6` reader - I2C Module 6 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R6_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R6` writer - I2C Module 6 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R6_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 6>;
#[doc = "Field `SYSCTL_RCGCI2C_R7` reader - I2C Module 7 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R7_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R7` writer - I2C Module 7 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R7_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 7>;
#[doc = "Field `SYSCTL_RCGCI2C_R8` reader - I2C Module 8 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R8_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R8` writer - I2C Module 8 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R8_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_RCGCI2C_R9` reader - I2C Module 9 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R9_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCI2C_R9` writer - I2C Module 9 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCI2C_R9_W<'a> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r0(&self) -> SYSCTL_RCGCI2C_R0_R {
        SYSCTL_RCGCI2C_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r1(&self) -> SYSCTL_RCGCI2C_R1_R {
        SYSCTL_RCGCI2C_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r2(&self) -> SYSCTL_RCGCI2C_R2_R {
        SYSCTL_RCGCI2C_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r3(&self) -> SYSCTL_RCGCI2C_R3_R {
        SYSCTL_RCGCI2C_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Module 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r4(&self) -> SYSCTL_RCGCI2C_R4_R {
        SYSCTL_RCGCI2C_R4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Module 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r5(&self) -> SYSCTL_RCGCI2C_R5_R {
        SYSCTL_RCGCI2C_R5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Module 6 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r6(&self) -> SYSCTL_RCGCI2C_R6_R {
        SYSCTL_RCGCI2C_R6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Module 7 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r7(&self) -> SYSCTL_RCGCI2C_R7_R {
        SYSCTL_RCGCI2C_R7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Module 8 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r8(&self) -> SYSCTL_RCGCI2C_R8_R {
        SYSCTL_RCGCI2C_R8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Module 9 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r9(&self) -> SYSCTL_RCGCI2C_R9_R {
        SYSCTL_RCGCI2C_R9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r0(&mut self) -> SYSCTL_RCGCI2C_R0_W {
        SYSCTL_RCGCI2C_R0_W::new(self)
    }
    #[doc = "Bit 1 - I2C Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r1(&mut self) -> SYSCTL_RCGCI2C_R1_W {
        SYSCTL_RCGCI2C_R1_W::new(self)
    }
    #[doc = "Bit 2 - I2C Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r2(&mut self) -> SYSCTL_RCGCI2C_R2_W {
        SYSCTL_RCGCI2C_R2_W::new(self)
    }
    #[doc = "Bit 3 - I2C Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r3(&mut self) -> SYSCTL_RCGCI2C_R3_W {
        SYSCTL_RCGCI2C_R3_W::new(self)
    }
    #[doc = "Bit 4 - I2C Module 4 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r4(&mut self) -> SYSCTL_RCGCI2C_R4_W {
        SYSCTL_RCGCI2C_R4_W::new(self)
    }
    #[doc = "Bit 5 - I2C Module 5 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r5(&mut self) -> SYSCTL_RCGCI2C_R5_W {
        SYSCTL_RCGCI2C_R5_W::new(self)
    }
    #[doc = "Bit 6 - I2C Module 6 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r6(&mut self) -> SYSCTL_RCGCI2C_R6_W {
        SYSCTL_RCGCI2C_R6_W::new(self)
    }
    #[doc = "Bit 7 - I2C Module 7 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r7(&mut self) -> SYSCTL_RCGCI2C_R7_W {
        SYSCTL_RCGCI2C_R7_W::new(self)
    }
    #[doc = "Bit 8 - I2C Module 8 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r8(&mut self) -> SYSCTL_RCGCI2C_R8_W {
        SYSCTL_RCGCI2C_R8_W::new(self)
    }
    #[doc = "Bit 9 - I2C Module 9 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgci2c_r9(&mut self) -> SYSCTL_RCGCI2C_R9_W {
        SYSCTL_RCGCI2C_R9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgci2c](index.html) module"]
pub struct RCGCI2C_SPEC;
impl crate::RegisterSpec for RCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgci2c::R](R) reader structure"]
impl crate::Readable for RCGCI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgci2c::W](W) writer structure"]
impl crate::Writable for RCGCI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCI2C to value 0"]
impl crate::Resettable for RCGCI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
