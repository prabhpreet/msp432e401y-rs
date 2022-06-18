#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernation RTC Counter"]
    pub rtcc: crate::Reg<rtcc::RTCC_SPEC>,
    #[doc = "0x04 - Hibernation RTC Match 0"]
    pub rtcm0: crate::Reg<rtcm0::RTCM0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Hibernation RTC Load"]
    pub rtcld: crate::Reg<rtcld::RTCLD_SPEC>,
    #[doc = "0x10 - Hibernation Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x14 - Hibernation Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x18 - Hibernation Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - Hibernation Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x20 - Hibernation Interrupt Clear"]
    pub ic: crate::Reg<ic::IC_SPEC>,
    #[doc = "0x24 - Hibernation RTC Trim"]
    pub rtct: crate::Reg<rtct::RTCT_SPEC>,
    #[doc = "0x28 - Hibernation RTC Sub Seconds"]
    pub rtcss: crate::Reg<rtcss::RTCSS_SPEC>,
    #[doc = "0x2c - Hibernation IO Configuration"]
    pub io: crate::Reg<io::IO_SPEC>,
    #[doc = "0x30 - Hibernation Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    _reserved12: [u8; 0x02cc],
    #[doc = "0x300 - Hibernation Calendar Control"]
    pub calctl: crate::Reg<calctl::CALCTL_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x310 - Hibernation Calendar 0"]
    pub cal0: crate::Reg<cal0::CAL0_SPEC>,
    #[doc = "0x314 - Hibernation Calendar 1"]
    pub cal1: crate::Reg<cal1::CAL1_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x320 - Hibernation Calendar Load 0"]
    pub calld0: crate::Reg<calld0::CALLD0_SPEC>,
    #[doc = "0x324 - Hibernation Calendar Load"]
    pub calld1: crate::Reg<calld1::CALLD1_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x330 - Hibernation Calendar Match 0"]
    pub calm0: crate::Reg<calm0::CALM0_SPEC>,
    #[doc = "0x334 - Hibernation Calendar Match 1"]
    pub calm1: crate::Reg<calm1::CALM1_SPEC>,
    _reserved19: [u8; 0x28],
    #[doc = "0x360 - Hibernation Lock"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    _reserved20: [u8; 0x9c],
    #[doc = "0x400 - HIB Tamper Control"]
    pub tpctl: crate::Reg<tpctl::TPCTL_SPEC>,
    #[doc = "0x404 - HIB Tamper Status"]
    pub tpstat: crate::Reg<tpstat::TPSTAT_SPEC>,
    _reserved22: [u8; 0x08],
    #[doc = "0x410 - HIB Tamper I/O Control"]
    pub tpio: crate::Reg<tpio::TPIO_SPEC>,
    _reserved23: [u8; 0xcc],
    #[doc = "0x4e0 - HIB Tamper Log 0"]
    pub tplog0: crate::Reg<tplog0::TPLOG0_SPEC>,
    #[doc = "0x4e4 - HIB Tamper Log 1"]
    pub tplog1: crate::Reg<tplog1::TPLOG1_SPEC>,
    #[doc = "0x4e8 - HIB Tamper Log 2"]
    pub tplog2: crate::Reg<tplog2::TPLOG2_SPEC>,
    #[doc = "0x4ec - HIB Tamper Log 3"]
    pub tplog3: crate::Reg<tplog3::TPLOG3_SPEC>,
    #[doc = "0x4f0 - HIB Tamper Log 4"]
    pub tplog4: crate::Reg<tplog4::TPLOG4_SPEC>,
    #[doc = "0x4f4 - HIB Tamper Log 5"]
    pub tplog5: crate::Reg<tplog5::TPLOG5_SPEC>,
    #[doc = "0x4f8 - HIB Tamper Log 6"]
    pub tplog6: crate::Reg<tplog6::TPLOG6_SPEC>,
    #[doc = "0x4fc - HIB Tamper Log 7"]
    pub tplog7: crate::Reg<tplog7::TPLOG7_SPEC>,
    _reserved31: [u8; 0x0ac0],
    #[doc = "0xfc0 - Hibernation Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0xfc8 - Hibernation Clock Control"]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
#[doc = "RTCC register accessor: an alias for `Reg<RTCC_SPEC>`"]
pub type RTCC = crate::Reg<rtcc::RTCC_SPEC>;
#[doc = "Hibernation RTC Counter"]
pub mod rtcc;
#[doc = "RTCM0 register accessor: an alias for `Reg<RTCM0_SPEC>`"]
pub type RTCM0 = crate::Reg<rtcm0::RTCM0_SPEC>;
#[doc = "Hibernation RTC Match 0"]
pub mod rtcm0;
#[doc = "RTCLD register accessor: an alias for `Reg<RTCLD_SPEC>`"]
pub type RTCLD = crate::Reg<rtcld::RTCLD_SPEC>;
#[doc = "Hibernation RTC Load"]
pub mod rtcld;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Hibernation Control"]
pub mod ctl;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "Hibernation Interrupt Mask"]
pub mod im;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Hibernation Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Hibernation Masked Interrupt Status"]
pub mod mis;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "Hibernation Interrupt Clear"]
pub mod ic;
#[doc = "RTCT register accessor: an alias for `Reg<RTCT_SPEC>`"]
pub type RTCT = crate::Reg<rtct::RTCT_SPEC>;
#[doc = "Hibernation RTC Trim"]
pub mod rtct;
#[doc = "RTCSS register accessor: an alias for `Reg<RTCSS_SPEC>`"]
pub type RTCSS = crate::Reg<rtcss::RTCSS_SPEC>;
#[doc = "Hibernation RTC Sub Seconds"]
pub mod rtcss;
#[doc = "IO register accessor: an alias for `Reg<IO_SPEC>`"]
pub type IO = crate::Reg<io::IO_SPEC>;
#[doc = "Hibernation IO Configuration"]
pub mod io;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Hibernation Data"]
pub mod data;
#[doc = "CALCTL register accessor: an alias for `Reg<CALCTL_SPEC>`"]
pub type CALCTL = crate::Reg<calctl::CALCTL_SPEC>;
#[doc = "Hibernation Calendar Control"]
pub mod calctl;
#[doc = "CAL0 register accessor: an alias for `Reg<CAL0_SPEC>`"]
pub type CAL0 = crate::Reg<cal0::CAL0_SPEC>;
#[doc = "Hibernation Calendar 0"]
pub mod cal0;
#[doc = "CAL1 register accessor: an alias for `Reg<CAL1_SPEC>`"]
pub type CAL1 = crate::Reg<cal1::CAL1_SPEC>;
#[doc = "Hibernation Calendar 1"]
pub mod cal1;
#[doc = "CALLD0 register accessor: an alias for `Reg<CALLD0_SPEC>`"]
pub type CALLD0 = crate::Reg<calld0::CALLD0_SPEC>;
#[doc = "Hibernation Calendar Load 0"]
pub mod calld0;
#[doc = "CALLD1 register accessor: an alias for `Reg<CALLD1_SPEC>`"]
pub type CALLD1 = crate::Reg<calld1::CALLD1_SPEC>;
#[doc = "Hibernation Calendar Load"]
pub mod calld1;
#[doc = "CALM0 register accessor: an alias for `Reg<CALM0_SPEC>`"]
pub type CALM0 = crate::Reg<calm0::CALM0_SPEC>;
#[doc = "Hibernation Calendar Match 0"]
pub mod calm0;
#[doc = "CALM1 register accessor: an alias for `Reg<CALM1_SPEC>`"]
pub type CALM1 = crate::Reg<calm1::CALM1_SPEC>;
#[doc = "Hibernation Calendar Match 1"]
pub mod calm1;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Hibernation Lock"]
pub mod lock;
#[doc = "TPCTL register accessor: an alias for `Reg<TPCTL_SPEC>`"]
pub type TPCTL = crate::Reg<tpctl::TPCTL_SPEC>;
#[doc = "HIB Tamper Control"]
pub mod tpctl;
#[doc = "TPSTAT register accessor: an alias for `Reg<TPSTAT_SPEC>`"]
pub type TPSTAT = crate::Reg<tpstat::TPSTAT_SPEC>;
#[doc = "HIB Tamper Status"]
pub mod tpstat;
#[doc = "TPIO register accessor: an alias for `Reg<TPIO_SPEC>`"]
pub type TPIO = crate::Reg<tpio::TPIO_SPEC>;
#[doc = "HIB Tamper I/O Control"]
pub mod tpio;
#[doc = "TPLOG0 register accessor: an alias for `Reg<TPLOG0_SPEC>`"]
pub type TPLOG0 = crate::Reg<tplog0::TPLOG0_SPEC>;
#[doc = "HIB Tamper Log 0"]
pub mod tplog0;
#[doc = "TPLOG1 register accessor: an alias for `Reg<TPLOG1_SPEC>`"]
pub type TPLOG1 = crate::Reg<tplog1::TPLOG1_SPEC>;
#[doc = "HIB Tamper Log 1"]
pub mod tplog1;
#[doc = "TPLOG2 register accessor: an alias for `Reg<TPLOG2_SPEC>`"]
pub type TPLOG2 = crate::Reg<tplog2::TPLOG2_SPEC>;
#[doc = "HIB Tamper Log 2"]
pub mod tplog2;
#[doc = "TPLOG3 register accessor: an alias for `Reg<TPLOG3_SPEC>`"]
pub type TPLOG3 = crate::Reg<tplog3::TPLOG3_SPEC>;
#[doc = "HIB Tamper Log 3"]
pub mod tplog3;
#[doc = "TPLOG4 register accessor: an alias for `Reg<TPLOG4_SPEC>`"]
pub type TPLOG4 = crate::Reg<tplog4::TPLOG4_SPEC>;
#[doc = "HIB Tamper Log 4"]
pub mod tplog4;
#[doc = "TPLOG5 register accessor: an alias for `Reg<TPLOG5_SPEC>`"]
pub type TPLOG5 = crate::Reg<tplog5::TPLOG5_SPEC>;
#[doc = "HIB Tamper Log 5"]
pub mod tplog5;
#[doc = "TPLOG6 register accessor: an alias for `Reg<TPLOG6_SPEC>`"]
pub type TPLOG6 = crate::Reg<tplog6::TPLOG6_SPEC>;
#[doc = "HIB Tamper Log 6"]
pub mod tplog6;
#[doc = "TPLOG7 register accessor: an alias for `Reg<TPLOG7_SPEC>`"]
pub type TPLOG7 = crate::Reg<tplog7::TPLOG7_SPEC>;
#[doc = "HIB Tamper Log 7"]
pub mod tplog7;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "Hibernation Peripheral Properties"]
pub mod pp;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Hibernation Clock Control"]
pub mod cc;
