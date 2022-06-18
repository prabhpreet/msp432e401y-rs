#[doc = "Register `RPSTD0` reader"]
pub struct R(crate::R<RPSTD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPSTD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPSTD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPSTD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPSTD0` writer"]
pub struct W(crate::W<RPSTD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPSTD0_SPEC>;
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
impl From<crate::W<RPSTD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPSTD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_RPSTD0_POSTCNT` reader - Post Count"]
pub type EPI_RPSTD0_POSTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_RPSTD0_POSTCNT` writer - Post Count"]
pub type EPI_RPSTD0_POSTCNT_W<'a> = crate::FieldWriter<'a, u32, RPSTD0_SPEC, u16, u16, 13, 0>;
impl R {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn epi_rpstd0_postcnt(&self) -> EPI_RPSTD0_POSTCNT_R {
        EPI_RPSTD0_POSTCNT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Post Count"]
    #[inline(always)]
    pub fn epi_rpstd0_postcnt(&mut self) -> EPI_RPSTD0_POSTCNT_W {
        EPI_RPSTD0_POSTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Non-Blocking Read Data 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpstd0](index.html) module"]
pub struct RPSTD0_SPEC;
impl crate::RegisterSpec for RPSTD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpstd0::R](R) reader structure"]
impl crate::Readable for RPSTD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpstd0::W](W) writer structure"]
impl crate::Writable for RPSTD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPSTD0 to value 0"]
impl crate::Resettable for RPSTD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
