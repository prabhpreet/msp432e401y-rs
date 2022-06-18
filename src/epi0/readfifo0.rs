#[doc = "Register `READFIFO0` reader"]
pub struct R(crate::R<READFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READFIFO0` writer"]
pub struct W(crate::W<READFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READFIFO0_SPEC>;
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
impl From<crate::W<READFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_READFIFO0_DATA` reader - Reads Data"]
pub type EPI_READFIFO0_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_READFIFO0_DATA` writer - Reads Data"]
pub type EPI_READFIFO0_DATA_W<'a> = crate::FieldWriter<'a, u32, READFIFO0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo0_data(&self) -> EPI_READFIFO0_DATA_R {
        EPI_READFIFO0_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reads Data"]
    #[inline(always)]
    pub fn epi_readfifo0_data(&mut self) -> EPI_READFIFO0_DATA_W {
        EPI_READFIFO0_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Read FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readfifo0](index.html) module"]
pub struct READFIFO0_SPEC;
impl crate::RegisterSpec for READFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readfifo0::R](R) reader structure"]
impl crate::Readable for READFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readfifo0::W](W) writer structure"]
impl crate::Writable for READFIFO0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets READFIFO0 to value 0"]
impl crate::Resettable for READFIFO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
