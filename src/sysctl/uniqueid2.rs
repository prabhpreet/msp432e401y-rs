#[doc = "Register `UNIQUEID2` reader"]
pub struct R(crate::R<UNIQUEID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIQUEID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIQUEID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIQUEID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNIQUEID2` writer"]
pub struct W(crate::W<UNIQUEID2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIQUEID2_SPEC>;
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
impl From<crate::W<UNIQUEID2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIQUEID2_SPEC>) -> Self {
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
#[doc = "Unique ID 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uniqueid2](index.html) module"]
pub struct UNIQUEID2_SPEC;
impl crate::RegisterSpec for UNIQUEID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uniqueid2::R](R) reader structure"]
impl crate::Readable for UNIQUEID2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uniqueid2::W](W) writer structure"]
impl crate::Writable for UNIQUEID2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNIQUEID2 to value 0"]
impl crate::Resettable for UNIQUEID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
