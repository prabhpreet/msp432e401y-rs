#[doc = "Register `RCGCSSI` reader"]
pub struct R(crate::R<RCGCSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCSSI` writer"]
pub struct W(crate::W<RCGCSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCSSI_SPEC>;
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
impl From<crate::W<RCGCSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCSSI_R0` reader - SSI Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCSSI_R0` writer - SSI Module 0 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R0_W<'a> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_RCGCSSI_R1` reader - SSI Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCSSI_R1` writer - SSI Module 1 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R1_W<'a> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_RCGCSSI_R2` reader - SSI Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCSSI_R2` writer - SSI Module 2 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R2_W<'a> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_RCGCSSI_R3` reader - SSI Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCSSI_R3` writer - SSI Module 3 Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCSSI_R3_W<'a> = crate::BitWriter<'a, u32, RCGCSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r0(&self) -> SYSCTL_RCGCSSI_R0_R {
        SYSCTL_RCGCSSI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r1(&self) -> SYSCTL_RCGCSSI_R1_R {
        SYSCTL_RCGCSSI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r2(&self) -> SYSCTL_RCGCSSI_R2_R {
        SYSCTL_RCGCSSI_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r3(&self) -> SYSCTL_RCGCSSI_R3_R {
        SYSCTL_RCGCSSI_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r0(&mut self) -> SYSCTL_RCGCSSI_R0_W {
        SYSCTL_RCGCSSI_R0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r1(&mut self) -> SYSCTL_RCGCSSI_R1_W {
        SYSCTL_RCGCSSI_R1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r2(&mut self) -> SYSCTL_RCGCSSI_R2_W {
        SYSCTL_RCGCSSI_R2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgcssi_r3(&mut self) -> SYSCTL_RCGCSSI_R3_W {
        SYSCTL_RCGCSSI_R3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcssi](index.html) module"]
pub struct RCGCSSI_SPEC;
impl crate::RegisterSpec for RCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcssi::R](R) reader structure"]
impl crate::Readable for RCGCSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcssi::W](W) writer structure"]
impl crate::Writable for RCGCSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCSSI to value 0"]
impl crate::Resettable for RCGCSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
