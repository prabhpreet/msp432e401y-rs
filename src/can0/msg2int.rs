#[doc = "Register `MSG2INT` reader"]
pub struct R(crate::R<MSG2INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSG2INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSG2INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSG2INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSG2INT` writer"]
pub struct W(crate::W<MSG2INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSG2INT_SPEC>;
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
impl From<crate::W<MSG2INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSG2INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_MSG2INT_INTPND` reader - Interrupt Pending Bits"]
pub type CAN_MSG2INT_INTPND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_MSG2INT_INTPND` writer - Interrupt Pending Bits"]
pub type CAN_MSG2INT_INTPND_W<'a> = crate::FieldWriter<'a, u32, MSG2INT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn can_msg2int_intpnd(&self) -> CAN_MSG2INT_INTPND_R {
        CAN_MSG2INT_INTPND_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt Pending Bits"]
    #[inline(always)]
    pub fn can_msg2int_intpnd(&mut self) -> CAN_MSG2INT_INTPND_W {
        CAN_MSG2INT_INTPND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Message 2 Interrupt Pending\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msg2int](index.html) module"]
pub struct MSG2INT_SPEC;
impl crate::RegisterSpec for MSG2INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msg2int::R](R) reader structure"]
impl crate::Readable for MSG2INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msg2int::W](W) writer structure"]
impl crate::Writable for MSG2INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSG2INT to value 0"]
impl crate::Resettable for MSG2INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
