#[doc = "Register `MSG1VAL` reader"]
pub struct R(crate::R<MSG1VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSG1VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSG1VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSG1VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSG1VAL` writer"]
pub struct W(crate::W<MSG1VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSG1VAL_SPEC>;
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
impl From<crate::W<MSG1VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSG1VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_MSG1VAL_MSGVAL` reader - Message Valid Bits"]
pub type CAN_MSG1VAL_MSGVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_MSG1VAL_MSGVAL` writer - Message Valid Bits"]
pub type CAN_MSG1VAL_MSGVAL_W<'a> = crate::FieldWriter<'a, u32, MSG1VAL_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Message Valid Bits"]
    #[inline(always)]
    pub fn can_msg1val_msgval(&self) -> CAN_MSG1VAL_MSGVAL_R {
        CAN_MSG1VAL_MSGVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Valid Bits"]
    #[inline(always)]
    pub fn can_msg1val_msgval(&mut self) -> CAN_MSG1VAL_MSGVAL_W {
        CAN_MSG1VAL_MSGVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Message 1 Valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg1val](index.html) module"]
pub struct MSG1VAL_SPEC;
impl crate::RegisterSpec for MSG1VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msg1val::R](R) reader structure"]
impl crate::Readable for MSG1VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msg1val::W](W) writer structure"]
impl crate::Writable for MSG1VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSG1VAL to value 0"]
impl crate::Resettable for MSG1VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
