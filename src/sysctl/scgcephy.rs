#[doc = "Register `SCGCEPHY` reader"]
pub struct R(crate::R<SCGCEPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCEPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCEPHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCEPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCEPHY` writer"]
pub struct W(crate::W<SCGCEPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCEPHY_SPEC>;
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
impl From<crate::W<SCGCEPHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCEPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCEPHY_S0` reader - PHY Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEPHY_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCEPHY_S0` writer - PHY Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEPHY_S0_W<'a> = crate::BitWriter<'a, u32, SCGCEPHY_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - PHY Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcephy_s0(&self) -> SYSCTL_SCGCEPHY_S0_R {
        SYSCTL_SCGCEPHY_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PHY Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcephy_s0(&mut self) -> SYSCTL_SCGCEPHY_S0_W {
        SYSCTL_SCGCEPHY_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcephy](index.html) module"]
pub struct SCGCEPHY_SPEC;
impl crate::RegisterSpec for SCGCEPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcephy::R](R) reader structure"]
impl crate::Readable for SCGCEPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcephy::W](W) writer structure"]
impl crate::Writable for SCGCEPHY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCEPHY to value 0"]
impl crate::Resettable for SCGCEPHY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
