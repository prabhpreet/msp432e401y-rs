#[doc = "Register `PCDMA` reader"]
pub struct R(crate::R<PCDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDMA` writer"]
pub struct W(crate::W<PCDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDMA_SPEC>;
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
impl From<crate::W<PCDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCDMA_P0` reader - uDMA Module Power Control"]
pub type SYSCTL_PCDMA_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCDMA_P0` writer - uDMA Module Power Control"]
pub type SYSCTL_PCDMA_P0_W<'a> = crate::BitWriter<'a, u32, PCDMA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcdma_p0(&self) -> SYSCTL_PCDMA_P0_R {
        SYSCTL_PCDMA_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcdma_p0(&mut self) -> SYSCTL_PCDMA_P0_W {
        SYSCTL_PCDMA_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro Direct Memory Access Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdma](index.html) module"]
pub struct PCDMA_SPEC;
impl crate::RegisterSpec for PCDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdma::R](R) reader structure"]
impl crate::Readable for PCDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdma::W](W) writer structure"]
impl crate::Writable for PCDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCDMA to value 0"]
impl crate::Resettable for PCDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
