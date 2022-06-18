#[doc = "Register `SRSSI` reader"]
pub struct R(crate::R<SRSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSSI` writer"]
pub struct W(crate::W<SRSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSSI_SPEC>;
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
impl From<crate::W<SRSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRSSI_R0` reader - SSI Module 0 Software Reset"]
pub type SYSCTL_SRSSI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRSSI_R0` writer - SSI Module 0 Software Reset"]
pub type SYSCTL_SRSSI_R0_W<'a> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_SRSSI_R1` reader - SSI Module 1 Software Reset"]
pub type SYSCTL_SRSSI_R1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRSSI_R1` writer - SSI Module 1 Software Reset"]
pub type SYSCTL_SRSSI_R1_W<'a> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_SRSSI_R2` reader - SSI Module 2 Software Reset"]
pub type SYSCTL_SRSSI_R2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRSSI_R2` writer - SSI Module 2 Software Reset"]
pub type SYSCTL_SRSSI_R2_W<'a> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_SRSSI_R3` reader - SSI Module 3 Software Reset"]
pub type SYSCTL_SRSSI_R3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRSSI_R3` writer - SSI Module 3 Software Reset"]
pub type SYSCTL_SRSSI_R3_W<'a> = crate::BitWriter<'a, u32, SRSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r0(&self) -> SYSCTL_SRSSI_R0_R {
        SYSCTL_SRSSI_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r1(&self) -> SYSCTL_SRSSI_R1_R {
        SYSCTL_SRSSI_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r2(&self) -> SYSCTL_SRSSI_R2_R {
        SYSCTL_SRSSI_R2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r3(&self) -> SYSCTL_SRSSI_R3_R {
        SYSCTL_SRSSI_R3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r0(&mut self) -> SYSCTL_SRSSI_R0_W {
        SYSCTL_SRSSI_R0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r1(&mut self) -> SYSCTL_SRSSI_R1_W {
        SYSCTL_SRSSI_R1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r2(&mut self) -> SYSCTL_SRSSI_R2_W {
        SYSCTL_SRSSI_R2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srssi_r3(&mut self) -> SYSCTL_SRSSI_R3_W {
        SYSCTL_SRSSI_R3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srssi](index.html) module"]
pub struct SRSSI_SPEC;
impl crate::RegisterSpec for SRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srssi::R](R) reader structure"]
impl crate::Readable for SRSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srssi::W](W) writer structure"]
impl crate::Writable for SRSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSSI to value 0"]
impl crate::Resettable for SRSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
