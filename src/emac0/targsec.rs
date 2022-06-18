#[doc = "Register `TARGSEC` reader"]
pub struct R(crate::R<TARGSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGSEC` writer"]
pub struct W(crate::W<TARGSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGSEC_SPEC>;
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
impl From<crate::W<TARGSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TARGSEC_TSTR` reader - Target Time Seconds Register"]
pub type EMAC_TARGSEC_TSTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TARGSEC_TSTR` writer - Target Time Seconds Register"]
pub type EMAC_TARGSEC_TSTR_W<'a> = crate::FieldWriter<'a, u32, TARGSEC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn emac_targsec_tstr(&self) -> EMAC_TARGSEC_TSTR_R {
        EMAC_TARGSEC_TSTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target Time Seconds Register"]
    #[inline(always)]
    pub fn emac_targsec_tstr(&mut self) -> EMAC_TARGSEC_TSTR_W {
        EMAC_TARGSEC_TSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Target Time Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targsec](index.html) module"]
pub struct TARGSEC_SPEC;
impl crate::RegisterSpec for TARGSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targsec::R](R) reader structure"]
impl crate::Readable for TARGSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targsec::W](W) writer structure"]
impl crate::Writable for TARGSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGSEC to value 0"]
impl crate::Resettable for TARGSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
