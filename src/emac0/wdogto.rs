#[doc = "Register `WDOGTO` reader"]
pub struct R(crate::R<WDOGTO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGTO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGTO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGTO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGTO` writer"]
pub struct W(crate::W<WDOGTO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGTO_SPEC>;
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
impl From<crate::W<WDOGTO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGTO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_WDOGTO_WTO` reader - Watchdog Timeout"]
pub type EMAC_WDOGTO_WTO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_WDOGTO_WTO` writer - Watchdog Timeout"]
pub type EMAC_WDOGTO_WTO_W<'a> = crate::FieldWriter<'a, u32, WDOGTO_SPEC, u16, u16, 14, 0>;
#[doc = "Field `EMAC_WDOGTO_PWE` reader - Programmable Watchdog Enable"]
pub type EMAC_WDOGTO_PWE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_WDOGTO_PWE` writer - Programmable Watchdog Enable"]
pub type EMAC_WDOGTO_PWE_W<'a> = crate::BitWriter<'a, u32, WDOGTO_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_wdogto_wto(&self) -> EMAC_WDOGTO_WTO_R {
        EMAC_WDOGTO_WTO_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn emac_wdogto_pwe(&self) -> EMAC_WDOGTO_PWE_R {
        EMAC_WDOGTO_PWE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn emac_wdogto_wto(&mut self) -> EMAC_WDOGTO_WTO_W {
        EMAC_WDOGTO_WTO_W::new(self)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn emac_wdogto_pwe(&mut self) -> EMAC_WDOGTO_PWE_W {
        EMAC_WDOGTO_PWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Watchdog Timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogto](index.html) module"]
pub struct WDOGTO_SPEC;
impl crate::RegisterSpec for WDOGTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogto::R](R) reader structure"]
impl crate::Readable for WDOGTO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogto::W](W) writer structure"]
impl crate::Writable for WDOGTO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGTO to value 0"]
impl crate::Resettable for WDOGTO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
