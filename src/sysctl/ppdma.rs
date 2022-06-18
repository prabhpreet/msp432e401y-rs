#[doc = "Register `PPDMA` reader"]
pub struct R(crate::R<PPDMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPDMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPDMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPDMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPDMA` writer"]
pub struct W(crate::W<PPDMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPDMA_SPEC>;
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
impl From<crate::W<PPDMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPDMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPDMA_P0` reader - uDMA Module Present"]
pub type SYSCTL_PPDMA_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPDMA_P0` writer - uDMA Module Present"]
pub type SYSCTL_PPDMA_P0_W<'a> = crate::BitWriter<'a, u32, PPDMA_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - uDMA Module Present"]
    #[inline(always)]
    pub fn sysctl_ppdma_p0(&self) -> SYSCTL_PPDMA_P0_R {
        SYSCTL_PPDMA_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA Module Present"]
    #[inline(always)]
    pub fn sysctl_ppdma_p0(&mut self) -> SYSCTL_PPDMA_P0_W {
        SYSCTL_PPDMA_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Micro Direct Memory Access Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppdma](index.html) module"]
pub struct PPDMA_SPEC;
impl crate::RegisterSpec for PPDMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppdma::R](R) reader structure"]
impl crate::Readable for PPDMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppdma::W](W) writer structure"]
impl crate::Writable for PPDMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPDMA to value 0"]
impl crate::Resettable for PPDMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
