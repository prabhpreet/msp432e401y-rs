#[doc = "Register `TAILR` reader"]
pub struct R(crate::R<TAILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAILR` writer"]
pub struct W(crate::W<TAILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAILR_SPEC>;
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
impl From<crate::W<TAILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAILR_SPEC>) -> Self {
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
#[doc = "GPTM Timer A Interval Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tailr](index.html) module"]
pub struct TAILR_SPEC;
impl crate::RegisterSpec for TAILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tailr::R](R) reader structure"]
impl crate::Readable for TAILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tailr::W](W) writer structure"]
impl crate::Writable for TAILR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAILR to value 0"]
impl crate::Resettable for TAILR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
