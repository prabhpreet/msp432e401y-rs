#[doc = "Register `READFIFO3` reader"]
pub struct R(crate::R<READFIFO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READFIFO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READFIFO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READFIFO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READFIFO3` writer"]
pub struct W(crate::W<READFIFO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READFIFO3_SPEC>;
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
impl From<crate::W<READFIFO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READFIFO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_READFIFO3_DATA` reader - Reads Data"]
pub type EPI_READFIFO3_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_READFIFO3_DATA` writer - Reads Data"]
pub type EPI_READFIFO3_DATA_W<'a> = crate::FieldWriter<'a, u32, READFIFO3_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo3_data(&self) -> EPI_READFIFO3_DATA_R {
        EPI_READFIFO3_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo3_data(&mut self) -> EPI_READFIFO3_DATA_W {
        EPI_READFIFO3_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO Alias 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo3](index.html) module"]
pub struct READFIFO3_SPEC;
impl crate::RegisterSpec for READFIFO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readfifo3::R](R) reader structure"]
impl crate::Readable for READFIFO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readfifo3::W](W) writer structure"]
impl crate::Writable for READFIFO3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READFIFO3 to value 0"]
impl crate::Resettable for READFIFO3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
