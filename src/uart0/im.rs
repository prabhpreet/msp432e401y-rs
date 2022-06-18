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
#[doc = "Field `UART_IM_RIMIM` reader - UART Ring Indicator Modem Interrupt Mask"]
pub type UART_IM_RIMIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_RIMIM` writer - UART Ring Indicator Modem Interrupt Mask"]
pub type UART_IM_RIMIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `UART_IM_CTSMIM` reader - UART Clear to Send Modem Interrupt Mask"]
pub type UART_IM_CTSMIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_CTSMIM` writer - UART Clear to Send Modem Interrupt Mask"]
pub type UART_IM_CTSMIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 1>;
#[doc = "Field `UART_IM_DCDMIM` reader - UART Data Carrier Detect Modem Interrupt Mask"]
pub type UART_IM_DCDMIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_DCDMIM` writer - UART Data Carrier Detect Modem Interrupt Mask"]
pub type UART_IM_DCDMIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `UART_IM_DSRMIM` reader - UART Data Set Ready Modem Interrupt Mask"]
pub type UART_IM_DSRMIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_DSRMIM` writer - UART Data Set Ready Modem Interrupt Mask"]
pub type UART_IM_DSRMIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `UART_IM_RXIM` reader - UART Receive Interrupt Mask"]
pub type UART_IM_RXIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_RXIM` writer - UART Receive Interrupt Mask"]
pub type UART_IM_RXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 4>;
#[doc = "Field `UART_IM_TXIM` reader - UART Transmit Interrupt Mask"]
pub type UART_IM_TXIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_TXIM` writer - UART Transmit Interrupt Mask"]
pub type UART_IM_TXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 5>;
#[doc = "Field `UART_IM_RTIM` reader - UART Receive Time-Out Interrupt Mask"]
pub type UART_IM_RTIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_RTIM` writer - UART Receive Time-Out Interrupt Mask"]
pub type UART_IM_RTIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 6>;
#[doc = "Field `UART_IM_FEIM` reader - UART Framing Error Interrupt Mask"]
pub type UART_IM_FEIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_FEIM` writer - UART Framing Error Interrupt Mask"]
pub type UART_IM_FEIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 7>;
#[doc = "Field `UART_IM_PEIM` reader - UART Parity Error Interrupt Mask"]
pub type UART_IM_PEIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_PEIM` writer - UART Parity Error Interrupt Mask"]
pub type UART_IM_PEIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 8>;
#[doc = "Field `UART_IM_BEIM` reader - UART Break Error Interrupt Mask"]
pub type UART_IM_BEIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_BEIM` writer - UART Break Error Interrupt Mask"]
pub type UART_IM_BEIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 9>;
#[doc = "Field `UART_IM_OEIM` reader - UART Overrun Error Interrupt Mask"]
pub type UART_IM_OEIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_OEIM` writer - UART Overrun Error Interrupt Mask"]
pub type UART_IM_OEIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 10>;
#[doc = "Field `UART_IM_EOTIM` reader - End of Transmission Interrupt Mask"]
pub type UART_IM_EOTIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_EOTIM` writer - End of Transmission Interrupt Mask"]
pub type UART_IM_EOTIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 11>;
#[doc = "Field `UART_IM_9BITIM` reader - 9-Bit Mode Interrupt Mask"]
pub type UART_IM_9BITIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_9BITIM` writer - 9-Bit Mode Interrupt Mask"]
pub type UART_IM_9BITIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 12>;
#[doc = "Field `UART_IM_DMARXIM` reader - Receive DMA Interrupt Mask"]
pub type UART_IM_DMARXIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_DMARXIM` writer - Receive DMA Interrupt Mask"]
pub type UART_IM_DMARXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 16>;
#[doc = "Field `UART_IM_DMATXIM` reader - Transmit DMA Interrupt Mask"]
pub type UART_IM_DMATXIM_R = crate::BitReader<bool>;
#[doc = "Field `UART_IM_DMATXIM` writer - Transmit DMA Interrupt Mask"]
pub type UART_IM_DMATXIM_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rimim(&self) -> UART_IM_RIMIM_R {
        UART_IM_RIMIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_ctsmim(&self) -> UART_IM_CTSMIM_R {
        UART_IM_CTSMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dcdmim(&self) -> UART_IM_DCDMIM_R {
        UART_IM_DCDMIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dsrmim(&self) -> UART_IM_DSRMIM_R {
        UART_IM_DSRMIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&self) -> UART_IM_RXIM_R {
        UART_IM_RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&self) -> UART_IM_TXIM_R {
        UART_IM_TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&self) -> UART_IM_RTIM_R {
        UART_IM_RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&self) -> UART_IM_FEIM_R {
        UART_IM_FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&self) -> UART_IM_PEIM_R {
        UART_IM_PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&self) -> UART_IM_BEIM_R {
        UART_IM_BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&self) -> UART_IM_OEIM_R {
        UART_IM_OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_eotim(&self) -> UART_IM_EOTIM_R {
        UART_IM_EOTIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_9bitim(&self) -> UART_IM_9BITIM_R {
        UART_IM_9BITIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmarxim(&self) -> UART_IM_DMARXIM_R {
        UART_IM_DMARXIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmatxim(&self) -> UART_IM_DMATXIM_R {
        UART_IM_DMATXIM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Ring Indicator Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rimim(&mut self) -> UART_IM_RIMIM_W {
        UART_IM_RIMIM_W::new(self)
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_ctsmim(&mut self) -> UART_IM_CTSMIM_W {
        UART_IM_CTSMIM_W::new(self)
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dcdmim(&mut self) -> UART_IM_DCDMIM_W {
        UART_IM_DCDMIM_W::new(self)
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dsrmim(&mut self) -> UART_IM_DSRMIM_W {
        UART_IM_DSRMIM_W::new(self)
    }
    #[doc = "Bit 4 - UART Receive Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rxim(&mut self) -> UART_IM_RXIM_W {
        UART_IM_RXIM_W::new(self)
    }
    #[doc = "Bit 5 - UART Transmit Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_txim(&mut self) -> UART_IM_TXIM_W {
        UART_IM_TXIM_W::new(self)
    }
    #[doc = "Bit 6 - UART Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_rtim(&mut self) -> UART_IM_RTIM_W {
        UART_IM_RTIM_W::new(self)
    }
    #[doc = "Bit 7 - UART Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_feim(&mut self) -> UART_IM_FEIM_W {
        UART_IM_FEIM_W::new(self)
    }
    #[doc = "Bit 8 - UART Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_peim(&mut self) -> UART_IM_PEIM_W {
        UART_IM_PEIM_W::new(self)
    }
    #[doc = "Bit 9 - UART Break Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_beim(&mut self) -> UART_IM_BEIM_W {
        UART_IM_BEIM_W::new(self)
    }
    #[doc = "Bit 10 - UART Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_oeim(&mut self) -> UART_IM_OEIM_W {
        UART_IM_OEIM_W::new(self)
    }
    #[doc = "Bit 11 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_eotim(&mut self) -> UART_IM_EOTIM_W {
        UART_IM_EOTIM_W::new(self)
    }
    #[doc = "Bit 12 - 9-Bit Mode Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_9bitim(&mut self) -> UART_IM_9BITIM_W {
        UART_IM_9BITIM_W::new(self)
    }
    #[doc = "Bit 16 - Receive DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmarxim(&mut self) -> UART_IM_DMARXIM_W {
        UART_IM_DMARXIM_W::new(self)
    }
    #[doc = "Bit 17 - Transmit DMA Interrupt Mask"]
    #[inline(always)]
    pub fn uart_im_dmatxim(&mut self) -> UART_IM_DMATXIM_W {
        UART_IM_DMATXIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
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
