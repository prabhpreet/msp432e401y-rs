#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI_ICR_RORIC` writer - SSI Receive Overrun Interrupt Clear"]
pub type SSI_ICR_RORIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 0>;
#[doc = "Field `SSI_ICR_RTIC` writer - SSI Receive Time-Out Interrupt Clear"]
pub type SSI_ICR_RTIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 1>;
#[doc = "Field `SSI_ICR_DMARXIC` writer - SSI Receive DMA Interrupt Clear"]
pub type SSI_ICR_DMARXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 4>;
#[doc = "Field `SSI_ICR_DMATXIC` writer - SSI Transmit DMA Interrupt Clear"]
pub type SSI_ICR_DMATXIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 5>;
#[doc = "Field `SSI_ICR_EOTIC` writer - End of Transmit Interrupt Clear"]
pub type SSI_ICR_EOTIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 6>;
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_roric(&mut self) -> SSI_ICR_RORIC_W {
        SSI_ICR_RORIC_W::new(self)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_rtic(&mut self) -> SSI_ICR_RTIC_W {
        SSI_ICR_RTIC_W::new(self)
    }
    #[doc = "Bit 4 - SSI Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_dmarxic(&mut self) -> SSI_ICR_DMARXIC_W {
        SSI_ICR_DMARXIC_W::new(self)
    }
    #[doc = "Bit 5 - SSI Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_dmatxic(&mut self) -> SSI_ICR_DMATXIC_W {
        SSI_ICR_DMATXIC_W::new(self)
    }
    #[doc = "Bit 6 - End of Transmit Interrupt Clear"]
    #[inline(always)]
    pub fn ssi_icr_eotic(&mut self) -> SSI_ICR_EOTIC_W {
        SSI_ICR_EOTIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
