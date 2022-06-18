#[doc = "Register `TBV` reader"]
pub struct R(crate::R<TBV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBV` writer"]
pub struct W(crate::W<TBV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBV_SPEC>;
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
impl From<crate::W<TBV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbv](index.html) module"]
pub struct TBV_SPEC;
impl crate::RegisterSpec for TBV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbv::R](R) reader structure"]
impl crate::Readable for TBV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbv::W](W) writer structure"]
impl crate::Writable for TBV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBV to value 0"]
impl crate::Resettable for TBV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
