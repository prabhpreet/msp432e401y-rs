#[doc = "Register `DATA_IN_1` reader"]
pub struct R(crate::R<DATA_IN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_IN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_IN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_IN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_IN_1` writer"]
pub struct W(crate::W<DATA_IN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_IN_1_SPEC>;
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
impl From<crate::W<DATA_IN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_IN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_DATA_IN_1_DATA` reader - Secure Data RW Plaintext/Ciphertext"]
pub type AES_DATA_IN_1_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_DATA_IN_1_DATA` writer - Secure Data RW Plaintext/Ciphertext"]
pub type AES_DATA_IN_1_DATA_W<'a> = crate::FieldWriter<'a, u32, DATA_IN_1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Secure Data RW Plaintext/Ciphertext"]
    #[inline(always)]
    pub fn aes_data_in_1_data(&self) -> AES_DATA_IN_1_DATA_R {
        AES_DATA_IN_1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Data RW Plaintext/Ciphertext"]
    #[inline(always)]
    pub fn aes_data_in_1_data(&mut self) -> AES_DATA_IN_1_DATA_W {
        AES_DATA_IN_1_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Data RW Plaintext/Ciphertext 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in_1](index.html) module"]
pub struct DATA_IN_1_SPEC;
impl crate::RegisterSpec for DATA_IN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_in_1::R](R) reader structure"]
impl crate::Readable for DATA_IN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_in_1::W](W) writer structure"]
impl crate::Writable for DATA_IN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_IN_1 to value 0"]
impl crate::Resettable for DATA_IN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
