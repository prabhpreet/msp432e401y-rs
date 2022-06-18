#[doc = "Register `FMPPE1` reader"]
pub struct R(crate::R<FMPPE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPPE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPPE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPPE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPPE1` writer"]
pub struct W(crate::W<FMPPE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPPE1_SPEC>;
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
impl From<crate::W<FMPPE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPPE1_SPEC>) -> Self {
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
#[doc = "Flash Memory Protection Program Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe1](index.html) module"]
pub struct FMPPE1_SPEC;
impl crate::RegisterSpec for FMPPE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmppe1::R](R) reader structure"]
impl crate::Readable for FMPPE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmppe1::W](W) writer structure"]
impl crate::Writable for FMPPE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMPPE1 to value 0"]
impl crate::Resettable for FMPPE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
