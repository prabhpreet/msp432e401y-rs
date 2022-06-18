#[doc = "Register `DCGCSSI` reader"]
pub struct R(crate::R<DCGCSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCSSI` writer"]
pub struct W(crate::W<DCGCSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCSSI_SPEC>;
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
impl From<crate::W<DCGCSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCSSI_D0` reader - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCSSI_D0` writer - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D0_W<'a> = crate::BitWriter<'a, u32, DCGCSSI_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_DCGCSSI_D1` reader - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D1_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCSSI_D1` writer - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D1_W<'a> = crate::BitWriter<'a, u32, DCGCSSI_SPEC, bool, 1>;
#[doc = "Field `SYSCTL_DCGCSSI_D2` reader - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D2_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCSSI_D2` writer - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D2_W<'a> = crate::BitWriter<'a, u32, DCGCSSI_SPEC, bool, 2>;
#[doc = "Field `SYSCTL_DCGCSSI_D3` reader - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D3_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCSSI_D3` writer - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCSSI_D3_W<'a> = crate::BitWriter<'a, u32, DCGCSSI_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d0(&self) -> SYSCTL_DCGCSSI_D0_R {
        SYSCTL_DCGCSSI_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d1(&self) -> SYSCTL_DCGCSSI_D1_R {
        SYSCTL_DCGCSSI_D1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d2(&self) -> SYSCTL_DCGCSSI_D2_R {
        SYSCTL_DCGCSSI_D2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d3(&self) -> SYSCTL_DCGCSSI_D3_R {
        SYSCTL_DCGCSSI_D3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Module 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d0(&mut self) -> SYSCTL_DCGCSSI_D0_W {
        SYSCTL_DCGCSSI_D0_W::new(self)
    }
    #[doc = "Bit 1 - SSI Module 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d1(&mut self) -> SYSCTL_DCGCSSI_D1_W {
        SYSCTL_DCGCSSI_D1_W::new(self)
    }
    #[doc = "Bit 2 - SSI Module 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d2(&mut self) -> SYSCTL_DCGCSSI_D2_W {
        SYSCTL_DCGCSSI_D2_W::new(self)
    }
    #[doc = "Bit 3 - SSI Module 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcssi_d3(&mut self) -> SYSCTL_DCGCSSI_D3_W {
        SYSCTL_DCGCSSI_D3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcssi](index.html) module"]
pub struct DCGCSSI_SPEC;
impl crate::RegisterSpec for DCGCSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcssi::R](R) reader structure"]
impl crate::Readable for DCGCSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcssi::W](W) writer structure"]
impl crate::Writable for DCGCSSI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCSSI to value 0"]
impl crate::Resettable for DCGCSSI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
