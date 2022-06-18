#[doc = "Register `IF1MSK1` reader"]
pub struct R(crate::R<IF1MSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF1MSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF1MSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF1MSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF1MSK1` writer"]
pub struct W(crate::W<IF1MSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF1MSK1_SPEC>;
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
impl From<crate::W<IF1MSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF1MSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF1MSK1_IDMSK` reader - Identifier Mask"]
pub type CAN_IF1MSK1_IDMSK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_IF1MSK1_IDMSK` writer - Identifier Mask"]
pub type CAN_IF1MSK1_IDMSK_W<'a> = crate::FieldWriter<'a, u32, IF1MSK1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Identifier Mask"]
    #[inline(always)]
    pub fn can_if1msk1_idmsk(&self) -> CAN_IF1MSK1_IDMSK_R {
        CAN_IF1MSK1_IDMSK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Identifier Mask"]
    #[inline(always)]
    pub fn can_if1msk1_idmsk(&mut self) -> CAN_IF1MSK1_IDMSK_W {
        CAN_IF1MSK1_IDMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF1 Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1msk1](index.html) module"]
pub struct IF1MSK1_SPEC;
impl crate::RegisterSpec for IF1MSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if1msk1::R](R) reader structure"]
impl crate::Readable for IF1MSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if1msk1::W](W) writer structure"]
impl crate::Writable for IF1MSK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF1MSK1 to value 0"]
impl crate::Resettable for IF1MSK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
