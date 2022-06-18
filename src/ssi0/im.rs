#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_IM_RORIM` reader - SSI Receive Overrun Interrupt Mask"]
pub type SSI_IM_RORIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_RORIM` writer - SSI Receive Overrun Interrupt Mask"]
pub type SSI_IM_RORIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `SSI_IM_RTIM` reader - SSI Receive Time-Out Interrupt Mask"]
pub type SSI_IM_RTIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_RTIM` writer - SSI Receive Time-Out Interrupt Mask"]
pub type SSI_IM_RTIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 1>;
#[doc = "Field `SSI_IM_RXIM` reader - SSI Receive FIFO Interrupt Mask"]
pub type SSI_IM_RXIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_RXIM` writer - SSI Receive FIFO Interrupt Mask"]
pub type SSI_IM_RXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `SSI_IM_TXIM` reader - SSI Transmit FIFO Interrupt Mask"]
pub type SSI_IM_TXIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_TXIM` writer - SSI Transmit FIFO Interrupt Mask"]
pub type SSI_IM_TXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `SSI_IM_DMARXIM` reader - SSI Receive DMA Interrupt Mask"]
pub type SSI_IM_DMARXIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_DMARXIM` writer - SSI Receive DMA Interrupt Mask"]
pub type SSI_IM_DMARXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 4>;
#[doc = "Field `SSI_IM_DMATXIM` reader - SSI Transmit DMA Interrupt Mask"]
pub type SSI_IM_DMATXIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_DMATXIM` writer - SSI Transmit DMA Interrupt Mask"]
pub type SSI_IM_DMATXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 5>;
#[doc = "Field `SSI_IM_EOTIM` reader - End of Transmit Interrupt Mask"]
pub type SSI_IM_EOTIM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_IM_EOTIM` writer - End of Transmit Interrupt Mask"]
pub type SSI_IM_EOTIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&self) -> SSI_IM_RORIM_R {
        SSI_IM_RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&self) -> SSI_IM_RTIM_R {
        SSI_IM_RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&self) -> SSI_IM_RXIM_R {
        SSI_IM_RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&self) -> SSI_IM_TXIM_R {
        SSI_IM_TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmarxim(&self) -> SSI_IM_DMARXIM_R {
        SSI_IM_DMARXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmatxim(&self) -> SSI_IM_DMATXIM_R {
        SSI_IM_DMATXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_eotim(&self) -> SSI_IM_EOTIM_R {
        SSI_IM_EOTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&mut self) -> SSI_IM_RORIM_W {
        SSI_IM_RORIM_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&mut self) -> SSI_IM_RTIM_W {
        SSI_IM_RTIM_W::new(self)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&mut self) -> SSI_IM_RXIM_W {
        SSI_IM_RXIM_W::new(self)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&mut self) -> SSI_IM_TXIM_W {
        SSI_IM_TXIM_W::new(self)
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmarxim(&mut self) -> SSI_IM_DMARXIM_W {
        SSI_IM_DMARXIM_W::new(self)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_dmatxim(&mut self) -> SSI_IM_DMATXIM_W {
        SSI_IM_DMATXIM_W::new(self)
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_eotim(&mut self) -> SSI_IM_EOTIM_W {
        SSI_IM_EOTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
