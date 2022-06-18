#[doc = "Register `DMAIM` reader"]
pub struct R(crate::R<DMAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAIM` writer"]
pub struct W(crate::W<DMAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIM_SPEC>;
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
impl From<crate::W<DMAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_DMAIM_TIE` reader - Transmit Interrupt Enable"]
pub type EMAC_DMAIM_TIE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_TIE` writer - Transmit Interrupt Enable"]
pub type EMAC_DMAIM_TIE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 0>;
#[doc = "Field `EMAC_DMAIM_TSE` reader - Transmit Stopped Enable"]
pub type EMAC_DMAIM_TSE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_TSE` writer - Transmit Stopped Enable"]
pub type EMAC_DMAIM_TSE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 1>;
#[doc = "Field `EMAC_DMAIM_TUE` reader - Transmit Buffer Unvailable Enable"]
pub type EMAC_DMAIM_TUE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_TUE` writer - Transmit Buffer Unvailable Enable"]
pub type EMAC_DMAIM_TUE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 2>;
#[doc = "Field `EMAC_DMAIM_TJE` reader - Transmit Jabber Timeout Enable"]
pub type EMAC_DMAIM_TJE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_TJE` writer - Transmit Jabber Timeout Enable"]
pub type EMAC_DMAIM_TJE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 3>;
#[doc = "Field `EMAC_DMAIM_OVE` reader - Overflow Interrupt Enable"]
pub type EMAC_DMAIM_OVE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_OVE` writer - Overflow Interrupt Enable"]
pub type EMAC_DMAIM_OVE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 4>;
#[doc = "Field `EMAC_DMAIM_UNE` reader - Underflow Interrupt Enable"]
pub type EMAC_DMAIM_UNE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_UNE` writer - Underflow Interrupt Enable"]
pub type EMAC_DMAIM_UNE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 5>;
#[doc = "Field `EMAC_DMAIM_RIE` reader - Receive Interrupt Enable"]
pub type EMAC_DMAIM_RIE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_RIE` writer - Receive Interrupt Enable"]
pub type EMAC_DMAIM_RIE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 6>;
#[doc = "Field `EMAC_DMAIM_RUE` reader - Receive Buffer Unavailable Enable"]
pub type EMAC_DMAIM_RUE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_RUE` writer - Receive Buffer Unavailable Enable"]
pub type EMAC_DMAIM_RUE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 7>;
#[doc = "Field `EMAC_DMAIM_RSE` reader - Receive Stopped Enable"]
pub type EMAC_DMAIM_RSE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_RSE` writer - Receive Stopped Enable"]
pub type EMAC_DMAIM_RSE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 8>;
#[doc = "Field `EMAC_DMAIM_RWE` reader - Receive Watchdog Timeout Enable"]
pub type EMAC_DMAIM_RWE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_RWE` writer - Receive Watchdog Timeout Enable"]
pub type EMAC_DMAIM_RWE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 9>;
#[doc = "Field `EMAC_DMAIM_ETE` reader - Early Transmit Interrupt Enable"]
pub type EMAC_DMAIM_ETE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_ETE` writer - Early Transmit Interrupt Enable"]
pub type EMAC_DMAIM_ETE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 10>;
#[doc = "Field `EMAC_DMAIM_FBE` reader - Fatal Bus Error Enable"]
pub type EMAC_DMAIM_FBE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_FBE` writer - Fatal Bus Error Enable"]
pub type EMAC_DMAIM_FBE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 13>;
#[doc = "Field `EMAC_DMAIM_ERE` reader - Early Receive Interrupt Enable"]
pub type EMAC_DMAIM_ERE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_ERE` writer - Early Receive Interrupt Enable"]
pub type EMAC_DMAIM_ERE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 14>;
#[doc = "Field `EMAC_DMAIM_AIE` reader - Abnormal Interrupt Summary Enable"]
pub type EMAC_DMAIM_AIE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_AIE` writer - Abnormal Interrupt Summary Enable"]
pub type EMAC_DMAIM_AIE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 15>;
#[doc = "Field `EMAC_DMAIM_NIE` reader - Normal Interrupt Summary Enable"]
pub type EMAC_DMAIM_NIE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_DMAIM_NIE` writer - Normal Interrupt Summary Enable"]
pub type EMAC_DMAIM_NIE_W<'a> = crate::BitWriter<'a, u32, DMAIM_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tie(&self) -> EMAC_DMAIM_TIE_R {
        EMAC_DMAIM_TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tse(&self) -> EMAC_DMAIM_TSE_R {
        EMAC_DMAIM_TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tue(&self) -> EMAC_DMAIM_TUE_R {
        EMAC_DMAIM_TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tje(&self) -> EMAC_DMAIM_TJE_R {
        EMAC_DMAIM_TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ove(&self) -> EMAC_DMAIM_OVE_R {
        EMAC_DMAIM_OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_une(&self) -> EMAC_DMAIM_UNE_R {
        EMAC_DMAIM_UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rie(&self) -> EMAC_DMAIM_RIE_R {
        EMAC_DMAIM_RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rue(&self) -> EMAC_DMAIM_RUE_R {
        EMAC_DMAIM_RUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rse(&self) -> EMAC_DMAIM_RSE_R {
        EMAC_DMAIM_RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rwe(&self) -> EMAC_DMAIM_RWE_R {
        EMAC_DMAIM_RWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ete(&self) -> EMAC_DMAIM_ETE_R {
        EMAC_DMAIM_ETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn emac_dmaim_fbe(&self) -> EMAC_DMAIM_FBE_R {
        EMAC_DMAIM_FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ere(&self) -> EMAC_DMAIM_ERE_R {
        EMAC_DMAIM_ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn emac_dmaim_aie(&self) -> EMAC_DMAIM_AIE_R {
        EMAC_DMAIM_AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn emac_dmaim_nie(&self) -> EMAC_DMAIM_NIE_R {
        EMAC_DMAIM_NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tie(&mut self) -> EMAC_DMAIM_TIE_W {
        EMAC_DMAIM_TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tse(&mut self) -> EMAC_DMAIM_TSE_W {
        EMAC_DMAIM_TSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tue(&mut self) -> EMAC_DMAIM_TUE_W {
        EMAC_DMAIM_TUE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn emac_dmaim_tje(&mut self) -> EMAC_DMAIM_TJE_W {
        EMAC_DMAIM_TJE_W::new(self)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ove(&mut self) -> EMAC_DMAIM_OVE_W {
        EMAC_DMAIM_OVE_W::new(self)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_une(&mut self) -> EMAC_DMAIM_UNE_W {
        EMAC_DMAIM_UNE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rie(&mut self) -> EMAC_DMAIM_RIE_W {
        EMAC_DMAIM_RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rue(&mut self) -> EMAC_DMAIM_RUE_W {
        EMAC_DMAIM_RUE_W::new(self)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rse(&mut self) -> EMAC_DMAIM_RSE_W {
        EMAC_DMAIM_RSE_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn emac_dmaim_rwe(&mut self) -> EMAC_DMAIM_RWE_W {
        EMAC_DMAIM_RWE_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ete(&mut self) -> EMAC_DMAIM_ETE_W {
        EMAC_DMAIM_ETE_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn emac_dmaim_fbe(&mut self) -> EMAC_DMAIM_FBE_W {
        EMAC_DMAIM_FBE_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn emac_dmaim_ere(&mut self) -> EMAC_DMAIM_ERE_W {
        EMAC_DMAIM_ERE_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn emac_dmaim_aie(&mut self) -> EMAC_DMAIM_AIE_W {
        EMAC_DMAIM_AIE_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn emac_dmaim_nie(&mut self) -> EMAC_DMAIM_NIE_W {
        EMAC_DMAIM_NIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC DMA Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaim](index.html) module"]
pub struct DMAIM_SPEC;
impl crate::RegisterSpec for DMAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaim::R](R) reader structure"]
impl crate::Readable for DMAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaim::W](W) writer structure"]
impl crate::Writable for DMAIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAIM to value 0"]
impl crate::Resettable for DMAIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
