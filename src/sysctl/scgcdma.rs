#[doc = "Register `SCGCDMA` reader"]
pub struct R(crate::R<SCGCDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCDMA` writer"]
pub struct W(crate::W<SCGCDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCDMA_SPEC>;
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
impl From<crate::W<SCGCDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCDMA_S0` reader - uDMA Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCDMA_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCDMA_S0` writer - uDMA Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCDMA_S0_W<'a> = crate::BitWriter<'a, u32, SCGCDMA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcdma_s0(&self) -> SYSCTL_SCGCDMA_S0_R {
        SYSCTL_SCGCDMA_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcdma_s0(&mut self) -> SYSCTL_SCGCDMA_S0_W {
        SYSCTL_SCGCDMA_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcdma](index.html) module"]
pub struct SCGCDMA_SPEC;
impl crate::RegisterSpec for SCGCDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcdma::R](R) reader structure"]
impl crate::Readable for SCGCDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcdma::W](W) writer structure"]
impl crate::Writable for SCGCDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCDMA to value 0"]
impl crate::Resettable for SCGCDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
