#[doc = "Register `PCEPHY` reader"]
pub struct R(crate::R<PCEPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCEPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCEPHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCEPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCEPHY` writer"]
pub struct W(crate::W<PCEPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCEPHY_SPEC>;
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
impl From<crate::W<PCEPHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCEPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCEPHY_P0` reader - Ethernet PHY Module Power Control"]
pub type SYSCTL_PCEPHY_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCEPHY_P0` writer - Ethernet PHY Module Power Control"]
pub type SYSCTL_PCEPHY_P0_W<'a> = crate::BitWriter<'a, u32, PCEPHY_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcephy_p0(&self) -> SYSCTL_PCEPHY_P0_R {
        SYSCTL_PCEPHY_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcephy_p0(&mut self) -> SYSCTL_PCEPHY_P0_W {
        SYSCTL_PCEPHY_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcephy](index.html) module"]
pub struct PCEPHY_SPEC;
impl crate::RegisterSpec for PCEPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcephy::R](R) reader structure"]
impl crate::Readable for PCEPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcephy::W](W) writer structure"]
impl crate::Writable for PCEPHY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCEPHY to value 0"]
impl crate::Resettable for PCEPHY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
