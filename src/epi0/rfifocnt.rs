#[doc = "Register `RFIFOCNT` reader"]
pub struct R(crate::R<RFIFOCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFOCNT` writer"]
pub struct W(crate::W<RFIFOCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFOCNT_SPEC>;
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
impl From<crate::W<RFIFOCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFOCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_RFIFOCNT_COUNT` reader - FIFO Count"]
pub type EPI_RFIFOCNT_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_RFIFOCNT_COUNT` writer - FIFO Count"]
pub type EPI_RFIFOCNT_COUNT_W<'a> = crate::FieldWriter<'a, u32, RFIFOCNT_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - FIFO Count"]
    #[inline(always)]
    pub fn epi_rfifocnt_count(&self) -> EPI_RFIFOCNT_COUNT_R {
        EPI_RFIFOCNT_COUNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO Count"]
    #[inline(always)]
    pub fn epi_rfifocnt_count(&mut self) -> EPI_RFIFOCNT_COUNT_W {
        EPI_RFIFOCNT_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifocnt](index.html) module"]
pub struct RFIFOCNT_SPEC;
impl crate::RegisterSpec for RFIFOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifocnt::R](R) reader structure"]
impl crate::Readable for RFIFOCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifocnt::W](W) writer structure"]
impl crate::Writable for RFIFOCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIFOCNT to value 0"]
impl crate::Resettable for RFIFOCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
