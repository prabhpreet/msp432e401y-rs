#[doc = "Register `TIMSEC` reader"]
pub struct R(crate::R<TIMSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSEC` writer"]
pub struct W(crate::W<TIMSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSEC_SPEC>;
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
impl From<crate::W<TIMSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMSEC_TSS` reader - Timestamp Second"]
pub type EMAC_TIMSEC_TSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TIMSEC_TSS` writer - Timestamp Second"]
pub type EMAC_TIMSEC_TSS_W<'a> = crate::FieldWriter<'a, u32, TIMSEC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn emac_timsec_tss(&self) -> EMAC_TIMSEC_TSS_R {
        EMAC_TIMSEC_TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn emac_timsec_tss(&mut self) -> EMAC_TIMSEC_TSS_W {
        EMAC_TIMSEC_TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC System Time - Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsec](index.html) module"]
pub struct TIMSEC_SPEC;
impl crate::RegisterSpec for TIMSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timsec::R](R) reader structure"]
impl crate::Readable for TIMSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsec::W](W) writer structure"]
impl crate::Writable for TIMSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMSEC to value 0"]
impl crate::Resettable for TIMSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
