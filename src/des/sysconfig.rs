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
#[doc = "Field `DES_SYSCONFIG_SOFTRESET` reader - Soft reset"]
pub type DES_SYSCONFIG_SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `DES_SYSCONFIG_SOFTRESET` writer - Soft reset"]
pub type DES_SYSCONFIG_SOFTRESET_W<'a> = crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 1>;
#[doc = "Sidle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DES_SYSCONFIG_SIDLE_A {
    #[doc = "0: Force-idle mode"]
    DES_SYSCONFIG_SIDLE_FORCE = 0,
}
impl From<DES_SYSCONFIG_SIDLE_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_SYSCONFIG_SIDLE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DES_SYSCONFIG_SIDLE` reader - Sidle mode"]
pub type DES_SYSCONFIG_SIDLE_R = crate::FieldReader<u8, DES_SYSCONFIG_SIDLE_A>;
impl DES_SYSCONFIG_SIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DES_SYSCONFIG_SIDLE_A> {
        match self.bits {
            0 => Some(DES_SYSCONFIG_SIDLE_A::DES_SYSCONFIG_SIDLE_FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DES_SYSCONFIG_SIDLE_FORCE`"]
    #[inline(always)]
    pub fn is_des_sysconfig_sidle_force(&self) -> bool {
        *self == DES_SYSCONFIG_SIDLE_A::DES_SYSCONFIG_SIDLE_FORCE
    }
}
#[doc = "Field `DES_SYSCONFIG_SIDLE` writer - Sidle mode"]
pub type DES_SYSCONFIG_SIDLE_W<'a> =
    crate::FieldWriter<'a, u32, SYSCONFIG_SPEC, u8, DES_SYSCONFIG_SIDLE_A, 2, 2>;
impl<'a> DES_SYSCONFIG_SIDLE_W<'a> {
    #[doc = "Force-idle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle_force(self) -> &'a mut W {
        self.variant(DES_SYSCONFIG_SIDLE_A::DES_SYSCONFIG_SIDLE_FORCE)
    }
}
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_DATA_IN_EN` reader - DMA Request Data In Enable"]
pub type DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R = crate::BitReader<bool>;
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_DATA_IN_EN` writer - DMA Request Data In Enable"]
pub type DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 5>;
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN` reader - DMA Request Data Out Enable"]
pub type DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN` writer - DMA Request Data Out Enable"]
pub type DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 6>;
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN` reader - DMA Request Context In Enable"]
pub type DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R = crate::BitReader<bool>;
#[doc = "Field `DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN` writer - DMA Request Context In Enable"]
pub type DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W<'a> =
    crate::BitWriter<'a, u32, SYSCONFIG_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn des_sysconfig_softreset(&self) -> DES_SYSCONFIG_SOFTRESET_R {
        DES_SYSCONFIG_SOFTRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Sidle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle(&self) -> DES_SYSCONFIG_SIDLE_R {
        DES_SYSCONFIG_SIDLE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_in_en(&self) -> DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R {
        DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_out_en(&self) -> DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R {
        DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_context_in_en(&self) -> DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R {
        DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Soft reset"]
    #[inline(always)]
    pub fn des_sysconfig_softreset(&mut self) -> DES_SYSCONFIG_SOFTRESET_W {
        DES_SYSCONFIG_SOFTRESET_W::new(self)
    }
    #[doc = "Bits 2:3 - Sidle mode"]
    #[inline(always)]
    pub fn des_sysconfig_sidle(&mut self) -> DES_SYSCONFIG_SIDLE_W {
        DES_SYSCONFIG_SIDLE_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Data In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_in_en(&mut self) -> DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W {
        DES_SYSCONFIG_DMA_REQ_DATA_IN_EN_W::new(self)
    }
    #[doc = "Bit 6 - DMA Request Data Out Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_data_out_en(&mut self) -> DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W {
        DES_SYSCONFIG_DMA_REQ_DATA_OUT_EN_W::new(self)
    }
    #[doc = "Bit 7 - DMA Request Context In Enable"]
    #[inline(always)]
    pub fn des_sysconfig_dma_req_context_in_en(&mut self) -> DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W {
        DES_SYSCONFIG_DMA_REQ_CONTEXT_IN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES System Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysconfig](index.html) module"]
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
