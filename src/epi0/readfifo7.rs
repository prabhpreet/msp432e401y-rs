#[doc = "Register `READFIFO7` reader"]
pub struct R(crate::R<READFIFO7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READFIFO7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READFIFO7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READFIFO7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READFIFO7` writer"]
pub struct W(crate::W<READFIFO7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READFIFO7_SPEC>;
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
impl From<crate::W<READFIFO7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READFIFO7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_READFIFO7_DATA` reader - Reads Data"]
pub type EPI_READFIFO7_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_READFIFO7_DATA` writer - Reads Data"]
pub type EPI_READFIFO7_DATA_W<'a> = crate::FieldWriter<'a, u32, READFIFO7_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo7_data(&self) -> EPI_READFIFO7_DATA_R {
        EPI_READFIFO7_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo7_data(&mut self) -> EPI_READFIFO7_DATA_W {
        EPI_READFIFO7_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO Alias 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo7](index.html) module"]
pub struct READFIFO7_SPEC;
impl crate::RegisterSpec for READFIFO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readfifo7::R](R) reader structure"]
impl crate::Readable for READFIFO7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readfifo7::W](W) writer structure"]
impl crate::Writable for READFIFO7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READFIFO7 to value 0"]
impl crate::Resettable for READFIFO7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
