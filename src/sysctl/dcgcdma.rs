#[doc = "Register `DCGCDMA` reader"]
pub struct R(crate::R<DCGCDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCDMA` writer"]
pub struct W(crate::W<DCGCDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCDMA_SPEC>;
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
impl From<crate::W<DCGCDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCDMA_D0` reader - uDMA Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCDMA_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCDMA_D0` writer - uDMA Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCDMA_D0_W<'a> = crate::BitWriter<'a, u32, DCGCDMA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcdma_d0(&self) -> SYSCTL_DCGCDMA_D0_R {
        SYSCTL_DCGCDMA_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcdma_d0(&mut self) -> SYSCTL_DCGCDMA_D0_W {
        SYSCTL_DCGCDMA_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcdma](index.html) module"]
pub struct DCGCDMA_SPEC;
impl crate::RegisterSpec for DCGCDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcdma::R](R) reader structure"]
impl crate::Readable for DCGCDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcdma::W](W) writer structure"]
impl crate::Writable for DCGCDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCDMA to value 0"]
impl crate::Resettable for DCGCDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
