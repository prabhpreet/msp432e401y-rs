#[doc = "Register `FMPRE12` reader"]
pub struct R(crate::R<FMPRE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPRE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPRE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPRE12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPRE12` writer"]
pub struct W(crate::W<FMPRE12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPRE12_SPEC>;
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
impl From<crate::W<FMPRE12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPRE12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMPRE12_READ_ENABLE` reader - Flash Read Enable"]
pub type FLASH_FMPRE12_READ_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMPRE12_READ_ENABLE` writer - Flash Read Enable"]
pub type FLASH_FMPRE12_READ_ENABLE_W<'a> =
    crate::FieldWriter<'a, u32, FMPRE12_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_fmpre12_read_enable(&self) -> FLASH_FMPRE12_READ_ENABLE_R {
        FLASH_FMPRE12_READ_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_fmpre12_read_enable(&mut self) -> FLASH_FMPRE12_READ_ENABLE_W {
        FLASH_FMPRE12_READ_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Protection Read Enable 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre12](index.html) module"]
pub struct FMPRE12_SPEC;
impl crate::RegisterSpec for FMPRE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmpre12::R](R) reader structure"]
impl crate::Readable for FMPRE12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmpre12::W](W) writer structure"]
impl crate::Writable for FMPRE12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPRE12 to value 0"]
impl crate::Resettable for FMPRE12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
