#[doc = "Register `TIMSTCTRL` reader"]
pub struct R(crate::R<TIMSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSTCTRL` writer"]
pub struct W(crate::W<TIMSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSTCTRL_SPEC>;
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
impl From<crate::W<TIMSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMSTCTRL_TSEN` reader - Timestamp Enable"]
pub type EMAC_TIMSTCTRL_TSEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSEN` writer - Timestamp Enable"]
pub type EMAC_TIMSTCTRL_TSEN_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 0>;
#[doc = "Field `EMAC_TIMSTCTRL_TSFCUPDT` reader - Timestamp Fine or Coarse Update"]
pub type EMAC_TIMSTCTRL_TSFCUPDT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSFCUPDT` writer - Timestamp Fine or Coarse Update"]
pub type EMAC_TIMSTCTRL_TSFCUPDT_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 1>;
#[doc = "Field `EMAC_TIMSTCTRL_TSINIT` reader - Timestamp Initialize"]
pub type EMAC_TIMSTCTRL_TSINIT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSINIT` writer - Timestamp Initialize"]
pub type EMAC_TIMSTCTRL_TSINIT_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 2>;
#[doc = "Field `EMAC_TIMSTCTRL_TSUPDT` reader - Timestamp Update"]
pub type EMAC_TIMSTCTRL_TSUPDT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSUPDT` writer - Timestamp Update"]
pub type EMAC_TIMSTCTRL_TSUPDT_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 3>;
#[doc = "Field `EMAC_TIMSTCTRL_INTTRIG` reader - Timestamp Interrupt Trigger Enable"]
pub type EMAC_TIMSTCTRL_INTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_INTTRIG` writer - Timestamp Interrupt Trigger Enable"]
pub type EMAC_TIMSTCTRL_INTTRIG_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 4>;
#[doc = "Field `EMAC_TIMSTCTRL_ADDREGUP` reader - Addend Register Update"]
pub type EMAC_TIMSTCTRL_ADDREGUP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_ADDREGUP` writer - Addend Register Update"]
pub type EMAC_TIMSTCTRL_ADDREGUP_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 5>;
#[doc = "Field `EMAC_TIMSTCTRL_ALLF` reader - Enable Timestamp For All Frames"]
pub type EMAC_TIMSTCTRL_ALLF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_ALLF` writer - Enable Timestamp For All Frames"]
pub type EMAC_TIMSTCTRL_ALLF_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 8>;
#[doc = "Field `EMAC_TIMSTCTRL_DGTLBIN` reader - Timestamp Digital or Binary Rollover Control"]
pub type EMAC_TIMSTCTRL_DGTLBIN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_DGTLBIN` writer - Timestamp Digital or Binary Rollover Control"]
pub type EMAC_TIMSTCTRL_DGTLBIN_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 9>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPVER2` reader - Enable PTP Packet Processing For Version 2 Format"]
pub type EMAC_TIMSTCTRL_PTPVER2_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPVER2` writer - Enable PTP Packet Processing For Version 2 Format"]
pub type EMAC_TIMSTCTRL_PTPVER2_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 10>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPETH` reader - Enable Processing of PTP Over Ethernet Frames"]
pub type EMAC_TIMSTCTRL_PTPETH_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPETH` writer - Enable Processing of PTP Over Ethernet Frames"]
pub type EMAC_TIMSTCTRL_PTPETH_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 11>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPIPV6` reader - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type EMAC_TIMSTCTRL_PTPIPV6_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPIPV6` writer - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
pub type EMAC_TIMSTCTRL_PTPIPV6_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 12>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPIPV4` reader - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type EMAC_TIMSTCTRL_PTPIPV4_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPIPV4` writer - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub type EMAC_TIMSTCTRL_PTPIPV4_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 13>;
#[doc = "Field `EMAC_TIMSTCTRL_TSEVNT` reader - Enable Timestamp Snapshot for Event Messages"]
pub type EMAC_TIMSTCTRL_TSEVNT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSEVNT` writer - Enable Timestamp Snapshot for Event Messages"]
pub type EMAC_TIMSTCTRL_TSEVNT_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 14>;
#[doc = "Field `EMAC_TIMSTCTRL_TSMAST` reader - Enable Snapshot for Messages Relevant to Master"]
pub type EMAC_TIMSTCTRL_TSMAST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_TSMAST` writer - Enable Snapshot for Messages Relevant to Master"]
pub type EMAC_TIMSTCTRL_TSMAST_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 15>;
#[doc = "Field `EMAC_TIMSTCTRL_SELPTP` reader - Select PTP packets for Taking Snapshots"]
pub type EMAC_TIMSTCTRL_SELPTP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_TIMSTCTRL_SELPTP` writer - Select PTP packets for Taking Snapshots"]
pub type EMAC_TIMSTCTRL_SELPTP_W<'a> = crate::FieldWriter<'a, u32, TIMSTCTRL_SPEC, u8, u8, 2, 16>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPFLTR` reader - Enable MAC address for PTP Frame Filtering"]
pub type EMAC_TIMSTCTRL_PTPFLTR_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTCTRL_PTPFLTR` writer - Enable MAC address for PTP Frame Filtering"]
pub type EMAC_TIMSTCTRL_PTPFLTR_W<'a> = crate::BitWriter<'a, u32, TIMSTCTRL_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_tsen(&self) -> EMAC_TIMSTCTRL_TSEN_R {
        EMAC_TIMSTCTRL_TSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsfcupdt(&self) -> EMAC_TIMSTCTRL_TSFCUPDT_R {
        EMAC_TIMSTCTRL_TSFCUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn emac_timstctrl_tsinit(&self) -> EMAC_TIMSTCTRL_TSINIT_R {
        EMAC_TIMSTCTRL_TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsupdt(&self) -> EMAC_TIMSTCTRL_TSUPDT_R {
        EMAC_TIMSTCTRL_TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_inttrig(&self) -> EMAC_TIMSTCTRL_INTTRIG_R {
        EMAC_TIMSTCTRL_INTTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn emac_timstctrl_addregup(&self) -> EMAC_TIMSTCTRL_ADDREGUP_R {
        EMAC_TIMSTCTRL_ADDREGUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_allf(&self) -> EMAC_TIMSTCTRL_ALLF_R {
        EMAC_TIMSTCTRL_ALLF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn emac_timstctrl_dgtlbin(&self) -> EMAC_TIMSTCTRL_DGTLBIN_R {
        EMAC_TIMSTCTRL_DGTLBIN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpver2(&self) -> EMAC_TIMSTCTRL_PTPVER2_R {
        EMAC_TIMSTCTRL_PTPVER2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpeth(&self) -> EMAC_TIMSTCTRL_PTPETH_R {
        EMAC_TIMSTCTRL_PTPETH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv6(&self) -> EMAC_TIMSTCTRL_PTPIPV6_R {
        EMAC_TIMSTCTRL_PTPIPV6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv4(&self) -> EMAC_TIMSTCTRL_PTPIPV4_R {
        EMAC_TIMSTCTRL_PTPIPV4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn emac_timstctrl_tsevnt(&self) -> EMAC_TIMSTCTRL_TSEVNT_R {
        EMAC_TIMSTCTRL_TSEVNT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn emac_timstctrl_tsmast(&self) -> EMAC_TIMSTCTRL_TSMAST_R {
        EMAC_TIMSTCTRL_TSMAST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn emac_timstctrl_selptp(&self) -> EMAC_TIMSTCTRL_SELPTP_R {
        EMAC_TIMSTCTRL_SELPTP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpfltr(&self) -> EMAC_TIMSTCTRL_PTPFLTR_R {
        EMAC_TIMSTCTRL_PTPFLTR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_tsen(&mut self) -> EMAC_TIMSTCTRL_TSEN_W {
        EMAC_TIMSTCTRL_TSEN_W::new(self)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsfcupdt(&mut self) -> EMAC_TIMSTCTRL_TSFCUPDT_W {
        EMAC_TIMSTCTRL_TSFCUPDT_W::new(self)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn emac_timstctrl_tsinit(&mut self) -> EMAC_TIMSTCTRL_TSINIT_W {
        EMAC_TIMSTCTRL_TSINIT_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn emac_timstctrl_tsupdt(&mut self) -> EMAC_TIMSTCTRL_TSUPDT_W {
        EMAC_TIMSTCTRL_TSUPDT_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn emac_timstctrl_inttrig(&mut self) -> EMAC_TIMSTCTRL_INTTRIG_W {
        EMAC_TIMSTCTRL_INTTRIG_W::new(self)
    }
    #[doc = "Bit 5 - Addend Register Update"]
    #[inline(always)]
    pub fn emac_timstctrl_addregup(&mut self) -> EMAC_TIMSTCTRL_ADDREGUP_W {
        EMAC_TIMSTCTRL_ADDREGUP_W::new(self)
    }
    #[doc = "Bit 8 - Enable Timestamp For All Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_allf(&mut self) -> EMAC_TIMSTCTRL_ALLF_W {
        EMAC_TIMSTCTRL_ALLF_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn emac_timstctrl_dgtlbin(&mut self) -> EMAC_TIMSTCTRL_DGTLBIN_W {
        EMAC_TIMSTCTRL_DGTLBIN_W::new(self)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing For Version 2 Format"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpver2(&mut self) -> EMAC_TIMSTCTRL_PTPVER2_W {
        EMAC_TIMSTCTRL_PTPVER2_W::new(self)
    }
    #[doc = "Bit 11 - Enable Processing of PTP Over Ethernet Frames"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpeth(&mut self) -> EMAC_TIMSTCTRL_PTPETH_W {
        EMAC_TIMSTCTRL_PTPETH_W::new(self)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv6(&mut self) -> EMAC_TIMSTCTRL_PTPIPV6_W {
        EMAC_TIMSTCTRL_PTPIPV6_W::new(self)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpipv4(&mut self) -> EMAC_TIMSTCTRL_PTPIPV4_W {
        EMAC_TIMSTCTRL_PTPIPV4_W::new(self)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn emac_timstctrl_tsevnt(&mut self) -> EMAC_TIMSTCTRL_TSEVNT_W {
        EMAC_TIMSTCTRL_TSEVNT_W::new(self)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn emac_timstctrl_tsmast(&mut self) -> EMAC_TIMSTCTRL_TSMAST_W {
        EMAC_TIMSTCTRL_TSMAST_W::new(self)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn emac_timstctrl_selptp(&mut self) -> EMAC_TIMSTCTRL_SELPTP_W {
        EMAC_TIMSTCTRL_SELPTP_W::new(self)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn emac_timstctrl_ptpfltr(&mut self) -> EMAC_TIMSTCTRL_PTPFLTR_W {
        EMAC_TIMSTCTRL_PTPFLTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Timestamp Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstctrl](index.html) module"]
pub struct TIMSTCTRL_SPEC;
impl crate::RegisterSpec for TIMSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timstctrl::R](R) reader structure"]
impl crate::Readable for TIMSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timstctrl::W](W) writer structure"]
impl crate::Writable for TIMSTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMSTCTRL to value 0"]
impl crate::Resettable for TIMSTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
