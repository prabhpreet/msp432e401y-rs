#[doc = "Register `BIT` reader"]
pub struct R(crate::R<BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIT` writer"]
pub struct W(crate::W<BIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIT_SPEC>;
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
impl From<crate::W<BIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_BIT_BRP` reader - Baud Rate Prescaler"]
pub type CAN_BIT_BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_BIT_BRP` writer - Baud Rate Prescaler"]
pub type CAN_BIT_BRP_W<'a> = crate::FieldWriter<'a, u32, BIT_SPEC, u8, u8, 6, 0>;
#[doc = "Field `CAN_BIT_SJW` reader - (Re)Synchronization Jump Width"]
pub type CAN_BIT_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_BIT_SJW` writer - (Re)Synchronization Jump Width"]
pub type CAN_BIT_SJW_W<'a> = crate::FieldWriter<'a, u32, BIT_SPEC, u8, u8, 2, 6>;
#[doc = "Field `CAN_BIT_TSEG1` reader - Time Segment Before Sample Point"]
pub type CAN_BIT_TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_BIT_TSEG1` writer - Time Segment Before Sample Point"]
pub type CAN_BIT_TSEG1_W<'a> = crate::FieldWriter<'a, u32, BIT_SPEC, u8, u8, 4, 8>;
#[doc = "Field `CAN_BIT_TSEG2` reader - Time Segment after Sample Point"]
pub type CAN_BIT_TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_BIT_TSEG2` writer - Time Segment after Sample Point"]
pub type CAN_BIT_TSEG2_W<'a> = crate::FieldWriter<'a, u32, BIT_SPEC, u8, u8, 3, 12>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn can_bit_brp(&self) -> CAN_BIT_BRP_R {
        CAN_BIT_BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn can_bit_sjw(&self) -> CAN_BIT_SJW_R {
        CAN_BIT_SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg1(&self) -> CAN_BIT_TSEG1_R {
        CAN_BIT_TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg2(&self) -> CAN_BIT_TSEG2_R {
        CAN_BIT_TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn can_bit_brp(&mut self) -> CAN_BIT_BRP_W {
        CAN_BIT_BRP_W::new(self)
    }
    #[doc = "Bits 6:7 - (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn can_bit_sjw(&mut self) -> CAN_BIT_SJW_W {
        CAN_BIT_SJW_W::new(self)
    }
    #[doc = "Bits 8:11 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg1(&mut self) -> CAN_BIT_TSEG1_W {
        CAN_BIT_TSEG1_W::new(self)
    }
    #[doc = "Bits 12:14 - Time Segment after Sample Point"]
    #[inline(always)]
    pub fn can_bit_tseg2(&mut self) -> CAN_BIT_TSEG2_W {
        CAN_BIT_TSEG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Bit Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bit_](index.html) module"]
pub struct BIT_SPEC;
impl crate::RegisterSpec for BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bit_::R](R) reader structure"]
impl crate::Readable for BIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bit_::W](W) writer structure"]
impl crate::Writable for BIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIT to value 0"]
impl crate::Resettable for BIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
