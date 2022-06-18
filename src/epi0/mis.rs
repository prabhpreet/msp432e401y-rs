#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_MIS_ERRMIS` reader - Error Masked Interrupt Status"]
pub type EPI_MIS_ERRMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_MIS_ERRMIS` writer - Error Masked Interrupt Status"]
pub type EPI_MIS_ERRMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `EPI_MIS_RDMIS` reader - Read Masked Interrupt Status"]
pub type EPI_MIS_RDMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_MIS_RDMIS` writer - Read Masked Interrupt Status"]
pub type EPI_MIS_RDMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 1>;
#[doc = "Field `EPI_MIS_WRMIS` reader - Write Masked Interrupt Status"]
pub type EPI_MIS_WRMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_MIS_WRMIS` writer - Write Masked Interrupt Status"]
pub type EPI_MIS_WRMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `EPI_MIS_DMARDMIS` reader - Read uDMA Masked Interrupt Status"]
pub type EPI_MIS_DMARDMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_MIS_DMARDMIS` writer - Read uDMA Masked Interrupt Status"]
pub type EPI_MIS_DMARDMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `EPI_MIS_DMAWRMIS` reader - Write uDMA Masked Interrupt Status"]
pub type EPI_MIS_DMAWRMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPI_MIS_DMAWRMIS` writer - Write uDMA Masked Interrupt Status"]
pub type EPI_MIS_DMAWRMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_errmis(&self) -> EPI_MIS_ERRMIS_R {
        EPI_MIS_ERRMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_rdmis(&self) -> EPI_MIS_RDMIS_R {
        EPI_MIS_RDMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_wrmis(&self) -> EPI_MIS_WRMIS_R {
        EPI_MIS_WRMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmardmis(&self) -> EPI_MIS_DMARDMIS_R {
        EPI_MIS_DMARDMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmawrmis(&self) -> EPI_MIS_DMAWRMIS_R {
        EPI_MIS_DMAWRMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_errmis(&mut self) -> EPI_MIS_ERRMIS_W {
        EPI_MIS_ERRMIS_W::new(self)
    }
    #[doc = "Bit 1 - Read Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_rdmis(&mut self) -> EPI_MIS_RDMIS_W {
        EPI_MIS_RDMIS_W::new(self)
    }
    #[doc = "Bit 2 - Write Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_wrmis(&mut self) -> EPI_MIS_WRMIS_W {
        EPI_MIS_WRMIS_W::new(self)
    }
    #[doc = "Bit 3 - Read uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmardmis(&mut self) -> EPI_MIS_DMARDMIS_W {
        EPI_MIS_DMARDMIS_W::new(self)
    }
    #[doc = "Bit 4 - Write uDMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn epi_mis_dmawrmis(&mut self) -> EPI_MIS_DMAWRMIS_W {
        EPI_MIS_DMAWRMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
