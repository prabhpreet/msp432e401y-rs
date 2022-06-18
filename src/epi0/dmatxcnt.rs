#[doc = "Register `DMATXCNT` reader"]
pub struct R(crate::R<DMATXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATXCNT` writer"]
pub struct W(crate::W<DMATXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATXCNT_SPEC>;
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
impl From<crate::W<DMATXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_DMATXCNT_TXCNT` reader - DMA Count"]
pub type EPI_DMATXCNT_TXCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_DMATXCNT_TXCNT` writer - DMA Count"]
pub type EPI_DMATXCNT_TXCNT_W<'a> = crate::FieldWriter<'a, u32, DMATXCNT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - DMA Count"]
    #[inline(always)]
    pub fn epi_dmatxcnt_txcnt(&self) -> EPI_DMATXCNT_TXCNT_R {
        EPI_DMATXCNT_TXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Count"]
    #[inline(always)]
    pub fn epi_dmatxcnt_txcnt(&mut self) -> EPI_DMATXCNT_TXCNT_W {
        EPI_DMATXCNT_TXCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI DMA Transmit Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatxcnt](index.html) module"]
pub struct DMATXCNT_SPEC;
impl crate::RegisterSpec for DMATXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatxcnt::R](R) reader structure"]
impl crate::Readable for DMATXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatxcnt::W](W) writer structure"]
impl crate::Writable for DMATXCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATXCNT to value 0"]
impl crate::Resettable for DMATXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
