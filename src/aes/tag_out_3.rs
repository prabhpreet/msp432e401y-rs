#[doc = "Register `TAG_OUT_3` reader"]
pub struct R(crate::R<TAG_OUT_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAG_OUT_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAG_OUT_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAG_OUT_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAG_OUT_3` writer"]
pub struct W(crate::W<TAG_OUT_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAG_OUT_3_SPEC>;
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
impl From<crate::W<TAG_OUT_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAG_OUT_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_TAG_OUT_3_HASH` reader - Hash Result"]
pub type AES_TAG_OUT_3_HASH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_TAG_OUT_3_HASH` writer - Hash Result"]
pub type AES_TAG_OUT_3_HASH_W<'a> = crate::FieldWriter<'a, u32, TAG_OUT_3_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hash Result"]
    #[inline(always)]
    pub fn aes_tag_out_3_hash(&self) -> AES_TAG_OUT_3_HASH_R {
        AES_TAG_OUT_3_HASH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Result"]
    #[inline(always)]
    pub fn aes_tag_out_3_hash(&mut self) -> AES_TAG_OUT_3_HASH_W {
        AES_TAG_OUT_3_HASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Hash Tag Out 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tag_out_3](index.html) module"]
pub struct TAG_OUT_3_SPEC;
impl crate::RegisterSpec for TAG_OUT_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tag_out_3::R](R) reader structure"]
impl crate::Readable for TAG_OUT_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tag_out_3::W](W) writer structure"]
impl crate::Writable for TAG_OUT_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAG_OUT_3 to value 0"]
impl crate::Resettable for TAG_OUT_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
