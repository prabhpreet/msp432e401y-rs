#[doc = "Register `HASHTBLL` reader"]
pub struct R(crate::R<HASHTBLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHTBLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHTBLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHTBLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHTBLL` writer"]
pub struct W(crate::W<HASHTBLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHTBLL_SPEC>;
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
impl From<crate::W<HASHTBLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHTBLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HASHTBLL_HTL` reader - Hash Table Low"]
pub type EMAC_HASHTBLL_HTL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_HASHTBLL_HTL` writer - Hash Table Low"]
pub type EMAC_HASHTBLL_HTL_W<'a> = crate::FieldWriter<'a, u32, HASHTBLL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn emac_hashtbll_htl(&self) -> EMAC_HASHTBLL_HTL_R {
        EMAC_HASHTBLL_HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn emac_hashtbll_htl(&mut self) -> EMAC_HASHTBLL_HTL_W {
        EMAC_HASHTBLL_HTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Hash Table Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashtbll](index.html) module"]
pub struct HASHTBLL_SPEC;
impl crate::RegisterSpec for HASHTBLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashtbll::R](R) reader structure"]
impl crate::Readable for HASHTBLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashtbll::W](W) writer structure"]
impl crate::Writable for HASHTBLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHTBLL to value 0"]
impl crate::Resettable for HASHTBLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
