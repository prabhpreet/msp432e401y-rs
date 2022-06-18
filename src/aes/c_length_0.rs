#[doc = "Register `C_LENGTH_0` reader"]
pub struct R(crate::R<C_LENGTH_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C_LENGTH_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C_LENGTH_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C_LENGTH_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C_LENGTH_0` writer"]
pub struct W(crate::W<C_LENGTH_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C_LENGTH_0_SPEC>;
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
impl From<crate::W<C_LENGTH_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C_LENGTH_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_C_LENGTH_0_LENGTH` reader - Data Length"]
pub type AES_C_LENGTH_0_LENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_C_LENGTH_0_LENGTH` writer - Data Length"]
pub type AES_C_LENGTH_0_LENGTH_W<'a> =
    crate::FieldWriter<'a, u32, C_LENGTH_0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data Length"]
    #[inline(always)]
    pub fn aes_c_length_0_length(&self) -> AES_C_LENGTH_0_LENGTH_R {
        AES_C_LENGTH_0_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Length"]
    #[inline(always)]
    pub fn aes_c_length_0_length(&mut self) -> AES_C_LENGTH_0_LENGTH_W {
        AES_C_LENGTH_0_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Crypto Data Length 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c_length_0](index.html) module"]
pub struct C_LENGTH_0_SPEC;
impl crate::RegisterSpec for C_LENGTH_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c_length_0::R](R) reader structure"]
impl crate::Readable for C_LENGTH_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c_length_0::W](W) writer structure"]
impl crate::Writable for C_LENGTH_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C_LENGTH_0 to value 0"]
impl crate::Resettable for C_LENGTH_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
