#[doc = "Register `CCMCGREQ` reader"]
pub struct R(crate::R<CCMCGREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMCGREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMCGREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMCGREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMCGREQ` writer"]
pub struct W(crate::W<CCMCGREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMCGREQ_SPEC>;
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
impl From<crate::W<CCMCGREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMCGREQ_SPEC>) -> Self {
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
#[doc = "Cryptographic Modules Clock Gating Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmcgreq](index.html) module"]
pub struct CCMCGREQ_SPEC;
impl crate::RegisterSpec for CCMCGREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmcgreq::R](R) reader structure"]
impl crate::Readable for CCMCGREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmcgreq::W](W) writer structure"]
impl crate::Writable for CCMCGREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMCGREQ to value 0"]
impl crate::Resettable for CCMCGREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
