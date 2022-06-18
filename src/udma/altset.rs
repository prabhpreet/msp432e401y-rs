#[doc = "Register `ALTSET` reader"]
pub struct R(crate::R<ALTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTSET` writer"]
pub struct W(crate::W<ALTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTSET_SPEC>;
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
impl From<crate::W<ALTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_ALTSET_SET` reader - Channel \\[n\\]
Alternate Set"]
pub type UDMA_ALTSET_SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_ALTSET_SET` writer - Channel \\[n\\]
Alternate Set"]
pub type UDMA_ALTSET_SET_W<'a> = crate::FieldWriter<'a, u32, ALTSET_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Alternate Set"]
    #[inline(always)]
    pub fn udma_altset_set(&self) -> UDMA_ALTSET_SET_R {
        UDMA_ALTSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Alternate Set"]
    #[inline(always)]
    pub fn udma_altset_set(&mut self) -> UDMA_ALTSET_SET_W {
        UDMA_ALTSET_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Primary Alternate Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altset](index.html) module"]
pub struct ALTSET_SPEC;
impl crate::RegisterSpec for ALTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altset::R](R) reader structure"]
impl crate::Readable for ALTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altset::W](W) writer structure"]
impl crate::Writable for ALTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTSET to value 0"]
impl crate::Resettable for ALTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
