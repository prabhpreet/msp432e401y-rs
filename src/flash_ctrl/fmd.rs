#[doc = "Register `FMD` reader"]
pub struct R(crate::R<FMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMD` writer"]
pub struct W(crate::W<FMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMD_SPEC>;
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
impl From<crate::W<FMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMD_DATA` reader - Data Value"]
pub type FLASH_FMD_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMD_DATA` writer - Data Value"]
pub type FLASH_FMD_DATA_W<'a> = crate::FieldWriter<'a, u32, FMD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn flash_fmd_data(&self) -> FLASH_FMD_DATA_R {
        FLASH_FMD_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn flash_fmd_data(&mut self) -> FLASH_FMD_DATA_W {
        FLASH_FMD_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmd](index.html) module"]
pub struct FMD_SPEC;
impl crate::RegisterSpec for FMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmd::R](R) reader structure"]
impl crate::Readable for FMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmd::W](W) writer structure"]
impl crate::Writable for FMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMD to value 0"]
impl crate::Resettable for FMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
