#[doc = "Register `PPSSI` reader"]
pub struct R(crate::R<PPSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPSSI` writer"]
pub struct W(crate::W<PPSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPSSI_SPEC>;
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
impl From<crate::W<PPSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPSSI_P0` reader - SSI Module 0 Present"]
pub type SYSCTL_PPSSI_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPSSI_P0` writer - SSI Module 0 Present"]
pub type SYSCTL_PPSSI_P0_W<'a> = crate::BitWriter<'a, u32, PPSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PPSSI_P1` reader - SSI Module 1 Present"]
pub type SYSCTL_PPSSI_P1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPSSI_P1` writer - SSI Module 1 Present"]
pub type SYSCTL_PPSSI_P1_W<'a> = crate::BitWriter<'a, u32, PPSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_PPSSI_P2` reader - SSI Module 2 Present"]
pub type SYSCTL_PPSSI_P2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPSSI_P2` writer - SSI Module 2 Present"]
pub type SYSCTL_PPSSI_P2_W<'a> = crate::BitWriter<'a, u32, PPSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_PPSSI_P3` reader - SSI Module 3 Present"]
pub type SYSCTL_PPSSI_P3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPSSI_P3` writer - SSI Module 3 Present"]
pub type SYSCTL_PPSSI_P3_W<'a> = crate::BitWriter<'a, u32, PPSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p0(&self) -> SYSCTL_PPSSI_P0_R {
        SYSCTL_PPSSI_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p1(&self) -> SYSCTL_PPSSI_P1_R {
        SYSCTL_PPSSI_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p2(&self) -> SYSCTL_PPSSI_P2_R {
        SYSCTL_PPSSI_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p3(&self) -> SYSCTL_PPSSI_P3_R {
        SYSCTL_PPSSI_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p0(&mut self) -> SYSCTL_PPSSI_P0_W {
        SYSCTL_PPSSI_P0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p1(&mut self) -> SYSCTL_PPSSI_P1_W {
        SYSCTL_PPSSI_P1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p2(&mut self) -> SYSCTL_PPSSI_P2_W {
        SYSCTL_PPSSI_P2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppssi_p3(&mut self) -> SYSCTL_PPSSI_P3_W {
        SYSCTL_PPSSI_P3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppssi](index.html) module"]
pub struct PPSSI_SPEC;
impl crate::RegisterSpec for PPSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppssi::R](R) reader structure"]
impl crate::Readable for PPSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppssi::W](W) writer structure"]
impl crate::Writable for PPSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPSSI to value 0"]
impl crate::Resettable for PPSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
