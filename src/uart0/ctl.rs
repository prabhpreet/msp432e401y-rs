#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_CTL_UARTEN` reader - UART Enable"]
pub type UART_CTL_UARTEN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_UARTEN` writer - UART Enable"]
pub type UART_CTL_UARTEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `UART_CTL_SIREN` reader - UART SIR Enable"]
pub type UART_CTL_SIREN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_SIREN` writer - UART SIR Enable"]
pub type UART_CTL_SIREN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `UART_CTL_SIRLP` reader - UART SIR Low-Power Mode"]
pub type UART_CTL_SIRLP_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_SIRLP` writer - UART SIR Low-Power Mode"]
pub type UART_CTL_SIRLP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 2>;
#[doc = "Field `UART_CTL_SMART` reader - ISO 7816 Smart Card Support"]
pub type UART_CTL_SMART_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_SMART` writer - ISO 7816 Smart Card Support"]
pub type UART_CTL_SMART_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `UART_CTL_EOT` reader - End of Transmission"]
pub type UART_CTL_EOT_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_EOT` writer - End of Transmission"]
pub type UART_CTL_EOT_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `UART_CTL_HSE` reader - High-Speed Enable"]
pub type UART_CTL_HSE_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_HSE` writer - High-Speed Enable"]
pub type UART_CTL_HSE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 5>;
#[doc = "Field `UART_CTL_LBE` reader - UART Loop Back Enable"]
pub type UART_CTL_LBE_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_LBE` writer - UART Loop Back Enable"]
pub type UART_CTL_LBE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
#[doc = "Field `UART_CTL_TXE` reader - UART Transmit Enable"]
pub type UART_CTL_TXE_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_TXE` writer - UART Transmit Enable"]
pub type UART_CTL_TXE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 8>;
#[doc = "Field `UART_CTL_RXE` reader - UART Receive Enable"]
pub type UART_CTL_RXE_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_RXE` writer - UART Receive Enable"]
pub type UART_CTL_RXE_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 9>;
#[doc = "Field `UART_CTL_DTR` reader - Data Terminal Ready"]
pub type UART_CTL_DTR_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_DTR` writer - Data Terminal Ready"]
pub type UART_CTL_DTR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 10>;
#[doc = "Field `UART_CTL_RTS` reader - Request to Send"]
pub type UART_CTL_RTS_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_RTS` writer - Request to Send"]
pub type UART_CTL_RTS_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 11>;
#[doc = "Field `UART_CTL_RTSEN` reader - Enable Request to Send"]
pub type UART_CTL_RTSEN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_RTSEN` writer - Enable Request to Send"]
pub type UART_CTL_RTSEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 14>;
#[doc = "Field `UART_CTL_CTSEN` reader - Enable Clear To Send"]
pub type UART_CTL_CTSEN_R = crate::BitReader<bool>;
#[doc = "Field `UART_CTL_CTSEN` writer - Enable Clear To Send"]
pub type UART_CTL_CTSEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&self) -> UART_CTL_UARTEN_R {
        UART_CTL_UARTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&self) -> UART_CTL_SIREN_R {
        UART_CTL_SIREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&self) -> UART_CTL_SIRLP_R {
        UART_CTL_SIRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    pub fn uart_ctl_smart(&self) -> UART_CTL_SMART_R {
        UART_CTL_SMART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn uart_ctl_eot(&self) -> UART_CTL_EOT_R {
        UART_CTL_EOT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    pub fn uart_ctl_hse(&self) -> UART_CTL_HSE_R {
        UART_CTL_HSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&self) -> UART_CTL_LBE_R {
        UART_CTL_LBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&self) -> UART_CTL_TXE_R {
        UART_CTL_TXE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&self) -> UART_CTL_RXE_R {
        UART_CTL_RXE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Terminal Ready"]
    #[inline(always)]
    pub fn uart_ctl_dtr(&self) -> UART_CTL_DTR_R {
        UART_CTL_DTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rts(&self) -> UART_CTL_RTS_R {
        UART_CTL_RTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rtsen(&self) -> UART_CTL_RTSEN_R {
        UART_CTL_RTSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    pub fn uart_ctl_ctsen(&self) -> UART_CTL_CTSEN_R {
        UART_CTL_CTSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&mut self) -> UART_CTL_UARTEN_W {
        UART_CTL_UARTEN_W::new(self)
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&mut self) -> UART_CTL_SIREN_W {
        UART_CTL_SIREN_W::new(self)
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&mut self) -> UART_CTL_SIRLP_W {
        UART_CTL_SIRLP_W::new(self)
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    pub fn uart_ctl_smart(&mut self) -> UART_CTL_SMART_W {
        UART_CTL_SMART_W::new(self)
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn uart_ctl_eot(&mut self) -> UART_CTL_EOT_W {
        UART_CTL_EOT_W::new(self)
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    pub fn uart_ctl_hse(&mut self) -> UART_CTL_HSE_W {
        UART_CTL_HSE_W::new(self)
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&mut self) -> UART_CTL_LBE_W {
        UART_CTL_LBE_W::new(self)
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&mut self) -> UART_CTL_TXE_W {
        UART_CTL_TXE_W::new(self)
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&mut self) -> UART_CTL_RXE_W {
        UART_CTL_RXE_W::new(self)
    }
    #[doc = "Bit 10 - Data Terminal Ready"]
    #[inline(always)]
    pub fn uart_ctl_dtr(&mut self) -> UART_CTL_DTR_W {
        UART_CTL_DTR_W::new(self)
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rts(&mut self) -> UART_CTL_RTS_W {
        UART_CTL_RTS_W::new(self)
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rtsen(&mut self) -> UART_CTL_RTSEN_W {
        UART_CTL_RTSEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    pub fn uart_ctl_ctsen(&mut self) -> UART_CTL_CTSEN_W {
        UART_CTL_CTSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
