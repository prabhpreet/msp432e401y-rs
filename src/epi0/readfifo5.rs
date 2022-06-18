#[doc = "Register `READFIFO5` reader"]
pub struct R(crate::R<READFIFO5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READFIFO5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READFIFO5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READFIFO5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READFIFO5` writer"]
pub struct W(crate::W<READFIFO5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READFIFO5_SPEC>;
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
impl From<crate::W<READFIFO5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READFIFO5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_READFIFO5_DATA` reader - Reads Data"]
pub type EPI_READFIFO5_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_READFIFO5_DATA` writer - Reads Data"]
pub type EPI_READFIFO5_DATA_W<'a> = crate::FieldWriter<'a, u32, READFIFO5_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo5_data(&self) -> EPI_READFIFO5_DATA_R {
        EPI_READFIFO5_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo5_data(&mut self) -> EPI_READFIFO5_DATA_W {
        EPI_READFIFO5_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO Alias 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo5](index.html) module"]
pub struct READFIFO5_SPEC;
impl crate::RegisterSpec for READFIFO5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readfifo5::R](R) reader structure"]
impl crate::Readable for READFIFO5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readfifo5::W](W) writer structure"]
impl crate::Writable for READFIFO5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READFIFO5 to value 0"]
impl crate::Resettable for READFIFO5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
