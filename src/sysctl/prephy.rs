#[doc = "Register `PREPHY` reader"]
pub struct R(crate::R<PREPHY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREPHY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREPHY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREPHY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREPHY` writer"]
pub struct W(crate::W<PREPHY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREPHY_SPEC>;
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
impl From<crate::W<PREPHY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREPHY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PREPHY_R0` reader - Ethernet PHY Module Peripheral Ready"]
pub type SYSCTL_PREPHY_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PREPHY_R0` writer - Ethernet PHY Module Peripheral Ready"]
pub type SYSCTL_PREPHY_R0_W<'a> = crate::BitWriter<'a, u32, PREPHY_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet PHY Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prephy_r0(&self) -> SYSCTL_PREPHY_R0_R {
        SYSCTL_PREPHY_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet PHY Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prephy_r0(&mut self) -> SYSCTL_PREPHY_R0_W {
        SYSCTL_PREPHY_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PHY Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prephy](index.html) module"]
pub struct PREPHY_SPEC;
impl crate::RegisterSpec for PREPHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prephy::R](R) reader structure"]
impl crate::Readable for PREPHY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prephy::W](W) writer structure"]
impl crate::Writable for PREPHY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREPHY to value 0"]
impl crate::Resettable for PREPHY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
