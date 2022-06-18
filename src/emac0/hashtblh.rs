#[doc = "Register `HASHTBLH` reader"]
pub struct R(crate::R<HASHTBLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHTBLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHTBLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHTBLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHTBLH` writer"]
pub struct W(crate::W<HASHTBLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHTBLH_SPEC>;
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
impl From<crate::W<HASHTBLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHTBLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HASHTBLH_HTH` reader - Hash Table High"]
pub type EMAC_HASHTBLH_HTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HASHTBLH_HTH` writer - Hash Table High"]
pub type EMAC_HASHTBLH_HTH_W<'a> = crate::FieldWriter<'a, u32, HASHTBLH_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn emac_hashtblh_hth(&self) -> EMAC_HASHTBLH_HTH_R {
        EMAC_HASHTBLH_HTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table High"]
    #[inline(always)]
    pub fn emac_hashtblh_hth(&mut self) -> EMAC_HASHTBLH_HTH_W {
        EMAC_HASHTBLH_HTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Hash Table High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashtblh](index.html) module"]
pub struct HASHTBLH_SPEC;
impl crate::RegisterSpec for HASHTBLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashtblh::R](R) reader structure"]
impl crate::Readable for HASHTBLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashtblh::W](W) writer structure"]
impl crate::Writable for HASHTBLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHTBLH to value 0"]
impl crate::Resettable for HASHTBLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
