#[doc = "Register `PDR` reader"]
pub struct R(crate::R<PDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDR` writer"]
pub struct W(crate::W<PDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDR_SPEC>;
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
impl From<crate::W<PDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDR_SPEC>) -> Self {
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
#[doc = "GPIO Pull-Down Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr](index.html) module"]
pub struct PDR_SPEC;
impl crate::RegisterSpec for PDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdr::R](R) reader structure"]
impl crate::Readable for PDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdr::W](W) writer structure"]
impl crate::Writable for PDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDR to value 0"]
impl crate::Resettable for PDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
