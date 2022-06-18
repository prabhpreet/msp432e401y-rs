#[doc = "Register `IDIGEST_H` reader"]
pub struct R(crate::R<IDIGEST_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDIGEST_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDIGEST_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDIGEST_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDIGEST_H` writer"]
pub struct W(crate::W<IDIGEST_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDIGEST_H_SPEC>;
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
impl From<crate::W<IDIGEST_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDIGEST_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_IDIGEST_H_DATA` reader - Digest/Key Data"]
pub type SHAMD5_IDIGEST_H_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHAMD5_IDIGEST_H_DATA` writer - Digest/Key Data"]
pub type SHAMD5_IDIGEST_H_DATA_W<'a> = crate::FieldWriter<'a, u32, IDIGEST_H_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_idigest_h_data(&self) -> SHAMD5_IDIGEST_H_DATA_R {
        SHAMD5_IDIGEST_H_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_idigest_h_data(&mut self) -> SHAMD5_IDIGEST_H_DATA_W {
        SHAMD5_IDIGEST_H_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Inner Digest H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idigest_h](index.html) module"]
pub struct IDIGEST_H_SPEC;
impl crate::RegisterSpec for IDIGEST_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idigest_h::R](R) reader structure"]
impl crate::Readable for IDIGEST_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idigest_h::W](W) writer structure"]
impl crate::Writable for IDIGEST_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDIGEST_H to value 0"]
impl crate::Resettable for IDIGEST_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
