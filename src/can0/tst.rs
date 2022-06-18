#[doc = "Register `TST` reader"]
pub struct R(crate::R<TST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST` writer"]
pub struct W(crate::W<TST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_SPEC>;
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
impl From<crate::W<TST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN_TST_BASIC` reader - Basic Mode"]
pub type CAN_TST_BASIC_R = crate::BitReader<bool>;
#[doc = "Field `CAN_TST_BASIC` writer - Basic Mode"]
pub type CAN_TST_BASIC_W<'a> = crate::BitWriter<'a, u32, TST_SPEC, bool, 2>;
#[doc = "Field `CAN_TST_SILENT` reader - Silent Mode"]
pub type CAN_TST_SILENT_R = crate::BitReader<bool>;
#[doc = "Field `CAN_TST_SILENT` writer - Silent Mode"]
pub type CAN_TST_SILENT_W<'a> = crate::BitWriter<'a, u32, TST_SPEC, bool, 3>;
#[doc = "Field `CAN_TST_LBACK` reader - Loopback Mode"]
pub type CAN_TST_LBACK_R = crate::BitReader<bool>;
#[doc = "Field `CAN_TST_LBACK` writer - Loopback Mode"]
pub type CAN_TST_LBACK_W<'a> = crate::BitWriter<'a, u32, TST_SPEC, bool, 4>;
#[doc = "Transmit Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAN_TST_TX_A {
    #[doc = "0: CAN Module Control"]
    CAN_TST_TX_CANCTL = 0,
    #[doc = "1: Sample Point"]
    CAN_TST_TX_SAMPLE = 1,
    #[doc = "2: Driven Low"]
    CAN_TST_TX_DOMINANT = 2,
    #[doc = "3: Driven High"]
    CAN_TST_TX_RECESSIVE = 3,
}
impl From<CAN_TST_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAN_TST_TX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAN_TST_TX` reader - Transmit Control"]
pub type CAN_TST_TX_R = crate::FieldReader<u8, CAN_TST_TX_A>;
impl CAN_TST_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAN_TST_TX_A {
        match self.bits {
            0 => CAN_TST_TX_A::CAN_TST_TX_CANCTL,
            1 => CAN_TST_TX_A::CAN_TST_TX_SAMPLE,
            2 => CAN_TST_TX_A::CAN_TST_TX_DOMINANT,
            3 => CAN_TST_TX_A::CAN_TST_TX_RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_CANCTL`"]
    #[inline(always)]
    pub fn is_can_tst_tx_canctl(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_CANCTL
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_SAMPLE`"]
    #[inline(always)]
    pub fn is_can_tst_tx_sample(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_SAMPLE
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_DOMINANT`"]
    #[inline(always)]
    pub fn is_can_tst_tx_dominant(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_DOMINANT
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_RECESSIVE`"]
    #[inline(always)]
    pub fn is_can_tst_tx_recessive(&self) -> bool {
        *self == CAN_TST_TX_A::CAN_TST_TX_RECESSIVE
    }
}
#[doc = "Field `CAN_TST_TX` writer - Transmit Control"]
pub type CAN_TST_TX_W<'a> = crate::FieldWriterSafe<'a, u32, TST_SPEC, u8, CAN_TST_TX_A, 2, 5>;
impl<'a> CAN_TST_TX_W<'a> {
    #[doc = "CAN Module Control"]
    #[inline(always)]
    pub fn can_tst_tx_canctl(self) -> &'a mut W {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_CANCTL)
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub fn can_tst_tx_sample(self) -> &'a mut W {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_SAMPLE)
    }
    #[doc = "Driven Low"]
    #[inline(always)]
    pub fn can_tst_tx_dominant(self) -> &'a mut W {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_DOMINANT)
    }
    #[doc = "Driven High"]
    #[inline(always)]
    pub fn can_tst_tx_recessive(self) -> &'a mut W {
        self.variant(CAN_TST_TX_A::CAN_TST_TX_RECESSIVE)
    }
}
#[doc = "Field `CAN_TST_RX` reader - Receive Observation"]
pub type CAN_TST_RX_R = crate::BitReader<bool>;
#[doc = "Field `CAN_TST_RX` writer - Receive Observation"]
pub type CAN_TST_RX_W<'a> = crate::BitWriter<'a, u32, TST_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn can_tst_basic(&self) -> CAN_TST_BASIC_R {
        CAN_TST_BASIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn can_tst_silent(&self) -> CAN_TST_SILENT_R {
        CAN_TST_SILENT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn can_tst_lback(&self) -> CAN_TST_LBACK_R {
        CAN_TST_LBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn can_tst_tx(&self) -> CAN_TST_TX_R {
        CAN_TST_TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn can_tst_rx(&self) -> CAN_TST_RX_R {
        CAN_TST_RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn can_tst_basic(&mut self) -> CAN_TST_BASIC_W {
        CAN_TST_BASIC_W::new(self)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn can_tst_silent(&mut self) -> CAN_TST_SILENT_W {
        CAN_TST_SILENT_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn can_tst_lback(&mut self) -> CAN_TST_LBACK_W {
        CAN_TST_LBACK_W::new(self)
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn can_tst_tx(&mut self) -> CAN_TST_TX_W {
        CAN_TST_TX_W::new(self)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn can_tst_rx(&mut self) -> CAN_TST_RX_W {
        CAN_TST_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](index.html) module"]
pub struct TST_SPEC;
impl crate::RegisterSpec for TST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst::R](R) reader structure"]
impl crate::Readable for TST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst::W](W) writer structure"]
impl crate::Writable for TST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
