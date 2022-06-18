#[doc = "Register `ENASET` reader"]
pub struct R(crate::R<ENASET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENASET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENASET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENASET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENASET` writer"]
pub struct W(crate::W<ENASET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENASET_SPEC>;
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
impl From<crate::W<ENASET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENASET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_ENASET_SET` reader - Channel \\[n\\]
Enable Set"]
pub type UDMA_ENASET_SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_ENASET_SET` writer - Channel \\[n\\]
Enable Set"]
pub type UDMA_ENASET_SET_W<'a> = crate::FieldWriter<'a, u32, ENASET_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Enable Set"]
    #[inline(always)]
    pub fn udma_enaset_set(&self) -> UDMA_ENASET_SET_R {
        UDMA_ENASET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Enable Set"]
    #[inline(always)]
    pub fn udma_enaset_set(&mut self) -> UDMA_ENASET_SET_W {
        UDMA_ENASET_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaset](index.html) module"]
pub struct ENASET_SPEC;
impl crate::RegisterSpec for ENASET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enaset::R](R) reader structure"]
impl crate::Readable for ENASET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enaset::W](W) writer structure"]
impl crate::Writable for ENASET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENASET to value 0"]
impl crate::Resettable for ENASET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
