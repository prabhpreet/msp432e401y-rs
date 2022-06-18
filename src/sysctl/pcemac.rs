#[doc = "Register `PCEMAC` reader"]
pub struct R(crate::R<PCEMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCEMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCEMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCEMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCEMAC` writer"]
pub struct W(crate::W<PCEMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCEMAC_SPEC>;
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
impl From<crate::W<PCEMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCEMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCEMAC_P0` reader - Ethernet MAC Module 0 Power Control"]
pub type SYSCTL_PCEMAC_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCEMAC_P0` writer - Ethernet MAC Module 0 Power Control"]
pub type SYSCTL_PCEMAC_P0_W<'a> = crate::BitWriter<'a, u32, PCEMAC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet MAC Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcemac_p0(&self) -> SYSCTL_PCEMAC_P0_R {
        SYSCTL_PCEMAC_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcemac_p0(&mut self) -> SYSCTL_PCEMAC_P0_W {
        SYSCTL_PCEMAC_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcemac](index.html) module"]
pub struct PCEMAC_SPEC;
impl crate::RegisterSpec for PCEMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcemac::R](R) reader structure"]
impl crate::Readable for PCEMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcemac::W](W) writer structure"]
impl crate::Writable for PCEMAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCEMAC to value 0"]
impl crate::Resettable for PCEMAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
