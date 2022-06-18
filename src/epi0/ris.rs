#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_RIS_ERRRIS` reader - Error Raw Interrupt Status"]
pub type EPI_RIS_ERRRIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_RIS_ERRRIS` writer - Error Raw Interrupt Status"]
pub type EPI_RIS_ERRRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `EPI_RIS_RDRIS` reader - Read Raw Interrupt Status"]
pub type EPI_RIS_RDRIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_RIS_RDRIS` writer - Read Raw Interrupt Status"]
pub type EPI_RIS_RDRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `EPI_RIS_WRRIS` reader - Write Raw Interrupt Status"]
pub type EPI_RIS_WRRIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_RIS_WRRIS` writer - Write Raw Interrupt Status"]
pub type EPI_RIS_WRRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `EPI_RIS_DMARDRIS` reader - Read uDMA Raw Interrupt Status"]
pub type EPI_RIS_DMARDRIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_RIS_DMARDRIS` writer - Read uDMA Raw Interrupt Status"]
pub type EPI_RIS_DMARDRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `EPI_RIS_DMAWRRIS` reader - Write uDMA Raw Interrupt Status"]
pub type EPI_RIS_DMAWRRIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_RIS_DMAWRRIS` writer - Write uDMA Raw Interrupt Status"]
pub type EPI_RIS_DMAWRRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_errris(&self) -> EPI_RIS_ERRRIS_R {
        EPI_RIS_ERRRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_rdris(&self) -> EPI_RIS_RDRIS_R {
        EPI_RIS_RDRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_wrris(&self) -> EPI_RIS_WRRIS_R {
        EPI_RIS_WRRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmardris(&self) -> EPI_RIS_DMARDRIS_R {
        EPI_RIS_DMARDRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmawrris(&self) -> EPI_RIS_DMAWRRIS_R {
        EPI_RIS_DMAWRRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_errris(&mut self) -> EPI_RIS_ERRRIS_W {
        EPI_RIS_ERRRIS_W::new(self)
    }
    #[doc = "Bit 1 - Read Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_rdris(&mut self) -> EPI_RIS_RDRIS_W {
        EPI_RIS_RDRIS_W::new(self)
    }
    #[doc = "Bit 2 - Write Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_wrris(&mut self) -> EPI_RIS_WRRIS_W {
        EPI_RIS_WRRIS_W::new(self)
    }
    #[doc = "Bit 3 - Read uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmardris(&mut self) -> EPI_RIS_DMARDRIS_W {
        EPI_RIS_DMARDRIS_W::new(self)
    }
    #[doc = "Bit 4 - Write uDMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn epi_ris_dmawrris(&mut self) -> EPI_RIS_DMAWRRIS_W {
        EPI_RIS_DMAWRRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
