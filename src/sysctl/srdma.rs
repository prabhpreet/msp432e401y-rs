#[doc = "Register `SRDMA` reader"]
pub struct R(crate::R<SRDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDMA` writer"]
pub struct W(crate::W<SRDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDMA_SPEC>;
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
impl From<crate::W<SRDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRDMA_R0` reader - uDMA Module Software Reset"]
pub type SYSCTL_SRDMA_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRDMA_R0` writer - uDMA Module Software Reset"]
pub type SYSCTL_SRDMA_R0_W<'a> = crate::BitWriter<'a, u32, SRDMA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srdma_r0(&self) -> SYSCTL_SRDMA_R0_R {
        SYSCTL_SRDMA_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srdma_r0(&mut self) -> SYSCTL_SRDMA_R0_W {
        SYSCTL_SRDMA_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro Direct Memory Access Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdma](index.html) module"]
pub struct SRDMA_SPEC;
impl crate::RegisterSpec for SRDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdma::R](R) reader structure"]
impl crate::Readable for SRDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdma::W](W) writer structure"]
impl crate::Writable for SRDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRDMA to value 0"]
impl crate::Resettable for SRDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
