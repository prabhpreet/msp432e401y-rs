#[doc = "Register `FMPRE11` reader"]
pub struct R(crate::R<FMPRE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPRE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPRE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPRE11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPRE11` writer"]
pub struct W(crate::W<FMPRE11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPRE11_SPEC>;
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
impl From<crate::W<FMPRE11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPRE11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMPRE11_READ_ENABLE` reader - Flash Read Enable"]
pub type FLASH_FMPRE11_READ_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMPRE11_READ_ENABLE` writer - Flash Read Enable"]
pub type FLASH_FMPRE11_READ_ENABLE_W<'a> =
    crate::FieldWriter<'a, u32, FMPRE11_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_fmpre11_read_enable(&self) -> FLASH_FMPRE11_READ_ENABLE_R {
        FLASH_FMPRE11_READ_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Read Enable"]
    #[inline(always)]
    pub fn flash_fmpre11_read_enable(&mut self) -> FLASH_FMPRE11_READ_ENABLE_W {
        FLASH_FMPRE11_READ_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Protection Read Enable 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre11](index.html) module"]
pub struct FMPRE11_SPEC;
impl crate::RegisterSpec for FMPRE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmpre11::R](R) reader structure"]
impl crate::Readable for FMPRE11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmpre11::W](W) writer structure"]
impl crate::Writable for FMPRE11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPRE11 to value 0"]
impl crate::Resettable for FMPRE11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}