#[doc = "Register `DIGEST_COUNT` reader"]
pub struct R(crate::R<DIGEST_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIGEST_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIGEST_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIGEST_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIGEST_COUNT` writer"]
pub struct W(crate::W<DIGEST_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIGEST_COUNT_SPEC>;
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
impl From<crate::W<DIGEST_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIGEST_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_DIGEST_COUNT` reader - Digest Count"]
pub type SHAMD5_DIGEST_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHAMD5_DIGEST_COUNT` writer - Digest Count"]
pub type SHAMD5_DIGEST_COUNT_W<'a> =
    crate::FieldWriter<'a, u32, DIGEST_COUNT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Digest Count"]
    #[inline(always)]
    pub fn shamd5_digest_count(&self) -> SHAMD5_DIGEST_COUNT_R {
        SHAMD5_DIGEST_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digest Count"]
    #[inline(always)]
    pub fn shamd5_digest_count(&mut self) -> SHAMD5_DIGEST_COUNT_W {
        SHAMD5_DIGEST_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Digest Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digest_count](index.html) module"]
pub struct DIGEST_COUNT_SPEC;
impl crate::RegisterSpec for DIGEST_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [digest_count::R](R) reader structure"]
impl crate::Readable for DIGEST_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [digest_count::W](W) writer structure"]
impl crate::Writable for DIGEST_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIGEST_COUNT to value 0"]
impl crate::Resettable for DIGEST_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
