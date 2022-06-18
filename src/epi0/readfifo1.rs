#[doc = "Register `READFIFO1` reader"]
pub struct R(crate::R<READFIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READFIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READFIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READFIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READFIFO1` writer"]
pub struct W(crate::W<READFIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READFIFO1_SPEC>;
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
impl From<crate::W<READFIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READFIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_READFIFO1_DATA` reader - Reads Data"]
pub type EPI_READFIFO1_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_READFIFO1_DATA` writer - Reads Data"]
pub type EPI_READFIFO1_DATA_W<'a> = crate::FieldWriter<'a, u32, READFIFO1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo1_data(&self) -> EPI_READFIFO1_DATA_R {
        EPI_READFIFO1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo1_data(&mut self) -> EPI_READFIFO1_DATA_W {
        EPI_READFIFO1_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO Alias 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo1](index.html) module"]
pub struct READFIFO1_SPEC;
impl crate::RegisterSpec for READFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readfifo1::R](R) reader structure"]
impl crate::Readable for READFIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readfifo1::W](W) writer structure"]
impl crate::Writable for READFIFO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READFIFO1 to value 0"]
impl crate::Resettable for READFIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
