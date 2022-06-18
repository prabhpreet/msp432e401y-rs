#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_MIS_RORMIS` reader - SSI Receive Overrun Masked Interrupt Status"]
pub type SSI_MIS_RORMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_RORMIS` writer - SSI Receive Overrun Masked Interrupt Status"]
pub type SSI_MIS_RORMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `SSI_MIS_RTMIS` reader - SSI Receive Time-Out Masked Interrupt Status"]
pub type SSI_MIS_RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_RTMIS` writer - SSI Receive Time-Out Masked Interrupt Status"]
pub type SSI_MIS_RTMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 1>;
#[doc = "Field `SSI_MIS_RXMIS` reader - SSI Receive FIFO Masked Interrupt Status"]
pub type SSI_MIS_RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_RXMIS` writer - SSI Receive FIFO Masked Interrupt Status"]
pub type SSI_MIS_RXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `SSI_MIS_TXMIS` reader - SSI Transmit FIFO Masked Interrupt Status"]
pub type SSI_MIS_TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_TXMIS` writer - SSI Transmit FIFO Masked Interrupt Status"]
pub type SSI_MIS_TXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `SSI_MIS_DMARXMIS` reader - SSI Receive DMA Masked Interrupt Status"]
pub type SSI_MIS_DMARXMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_DMARXMIS` writer - SSI Receive DMA Masked Interrupt Status"]
pub type SSI_MIS_DMARXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
#[doc = "Field `SSI_MIS_DMATXMIS` reader - SSI Transmit DMA Masked Interrupt Status"]
pub type SSI_MIS_DMATXMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_DMATXMIS` writer - SSI Transmit DMA Masked Interrupt Status"]
pub type SSI_MIS_DMATXMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 5>;
#[doc = "Field `SSI_MIS_EOTMIS` reader - End of Transmit Masked Interrupt Status"]
pub type SSI_MIS_EOTMIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_MIS_EOTMIS` writer - End of Transmit Masked Interrupt Status"]
pub type SSI_MIS_EOTMIS_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&self) -> SSI_MIS_RORMIS_R {
        SSI_MIS_RORMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&self) -> SSI_MIS_RTMIS_R {
        SSI_MIS_RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&self) -> SSI_MIS_RXMIS_R {
        SSI_MIS_RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&self) -> SSI_MIS_TXMIS_R {
        SSI_MIS_TXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmarxmis(&self) -> SSI_MIS_DMARXMIS_R {
        SSI_MIS_DMARXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmatxmis(&self) -> SSI_MIS_DMATXMIS_R {
        SSI_MIS_DMATXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_eotmis(&self) -> SSI_MIS_EOTMIS_R {
        SSI_MIS_EOTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rormis(&mut self) -> SSI_MIS_RORMIS_W {
        SSI_MIS_RORMIS_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rtmis(&mut self) -> SSI_MIS_RTMIS_W {
        SSI_MIS_RTMIS_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_rxmis(&mut self) -> SSI_MIS_RXMIS_W {
        SSI_MIS_RXMIS_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_txmis(&mut self) -> SSI_MIS_TXMIS_W {
        SSI_MIS_TXMIS_W::new(self)
    }
    #[doc = "Bit 4 - SSI Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmarxmis(&mut self) -> SSI_MIS_DMARXMIS_W {
        SSI_MIS_DMARXMIS_W::new(self)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_dmatxmis(&mut self) -> SSI_MIS_DMATXMIS_W {
        SSI_MIS_DMATXMIS_W::new(self)
    }
    #[doc = "Bit 6 - End of Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn ssi_mis_eotmis(&mut self) -> SSI_MIS_EOTMIS_W {
        SSI_MIS_EOTMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
