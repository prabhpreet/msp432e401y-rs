#[doc = "Register `IF2MCTL` reader"]
pub struct R(crate::R<IF2MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF2MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF2MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF2MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF2MCTL` writer"]
pub struct W(crate::W<IF2MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF2MCTL_SPEC>;
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
impl From<crate::W<IF2MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF2MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_IF2MCTL_DLC` reader - Data Length Code"]
pub type CAN_IF2MCTL_DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN_IF2MCTL_DLC` writer - Data Length Code"]
pub type CAN_IF2MCTL_DLC_W<'a> = crate::FieldWriter<'a, u32, IF2MCTL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `CAN_IF2MCTL_EOB` reader - End of Buffer"]
pub type CAN_IF2MCTL_EOB_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_EOB` writer - End of Buffer"]
pub type CAN_IF2MCTL_EOB_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 7>;
#[doc = "Field `CAN_IF2MCTL_TXRQST` reader - Transmit Request"]
pub type CAN_IF2MCTL_TXRQST_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_TXRQST` writer - Transmit Request"]
pub type CAN_IF2MCTL_TXRQST_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 8>;
#[doc = "Field `CAN_IF2MCTL_RMTEN` reader - Remote Enable"]
pub type CAN_IF2MCTL_RMTEN_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_RMTEN` writer - Remote Enable"]
pub type CAN_IF2MCTL_RMTEN_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 9>;
#[doc = "Field `CAN_IF2MCTL_RXIE` reader - Receive Interrupt Enable"]
pub type CAN_IF2MCTL_RXIE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_RXIE` writer - Receive Interrupt Enable"]
pub type CAN_IF2MCTL_RXIE_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 10>;
#[doc = "Field `CAN_IF2MCTL_TXIE` reader - Transmit Interrupt Enable"]
pub type CAN_IF2MCTL_TXIE_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_TXIE` writer - Transmit Interrupt Enable"]
pub type CAN_IF2MCTL_TXIE_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 11>;
#[doc = "Field `CAN_IF2MCTL_UMASK` reader - Use Acceptance Mask"]
pub type CAN_IF2MCTL_UMASK_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_UMASK` writer - Use Acceptance Mask"]
pub type CAN_IF2MCTL_UMASK_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 12>;
#[doc = "Field `CAN_IF2MCTL_INTPND` reader - Interrupt Pending"]
pub type CAN_IF2MCTL_INTPND_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_INTPND` writer - Interrupt Pending"]
pub type CAN_IF2MCTL_INTPND_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 13>;
#[doc = "Field `CAN_IF2MCTL_MSGLST` reader - Message Lost"]
pub type CAN_IF2MCTL_MSGLST_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_MSGLST` writer - Message Lost"]
pub type CAN_IF2MCTL_MSGLST_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 14>;
#[doc = "Field `CAN_IF2MCTL_NEWDAT` reader - New Data"]
pub type CAN_IF2MCTL_NEWDAT_R = crate::BitReader<bool>;
#[doc = "Field `CAN_IF2MCTL_NEWDAT` writer - New Data"]
pub type CAN_IF2MCTL_NEWDAT_W<'a> = crate::BitWriter<'a, u32, IF2MCTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn can_if2mctl_dlc(&self) -> CAN_IF2MCTL_DLC_R {
        CAN_IF2MCTL_DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn can_if2mctl_eob(&self) -> CAN_IF2MCTL_EOB_R {
        CAN_IF2MCTL_EOB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn can_if2mctl_txrqst(&self) -> CAN_IF2MCTL_TXRQST_R {
        CAN_IF2MCTL_TXRQST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn can_if2mctl_rmten(&self) -> CAN_IF2MCTL_RMTEN_R {
        CAN_IF2MCTL_RMTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn can_if2mctl_rxie(&self) -> CAN_IF2MCTL_RXIE_R {
        CAN_IF2MCTL_RXIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn can_if2mctl_txie(&self) -> CAN_IF2MCTL_TXIE_R {
        CAN_IF2MCTL_TXIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn can_if2mctl_umask(&self) -> CAN_IF2MCTL_UMASK_R {
        CAN_IF2MCTL_UMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn can_if2mctl_intpnd(&self) -> CAN_IF2MCTL_INTPND_R {
        CAN_IF2MCTL_INTPND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    pub fn can_if2mctl_msglst(&self) -> CAN_IF2MCTL_MSGLST_R {
        CAN_IF2MCTL_MSGLST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn can_if2mctl_newdat(&self) -> CAN_IF2MCTL_NEWDAT_R {
        CAN_IF2MCTL_NEWDAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn can_if2mctl_dlc(&mut self) -> CAN_IF2MCTL_DLC_W {
        CAN_IF2MCTL_DLC_W::new(self)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn can_if2mctl_eob(&mut self) -> CAN_IF2MCTL_EOB_W {
        CAN_IF2MCTL_EOB_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn can_if2mctl_txrqst(&mut self) -> CAN_IF2MCTL_TXRQST_W {
        CAN_IF2MCTL_TXRQST_W::new(self)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn can_if2mctl_rmten(&mut self) -> CAN_IF2MCTL_RMTEN_W {
        CAN_IF2MCTL_RMTEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn can_if2mctl_rxie(&mut self) -> CAN_IF2MCTL_RXIE_W {
        CAN_IF2MCTL_RXIE_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn can_if2mctl_txie(&mut self) -> CAN_IF2MCTL_TXIE_W {
        CAN_IF2MCTL_TXIE_W::new(self)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn can_if2mctl_umask(&mut self) -> CAN_IF2MCTL_UMASK_W {
        CAN_IF2MCTL_UMASK_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn can_if2mctl_intpnd(&mut self) -> CAN_IF2MCTL_INTPND_W {
        CAN_IF2MCTL_INTPND_W::new(self)
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    pub fn can_if2mctl_msglst(&mut self) -> CAN_IF2MCTL_MSGLST_W {
        CAN_IF2MCTL_MSGLST_W::new(self)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn can_if2mctl_newdat(&mut self) -> CAN_IF2MCTL_NEWDAT_W {
        CAN_IF2MCTL_NEWDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN IF2 Message Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if2mctl](index.html) module"]
pub struct IF2MCTL_SPEC;
impl crate::RegisterSpec for IF2MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if2mctl::R](R) reader structure"]
impl crate::Readable for IF2MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if2mctl::W](W) writer structure"]
impl crate::Writable for IF2MCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF2MCTL to value 0"]
impl crate::Resettable for IF2MCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
