#[doc = "Register `IV_IN_2` reader"]
pub struct R(crate::R<IV_IN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV_IN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV_IN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV_IN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV_IN_2` writer"]
pub struct W(crate::W<IV_IN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV_IN_2_SPEC>;
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
impl From<crate::W<IV_IN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV_IN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_IV_IN_2_DATA` reader - Initialization Vector Input"]
pub type AES_IV_IN_2_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_IV_IN_2_DATA` writer - Initialization Vector Input"]
pub type AES_IV_IN_2_DATA_W<'a> = crate::FieldWriter<'a, u32, IV_IN_2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Input"]
    #[inline(always)]
    pub fn aes_iv_in_2_data(&self) -> AES_IV_IN_2_DATA_R {
        AES_IV_IN_2_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Input"]
    #[inline(always)]
    pub fn aes_iv_in_2_data(&mut self) -> AES_IV_IN_2_DATA_W {
        AES_IV_IN_2_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Initialization Vector Input 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_in_2](index.html) module"]
pub struct IV_IN_2_SPEC;
impl crate::RegisterSpec for IV_IN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv_in_2::R](R) reader structure"]
impl crate::Readable for IV_IN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv_in_2::W](W) writer structure"]
impl crate::Writable for IV_IN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV_IN_2 to value 0"]
impl crate::Resettable for IV_IN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
