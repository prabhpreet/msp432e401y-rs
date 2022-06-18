#[doc = "Register `IF2ARB1` reader"]
pub struct R(crate::R<IF2ARB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF2ARB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF2ARB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF2ARB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2ARB1` writer"]
pub struct W(crate::W<IF2ARB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF2ARB1_SPEC>;
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
impl From<crate::W<IF2ARB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF2ARB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2ARB1_ID` reader - Message Identifier"]
pub type CAN_IF2ARB1_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_IF2ARB1_ID` writer - Message Identifier"]
pub type CAN_IF2ARB1_ID_W<'a> = crate::FieldWriter<'a, u32, IF2ARB1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb1_id(&self) -> CAN_IF2ARB1_ID_R {
        CAN_IF2ARB1_ID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb1_id(&mut self) -> CAN_IF2ARB1_ID_W {
        CAN_IF2ARB1_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Arbitration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2arb1](index.html) module"]
pub struct IF2ARB1_SPEC;
impl crate::RegisterSpec for IF2ARB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if2arb1::R](R) reader structure"]
impl crate::Readable for IF2ARB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if2arb1::W](W) writer structure"]
impl crate::Writable for IF2ARB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2ARB1 to value 0"]
impl crate::Resettable for IF2ARB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
