#[doc = "Register `PRIOSET` reader"]
pub struct R(crate::R<PRIOSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIOSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIOSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIOSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIOSET` writer"]
pub struct W(crate::W<PRIOSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIOSET_SPEC>;
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
impl From<crate::W<PRIOSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIOSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_PRIOSET_SET` reader - Channel \\[n\\]
Priority Set"]
pub type UDMA_PRIOSET_SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_PRIOSET_SET` writer - Channel \\[n\\]
Priority Set"]
pub type UDMA_PRIOSET_SET_W<'a> = crate::FieldWriter<'a, u32, PRIOSET_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Priority Set"]
    #[inline(always)]
    pub fn udma_prioset_set(&self) -> UDMA_PRIOSET_SET_R {
        UDMA_PRIOSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Priority Set"]
    #[inline(always)]
    pub fn udma_prioset_set(&mut self) -> UDMA_PRIOSET_SET_W {
        UDMA_PRIOSET_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Priority Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioset](index.html) module"]
pub struct PRIOSET_SPEC;
impl crate::RegisterSpec for PRIOSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prioset::R](R) reader structure"]
impl crate::Readable for PRIOSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prioset::W](W) writer structure"]
impl crate::Writable for PRIOSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIOSET to value 0"]
impl crate::Resettable for PRIOSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
