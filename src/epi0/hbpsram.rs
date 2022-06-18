#[doc = "Register `HBPSRAM` reader"]
pub struct R(crate::R<HBPSRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBPSRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBPSRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBPSRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBPSRAM` writer"]
pub struct W(crate::W<HBPSRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBPSRAM_SPEC>;
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
impl From<crate::W<HBPSRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBPSRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_HBPSRAM_CR` reader - PSRAM Config Register"]
pub type EPI_HBPSRAM_CR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPI_HBPSRAM_CR` writer - PSRAM Config Register"]
pub type EPI_HBPSRAM_CR_W<'a> = crate::FieldWriter<'a, u32, HBPSRAM_SPEC, u32, u32, 21, 0>;
impl R {
    #[doc = "Bits 0:20 - PSRAM Config Register"]
    #[inline(always)]
    pub fn epi_hbpsram_cr(&self) -> EPI_HBPSRAM_CR_R {
        EPI_HBPSRAM_CR_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - PSRAM Config Register"]
    #[inline(always)]
    pub fn epi_hbpsram_cr(&mut self) -> EPI_HBPSRAM_CR_W {
        EPI_HBPSRAM_CR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus PSRAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbpsram](index.html) module"]
pub struct HBPSRAM_SPEC;
impl crate::RegisterSpec for HBPSRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbpsram::R](R) reader structure"]
impl crate::Readable for HBPSRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbpsram::W](W) writer structure"]
impl crate::Writable for HBPSRAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBPSRAM to value 0"]
impl crate::Resettable for HBPSRAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
