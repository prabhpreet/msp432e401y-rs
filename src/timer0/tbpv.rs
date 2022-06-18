#[doc = "Register `TBPV` reader"]
pub struct R(crate::R<TBPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPV` writer"]
pub struct W(crate::W<TBPV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPV_SPEC>;
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
impl From<crate::W<TBPV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPV_SPEC>) -> Self {
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
#[doc = "GPTM Timer B Prescale Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpv](index.html) module"]
pub struct TBPV_SPEC;
impl crate::RegisterSpec for TBPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpv::R](R) reader structure"]
impl crate::Readable for TBPV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpv::W](W) writer structure"]
impl crate::Writable for TBPV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBPV to value 0"]
impl crate::Resettable for TBPV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
