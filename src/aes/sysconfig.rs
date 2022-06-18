#[doc = "Register `SYSCONFIG` reader"]
pub struct R(crate::R<SYSCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCONFIG` writer"]
pub struct W(crate::W<SYSCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCONFIG_SPEC>;
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
impl From<crate::W<SYSCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_SYSCONFIG_SOFTRESET` reader - Soft reset"]
pub type AES_SYSCONFIG_SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_SOFTRESET` writer - Soft reset"]
pub type AES_SYSCONFIG_SOFTRESET_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 1>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_DATA_IN_EN` reader - DMA Request Data In Enable"]
pub type AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_DATA_IN_EN` writer - DMA Request Data In Enable"]
pub type AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 5>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN` reader - DMA Request Data Out Enable"]
pub type AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN` writer - DMA Request Data Out Enable"]
pub type AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 6>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN` reader - DMA Request Context In Enable"]
pub type AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN` writer - DMA Request Context In Enable"]
pub type AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 7>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN` reader - DMA Request Context Out Enable"]
pub type AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN` writer - DMA Request Context Out Enable"]
pub type AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 8>;
#[doc = "Field `AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT` reader - Map Context Out on Data Out Enable"]
pub type AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT` writer - Map Context Out on Data Out Enable"]
pub type AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 9>;
#[doc = "Field `AES_SYSCONFIG_KEYENC` reader - Key Encoding"]
pub type AES_SYSCONFIG_KEYENC_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_KEYENC` writer - Key Encoding"]
pub type AES_SYSCONFIG_KEYENC_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 11>;
#[doc = "Field `AES_SYSCONFIG_K3` reader - K3 Select"]
pub type AES_SYSCONFIG_K3_R = crate::BitReader<bool>;
#[doc = "Field `AES_SYSCONFIG_K3` writer - K3 Select"]
pub type AES_SYSCONFIG_K3_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn aes_sysconfig_softreset(&self) -> AES_SYSCONFIG_SOFTRESET_R {
        AES_SYSCONFIG_SOFTRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_data_in_en(&self) -> AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R {
        AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_data_out_en(&self) -> AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R {
        AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_context_in_en(&self) -> AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R {
        AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Request Context Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_context_out_en(&self) -> AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_R {
        AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Map Context Out on Data Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_map_context_out_on_data_out(
        &self,
    ) -> AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_R {
        AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Key Encoding"]
    #[inline(always)]
    pub fn aes_sysconfig_keyenc(&self) -> AES_SYSCONFIG_KEYENC_R {
        AES_SYSCONFIG_KEYENC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - K3 Select"]
    #[inline(always)]
    pub fn aes_sysconfig_k3(&self) -> AES_SYSCONFIG_K3_R {
        AES_SYSCONFIG_K3_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn aes_sysconfig_softreset(&mut self) -> AES_SYSCONFIG_SOFTRESET_W {
        AES_SYSCONFIG_SOFTRESET_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_data_in_en(&mut self) -> AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W {
        AES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W::new(self)
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_data_out_en(&mut self) -> AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W {
        AES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W::new(self)
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_context_in_en(&mut self) -> AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W {
        AES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W::new(self)
    }
    #[doc = "Bit 8 - DMA Request Context Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_dma_req_context_out_en(
        &mut self,
    ) -> AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_W {
        AES_SYSCONFIG_DMA_REQ_CONTEXT_OUT_EN_W::new(self)
    }
    #[doc = "Bit 9 - Map Context Out on Data Out Enable"]
    #[inline(always)]
    pub fn aes_sysconfig_map_context_out_on_data_out(
        &mut self,
    ) -> AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_W {
        AES_SYSCONFIG_MAP_CONTEXT_OUT_ON_DATA_OUT_W::new(self)
    }
    #[doc = "Bit 11 - Key Encoding"]
    #[inline(always)]
    pub fn aes_sysconfig_keyenc(&mut self) -> AES_SYSCONFIG_KEYENC_W {
        AES_SYSCONFIG_KEYENC_W::new(self)
    }
    #[doc = "Bit 12 - K3 Select"]
    #[inline(always)]
    pub fn aes_sysconfig_k3(&mut self) -> AES_SYSCONFIG_K3_W {
        AES_SYSCONFIG_K3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES System Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](index.html) module"]
pub struct SYSCONFIG_SPEC;
impl crate::RegisterSpec for SYSCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysconfig::R](R) reader structure"]
impl crate::Readable for SYSCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysconfig::W](W) writer structure"]
impl crate::Writable for SYSCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCONFIG to value 0"]
impl crate::Resettable for SYSCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
