#[doc = "Register `DATA_H` reader"]
pub struct R(crate::R<DATA_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_H` writer"]
pub struct W(crate::W<DATA_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_H_SPEC>;
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
impl From<crate::W<DATA_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_DATA_H` reader - Data for encryption/decryption, MSW"]
pub type DES_DATA_H_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DES_DATA_H` writer - Data for encryption/decryption, MSW"]
pub type DES_DATA_H_W<'a> = crate::FieldWriter<'a, u32, DATA_H_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data for encryption/decryption, MSW"]
    #[inline(always)]
    pub fn des_data_h(&self) -> DES_DATA_H_R {
        DES_DATA_H_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data for encryption/decryption, MSW"]
    #[inline(always)]
    pub fn des_data_h(&mut self) -> DES_DATA_H_W {
        DES_DATA_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES MSW Data RW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_h](index.html) module"]
pub struct DATA_H_SPEC;
impl crate::RegisterSpec for DATA_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_h::R](R) reader structure"]
impl crate::Readable for DATA_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_h::W](W) writer structure"]
impl crate::Writable for DATA_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_H to value 0"]
impl crate::Resettable for DATA_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
