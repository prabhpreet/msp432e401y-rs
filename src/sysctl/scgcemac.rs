#[doc = "Register `SCGCEMAC` reader"]
pub struct R(crate::R<SCGCEMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCEMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCEMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCEMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCEMAC` writer"]
pub struct W(crate::W<SCGCEMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCEMAC_SPEC>;
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
impl From<crate::W<SCGCEMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCEMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCEMAC_S0` reader - Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEMAC_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCEMAC_S0` writer - Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEMAC_S0_W<'a> = crate::BitWriter<'a, u32, SCGCEMAC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcemac_s0(&self) -> SYSCTL_SCGCEMAC_S0_R {
        SYSCTL_SCGCEMAC_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcemac_s0(&mut self) -> SYSCTL_SCGCEMAC_S0_W {
        SYSCTL_SCGCEMAC_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcemac](index.html) module"]
pub struct SCGCEMAC_SPEC;
impl crate::RegisterSpec for SCGCEMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcemac::R](R) reader structure"]
impl crate::Readable for SCGCEMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcemac::W](W) writer structure"]
impl crate::Writable for SCGCEMAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCEMAC to value 0"]
impl crate::Resettable for SCGCEMAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
