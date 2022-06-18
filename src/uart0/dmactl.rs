#[doc = "Register `DMACTL` reader"]
pub struct R(crate::R<DMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTL` writer"]
pub struct W(crate::W<DMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTL_SPEC>;
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
impl From<crate::W<DMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_DMACTL_RXDMAE` reader - Receive DMA Enable"]
pub type UART_DMACTL_RXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DMACTL_RXDMAE` writer - Receive DMA Enable"]
pub type UART_DMACTL_RXDMAE_W<'a> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, 0>;
#[doc = "Field `UART_DMACTL_TXDMAE` reader - Transmit DMA Enable"]
pub type UART_DMACTL_TXDMAE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DMACTL_TXDMAE` writer - Transmit DMA Enable"]
pub type UART_DMACTL_TXDMAE_W<'a> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, 1>;
#[doc = "Field `UART_DMACTL_DMAERR` reader - DMA on Error"]
pub type UART_DMACTL_DMAERR_R = crate::BitReader<bool>;
#[doc = "Field `UART_DMACTL_DMAERR` writer - DMA on Error"]
pub type UART_DMACTL_DMAERR_W<'a> = crate::BitWriter<'a, u32, DMACTL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_rxdmae(&self) -> UART_DMACTL_RXDMAE_R {
        UART_DMACTL_RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_txdmae(&self) -> UART_DMACTL_TXDMAE_R {
        UART_DMACTL_TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on Error"]
    #[inline(always)]
    pub fn uart_dmactl_dmaerr(&self) -> UART_DMACTL_DMAERR_R {
        UART_DMACTL_DMAERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_rxdmae(&mut self) -> UART_DMACTL_RXDMAE_W {
        UART_DMACTL_RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA Enable"]
    #[inline(always)]
    pub fn uart_dmactl_txdmae(&mut self) -> UART_DMACTL_TXDMAE_W {
        UART_DMACTL_TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - DMA on Error"]
    #[inline(always)]
    pub fn uart_dmactl_dmaerr(&mut self) -> UART_DMACTL_DMAERR_W {
        UART_DMACTL_DMAERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](index.html) module"]
pub struct DMACTL_SPEC;
impl crate::RegisterSpec for DMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactl::R](R) reader structure"]
impl crate::Readable for DMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactl::W](W) writer structure"]
impl crate::Writable for DMACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DMACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
