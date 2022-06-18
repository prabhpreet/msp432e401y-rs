#[doc = "Register `ERR` reader"]
pub struct R(crate::R<ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR` writer"]
pub struct W(crate::W<ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_SPEC>;
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
impl From<crate::W<ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_ERR_TEC` reader - Transmit Error Counter"]
pub type CAN_ERR_TEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_ERR_TEC` writer - Transmit Error Counter"]
pub type CAN_ERR_TEC_W<'a> = crate::FieldWriter<'a, u32, ERR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `CAN_ERR_REC` reader - Receive Error Counter"]
pub type CAN_ERR_REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_ERR_REC` writer - Receive Error Counter"]
pub type CAN_ERR_REC_W<'a> = crate::FieldWriter<'a, u32, ERR_SPEC, u8, u8, 7, 8>;
#[doc = "Field `CAN_ERR_RP` reader - Received Error Passive"]
pub type CAN_ERR_RP_R = crate::BitReader<bool>;
#[doc = "Field `CAN_ERR_RP` writer - Received Error Passive"]
pub type CAN_ERR_RP_W<'a> = crate::BitWriter<'a, u32, ERR_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn can_err_tec(&self) -> CAN_ERR_TEC_R {
        CAN_ERR_TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn can_err_rec(&self) -> CAN_ERR_REC_R {
        CAN_ERR_REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Received Error Passive"]
    #[inline(always)]
    pub fn can_err_rp(&self) -> CAN_ERR_RP_R {
        CAN_ERR_RP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn can_err_tec(&mut self) -> CAN_ERR_TEC_W {
        CAN_ERR_TEC_W::new(self)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn can_err_rec(&mut self) -> CAN_ERR_REC_W {
        CAN_ERR_REC_W::new(self)
    }
    #[doc = "Bit 15 - Received Error Passive"]
    #[inline(always)]
    pub fn can_err_rp(&mut self) -> CAN_ERR_RP_W {
        CAN_ERR_RP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](index.html) module"]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err::R](R) reader structure"]
impl crate::Readable for ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err::W](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
