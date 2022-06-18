#[doc = "Register `CALM1` reader"]
pub struct R(crate::R<CALM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALM1` writer"]
pub struct W(crate::W<CALM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALM1_SPEC>;
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
impl From<crate::W<CALM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CALM1_DOM` reader - Day of Month"]
pub type HIB_CALM1_DOM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIB_CALM1_DOM` writer - Day of Month"]
pub type HIB_CALM1_DOM_W<'a> = crate::FieldWriter<'a, u32, CALM1_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_calm1_dom(&self) -> HIB_CALM1_DOM_R {
        HIB_CALM1_DOM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_calm1_dom(&mut self) -> HIB_CALM1_DOM_W {
        HIB_CALM1_DOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar Match 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calm1](index.html) module"]
pub struct CALM1_SPEC;
impl crate::RegisterSpec for CALM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calm1::R](R) reader structure"]
impl crate::Readable for CALM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calm1::W](W) writer structure"]
impl crate::Writable for CALM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALM1 to value 0"]
impl crate::Resettable for CALM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
