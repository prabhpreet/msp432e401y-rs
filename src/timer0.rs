#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - GPTM Timer A Mode"]
    pub tamr: crate::Reg<tamr::TAMR_SPEC>,
    #[doc = "0x08 - GPTM Timer B Mode"]
    pub tbmr: crate::Reg<tbmr::TBMR_SPEC>,
    #[doc = "0x0c - GPTM Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x10 - GPTM Synchronize"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPTM Interrupt Mask"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - GPTM Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x20 - GPTM Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x24 - GPTM Interrupt Clear"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x28 - GPTM Timer A Interval Load"]
    pub tailr: crate::Reg<tailr::TAILR_SPEC>,
    #[doc = "0x2c - GPTM Timer B Interval Load"]
    pub tbilr: crate::Reg<tbilr::TBILR_SPEC>,
    #[doc = "0x30 - GPTM Timer A Match"]
    pub tamatchr: crate::Reg<tamatchr::TAMATCHR_SPEC>,
    #[doc = "0x34 - GPTM Timer B Match"]
    pub tbmatchr: crate::Reg<tbmatchr::TBMATCHR_SPEC>,
    #[doc = "0x38 - GPTM Timer A Prescale"]
    pub tapr: crate::Reg<tapr::TAPR_SPEC>,
    #[doc = "0x3c - GPTM Timer B Prescale"]
    pub tbpr: crate::Reg<tbpr::TBPR_SPEC>,
    #[doc = "0x40 - GPTM TimerA Prescale Match"]
    pub tapmr: crate::Reg<tapmr::TAPMR_SPEC>,
    #[doc = "0x44 - GPTM TimerB Prescale Match"]
    pub tbpmr: crate::Reg<tbpmr::TBPMR_SPEC>,
    #[doc = "0x48 - GPTM Timer A"]
    pub tar: crate::Reg<tar::TAR_SPEC>,
    #[doc = "0x4c - GPTM Timer B"]
    pub tbr: crate::Reg<tbr::TBR_SPEC>,
    #[doc = "0x50 - GPTM Timer A Value"]
    pub tav: crate::Reg<tav::TAV_SPEC>,
    #[doc = "0x54 - GPTM Timer B Value"]
    pub tbv: crate::Reg<tbv::TBV_SPEC>,
    #[doc = "0x58 - GPTM RTC Predivide"]
    pub rtcpd: crate::Reg<rtcpd::RTCPD_SPEC>,
    #[doc = "0x5c - GPTM Timer A Prescale Snapshot"]
    pub taps: crate::Reg<taps::TAPS_SPEC>,
    #[doc = "0x60 - GPTM Timer B Prescale Snapshot"]
    pub tbps: crate::Reg<tbps::TBPS_SPEC>,
    #[doc = "0x64 - GPTM Timer A Prescale Value"]
    pub tapv: crate::Reg<tapv::TAPV_SPEC>,
    #[doc = "0x68 - GPTM Timer B Prescale Value"]
    pub tbpv: crate::Reg<tbpv::TBPV_SPEC>,
    #[doc = "0x6c - GPTM DMA Event"]
    pub dmaev: crate::Reg<dmaev::DMAEV_SPEC>,
    #[doc = "0x70 - GPTM ADC Event"]
    pub adcev: crate::Reg<adcev::ADCEV_SPEC>,
    _reserved28: [u8; 0x0f4c],
    #[doc = "0xfc0 - GPTM Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "GPTM Configuration"]
pub mod cfg;
#[doc = "TAMR register accessor: an alias for `Reg<TAMR_SPEC>`"]
pub type TAMR = crate::Reg<tamr::TAMR_SPEC>;
#[doc = "GPTM Timer A Mode"]
pub mod tamr;
#[doc = "TBMR register accessor: an alias for `Reg<TBMR_SPEC>`"]
pub type TBMR = crate::Reg<tbmr::TBMR_SPEC>;
#[doc = "GPTM Timer B Mode"]
pub mod tbmr;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "GPTM Control"]
pub mod ctl;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "GPTM Synchronize"]
pub mod sync;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "GPTM Interrupt Mask"]
pub mod imr;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "GPTM Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "GPTM Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "GPTM Interrupt Clear"]
pub mod icr;
#[doc = "TAILR register accessor: an alias for `Reg<TAILR_SPEC>`"]
pub type TAILR = crate::Reg<tailr::TAILR_SPEC>;
#[doc = "GPTM Timer A Interval Load"]
pub mod tailr;
#[doc = "TBILR register accessor: an alias for `Reg<TBILR_SPEC>`"]
pub type TBILR = crate::Reg<tbilr::TBILR_SPEC>;
#[doc = "GPTM Timer B Interval Load"]
pub mod tbilr;
#[doc = "TAMATCHR register accessor: an alias for `Reg<TAMATCHR_SPEC>`"]
pub type TAMATCHR = crate::Reg<tamatchr::TAMATCHR_SPEC>;
#[doc = "GPTM Timer A Match"]
pub mod tamatchr;
#[doc = "TBMATCHR register accessor: an alias for `Reg<TBMATCHR_SPEC>`"]
pub type TBMATCHR = crate::Reg<tbmatchr::TBMATCHR_SPEC>;
#[doc = "GPTM Timer B Match"]
pub mod tbmatchr;
#[doc = "TAPR register accessor: an alias for `Reg<TAPR_SPEC>`"]
pub type TAPR = crate::Reg<tapr::TAPR_SPEC>;
#[doc = "GPTM Timer A Prescale"]
pub mod tapr;
#[doc = "TBPR register accessor: an alias for `Reg<TBPR_SPEC>`"]
pub type TBPR = crate::Reg<tbpr::TBPR_SPEC>;
#[doc = "GPTM Timer B Prescale"]
pub mod tbpr;
#[doc = "TAPMR register accessor: an alias for `Reg<TAPMR_SPEC>`"]
pub type TAPMR = crate::Reg<tapmr::TAPMR_SPEC>;
#[doc = "GPTM TimerA Prescale Match"]
pub mod tapmr;
#[doc = "TBPMR register accessor: an alias for `Reg<TBPMR_SPEC>`"]
pub type TBPMR = crate::Reg<tbpmr::TBPMR_SPEC>;
#[doc = "GPTM TimerB Prescale Match"]
pub mod tbpmr;
#[doc = "TAR register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "GPTM Timer A"]
pub mod tar;
#[doc = "TBR register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "GPTM Timer B"]
pub mod tbr;
#[doc = "TAV register accessor: an alias for `Reg<TAV_SPEC>`"]
pub type TAV = crate::Reg<tav::TAV_SPEC>;
#[doc = "GPTM Timer A Value"]
pub mod tav;
#[doc = "TBV register accessor: an alias for `Reg<TBV_SPEC>`"]
pub type TBV = crate::Reg<tbv::TBV_SPEC>;
#[doc = "GPTM Timer B Value"]
pub mod tbv;
#[doc = "RTCPD register accessor: an alias for `Reg<RTCPD_SPEC>`"]
pub type RTCPD = crate::Reg<rtcpd::RTCPD_SPEC>;
#[doc = "GPTM RTC Predivide"]
pub mod rtcpd;
#[doc = "TAPS register accessor: an alias for `Reg<TAPS_SPEC>`"]
pub type TAPS = crate::Reg<taps::TAPS_SPEC>;
#[doc = "GPTM Timer A Prescale Snapshot"]
pub mod taps;
#[doc = "TBPS register accessor: an alias for `Reg<TBPS_SPEC>`"]
pub type TBPS = crate::Reg<tbps::TBPS_SPEC>;
#[doc = "GPTM Timer B Prescale Snapshot"]
pub mod tbps;
#[doc = "TAPV register accessor: an alias for `Reg<TAPV_SPEC>`"]
pub type TAPV = crate::Reg<tapv::TAPV_SPEC>;
#[doc = "GPTM Timer A Prescale Value"]
pub mod tapv;
#[doc = "TBPV register accessor: an alias for `Reg<TBPV_SPEC>`"]
pub type TBPV = crate::Reg<tbpv::TBPV_SPEC>;
#[doc = "GPTM Timer B Prescale Value"]
pub mod tbpv;
#[doc = "DMAEV register accessor: an alias for `Reg<DMAEV_SPEC>`"]
pub type DMAEV = crate::Reg<dmaev::DMAEV_SPEC>;
#[doc = "GPTM DMA Event"]
pub mod dmaev;
#[doc = "ADCEV register accessor: an alias for `Reg<ADCEV_SPEC>`"]
pub type ADCEV = crate::Reg<adcev::ADCEV_SPEC>;
#[doc = "GPTM ADC Event"]
pub mod adcev;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "GPTM Peripheral Properties"]
pub mod pp;
