#[doc = "Register `USEBURSTSET` reader"]
pub struct R(crate::R<USEBURSTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USEBURSTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USEBURSTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USEBURSTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USEBURSTSET` writer"]
pub struct W(crate::W<USEBURSTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USEBURSTSET_SPEC>;
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
impl From<crate::W<USEBURSTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USEBURSTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDMA_USEBURSTSET_SET` reader - Channel \\[n\\]
Useburst Set"]
pub type UDMA_USEBURSTSET_SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDMA_USEBURSTSET_SET` writer - Channel \\[n\\]
Useburst Set"]
pub type UDMA_USEBURSTSET_SET_W<'a> =
    crate::FieldWriter<'a, u32, USEBURSTSET_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Useburst Set"]
    #[inline(always)]
    pub fn udma_useburstset_set(&self) -> UDMA_USEBURSTSET_SET_R {
        UDMA_USEBURSTSET_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
Useburst Set"]
    #[inline(always)]
    pub fn udma_useburstset_set(&mut self) -> UDMA_USEBURSTSET_SET_W {
        UDMA_USEBURSTSET_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Useburst Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstset](index.html) module"]
pub struct USEBURSTSET_SPEC;
impl crate::RegisterSpec for USEBURSTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [useburstset::R](R) reader structure"]
impl crate::Readable for USEBURSTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [useburstset::W](W) writer structure"]
impl crate::Writable for USEBURSTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USEBURSTSET to value 0"]
impl crate::Resettable for USEBURSTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
