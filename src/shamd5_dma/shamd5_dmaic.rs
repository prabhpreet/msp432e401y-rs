#[doc = "Register `SHAMD5_DMAIC` reader"]
pub struct R(crate::R<SHAMD5_DMAIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAMD5_DMAIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAMD5_DMAIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAMD5_DMAIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHAMD5_DMAIC` writer"]
pub struct W(crate::W<SHAMD5_DMAIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHAMD5_DMAIC_SPEC>;
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
impl From<crate::W<SHAMD5_DMAIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHAMD5_DMAIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_DMAIC_CIN` reader - Context In DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_CIN_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_DMAIC_CIN` writer - Context In DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_CIN_W<'a> = crate::BitWriter<'a, u32, SHAMD5_DMAIC_SPEC, bool, 0>;
#[doc = "Field `SHAMD5_DMAIC_COUT` reader - Context Out DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_COUT_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_DMAIC_COUT` writer - Context Out DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_COUT_W<'a> = crate::BitWriter<'a, u32, SHAMD5_DMAIC_SPEC, bool, 1>;
#[doc = "Field `SHAMD5_DMAIC_DIN` reader - Data In DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_DIN_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_DMAIC_DIN` writer - Data In DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_DIN_W<'a> = crate::BitWriter<'a, u32, SHAMD5_DMAIC_SPEC, bool, 2>;
#[doc = "Field `SHAMD5_DMAIC_DOUT` reader - Data Out DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_DOUT_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_DMAIC_DOUT` writer - Data Out DMA Done Interrupt Clear"]
pub type SHAMD5_DMAIC_DOUT_W<'a> = crate::BitWriter<'a, u32, SHAMD5_DMAIC_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Context In DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_cin(&self) -> SHAMD5_DMAIC_CIN_R {
        SHAMD5_DMAIC_CIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Context Out DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_cout(&self) -> SHAMD5_DMAIC_COUT_R {
        SHAMD5_DMAIC_COUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data In DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_din(&self) -> SHAMD5_DMAIC_DIN_R {
        SHAMD5_DMAIC_DIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Out DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_dout(&self) -> SHAMD5_DMAIC_DOUT_R {
        SHAMD5_DMAIC_DOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Context In DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_cin(&mut self) -> SHAMD5_DMAIC_CIN_W {
        SHAMD5_DMAIC_CIN_W::new(self)
    }
    #[doc = "Bit 1 - Context Out DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_cout(&mut self) -> SHAMD5_DMAIC_COUT_W {
        SHAMD5_DMAIC_COUT_W::new(self)
    }
    #[doc = "Bit 2 - Data In DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_din(&mut self) -> SHAMD5_DMAIC_DIN_W {
        SHAMD5_DMAIC_DIN_W::new(self)
    }
    #[doc = "Bit 3 - Data Out DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn shamd5_dmaic_dout(&mut self) -> SHAMD5_DMAIC_DOUT_W {
        SHAMD5_DMAIC_DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHAMD5 DMA Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shamd5_dmaic](index.html) module"]
pub struct SHAMD5_DMAIC_SPEC;
impl crate::RegisterSpec for SHAMD5_DMAIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shamd5_dmaic::R](R) reader structure"]
impl crate::Readable for SHAMD5_DMAIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shamd5_dmaic::W](W) writer structure"]
impl crate::Writable for SHAMD5_DMAIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHAMD5_DMAIC to value 0"]
impl crate::Resettable for SHAMD5_DMAIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
