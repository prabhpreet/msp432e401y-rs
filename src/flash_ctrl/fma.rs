#[doc = "Register `FMA` reader"]
pub struct R(crate::R<FMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMA` writer"]
pub struct W(crate::W<FMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMA_SPEC>;
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
impl From<crate::W<FMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMA_OFFSET` reader - Address Offset"]
pub type FLASH_FMA_OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMA_OFFSET` writer - Address Offset"]
pub type FLASH_FMA_OFFSET_W<'a> = crate::FieldWriter<'a, u32, FMA_SPEC, u32, u32, 20, 0>;
impl R {
    #[doc = "Bits 0:19 - Address Offset"]
    #[inline(always)]
    pub fn flash_fma_offset(&self) -> FLASH_FMA_OFFSET_R {
        FLASH_FMA_OFFSET_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Address Offset"]
    #[inline(always)]
    pub fn flash_fma_offset(&mut self) -> FLASH_FMA_OFFSET_W {
        FLASH_FMA_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fma](index.html) module"]
pub struct FMA_SPEC;
impl crate::RegisterSpec for FMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fma::R](R) reader structure"]
impl crate::Readable for FMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fma::W](W) writer structure"]
impl crate::Writable for FMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMA to value 0"]
impl crate::Resettable for FMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
