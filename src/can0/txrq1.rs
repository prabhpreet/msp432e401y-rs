#[doc = "Register `TXRQ1` reader"]
pub struct R(crate::R<TXRQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXRQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXRQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXRQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXRQ1` writer"]
pub struct W(crate::W<TXRQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXRQ1_SPEC>;
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
impl From<crate::W<TXRQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXRQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_TXRQ1_TXRQST` reader - Transmission Request Bits"]
pub type CAN_TXRQ1_TXRQST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAN_TXRQ1_TXRQST` writer - Transmission Request Bits"]
pub type CAN_TXRQ1_TXRQST_W<'a> = crate::FieldWriter<'a, u32, TXRQ1_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Transmission Request Bits"]
    #[inline(always)]
    pub fn can_txrq1_txrqst(&self) -> CAN_TXRQ1_TXRQST_R {
        CAN_TXRQ1_TXRQST_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmission Request Bits"]
    #[inline(always)]
    pub fn can_txrq1_txrqst(&mut self) -> CAN_TXRQ1_TXRQST_W {
        CAN_TXRQ1_TXRQST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Transmission Request 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txrq1](index.html) module"]
pub struct TXRQ1_SPEC;
impl crate::RegisterSpec for TXRQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txrq1::R](R) reader structure"]
impl crate::Readable for TXRQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txrq1::W](W) writer structure"]
impl crate::Writable for TXRQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXRQ1 to value 0"]
impl crate::Resettable for TXRQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
