#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_INT_INTID` reader - Interrupt Identifier"]
pub type CAN_INT_INTID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_INT_INTID` writer - Interrupt Identifier"]
pub type CAN_INT_INTID_W<'a> = crate::FieldWriter<'a, u32, INT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline(always)]
    pub fn can_int_intid(&self) -> CAN_INT_INTID_R {
        CAN_INT_INTID_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt Identifier"]
    #[inline(always)]
    pub fn can_int_intid(&mut self) -> CAN_INT_INTID_W {
        CAN_INT_INTID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
