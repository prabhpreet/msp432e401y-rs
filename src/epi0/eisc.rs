#[doc = "Register `EISC` reader"]
pub struct R(crate::R<EISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EISC` writer"]
pub struct W(crate::W<EISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EISC_SPEC>;
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
impl From<crate::W<EISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_EISC_TOUT` reader - Timeout Error"]
pub type EPI_EISC_TOUT_R = crate::BitReader<bool>;
#[doc = "Field `EPI_EISC_TOUT` writer - Timeout Error"]
pub type EPI_EISC_TOUT_W<'a> = crate::BitWriter<'a, u32, EISC_SPEC, bool, 0>;
#[doc = "Field `EPI_EISC_RSTALL` reader - Read Stalled Error"]
pub type EPI_EISC_RSTALL_R = crate::BitReader<bool>;
#[doc = "Field `EPI_EISC_RSTALL` writer - Read Stalled Error"]
pub type EPI_EISC_RSTALL_W<'a> = crate::BitWriter<'a, u32, EISC_SPEC, bool, 1>;
#[doc = "Field `EPI_EISC_WTFULL` reader - Write FIFO Full Error"]
pub type EPI_EISC_WTFULL_R = crate::BitReader<bool>;
#[doc = "Field `EPI_EISC_WTFULL` writer - Write FIFO Full Error"]
pub type EPI_EISC_WTFULL_W<'a> = crate::BitWriter<'a, u32, EISC_SPEC, bool, 2>;
#[doc = "Field `EPI_EISC_DMARDIC` reader - Read uDMA Interrupt Clear"]
pub type EPI_EISC_DMARDIC_R = crate::BitReader<bool>;
#[doc = "Field `EPI_EISC_DMARDIC` writer - Read uDMA Interrupt Clear"]
pub type EPI_EISC_DMARDIC_W<'a> = crate::BitWriter<'a, u32, EISC_SPEC, bool, 3>;
#[doc = "Field `EPI_EISC_DMAWRIC` reader - Write uDMA Interrupt Clear"]
pub type EPI_EISC_DMAWRIC_R = crate::BitReader<bool>;
#[doc = "Field `EPI_EISC_DMAWRIC` writer - Write uDMA Interrupt Clear"]
pub type EPI_EISC_DMAWRIC_W<'a> = crate::BitWriter<'a, u32, EISC_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn epi_eisc_tout(&self) -> EPI_EISC_TOUT_R {
        EPI_EISC_TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn epi_eisc_rstall(&self) -> EPI_EISC_RSTALL_R {
        EPI_EISC_RSTALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn epi_eisc_wtfull(&self) -> EPI_EISC_WTFULL_R {
        EPI_EISC_WTFULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmardic(&self) -> EPI_EISC_DMARDIC_R {
        EPI_EISC_DMARDIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmawric(&self) -> EPI_EISC_DMAWRIC_R {
        EPI_EISC_DMAWRIC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Error"]
    #[inline(always)]
    pub fn epi_eisc_tout(&mut self) -> EPI_EISC_TOUT_W {
        EPI_EISC_TOUT_W::new(self)
    }
    #[doc = "Bit 1 - Read Stalled Error"]
    #[inline(always)]
    pub fn epi_eisc_rstall(&mut self) -> EPI_EISC_RSTALL_W {
        EPI_EISC_RSTALL_W::new(self)
    }
    #[doc = "Bit 2 - Write FIFO Full Error"]
    #[inline(always)]
    pub fn epi_eisc_wtfull(&mut self) -> EPI_EISC_WTFULL_W {
        EPI_EISC_WTFULL_W::new(self)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmardic(&mut self) -> EPI_EISC_DMARDIC_W {
        EPI_EISC_DMARDIC_W::new(self)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn epi_eisc_dmawric(&mut self) -> EPI_EISC_DMAWRIC_W {
        EPI_EISC_DMAWRIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Error and Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisc](index.html) module"]
pub struct EISC_SPEC;
impl crate::RegisterSpec for EISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eisc::R](R) reader structure"]
impl crate::Readable for EISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eisc::W](W) writer structure"]
impl crate::Writable for EISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EISC to value 0"]
impl crate::Resettable for EISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
