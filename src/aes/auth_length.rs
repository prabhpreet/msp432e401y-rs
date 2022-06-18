#[doc = "Register `AUTH_LENGTH` reader"]
pub struct R(crate::R<AUTH_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTH_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTH_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTH_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTH_LENGTH` writer"]
pub struct W(crate::W<AUTH_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTH_LENGTH_SPEC>;
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
impl From<crate::W<AUTH_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTH_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_AUTH_LENGTH_AUTH` reader - Authentication Data Length"]
pub type AES_AUTH_LENGTH_AUTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_AUTH_LENGTH_AUTH` writer - Authentication Data Length"]
pub type AES_AUTH_LENGTH_AUTH_W<'a> =
    crate::FieldWriter<'a, u32, AUTH_LENGTH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Authentication Data Length"]
    #[inline(always)]
    pub fn aes_auth_length_auth(&self) -> AES_AUTH_LENGTH_AUTH_R {
        AES_AUTH_LENGTH_AUTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Authentication Data Length"]
    #[inline(always)]
    pub fn aes_auth_length_auth(&mut self) -> AES_AUTH_LENGTH_AUTH_W {
        AES_AUTH_LENGTH_AUTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Authentication Data Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auth_length](index.html) module"]
pub struct AUTH_LENGTH_SPEC;
impl crate::RegisterSpec for AUTH_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auth_length::R](R) reader structure"]
impl crate::Readable for AUTH_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auth_length::W](W) writer structure"]
impl crate::Writable for AUTH_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTH_LENGTH to value 0"]
impl crate::Resettable for AUTH_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
