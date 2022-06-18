#[doc = "Register `KEY1_6` reader"]
pub struct R(crate::R<KEY1_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY1_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY1_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY1_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY1_6` writer"]
pub struct W(crate::W<KEY1_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY1_6_SPEC>;
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
impl From<crate::W<KEY1_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY1_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEY1_6_KEY` reader - Key Data"]
pub type AES_KEY1_6_KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_KEY1_6_KEY` writer - Key Data"]
pub type AES_KEY1_6_KEY_W<'a> = crate::FieldWriter<'a, u32, KEY1_6_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key Data"]
    #[inline(always)]
    pub fn aes_key1_6_key(&self) -> AES_KEY1_6_KEY_R {
        AES_KEY1_6_KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Data"]
    #[inline(always)]
    pub fn aes_key1_6_key(&mut self) -> AES_KEY1_6_KEY_W {
        AES_KEY1_6_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Key 1_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key1_6](index.html) module"]
pub struct KEY1_6_SPEC;
impl crate::RegisterSpec for KEY1_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key1_6::R](R) reader structure"]
impl crate::Readable for KEY1_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key1_6::W](W) writer structure"]
impl crate::Writable for KEY1_6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY1_6 to value 0"]
impl crate::Resettable for KEY1_6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
