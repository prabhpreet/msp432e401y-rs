#[doc = "Register `PPEPHY` reader"]
pub struct R(crate::R<PPEPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPEPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPEPHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPEPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPEPHY` writer"]
pub struct W(crate::W<PPEPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPEPHY_SPEC>;
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
impl From<crate::W<PPEPHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPEPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPEPHY_P0` reader - Ethernet PHY Module Present"]
pub type SYSCTL_PPEPHY_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPEPHY_P0` writer - Ethernet PHY Module Present"]
pub type SYSCTL_PPEPHY_P0_W<'a> = crate::BitWriter<'a, u32, PPEPHY_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Module Present"]
    #[inline(always)]
    pub fn sysctl_ppephy_p0(&self) -> SYSCTL_PPEPHY_P0_R {
        SYSCTL_PPEPHY_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Module Present"]
    #[inline(always)]
    pub fn sysctl_ppephy_p0(&mut self) -> SYSCTL_PPEPHY_P0_W {
        SYSCTL_PPEPHY_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppephy](index.html) module"]
pub struct PPEPHY_SPEC;
impl crate::RegisterSpec for PPEPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppephy::R](R) reader structure"]
impl crate::Readable for PPEPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppephy::W](W) writer structure"]
impl crate::Writable for PPEPHY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPEPHY to value 0"]
impl crate::Resettable for PPEPHY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
