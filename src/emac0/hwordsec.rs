#[doc = "Register `HWORDSEC` reader"]
pub struct R(crate::R<HWORDSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWORDSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWORDSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWORDSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWORDSEC` writer"]
pub struct W(crate::W<HWORDSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWORDSEC_SPEC>;
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
impl From<crate::W<HWORDSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWORDSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_HWORDSEC_TSHWR` reader - Target Timestamp Higher Word Register"]
pub type EMAC_HWORDSEC_TSHWR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_HWORDSEC_TSHWR` writer - Target Timestamp Higher Word Register"]
pub type EMAC_HWORDSEC_TSHWR_W<'a> = crate::FieldWriter<'a, u32, HWORDSEC_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Target Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn emac_hwordsec_tshwr(&self) -> EMAC_HWORDSEC_TSHWR_R {
        EMAC_HWORDSEC_TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Target Timestamp Higher Word Register"]
    #[inline(always)]
    pub fn emac_hwordsec_tshwr(&mut self) -> EMAC_HWORDSEC_TSHWR_W {
        EMAC_HWORDSEC_TSHWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC System Time-Higher Word Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwordsec](index.html) module"]
pub struct HWORDSEC_SPEC;
impl crate::RegisterSpec for HWORDSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwordsec::R](R) reader structure"]
impl crate::Readable for HWORDSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwordsec::W](W) writer structure"]
impl crate::Writable for HWORDSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWORDSEC to value 0"]
impl crate::Resettable for HWORDSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
