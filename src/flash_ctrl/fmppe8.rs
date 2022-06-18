#[doc = "Register `FMPPE8` reader"]
pub struct R(crate::R<FMPPE8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPPE8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPPE8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPPE8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPPE8` writer"]
pub struct W(crate::W<FMPPE8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPPE8_SPEC>;
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
impl From<crate::W<FMPPE8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPPE8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMPPE8_PROG_ENABLE` reader - Flash Programming Enable"]
pub type FLASH_FMPPE8_PROG_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMPPE8_PROG_ENABLE` writer - Flash Programming Enable"]
pub type FLASH_FMPPE8_PROG_ENABLE_W<'a> = crate::FieldWriter<'a, u32, FMPPE8_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe8_prog_enable(&self) -> FLASH_FMPPE8_PROG_ENABLE_R {
        FLASH_FMPPE8_PROG_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe8_prog_enable(&mut self) -> FLASH_FMPPE8_PROG_ENABLE_W {
        FLASH_FMPPE8_PROG_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Protection Program Enable 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe8](index.html) module"]
pub struct FMPPE8_SPEC;
impl crate::RegisterSpec for FMPPE8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmppe8::R](R) reader structure"]
impl crate::Readable for FMPPE8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmppe8::W](W) writer structure"]
impl crate::Writable for FMPPE8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPPE8 to value 0"]
impl crate::Resettable for FMPPE8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
