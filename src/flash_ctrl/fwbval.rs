#[doc = "Register `FWBVAL` reader"]
pub struct R(crate::R<FWBVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWBVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWBVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWBVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWBVAL` writer"]
pub struct W(crate::W<FWBVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWBVAL_SPEC>;
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
impl From<crate::W<FWBVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWBVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FWBVAL_FWB` reader - Flash Memory Write Buffer"]
pub type FLASH_FWBVAL_FWB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_FWBVAL_FWB` writer - Flash Memory Write Buffer"]
pub type FLASH_FWBVAL_FWB_W<'a> = crate::FieldWriter<'a, u32, FWBVAL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Flash Memory Write Buffer"]
    #[inline(always)]
    pub fn flash_fwbval_fwb(&self) -> FLASH_FWBVAL_FWB_R {
        FLASH_FWBVAL_FWB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Memory Write Buffer"]
    #[inline(always)]
    pub fn flash_fwbval_fwb(&mut self) -> FLASH_FWBVAL_FWB_W {
        FLASH_FWBVAL_FWB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Buffer Valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbval](index.html) module"]
pub struct FWBVAL_SPEC;
impl crate::RegisterSpec for FWBVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwbval::R](R) reader structure"]
impl crate::Readable for FWBVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwbval::W](W) writer structure"]
impl crate::Writable for FWBVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWBVAL to value 0"]
impl crate::Resettable for FWBVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
