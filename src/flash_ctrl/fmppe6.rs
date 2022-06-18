#[doc = "Register `FMPPE6` reader"]
pub struct R(crate::R<FMPPE6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPPE6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPPE6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPPE6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPPE6` writer"]
pub struct W(crate::W<FMPPE6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPPE6_SPEC>;
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
impl From<crate::W<FMPPE6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPPE6_SPEC>) -> Self {
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
#[doc = "Flash Memory Protection Program Enable 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe6](index.html) module"]
pub struct FMPPE6_SPEC;
impl crate::RegisterSpec for FMPPE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmppe6::R](R) reader structure"]
impl crate::Readable for FMPPE6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmppe6::W](W) writer structure"]
impl crate::Writable for FMPPE6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPPE6 to value 0"]
impl crate::Resettable for FMPPE6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
