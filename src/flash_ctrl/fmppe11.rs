#[doc = "Register `FMPPE11` reader"]
pub struct R(crate::R<FMPPE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPPE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPPE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPPE11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPPE11` writer"]
pub struct W(crate::W<FMPPE11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPPE11_SPEC>;
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
impl From<crate::W<FMPPE11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPPE11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FMPPE11_PROG_ENABLE` reader - Flash Programming Enable"]
pub type FLASH_FMPPE11_PROG_ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FMPPE11_PROG_ENABLE` writer - Flash Programming Enable"]
pub type FLASH_FMPPE11_PROG_ENABLE_W<'a> =
    crate::FieldWriter<'a, u32, FMPPE11_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe11_prog_enable(&self) -> FLASH_FMPPE11_PROG_ENABLE_R {
        FLASH_FMPPE11_PROG_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Programming Enable"]
    #[inline(always)]
    pub fn flash_fmppe11_prog_enable(&mut self) -> FLASH_FMPPE11_PROG_ENABLE_W {
        FLASH_FMPPE11_PROG_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Memory Protection Program Enable 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe11](index.html) module"]
pub struct FMPPE11_SPEC;
impl crate::RegisterSpec for FMPPE11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmppe11::R](R) reader structure"]
impl crate::Readable for FMPPE11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmppe11::W](W) writer structure"]
impl crate::Writable for FMPPE11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPPE11 to value 0"]
impl crate::Resettable for FMPPE11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
