#[doc = "Register `TIMSECU` reader"]
pub struct R(crate::R<TIMSECU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSECU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSECU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSECU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSECU` writer"]
pub struct W(crate::W<TIMSECU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSECU_SPEC>;
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
impl From<crate::W<TIMSECU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSECU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMSECU_TSS` reader - Timestamp Second"]
pub type EMAC_TIMSECU_TSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMAC_TIMSECU_TSS` writer - Timestamp Second"]
pub type EMAC_TIMSECU_TSS_W<'a> = crate::FieldWriter<'a, u32, TIMSECU_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn emac_timsecu_tss(&self) -> EMAC_TIMSECU_TSS_R {
        EMAC_TIMSECU_TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn emac_timsecu_tss(&mut self) -> EMAC_TIMSECU_TSS_W {
        EMAC_TIMSECU_TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC System Time - Seconds Update\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timsecu](index.html) module"]
pub struct TIMSECU_SPEC;
impl crate::RegisterSpec for TIMSECU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timsecu::R](R) reader structure"]
impl crate::Readable for TIMSECU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timsecu::W](W) writer structure"]
impl crate::Writable for TIMSECU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMSECU to value 0"]
impl crate::Resettable for TIMSECU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
