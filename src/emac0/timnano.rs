#[doc = "Register `TIMNANO` reader"]
pub struct R(crate::R<TIMNANO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMNANO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMNANO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMNANO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMNANO` writer"]
pub struct W(crate::W<TIMNANO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMNANO_SPEC>;
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
impl From<crate::W<TIMNANO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMNANO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMNANO_TSSS` reader - Timestamp Sub-Seconds"]
pub type EMAC_TIMNANO_TSSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TIMNANO_TSSS` writer - Timestamp Sub-Seconds"]
pub type EMAC_TIMNANO_TSSS_W<'a> = crate::FieldWriter<'a, u32, TIMNANO_SPEC, u32, u32, 31, 0>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub-Seconds"]
    #[inline(always)]
    pub fn emac_timnano_tsss(&self) -> EMAC_TIMNANO_TSSS_R {
        EMAC_TIMNANO_TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-Seconds"]
    #[inline(always)]
    pub fn emac_timnano_tsss(&mut self) -> EMAC_TIMNANO_TSSS_W {
        EMAC_TIMNANO_TSSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC System Time - Nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timnano](index.html) module"]
pub struct TIMNANO_SPEC;
impl crate::RegisterSpec for TIMNANO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timnano::R](R) reader structure"]
impl crate::Readable for TIMNANO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timnano::W](W) writer structure"]
impl crate::Writable for TIMNANO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMNANO to value 0"]
impl crate::Resettable for TIMNANO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
