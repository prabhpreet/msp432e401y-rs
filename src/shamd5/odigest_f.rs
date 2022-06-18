#[doc = "Register `ODIGEST_F` reader"]
pub struct R(crate::R<ODIGEST_F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODIGEST_F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODIGEST_F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODIGEST_F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODIGEST_F` writer"]
pub struct W(crate::W<ODIGEST_F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODIGEST_F_SPEC>;
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
impl From<crate::W<ODIGEST_F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODIGEST_F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_ODIGEST_F_DATA` reader - Digest/Key Data"]
pub type SHAMD5_ODIGEST_F_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHAMD5_ODIGEST_F_DATA` writer - Digest/Key Data"]
pub type SHAMD5_ODIGEST_F_DATA_W<'a> = crate::FieldWriter<'a, u32, ODIGEST_F_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_odigest_f_data(&self) -> SHAMD5_ODIGEST_F_DATA_R {
        SHAMD5_ODIGEST_F_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_odigest_f_data(&mut self) -> SHAMD5_ODIGEST_F_DATA_W {
        SHAMD5_ODIGEST_F_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Outer Digest F\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odigest_f](index.html) module"]
pub struct ODIGEST_F_SPEC;
impl crate::RegisterSpec for ODIGEST_F_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odigest_f::R](R) reader structure"]
impl crate::Readable for ODIGEST_F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odigest_f::W](W) writer structure"]
impl crate::Writable for ODIGEST_F_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODIGEST_F to value 0"]
impl crate::Resettable for ODIGEST_F_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
