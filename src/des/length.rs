#[doc = "Register `LENGTH` reader"]
pub struct R(crate::R<LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LENGTH` writer"]
pub struct W(crate::W<LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LENGTH_SPEC>;
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
impl From<crate::W<LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_LENGTH` reader - Cryptographic data length in bytes for all modes"]
pub type DES_LENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DES_LENGTH` writer - Cryptographic data length in bytes for all modes"]
pub type DES_LENGTH_W<'a> = crate::FieldWriter<'a, u32, LENGTH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Cryptographic data length in bytes for all modes"]
    #[inline(always)]
    pub fn des_length(&self) -> DES_LENGTH_R {
        DES_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic data length in bytes for all modes"]
    #[inline(always)]
    pub fn des_length(&mut self) -> DES_LENGTH_W {
        DES_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES Cryptographic Data Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [length](index.html) module"]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [length::R](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [length::W](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
