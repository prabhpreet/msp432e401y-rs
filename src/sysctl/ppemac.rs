#[doc = "Register `PPEMAC` reader"]
pub struct R(crate::R<PPEMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPEMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPEMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPEMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPEMAC` writer"]
pub struct W(crate::W<PPEMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPEMAC_SPEC>;
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
impl From<crate::W<PPEMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPEMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPEMAC_P0` reader - Ethernet Controller Module Present"]
pub type SYSCTL_PPEMAC_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPEMAC_P0` writer - Ethernet Controller Module Present"]
pub type SYSCTL_PPEMAC_P0_W<'a> = crate::BitWriter<'a, u32, PPEMAC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet Controller Module Present"]
    #[inline(always)]
    pub fn sysctl_ppemac_p0(&self) -> SYSCTL_PPEMAC_P0_R {
        SYSCTL_PPEMAC_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet Controller Module Present"]
    #[inline(always)]
    pub fn sysctl_ppemac_p0(&mut self) -> SYSCTL_PPEMAC_P0_W {
        SYSCTL_PPEMAC_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppemac](index.html) module"]
pub struct PPEMAC_SPEC;
impl crate::RegisterSpec for PPEMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppemac::R](R) reader structure"]
impl crate::Readable for PPEMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppemac::W](W) writer structure"]
impl crate::Writable for PPEMAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPEMAC to value 0"]
impl crate::Resettable for PPEMAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
