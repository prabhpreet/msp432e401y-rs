#[doc = "Register `SUBSECINC` reader"]
pub struct R(crate::R<SUBSECINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSECINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSECINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSECINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSECINC` writer"]
pub struct W(crate::W<SUBSECINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSECINC_SPEC>;
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
impl From<crate::W<SUBSECINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSECINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_SUBSECINC_SSINC` reader - Sub-second Increment Value"]
pub type EMAC_SUBSECINC_SSINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_SUBSECINC_SSINC` writer - Sub-second Increment Value"]
pub type EMAC_SUBSECINC_SSINC_W<'a> = crate::FieldWriter<'a, u32, SUBSECINC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn emac_subsecinc_ssinc(&self) -> EMAC_SUBSECINC_SSINC_R {
        EMAC_SUBSECINC_SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn emac_subsecinc_ssinc(&mut self) -> EMAC_SUBSECINC_SSINC_W {
        EMAC_SUBSECINC_SSINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Sub-Second Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsecinc](index.html) module"]
pub struct SUBSECINC_SPEC;
impl crate::RegisterSpec for SUBSECINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsecinc::R](R) reader structure"]
impl crate::Readable for SUBSECINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsecinc::W](W) writer structure"]
impl crate::Writable for SUBSECINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSECINC to value 0"]
impl crate::Resettable for SUBSECINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
