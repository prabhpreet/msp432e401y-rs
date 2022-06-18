#[doc = "Register `CRCSEED` reader"]
pub struct R(crate::R<CRCSEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCSEED` writer"]
pub struct W(crate::W<CRCSEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCSEED_SPEC>;
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
impl From<crate::W<CRCSEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCSEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_SEED_SEED` reader - SEED/Context Value"]
pub type CRC_SEED_SEED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC_SEED_SEED` writer - SEED/Context Value"]
pub type CRC_SEED_SEED_W<'a> = crate::FieldWriter<'a, u32, CRCSEED_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - SEED/Context Value"]
    #[inline(always)]
    pub fn crc_seed_seed(&self) -> CRC_SEED_SEED_R {
        CRC_SEED_SEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED/Context Value"]
    #[inline(always)]
    pub fn crc_seed_seed(&mut self) -> CRC_SEED_SEED_W {
        CRC_SEED_SEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC SEED/Context\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcseed](index.html) module"]
pub struct CRCSEED_SPEC;
impl crate::RegisterSpec for CRCSEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcseed::R](R) reader structure"]
impl crate::Readable for CRCSEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcseed::W](W) writer structure"]
impl crate::Writable for CRCSEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCSEED to value 0"]
impl crate::Resettable for CRCSEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
