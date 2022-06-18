#[doc = "Register `DATA_13_IN` reader"]
pub struct R(crate::R<DATA_13_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_13_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_13_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_13_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_13_IN` writer"]
pub struct W(crate::W<DATA_13_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_13_IN_SPEC>;
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
impl From<crate::W<DATA_13_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_13_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHAMD5_DATA_13_IN_DATA` reader - Digest/Key Data"]
pub type SHAMD5_DATA_13_IN_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHAMD5_DATA_13_IN_DATA` writer - Digest/Key Data"]
pub type SHAMD5_DATA_13_IN_DATA_W<'a> =
    crate::FieldWriter<'a, u32, DATA_13_IN_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_data_13_in_data(&self) -> SHAMD5_DATA_13_IN_DATA_R {
        SHAMD5_DATA_13_IN_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digest/Key Data"]
    #[inline(always)]
    pub fn shamd5_data_13_in_data(&mut self) -> SHAMD5_DATA_13_IN_DATA_W {
        SHAMD5_DATA_13_IN_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Data 13 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_13_in](index.html) module"]
pub struct DATA_13_IN_SPEC;
impl crate::RegisterSpec for DATA_13_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_13_in::R](R) reader structure"]
impl crate::Readable for DATA_13_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_13_in::W](W) writer structure"]
impl crate::Writable for DATA_13_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_13_IN to value 0"]
impl crate::Resettable for DATA_13_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
