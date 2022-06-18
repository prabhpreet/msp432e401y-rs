#[doc = "Register `DEN` reader"]
pub struct R(crate::R<DEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEN` writer"]
pub struct W(crate::W<DEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEN_SPEC>;
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
impl From<crate::W<DEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEN_SPEC>) -> Self {
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
#[doc = "GPIO Digital Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [den](index.html) module"]
pub struct DEN_SPEC;
impl crate::RegisterSpec for DEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [den::R](R) reader structure"]
impl crate::Readable for DEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [den::W](W) writer structure"]
impl crate::Writable for DEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEN to value 0"]
impl crate::Resettable for DEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
