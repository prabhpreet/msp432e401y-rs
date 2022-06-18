#[doc = "Register `SCGCSSI` reader"]
pub struct R(crate::R<SCGCSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCSSI` writer"]
pub struct W(crate::W<SCGCSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCSSI_SPEC>;
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
impl From<crate::W<SCGCSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCSSI_S0` reader - SSI Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCSSI_S0` writer - SSI Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S0_W<'a> = crate::BitWriter<'a, u32, SCGCSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SCGCSSI_S1` reader - SSI Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCSSI_S1` writer - SSI Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S1_W<'a> = crate::BitWriter<'a, u32, SCGCSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SCGCSSI_S2` reader - SSI Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCSSI_S2` writer - SSI Module 2 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S2_W<'a> = crate::BitWriter<'a, u32, SCGCSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SCGCSSI_S3` reader - SSI Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCSSI_S3` writer - SSI Module 3 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCSSI_S3_W<'a> = crate::BitWriter<'a, u32, SCGCSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s0(&self) -> SYSCTL_SCGCSSI_S0_R {
        SYSCTL_SCGCSSI_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s1(&self) -> SYSCTL_SCGCSSI_S1_R {
        SYSCTL_SCGCSSI_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s2(&self) -> SYSCTL_SCGCSSI_S2_R {
        SYSCTL_SCGCSSI_S2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s3(&self) -> SYSCTL_SCGCSSI_S3_R {
        SYSCTL_SCGCSSI_S3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s0(&mut self) -> SYSCTL_SCGCSSI_S0_W {
        SYSCTL_SCGCSSI_S0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s1(&mut self) -> SYSCTL_SCGCSSI_S1_W {
        SYSCTL_SCGCSSI_S1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s2(&mut self) -> SYSCTL_SCGCSSI_S2_W {
        SYSCTL_SCGCSSI_S2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcssi_s3(&mut self) -> SYSCTL_SCGCSSI_S3_W {
        SYSCTL_SCGCSSI_S3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcssi](index.html) module"]
pub struct SCGCSSI_SPEC;
impl crate::RegisterSpec for SCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcssi::R](R) reader structure"]
impl crate::Readable for SCGCSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcssi::W](W) writer structure"]
impl crate::Writable for SCGCSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCSSI to value 0"]
impl crate::Resettable for SCGCSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
