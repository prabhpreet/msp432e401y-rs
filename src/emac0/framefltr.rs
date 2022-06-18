#[doc = "Register `FRAMEFLTR` reader"]
pub struct R(crate::R<FRAMEFLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEFLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEFLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEFLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEFLTR` writer"]
pub struct W(crate::W<FRAMEFLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEFLTR_SPEC>;
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
impl From<crate::W<FRAMEFLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEFLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_FRAMEFLTR_PR` reader - Promiscuous Mode"]
pub type EMAC_FRAMEFLTR_PR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_PR` writer - Promiscuous Mode"]
pub type EMAC_FRAMEFLTR_PR_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 0>;
#[doc = "Field `EMAC_FRAMEFLTR_HUC` reader - Hash Unicast"]
pub type EMAC_FRAMEFLTR_HUC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_HUC` writer - Hash Unicast"]
pub type EMAC_FRAMEFLTR_HUC_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 1>;
#[doc = "Field `EMAC_FRAMEFLTR_HMC` reader - Hash Multicast"]
pub type EMAC_FRAMEFLTR_HMC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_HMC` writer - Hash Multicast"]
pub type EMAC_FRAMEFLTR_HMC_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 2>;
#[doc = "Field `EMAC_FRAMEFLTR_DAIF` reader - Destination Address (DA) Inverse Filtering"]
pub type EMAC_FRAMEFLTR_DAIF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_DAIF` writer - Destination Address (DA) Inverse Filtering"]
pub type EMAC_FRAMEFLTR_DAIF_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 3>;
#[doc = "Field `EMAC_FRAMEFLTR_PM` reader - Pass All Multicast"]
pub type EMAC_FRAMEFLTR_PM_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_PM` writer - Pass All Multicast"]
pub type EMAC_FRAMEFLTR_PM_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 4>;
#[doc = "Field `EMAC_FRAMEFLTR_DBF` reader - Disable Broadcast Frames"]
pub type EMAC_FRAMEFLTR_DBF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_DBF` writer - Disable Broadcast Frames"]
pub type EMAC_FRAMEFLTR_DBF_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 5>;
#[doc = "Pass Control Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_FRAMEFLTR_PCF_A {
    #[doc = "0: The MAC filters all control frames from reaching application"]
    EMAC_FRAMEFLTR_PCF_ALL = 0,
    #[doc = "1: MAC forwards all control frames except PAUSE control frames to application even if they fail the address filter"]
    EMAC_FRAMEFLTR_PCF_PAUSE = 1,
    #[doc = "2: MAC forwards all control frames to application even if they fail the address Filter"]
    EMAC_FRAMEFLTR_PCF_NONE = 2,
    #[doc = "3: MAC forwards control frames that pass the address Filter"]
    EMAC_FRAMEFLTR_PCF_ADDR = 3,
}
impl From<EMAC_FRAMEFLTR_PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_FRAMEFLTR_PCF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_FRAMEFLTR_PCF` reader - Pass Control Frames"]
pub type EMAC_FRAMEFLTR_PCF_R = crate::FieldReader<u8, EMAC_FRAMEFLTR_PCF_A>;
impl EMAC_FRAMEFLTR_PCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_FRAMEFLTR_PCF_A {
        match self.bits {
            0 => EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ALL,
            1 => EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_PAUSE,
            2 => EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_NONE,
            3 => EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ADDR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_ALL`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_all(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ALL
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_PAUSE`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_pause(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_PAUSE
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_NONE`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_none(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_FRAMEFLTR_PCF_ADDR`"]
    #[inline(always)]
    pub fn is_emac_framefltr_pcf_addr(&self) -> bool {
        *self == EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ADDR
    }
}
#[doc = "Field `EMAC_FRAMEFLTR_PCF` writer - Pass Control Frames"]
pub type EMAC_FRAMEFLTR_PCF_W<'a> =
    crate::FieldWriterSafe<'a, u32, FRAMEFLTR_SPEC, u8, EMAC_FRAMEFLTR_PCF_A, 2, 6>;
impl<'a> EMAC_FRAMEFLTR_PCF_W<'a> {
    #[doc = "The MAC filters all control frames from reaching application"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_all(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ALL)
    }
    #[doc = "MAC forwards all control frames except PAUSE control frames to application even if they fail the address filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_pause(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_PAUSE)
    }
    #[doc = "MAC forwards all control frames to application even if they fail the address Filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_none(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_NONE)
    }
    #[doc = "MAC forwards control frames that pass the address Filter"]
    #[inline(always)]
    pub fn emac_framefltr_pcf_addr(self) -> &'a mut W {
        self.variant(EMAC_FRAMEFLTR_PCF_A::EMAC_FRAMEFLTR_PCF_ADDR)
    }
}
#[doc = "Field `EMAC_FRAMEFLTR_SAIF` reader - Source Address (SA) Inverse Filtering"]
pub type EMAC_FRAMEFLTR_SAIF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_SAIF` writer - Source Address (SA) Inverse Filtering"]
pub type EMAC_FRAMEFLTR_SAIF_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 8>;
#[doc = "Field `EMAC_FRAMEFLTR_SAF` reader - Source Address Filter Enable"]
pub type EMAC_FRAMEFLTR_SAF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_SAF` writer - Source Address Filter Enable"]
pub type EMAC_FRAMEFLTR_SAF_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 9>;
#[doc = "Field `EMAC_FRAMEFLTR_HPF` reader - Hash or Perfect Filter"]
pub type EMAC_FRAMEFLTR_HPF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_HPF` writer - Hash or Perfect Filter"]
pub type EMAC_FRAMEFLTR_HPF_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 10>;
#[doc = "Field `EMAC_FRAMEFLTR_VTFE` reader - VLAN Tag Filter Enable"]
pub type EMAC_FRAMEFLTR_VTFE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_VTFE` writer - VLAN Tag Filter Enable"]
pub type EMAC_FRAMEFLTR_VTFE_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 16>;
#[doc = "Field `EMAC_FRAMEFLTR_RA` reader - Receive All"]
pub type EMAC_FRAMEFLTR_RA_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_FRAMEFLTR_RA` writer - Receive All"]
pub type EMAC_FRAMEFLTR_RA_W<'a> = crate::BitWriter<'a, u32, FRAMEFLTR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn emac_framefltr_pr(&self) -> EMAC_FRAMEFLTR_PR_R {
        EMAC_FRAMEFLTR_PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn emac_framefltr_huc(&self) -> EMAC_FRAMEFLTR_HUC_R {
        EMAC_FRAMEFLTR_HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_hmc(&self) -> EMAC_FRAMEFLTR_HMC_R {
        EMAC_FRAMEFLTR_HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Address (DA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_daif(&self) -> EMAC_FRAMEFLTR_DAIF_R {
        EMAC_FRAMEFLTR_DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_pm(&self) -> EMAC_FRAMEFLTR_PM_R {
        EMAC_FRAMEFLTR_PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn emac_framefltr_dbf(&self) -> EMAC_FRAMEFLTR_DBF_R {
        EMAC_FRAMEFLTR_DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn emac_framefltr_pcf(&self) -> EMAC_FRAMEFLTR_PCF_R {
        EMAC_FRAMEFLTR_PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Address (SA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_saif(&self) -> EMAC_FRAMEFLTR_SAIF_R {
        EMAC_FRAMEFLTR_SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_saf(&self) -> EMAC_FRAMEFLTR_SAF_R {
        EMAC_FRAMEFLTR_SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn emac_framefltr_hpf(&self) -> EMAC_FRAMEFLTR_HPF_R {
        EMAC_FRAMEFLTR_HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_vtfe(&self) -> EMAC_FRAMEFLTR_VTFE_R {
        EMAC_FRAMEFLTR_VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn emac_framefltr_ra(&self) -> EMAC_FRAMEFLTR_RA_R {
        EMAC_FRAMEFLTR_RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn emac_framefltr_pr(&mut self) -> EMAC_FRAMEFLTR_PR_W {
        EMAC_FRAMEFLTR_PR_W::new(self)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn emac_framefltr_huc(&mut self) -> EMAC_FRAMEFLTR_HUC_W {
        EMAC_FRAMEFLTR_HUC_W::new(self)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_hmc(&mut self) -> EMAC_FRAMEFLTR_HMC_W {
        EMAC_FRAMEFLTR_HMC_W::new(self)
    }
    #[doc = "Bit 3 - Destination Address (DA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_daif(&mut self) -> EMAC_FRAMEFLTR_DAIF_W {
        EMAC_FRAMEFLTR_DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn emac_framefltr_pm(&mut self) -> EMAC_FRAMEFLTR_PM_W {
        EMAC_FRAMEFLTR_PM_W::new(self)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn emac_framefltr_dbf(&mut self) -> EMAC_FRAMEFLTR_DBF_W {
        EMAC_FRAMEFLTR_DBF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn emac_framefltr_pcf(&mut self) -> EMAC_FRAMEFLTR_PCF_W {
        EMAC_FRAMEFLTR_PCF_W::new(self)
    }
    #[doc = "Bit 8 - Source Address (SA) Inverse Filtering"]
    #[inline(always)]
    pub fn emac_framefltr_saif(&mut self) -> EMAC_FRAMEFLTR_SAIF_W {
        EMAC_FRAMEFLTR_SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_saf(&mut self) -> EMAC_FRAMEFLTR_SAF_W {
        EMAC_FRAMEFLTR_SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn emac_framefltr_hpf(&mut self) -> EMAC_FRAMEFLTR_HPF_W {
        EMAC_FRAMEFLTR_HPF_W::new(self)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn emac_framefltr_vtfe(&mut self) -> EMAC_FRAMEFLTR_VTFE_W {
        EMAC_FRAMEFLTR_VTFE_W::new(self)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn emac_framefltr_ra(&mut self) -> EMAC_FRAMEFLTR_RA_W {
        EMAC_FRAMEFLTR_RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framefltr](index.html) module"]
pub struct FRAMEFLTR_SPEC;
impl crate::RegisterSpec for FRAMEFLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framefltr::R](R) reader structure"]
impl crate::Readable for FRAMEFLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framefltr::W](W) writer structure"]
impl crate::Writable for FRAMEFLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMEFLTR to value 0"]
impl crate::Resettable for FRAMEFLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
