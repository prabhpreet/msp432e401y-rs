#[doc = "Register `SREMAC` reader"]
pub struct R(crate::R<SREMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREMAC` writer"]
pub struct W(crate::W<SREMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREMAC_SPEC>;
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
impl From<crate::W<SREMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SREMAC_R0` reader - Ethernet Controller MAC Module 0 Software Reset"]
pub type SYSCTL_SREMAC_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SREMAC_R0` writer - Ethernet Controller MAC Module 0 Software Reset"]
pub type SYSCTL_SREMAC_R0_W<'a> = crate::BitWriter<'a, u32, SREMAC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Ethernet Controller MAC Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sremac_r0(&self) -> SYSCTL_SREMAC_R0_R {
        SYSCTL_SREMAC_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet Controller MAC Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sremac_r0(&mut self) -> SYSCTL_SREMAC_R0_W {
        SYSCTL_SREMAC_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sremac](index.html) module"]
pub struct SREMAC_SPEC;
impl crate::RegisterSpec for SREMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sremac::R](R) reader structure"]
impl crate::Readable for SREMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sremac::W](W) writer structure"]
impl crate::Writable for SREMAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREMAC to value 0"]
impl crate::Resettable for SREMAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
