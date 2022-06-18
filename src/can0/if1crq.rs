#[doc = "Register `IF1CRQ` reader"]
pub struct R(crate::R<IF1CRQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF1CRQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF1CRQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF1CRQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF1CRQ` writer"]
pub struct W(crate::W<IF1CRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF1CRQ_SPEC>;
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
impl From<crate::W<IF1CRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF1CRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF1CRQ_MNUM` reader - Message Number"]
pub type CAN_IF1CRQ_MNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_IF1CRQ_MNUM` writer - Message Number"]
pub type CAN_IF1CRQ_MNUM_W<'a> = crate::FieldWriter<'a, u32, IF1CRQ_SPEC, u8, u8, 6, 0>;
#[doc = "Field `CAN_IF1CRQ_BUSY` reader - Busy Flag"]
pub type CAN_IF1CRQ_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF1CRQ_BUSY` writer - Busy Flag"]
pub type CAN_IF1CRQ_BUSY_W<'a> = crate::BitWriter<'a, u32, IF1CRQ_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn can_if1crq_mnum(&self) -> CAN_IF1CRQ_MNUM_R {
        CAN_IF1CRQ_MNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn can_if1crq_busy(&self) -> CAN_IF1CRQ_BUSY_R {
        CAN_IF1CRQ_BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn can_if1crq_mnum(&mut self) -> CAN_IF1CRQ_MNUM_W {
        CAN_IF1CRQ_MNUM_W::new(self)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn can_if1crq_busy(&mut self) -> CAN_IF1CRQ_BUSY_W {
        CAN_IF1CRQ_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF1 Command Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1crq](index.html) module"]
pub struct IF1CRQ_SPEC;
impl crate::RegisterSpec for IF1CRQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if1crq::R](R) reader structure"]
impl crate::Readable for IF1CRQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if1crq::W](W) writer structure"]
impl crate::Writable for IF1CRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF1CRQ to value 0"]
impl crate::Resettable for IF1CRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
