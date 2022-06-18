#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_IM_ERRIM` reader - Error Interrupt Mask"]
pub type EPI_IM_ERRIM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_IM_ERRIM` writer - Error Interrupt Mask"]
pub type EPI_IM_ERRIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `EPI_IM_RDIM` reader - Read FIFO Full Interrupt Mask"]
pub type EPI_IM_RDIM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_IM_RDIM` writer - Read FIFO Full Interrupt Mask"]
pub type EPI_IM_RDIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 1>;
#[doc = "Field `EPI_IM_WRIM` reader - Write FIFO Empty Interrupt Mask"]
pub type EPI_IM_WRIM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_IM_WRIM` writer - Write FIFO Empty Interrupt Mask"]
pub type EPI_IM_WRIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `EPI_IM_DMARDIM` reader - Read uDMA Interrupt Mask"]
pub type EPI_IM_DMARDIM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_IM_DMARDIM` writer - Read uDMA Interrupt Mask"]
pub type EPI_IM_DMARDIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `EPI_IM_DMAWRIM` reader - Write uDMA Interrupt Mask"]
pub type EPI_IM_DMAWRIM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_IM_DMAWRIM` writer - Write uDMA Interrupt Mask"]
pub type EPI_IM_DMAWRIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_errim(&self) -> EPI_IM_ERRIM_R {
        EPI_IM_ERRIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_rdim(&self) -> EPI_IM_RDIM_R {
        EPI_IM_RDIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_wrim(&self) -> EPI_IM_WRIM_R {
        EPI_IM_WRIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmardim(&self) -> EPI_IM_DMARDIM_R {
        EPI_IM_DMARDIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmawrim(&self) -> EPI_IM_DMAWRIM_R {
        EPI_IM_DMAWRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_errim(&mut self) -> EPI_IM_ERRIM_W {
        EPI_IM_ERRIM_W::new(self)
    }
    #[doc = "Bit 1 - Read FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_rdim(&mut self) -> EPI_IM_RDIM_W {
        EPI_IM_RDIM_W::new(self)
    }
    #[doc = "Bit 2 - Write FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_wrim(&mut self) -> EPI_IM_WRIM_W {
        EPI_IM_WRIM_W::new(self)
    }
    #[doc = "Bit 3 - Read uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmardim(&mut self) -> EPI_IM_DMARDIM_W {
        EPI_IM_DMARDIM_W::new(self)
    }
    #[doc = "Bit 4 - Write uDMA Interrupt Mask"]
    #[inline(always)]
    pub fn epi_im_dmawrim(&mut self) -> EPI_IM_DMAWRIM_W {
        EPI_IM_DMAWRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
