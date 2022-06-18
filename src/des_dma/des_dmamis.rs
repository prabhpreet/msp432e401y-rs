#[doc = "Register `DES_DMAMIS` reader"]
pub struct R(crate::R<DES_DMAMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DES_DMAMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DES_DMAMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DES_DMAMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DES_DMAMIS` writer"]
pub struct W(crate::W<DES_DMAMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DES_DMAMIS_SPEC>;
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
impl From<crate::W<DES_DMAMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DES_DMAMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_DMAMIS_CIN` reader - Context In DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_CIN_R = crate::BitReader<bool>;
#[doc = "Field `DES_DMAMIS_CIN` writer - Context In DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_CIN_W<'a> = crate::BitWriter<'a, u32, DES_DMAMIS_SPEC, bool, 0>;
#[doc = "Field `DES_DMAMIS_COUT` reader - Context Out DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_COUT_R = crate::BitReader<bool>;
#[doc = "Field `DES_DMAMIS_COUT` writer - Context Out DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_COUT_W<'a> = crate::BitWriter<'a, u32, DES_DMAMIS_SPEC, bool, 1>;
#[doc = "Field `DES_DMAMIS_DIN` reader - Data In DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_DIN_R = crate::BitReader<bool>;
#[doc = "Field `DES_DMAMIS_DIN` writer - Data In DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_DIN_W<'a> = crate::BitWriter<'a, u32, DES_DMAMIS_SPEC, bool, 2>;
#[doc = "Field `DES_DMAMIS_DOUT` reader - Data Out DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_DOUT_R = crate::BitReader<bool>;
#[doc = "Field `DES_DMAMIS_DOUT` writer - Data Out DMA Done Masked Interrupt Status"]
pub type DES_DMAMIS_DOUT_W<'a> = crate::BitWriter<'a, u32, DES_DMAMIS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Context In DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_cin(&self) -> DES_DMAMIS_CIN_R {
        DES_DMAMIS_CIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Context Out DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_cout(&self) -> DES_DMAMIS_COUT_R {
        DES_DMAMIS_COUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data In DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_din(&self) -> DES_DMAMIS_DIN_R {
        DES_DMAMIS_DIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Out DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_dout(&self) -> DES_DMAMIS_DOUT_R {
        DES_DMAMIS_DOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Context In DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_cin(&mut self) -> DES_DMAMIS_CIN_W {
        DES_DMAMIS_CIN_W::new(self)
    }
    #[doc = "Bit 1 - Context Out DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_cout(&mut self) -> DES_DMAMIS_COUT_W {
        DES_DMAMIS_COUT_W::new(self)
    }
    #[doc = "Bit 2 - Data In DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_din(&mut self) -> DES_DMAMIS_DIN_W {
        DES_DMAMIS_DIN_W::new(self)
    }
    #[doc = "Bit 3 - Data Out DMA Done Masked Interrupt Status"]
    #[inline(always)]
    pub fn des_dmamis_dout(&mut self) -> DES_DMAMIS_DOUT_W {
        DES_DMAMIS_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES DMA Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [des_dmamis](index.html) module"]
pub struct DES_DMAMIS_SPEC;
impl crate::RegisterSpec for DES_DMAMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [des_dmamis::R](R) reader structure"]
impl crate::Readable for DES_DMAMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [des_dmamis::W](W) writer structure"]
impl crate::Writable for DES_DMAMIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DES_DMAMIS to value 0"]
impl crate::Resettable for DES_DMAMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
