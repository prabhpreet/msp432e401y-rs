#[doc = "Register `FMPPE14` reader"]
pub struct R(crate::R<FMPPE14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPPE14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPPE14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPPE14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPPE14` writer"]
pub struct W(crate::W<FMPPE14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPPE14_SPEC>;
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
impl From<crate::W<FMPPE14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPPE14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMPPE14_PROG_ENABLE` reader - Flash Programming Enable"]
pub type FLASH_FMPPE14_PROG_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMPPE14_PROG_ENABLE` writer - Flash Programming Enable"]
pub type FLASH_FMPPE14_PROG_ENABLE_W<'a> =
    crate::FieldWriter<'a, u32, FMPPE14_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe14_prog_enable(&self) -> FLASH_FMPPE14_PROG_ENABLE_R {
        FLASH_FMPPE14_PROG_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe14_prog_enable(&mut self) -> FLASH_FMPPE14_PROG_ENABLE_W {
        FLASH_FMPPE14_PROG_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Protection Program Enable 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe14](index.html) module"]
pub struct FMPPE14_SPEC;
impl crate::RegisterSpec for FMPPE14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmppe14::R](R) reader structure"]
impl crate::Readable for FMPPE14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmppe14::W](W) writer structure"]
impl crate::Writable for FMPPE14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPPE14 to value 0"]
impl crate::Resettable for FMPPE14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
