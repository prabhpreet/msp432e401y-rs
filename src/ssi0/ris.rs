#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_RIS_RORRIS` reader - SSI Receive Overrun Raw Interrupt Status"]
pub type SSI_RIS_RORRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_RORRIS` writer - SSI Receive Overrun Raw Interrupt Status"]
pub type SSI_RIS_RORRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `SSI_RIS_RTRIS` reader - SSI Receive Time-Out Raw Interrupt Status"]
pub type SSI_RIS_RTRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_RTRIS` writer - SSI Receive Time-Out Raw Interrupt Status"]
pub type SSI_RIS_RTRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `SSI_RIS_RXRIS` reader - SSI Receive FIFO Raw Interrupt Status"]
pub type SSI_RIS_RXRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_RXRIS` writer - SSI Receive FIFO Raw Interrupt Status"]
pub type SSI_RIS_RXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `SSI_RIS_TXRIS` reader - SSI Transmit FIFO Raw Interrupt Status"]
pub type SSI_RIS_TXRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_TXRIS` writer - SSI Transmit FIFO Raw Interrupt Status"]
pub type SSI_RIS_TXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `SSI_RIS_DMARXRIS` reader - SSI Receive DMA Raw Interrupt Status"]
pub type SSI_RIS_DMARXRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_DMARXRIS` writer - SSI Receive DMA Raw Interrupt Status"]
pub type SSI_RIS_DMARXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `SSI_RIS_DMATXRIS` reader - SSI Transmit DMA Raw Interrupt Status"]
pub type SSI_RIS_DMATXRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_DMATXRIS` writer - SSI Transmit DMA Raw Interrupt Status"]
pub type SSI_RIS_DMATXRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
#[doc = "Field `SSI_RIS_EOTRIS` reader - End of Transmit Raw Interrupt Status"]
pub type SSI_RIS_EOTRIS_R = crate::BitReader<bool>;
#[doc = "Field `SSI_RIS_EOTRIS` writer - End of Transmit Raw Interrupt Status"]
pub type SSI_RIS_EOTRIS_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&self) -> SSI_RIS_RORRIS_R {
        SSI_RIS_RORRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&self) -> SSI_RIS_RTRIS_R {
        SSI_RIS_RTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&self) -> SSI_RIS_RXRIS_R {
        SSI_RIS_RXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&self) -> SSI_RIS_TXRIS_R {
        SSI_RIS_TXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmarxris(&self) -> SSI_RIS_DMARXRIS_R {
        SSI_RIS_DMARXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmatxris(&self) -> SSI_RIS_DMATXRIS_R {
        SSI_RIS_DMATXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_eotris(&self) -> SSI_RIS_EOTRIS_R {
        SSI_RIS_EOTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rorris(&mut self) -> SSI_RIS_RORRIS_W {
        SSI_RIS_RORRIS_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rtris(&mut self) -> SSI_RIS_RTRIS_W {
        SSI_RIS_RTRIS_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_rxris(&mut self) -> SSI_RIS_RXRIS_W {
        SSI_RIS_RXRIS_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_txris(&mut self) -> SSI_RIS_TXRIS_W {
        SSI_RIS_TXRIS_W::new(self)
    }
    #[doc = "Bit 4 - SSI Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmarxris(&mut self) -> SSI_RIS_DMARXRIS_W {
        SSI_RIS_DMARXRIS_W::new(self)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_dmatxris(&mut self) -> SSI_RIS_DMATXRIS_W {
        SSI_RIS_DMATXRIS_W::new(self)
    }
    #[doc = "Bit 6 - End of Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn ssi_ris_eotris(&mut self) -> SSI_RIS_EOTRIS_W {
        SSI_RIS_EOTRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
