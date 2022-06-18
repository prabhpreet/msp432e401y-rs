#[doc = "Register `REVISION` reader"]
pub struct R(crate::R<REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REVISION` writer"]
pub struct W(crate::W<REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REVISION_SPEC>;
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
impl From<crate::W<REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_REVISION` reader - Revision number"]
pub type AES_REVISION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_REVISION` writer - Revision number"]
pub type AES_REVISION_W<'a> = crate::FieldWriter<'a, u32, REVISION_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Revision number"]
    #[inline(always)]
    pub fn aes_revision(&self) -> AES_REVISION_R {
        AES_REVISION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Revision number"]
    #[inline(always)]
    pub fn aes_revision(&mut self) -> AES_REVISION_W {
        AES_REVISION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES IP Revision Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [revision](index.html) module"]
pub struct REVISION_SPEC;
impl crate::RegisterSpec for REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [revision::R](R) reader structure"]
impl crate::Readable for REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [revision::W](W) writer structure"]
impl crate::Writable for REVISION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for REVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
