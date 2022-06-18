#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Ethernet MAC Frame Filter"]
    pub framefltr: crate::Reg<framefltr::FRAMEFLTR_SPEC>,
    #[doc = "0x08 - Ethernet MAC Hash Table High"]
    pub hashtblh: crate::Reg<hashtblh::HASHTBLH_SPEC>,
    #[doc = "0x0c - Ethernet MAC Hash Table Low"]
    pub hashtbll: crate::Reg<hashtbll::HASHTBLL_SPEC>,
    #[doc = "0x10 - Ethernet MAC MII Address"]
    pub miiaddr: crate::Reg<miiaddr::MIIADDR_SPEC>,
    #[doc = "0x14 - Ethernet MAC MII Data Register"]
    pub miidata: crate::Reg<miidata::MIIDATA_SPEC>,
    #[doc = "0x18 - Ethernet MAC Flow Control"]
    pub flowctl: crate::Reg<flowctl::FLOWCTL_SPEC>,
    #[doc = "0x1c - Ethernet MAC VLAN Tag"]
    pub vlantg: crate::Reg<vlantg::VLANTG_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - Ethernet MAC Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x28 - Ethernet MAC Remote Wake-Up Frame Filter"]
    pub rwuff: crate::Reg<rwuff::RWUFF_SPEC>,
    #[doc = "0x2c - Ethernet MAC PMT Control and Status Register"]
    pub pmtctlstat: crate::Reg<pmtctlstat::PMTCTLSTAT_SPEC>,
    #[doc = "0x30 - Ethernet MAC Low Power Idle Control and Status Register"]
    pub lpictlstat: crate::Reg<lpictlstat::LPICTLSTAT_SPEC>,
    #[doc = "0x34 - Ethernet MAC Low Power Idle Timer Control Register"]
    pub lpitimerctl: crate::Reg<lpitimerctl::LPITIMERCTL_SPEC>,
    #[doc = "0x38 - Ethernet MAC Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x3c - Ethernet MAC Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x40 - Ethernet MAC Address 0 High"]
    pub addr0h: crate::Reg<addr0h::ADDR0H_SPEC>,
    #[doc = "0x44 - Ethernet MAC Address 0 Low Register"]
    pub addr0l: crate::Reg<addr0l::ADDR0L_SPEC>,
    #[doc = "0x48 - Ethernet MAC Address 1 High"]
    pub addr1h: crate::Reg<addr1h::ADDR1H_SPEC>,
    #[doc = "0x4c - Ethernet MAC Address 1 Low"]
    pub addr1l: crate::Reg<addr1l::ADDR1L_SPEC>,
    #[doc = "0x50 - Ethernet MAC Address 2 High"]
    pub addr2h: crate::Reg<addr2h::ADDR2H_SPEC>,
    #[doc = "0x54 - Ethernet MAC Address 2 Low"]
    pub addr2l: crate::Reg<addr2l::ADDR2L_SPEC>,
    #[doc = "0x58 - Ethernet MAC Address 3 High"]
    pub addr3h: crate::Reg<addr3h::ADDR3H_SPEC>,
    #[doc = "0x5c - Ethernet MAC Address 3 Low"]
    pub addr3l: crate::Reg<addr3l::ADDR3L_SPEC>,
    _reserved23: [u8; 0x7c],
    #[doc = "0xdc - Ethernet MAC Watchdog Timeout"]
    pub wdogto: crate::Reg<wdogto::WDOGTO_SPEC>,
    _reserved24: [u8; 0x20],
    #[doc = "0x100 - Ethernet MAC MMC Control"]
    pub mmcctrl: crate::Reg<mmcctrl::MMCCTRL_SPEC>,
    #[doc = "0x104 - Ethernet MAC MMC Receive Raw Interrupt Status"]
    pub mmcrxris: crate::Reg<mmcrxris::MMCRXRIS_SPEC>,
    #[doc = "0x108 - Ethernet MAC MMC Transmit Raw Interrupt Status"]
    pub mmctxris: crate::Reg<mmctxris::MMCTXRIS_SPEC>,
    #[doc = "0x10c - Ethernet MAC MMC Receive Interrupt Mask"]
    pub mmcrxim: crate::Reg<mmcrxim::MMCRXIM_SPEC>,
    #[doc = "0x110 - Ethernet MAC MMC Transmit Interrupt Mask"]
    pub mmctxim: crate::Reg<mmctxim::MMCTXIM_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x118 - Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
    pub txcntgb: crate::Reg<txcntgb::TXCNTGB_SPEC>,
    _reserved30: [u8; 0x30],
    #[doc = "0x14c - Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
    pub txcntscol: crate::Reg<txcntscol::TXCNTSCOL_SPEC>,
    #[doc = "0x150 - Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
    pub txcntmcol: crate::Reg<txcntmcol::TXCNTMCOL_SPEC>,
    _reserved32: [u8; 0x10],
    #[doc = "0x164 - Ethernet MAC Transmit Octet Count Good"]
    pub txoctcntg: crate::Reg<txoctcntg::TXOCTCNTG_SPEC>,
    _reserved33: [u8; 0x18],
    #[doc = "0x180 - Ethernet MAC Receive Frame Count for Good and Bad Frames"]
    pub rxcntgb: crate::Reg<rxcntgb::RXCNTGB_SPEC>,
    _reserved34: [u8; 0x10],
    #[doc = "0x194 - Ethernet MAC Receive Frame Count for CRC Error Frames"]
    pub rxcntcrcerr: crate::Reg<rxcntcrcerr::RXCNTCRCERR_SPEC>,
    #[doc = "0x198 - Ethernet MAC Receive Frame Count for Alignment Error Frames"]
    pub rxcntalgnerr: crate::Reg<rxcntalgnerr::RXCNTALGNERR_SPEC>,
    _reserved36: [u8; 0x28],
    #[doc = "0x1c4 - Ethernet MAC Receive Frame Count for Good Unicast Frames"]
    pub rxcntguni: crate::Reg<rxcntguni::RXCNTGUNI_SPEC>,
    _reserved37: [u8; 0x03bc],
    #[doc = "0x584 - Ethernet MAC VLAN Tag Inclusion or Replacement"]
    pub vlnincrep: crate::Reg<vlnincrep::VLNINCREP_SPEC>,
    #[doc = "0x588 - Ethernet MAC VLAN Hash Table"]
    pub vlanhash: crate::Reg<vlanhash::VLANHASH_SPEC>,
    _reserved39: [u8; 0x0174],
    #[doc = "0x700 - Ethernet MAC Timestamp Control"]
    pub timstctrl: crate::Reg<timstctrl::TIMSTCTRL_SPEC>,
    #[doc = "0x704 - Ethernet MAC Sub-Second Increment"]
    pub subsecinc: crate::Reg<subsecinc::SUBSECINC_SPEC>,
    #[doc = "0x708 - Ethernet MAC System Time - Seconds"]
    pub timsec: crate::Reg<timsec::TIMSEC_SPEC>,
    #[doc = "0x70c - Ethernet MAC System Time - Nanoseconds"]
    pub timnano: crate::Reg<timnano::TIMNANO_SPEC>,
    #[doc = "0x710 - Ethernet MAC System Time - Seconds Update"]
    pub timsecu: crate::Reg<timsecu::TIMSECU_SPEC>,
    #[doc = "0x714 - Ethernet MAC System Time - Nanoseconds Update"]
    pub timnanou: crate::Reg<timnanou::TIMNANOU_SPEC>,
    #[doc = "0x718 - Ethernet MAC Timestamp Addend"]
    pub timadd: crate::Reg<timadd::TIMADD_SPEC>,
    #[doc = "0x71c - Ethernet MAC Target Time Seconds"]
    pub targsec: crate::Reg<targsec::TARGSEC_SPEC>,
    #[doc = "0x720 - Ethernet MAC Target Time Nanoseconds"]
    pub targnano: crate::Reg<targnano::TARGNANO_SPEC>,
    #[doc = "0x724 - Ethernet MAC System Time-Higher Word Seconds"]
    pub hwordsec: crate::Reg<hwordsec::HWORDSEC_SPEC>,
    #[doc = "0x728 - Ethernet MAC Timestamp Status"]
    pub timstat: crate::Reg<timstat::TIMSTAT_SPEC>,
    #[doc = "0x72c - Ethernet MAC PPS Control"]
    pub ppsctrl: crate::Reg<ppsctrl::PPSCTRL_SPEC>,
    _reserved51: [u8; 0x30],
    #[doc = "0x760 - Ethernet MAC PPS0 Interval"]
    pub pps0intvl: crate::Reg<pps0intvl::PPS0INTVL_SPEC>,
    #[doc = "0x764 - Ethernet MAC PPS0 Width"]
    pub pps0width: crate::Reg<pps0width::PPS0WIDTH_SPEC>,
    _reserved53: [u8; 0x0498],
    #[doc = "0xc00 - Ethernet MAC DMA Bus Mode"]
    pub dmabusmod: crate::Reg<dmabusmod::DMABUSMOD_SPEC>,
    #[doc = "0xc04 - Ethernet MAC Transmit Poll Demand"]
    pub txpolld: crate::Reg<txpolld::TXPOLLD_SPEC>,
    #[doc = "0xc08 - Ethernet MAC Receive Poll Demand"]
    pub rxpolld: crate::Reg<rxpolld::RXPOLLD_SPEC>,
    #[doc = "0xc0c - Ethernet MAC Receive Descriptor List Address"]
    pub rxdladdr: crate::Reg<rxdladdr::RXDLADDR_SPEC>,
    #[doc = "0xc10 - Ethernet MAC Transmit Descriptor List Address"]
    pub txdladdr: crate::Reg<txdladdr::TXDLADDR_SPEC>,
    #[doc = "0xc14 - Ethernet MAC DMA Interrupt Status"]
    pub dmaris: crate::Reg<dmaris::DMARIS_SPEC>,
    #[doc = "0xc18 - Ethernet MAC DMA Operation Mode"]
    pub dmaopmode: crate::Reg<dmaopmode::DMAOPMODE_SPEC>,
    #[doc = "0xc1c - Ethernet MAC DMA Interrupt Mask Register"]
    pub dmaim: crate::Reg<dmaim::DMAIM_SPEC>,
    #[doc = "0xc20 - Ethernet MAC Missed Frame and Buffer Overflow Counter"]
    pub mfboc: crate::Reg<mfboc::MFBOC_SPEC>,
    #[doc = "0xc24 - Ethernet MAC Receive Interrupt Watchdog Timer"]
    pub rxintwdt: crate::Reg<rxintwdt::RXINTWDT_SPEC>,
    _reserved63: [u8; 0x20],
    #[doc = "0xc48 - Ethernet MAC Current Host Transmit Descriptor"]
    pub hostxdesc: crate::Reg<hostxdesc::HOSTXDESC_SPEC>,
    #[doc = "0xc4c - Ethernet MAC Current Host Receive Descriptor"]
    pub hosrxdesc: crate::Reg<hosrxdesc::HOSRXDESC_SPEC>,
    #[doc = "0xc50 - Ethernet MAC Current Host Transmit Buffer Address"]
    pub hostxba: crate::Reg<hostxba::HOSTXBA_SPEC>,
    #[doc = "0xc54 - Ethernet MAC Current Host Receive Buffer Address"]
    pub hosrxba: crate::Reg<hosrxba::HOSRXBA_SPEC>,
    _reserved67: [u8; 0x0368],
    #[doc = "0xfc0 - Ethernet MAC Peripheral Property Register"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - Ethernet MAC Peripheral Configuration Register"]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0xfc8 - Ethernet MAC Clock Configuration Register"]
    pub cc: crate::Reg<cc::CC_SPEC>,
    #[doc = "0xfcc - Ethernet PHY Raw Interrupt Status"]
    pub ephyris: crate::Reg<ephyris::EPHYRIS_SPEC>,
    #[doc = "0xfd0 - Ethernet PHY Interrupt Mask"]
    pub ephyim: crate::Reg<ephyim::EPHYIM_SPEC>,
    #[doc = "0xfd4 - Ethernet PHY Masked Interrupt Status and Clear"]
    pub ephymisc: crate::Reg<ephymisc::EPHYMISC_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Ethernet MAC Configuration"]
pub mod cfg;
#[doc = "FRAMEFLTR register accessor: an alias for `Reg<FRAMEFLTR_SPEC>`"]
pub type FRAMEFLTR = crate::Reg<framefltr::FRAMEFLTR_SPEC>;
#[doc = "Ethernet MAC Frame Filter"]
pub mod framefltr;
#[doc = "HASHTBLH register accessor: an alias for `Reg<HASHTBLH_SPEC>`"]
pub type HASHTBLH = crate::Reg<hashtblh::HASHTBLH_SPEC>;
#[doc = "Ethernet MAC Hash Table High"]
pub mod hashtblh;
#[doc = "HASHTBLL register accessor: an alias for `Reg<HASHTBLL_SPEC>`"]
pub type HASHTBLL = crate::Reg<hashtbll::HASHTBLL_SPEC>;
#[doc = "Ethernet MAC Hash Table Low"]
pub mod hashtbll;
#[doc = "MIIADDR register accessor: an alias for `Reg<MIIADDR_SPEC>`"]
pub type MIIADDR = crate::Reg<miiaddr::MIIADDR_SPEC>;
#[doc = "Ethernet MAC MII Address"]
pub mod miiaddr;
#[doc = "MIIDATA register accessor: an alias for `Reg<MIIDATA_SPEC>`"]
pub type MIIDATA = crate::Reg<miidata::MIIDATA_SPEC>;
#[doc = "Ethernet MAC MII Data Register"]
pub mod miidata;
#[doc = "FLOWCTL register accessor: an alias for `Reg<FLOWCTL_SPEC>`"]
pub type FLOWCTL = crate::Reg<flowctl::FLOWCTL_SPEC>;
#[doc = "Ethernet MAC Flow Control"]
pub mod flowctl;
#[doc = "VLANTG register accessor: an alias for `Reg<VLANTG_SPEC>`"]
pub type VLANTG = crate::Reg<vlantg::VLANTG_SPEC>;
#[doc = "Ethernet MAC VLAN Tag"]
pub mod vlantg;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Ethernet MAC Status"]
pub mod status;
#[doc = "RWUFF register accessor: an alias for `Reg<RWUFF_SPEC>`"]
pub type RWUFF = crate::Reg<rwuff::RWUFF_SPEC>;
#[doc = "Ethernet MAC Remote Wake-Up Frame Filter"]
pub mod rwuff;
#[doc = "PMTCTLSTAT register accessor: an alias for `Reg<PMTCTLSTAT_SPEC>`"]
pub type PMTCTLSTAT = crate::Reg<pmtctlstat::PMTCTLSTAT_SPEC>;
#[doc = "Ethernet MAC PMT Control and Status Register"]
pub mod pmtctlstat;
#[doc = "LPICTLSTAT register accessor: an alias for `Reg<LPICTLSTAT_SPEC>`"]
pub type LPICTLSTAT = crate::Reg<lpictlstat::LPICTLSTAT_SPEC>;
#[doc = "Ethernet MAC Low Power Idle Control and Status Register"]
pub mod lpictlstat;
#[doc = "LPITIMERCTL register accessor: an alias for `Reg<LPITIMERCTL_SPEC>`"]
pub type LPITIMERCTL = crate::Reg<lpitimerctl::LPITIMERCTL_SPEC>;
#[doc = "Ethernet MAC Low Power Idle Timer Control Register"]
pub mod lpitimerctl;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Ethernet MAC Raw Interrupt Status"]
pub mod ris;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "Ethernet MAC Interrupt Mask"]
pub mod im;
#[doc = "ADDR0H register accessor: an alias for `Reg<ADDR0H_SPEC>`"]
pub type ADDR0H = crate::Reg<addr0h::ADDR0H_SPEC>;
#[doc = "Ethernet MAC Address 0 High"]
pub mod addr0h;
#[doc = "ADDR0L register accessor: an alias for `Reg<ADDR0L_SPEC>`"]
pub type ADDR0L = crate::Reg<addr0l::ADDR0L_SPEC>;
#[doc = "Ethernet MAC Address 0 Low Register"]
pub mod addr0l;
#[doc = "ADDR1H register accessor: an alias for `Reg<ADDR1H_SPEC>`"]
pub type ADDR1H = crate::Reg<addr1h::ADDR1H_SPEC>;
#[doc = "Ethernet MAC Address 1 High"]
pub mod addr1h;
#[doc = "ADDR1L register accessor: an alias for `Reg<ADDR1L_SPEC>`"]
pub type ADDR1L = crate::Reg<addr1l::ADDR1L_SPEC>;
#[doc = "Ethernet MAC Address 1 Low"]
pub mod addr1l;
#[doc = "ADDR2H register accessor: an alias for `Reg<ADDR2H_SPEC>`"]
pub type ADDR2H = crate::Reg<addr2h::ADDR2H_SPEC>;
#[doc = "Ethernet MAC Address 2 High"]
pub mod addr2h;
#[doc = "ADDR2L register accessor: an alias for `Reg<ADDR2L_SPEC>`"]
pub type ADDR2L = crate::Reg<addr2l::ADDR2L_SPEC>;
#[doc = "Ethernet MAC Address 2 Low"]
pub mod addr2l;
#[doc = "ADDR3H register accessor: an alias for `Reg<ADDR3H_SPEC>`"]
pub type ADDR3H = crate::Reg<addr3h::ADDR3H_SPEC>;
#[doc = "Ethernet MAC Address 3 High"]
pub mod addr3h;
#[doc = "ADDR3L register accessor: an alias for `Reg<ADDR3L_SPEC>`"]
pub type ADDR3L = crate::Reg<addr3l::ADDR3L_SPEC>;
#[doc = "Ethernet MAC Address 3 Low"]
pub mod addr3l;
#[doc = "WDOGTO register accessor: an alias for `Reg<WDOGTO_SPEC>`"]
pub type WDOGTO = crate::Reg<wdogto::WDOGTO_SPEC>;
#[doc = "Ethernet MAC Watchdog Timeout"]
pub mod wdogto;
#[doc = "MMCCTRL register accessor: an alias for `Reg<MMCCTRL_SPEC>`"]
pub type MMCCTRL = crate::Reg<mmcctrl::MMCCTRL_SPEC>;
#[doc = "Ethernet MAC MMC Control"]
pub mod mmcctrl;
#[doc = "MMCRXRIS register accessor: an alias for `Reg<MMCRXRIS_SPEC>`"]
pub type MMCRXRIS = crate::Reg<mmcrxris::MMCRXRIS_SPEC>;
#[doc = "Ethernet MAC MMC Receive Raw Interrupt Status"]
pub mod mmcrxris;
#[doc = "MMCTXRIS register accessor: an alias for `Reg<MMCTXRIS_SPEC>`"]
pub type MMCTXRIS = crate::Reg<mmctxris::MMCTXRIS_SPEC>;
#[doc = "Ethernet MAC MMC Transmit Raw Interrupt Status"]
pub mod mmctxris;
#[doc = "MMCRXIM register accessor: an alias for `Reg<MMCRXIM_SPEC>`"]
pub type MMCRXIM = crate::Reg<mmcrxim::MMCRXIM_SPEC>;
#[doc = "Ethernet MAC MMC Receive Interrupt Mask"]
pub mod mmcrxim;
#[doc = "MMCTXIM register accessor: an alias for `Reg<MMCTXIM_SPEC>`"]
pub type MMCTXIM = crate::Reg<mmctxim::MMCTXIM_SPEC>;
#[doc = "Ethernet MAC MMC Transmit Interrupt Mask"]
pub mod mmctxim;
#[doc = "TXCNTGB register accessor: an alias for `Reg<TXCNTGB_SPEC>`"]
pub type TXCNTGB = crate::Reg<txcntgb::TXCNTGB_SPEC>;
#[doc = "Ethernet MAC Transmit Frame Count for Good and Bad Frames"]
pub mod txcntgb;
#[doc = "TXCNTSCOL register accessor: an alias for `Reg<TXCNTSCOL_SPEC>`"]
pub type TXCNTSCOL = crate::Reg<txcntscol::TXCNTSCOL_SPEC>;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Single Collision"]
pub mod txcntscol;
#[doc = "TXCNTMCOL register accessor: an alias for `Reg<TXCNTMCOL_SPEC>`"]
pub type TXCNTMCOL = crate::Reg<txcntmcol::TXCNTMCOL_SPEC>;
#[doc = "Ethernet MAC Transmit Frame Count for Frames Transmitted after Multiple Collisions"]
pub mod txcntmcol;
#[doc = "TXOCTCNTG register accessor: an alias for `Reg<TXOCTCNTG_SPEC>`"]
pub type TXOCTCNTG = crate::Reg<txoctcntg::TXOCTCNTG_SPEC>;
#[doc = "Ethernet MAC Transmit Octet Count Good"]
pub mod txoctcntg;
#[doc = "RXCNTGB register accessor: an alias for `Reg<RXCNTGB_SPEC>`"]
pub type RXCNTGB = crate::Reg<rxcntgb::RXCNTGB_SPEC>;
#[doc = "Ethernet MAC Receive Frame Count for Good and Bad Frames"]
pub mod rxcntgb;
#[doc = "RXCNTCRCERR register accessor: an alias for `Reg<RXCNTCRCERR_SPEC>`"]
pub type RXCNTCRCERR = crate::Reg<rxcntcrcerr::RXCNTCRCERR_SPEC>;
#[doc = "Ethernet MAC Receive Frame Count for CRC Error Frames"]
pub mod rxcntcrcerr;
#[doc = "RXCNTALGNERR register accessor: an alias for `Reg<RXCNTALGNERR_SPEC>`"]
pub type RXCNTALGNERR = crate::Reg<rxcntalgnerr::RXCNTALGNERR_SPEC>;
#[doc = "Ethernet MAC Receive Frame Count for Alignment Error Frames"]
pub mod rxcntalgnerr;
#[doc = "RXCNTGUNI register accessor: an alias for `Reg<RXCNTGUNI_SPEC>`"]
pub type RXCNTGUNI = crate::Reg<rxcntguni::RXCNTGUNI_SPEC>;
#[doc = "Ethernet MAC Receive Frame Count for Good Unicast Frames"]
pub mod rxcntguni;
#[doc = "VLNINCREP register accessor: an alias for `Reg<VLNINCREP_SPEC>`"]
pub type VLNINCREP = crate::Reg<vlnincrep::VLNINCREP_SPEC>;
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement"]
pub mod vlnincrep;
#[doc = "VLANHASH register accessor: an alias for `Reg<VLANHASH_SPEC>`"]
pub type VLANHASH = crate::Reg<vlanhash::VLANHASH_SPEC>;
#[doc = "Ethernet MAC VLAN Hash Table"]
pub mod vlanhash;
#[doc = "TIMSTCTRL register accessor: an alias for `Reg<TIMSTCTRL_SPEC>`"]
pub type TIMSTCTRL = crate::Reg<timstctrl::TIMSTCTRL_SPEC>;
#[doc = "Ethernet MAC Timestamp Control"]
pub mod timstctrl;
#[doc = "SUBSECINC register accessor: an alias for `Reg<SUBSECINC_SPEC>`"]
pub type SUBSECINC = crate::Reg<subsecinc::SUBSECINC_SPEC>;
#[doc = "Ethernet MAC Sub-Second Increment"]
pub mod subsecinc;
#[doc = "TIMSEC register accessor: an alias for `Reg<TIMSEC_SPEC>`"]
pub type TIMSEC = crate::Reg<timsec::TIMSEC_SPEC>;
#[doc = "Ethernet MAC System Time - Seconds"]
pub mod timsec;
#[doc = "TIMNANO register accessor: an alias for `Reg<TIMNANO_SPEC>`"]
pub type TIMNANO = crate::Reg<timnano::TIMNANO_SPEC>;
#[doc = "Ethernet MAC System Time - Nanoseconds"]
pub mod timnano;
#[doc = "TIMSECU register accessor: an alias for `Reg<TIMSECU_SPEC>`"]
pub type TIMSECU = crate::Reg<timsecu::TIMSECU_SPEC>;
#[doc = "Ethernet MAC System Time - Seconds Update"]
pub mod timsecu;
#[doc = "TIMNANOU register accessor: an alias for `Reg<TIMNANOU_SPEC>`"]
pub type TIMNANOU = crate::Reg<timnanou::TIMNANOU_SPEC>;
#[doc = "Ethernet MAC System Time - Nanoseconds Update"]
pub mod timnanou;
#[doc = "TIMADD register accessor: an alias for `Reg<TIMADD_SPEC>`"]
pub type TIMADD = crate::Reg<timadd::TIMADD_SPEC>;
#[doc = "Ethernet MAC Timestamp Addend"]
pub mod timadd;
#[doc = "TARGSEC register accessor: an alias for `Reg<TARGSEC_SPEC>`"]
pub type TARGSEC = crate::Reg<targsec::TARGSEC_SPEC>;
#[doc = "Ethernet MAC Target Time Seconds"]
pub mod targsec;
#[doc = "TARGNANO register accessor: an alias for `Reg<TARGNANO_SPEC>`"]
pub type TARGNANO = crate::Reg<targnano::TARGNANO_SPEC>;
#[doc = "Ethernet MAC Target Time Nanoseconds"]
pub mod targnano;
#[doc = "HWORDSEC register accessor: an alias for `Reg<HWORDSEC_SPEC>`"]
pub type HWORDSEC = crate::Reg<hwordsec::HWORDSEC_SPEC>;
#[doc = "Ethernet MAC System Time-Higher Word Seconds"]
pub mod hwordsec;
#[doc = "TIMSTAT register accessor: an alias for `Reg<TIMSTAT_SPEC>`"]
pub type TIMSTAT = crate::Reg<timstat::TIMSTAT_SPEC>;
#[doc = "Ethernet MAC Timestamp Status"]
pub mod timstat;
#[doc = "PPSCTRL register accessor: an alias for `Reg<PPSCTRL_SPEC>`"]
pub type PPSCTRL = crate::Reg<ppsctrl::PPSCTRL_SPEC>;
#[doc = "Ethernet MAC PPS Control"]
pub mod ppsctrl;
#[doc = "PPS0INTVL register accessor: an alias for `Reg<PPS0INTVL_SPEC>`"]
pub type PPS0INTVL = crate::Reg<pps0intvl::PPS0INTVL_SPEC>;
#[doc = "Ethernet MAC PPS0 Interval"]
pub mod pps0intvl;
#[doc = "PPS0WIDTH register accessor: an alias for `Reg<PPS0WIDTH_SPEC>`"]
pub type PPS0WIDTH = crate::Reg<pps0width::PPS0WIDTH_SPEC>;
#[doc = "Ethernet MAC PPS0 Width"]
pub mod pps0width;
#[doc = "DMABUSMOD register accessor: an alias for `Reg<DMABUSMOD_SPEC>`"]
pub type DMABUSMOD = crate::Reg<dmabusmod::DMABUSMOD_SPEC>;
#[doc = "Ethernet MAC DMA Bus Mode"]
pub mod dmabusmod;
#[doc = "TXPOLLD register accessor: an alias for `Reg<TXPOLLD_SPEC>`"]
pub type TXPOLLD = crate::Reg<txpolld::TXPOLLD_SPEC>;
#[doc = "Ethernet MAC Transmit Poll Demand"]
pub mod txpolld;
#[doc = "RXPOLLD register accessor: an alias for `Reg<RXPOLLD_SPEC>`"]
pub type RXPOLLD = crate::Reg<rxpolld::RXPOLLD_SPEC>;
#[doc = "Ethernet MAC Receive Poll Demand"]
pub mod rxpolld;
#[doc = "RXDLADDR register accessor: an alias for `Reg<RXDLADDR_SPEC>`"]
pub type RXDLADDR = crate::Reg<rxdladdr::RXDLADDR_SPEC>;
#[doc = "Ethernet MAC Receive Descriptor List Address"]
pub mod rxdladdr;
#[doc = "TXDLADDR register accessor: an alias for `Reg<TXDLADDR_SPEC>`"]
pub type TXDLADDR = crate::Reg<txdladdr::TXDLADDR_SPEC>;
#[doc = "Ethernet MAC Transmit Descriptor List Address"]
pub mod txdladdr;
#[doc = "DMARIS register accessor: an alias for `Reg<DMARIS_SPEC>`"]
pub type DMARIS = crate::Reg<dmaris::DMARIS_SPEC>;
#[doc = "Ethernet MAC DMA Interrupt Status"]
pub mod dmaris;
#[doc = "DMAOPMODE register accessor: an alias for `Reg<DMAOPMODE_SPEC>`"]
pub type DMAOPMODE = crate::Reg<dmaopmode::DMAOPMODE_SPEC>;
#[doc = "Ethernet MAC DMA Operation Mode"]
pub mod dmaopmode;
#[doc = "DMAIM register accessor: an alias for `Reg<DMAIM_SPEC>`"]
pub type DMAIM = crate::Reg<dmaim::DMAIM_SPEC>;
#[doc = "Ethernet MAC DMA Interrupt Mask Register"]
pub mod dmaim;
#[doc = "MFBOC register accessor: an alias for `Reg<MFBOC_SPEC>`"]
pub type MFBOC = crate::Reg<mfboc::MFBOC_SPEC>;
#[doc = "Ethernet MAC Missed Frame and Buffer Overflow Counter"]
pub mod mfboc;
#[doc = "RXINTWDT register accessor: an alias for `Reg<RXINTWDT_SPEC>`"]
pub type RXINTWDT = crate::Reg<rxintwdt::RXINTWDT_SPEC>;
#[doc = "Ethernet MAC Receive Interrupt Watchdog Timer"]
pub mod rxintwdt;
#[doc = "HOSTXDESC register accessor: an alias for `Reg<HOSTXDESC_SPEC>`"]
pub type HOSTXDESC = crate::Reg<hostxdesc::HOSTXDESC_SPEC>;
#[doc = "Ethernet MAC Current Host Transmit Descriptor"]
pub mod hostxdesc;
#[doc = "HOSRXDESC register accessor: an alias for `Reg<HOSRXDESC_SPEC>`"]
pub type HOSRXDESC = crate::Reg<hosrxdesc::HOSRXDESC_SPEC>;
#[doc = "Ethernet MAC Current Host Receive Descriptor"]
pub mod hosrxdesc;
#[doc = "HOSTXBA register accessor: an alias for `Reg<HOSTXBA_SPEC>`"]
pub type HOSTXBA = crate::Reg<hostxba::HOSTXBA_SPEC>;
#[doc = "Ethernet MAC Current Host Transmit Buffer Address"]
pub mod hostxba;
#[doc = "HOSRXBA register accessor: an alias for `Reg<HOSRXBA_SPEC>`"]
pub type HOSRXBA = crate::Reg<hosrxba::HOSRXBA_SPEC>;
#[doc = "Ethernet MAC Current Host Receive Buffer Address"]
pub mod hosrxba;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "Ethernet MAC Peripheral Property Register"]
pub mod pp;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Ethernet MAC Peripheral Configuration Register"]
pub mod pc;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Ethernet MAC Clock Configuration Register"]
pub mod cc;
#[doc = "EPHYRIS register accessor: an alias for `Reg<EPHYRIS_SPEC>`"]
pub type EPHYRIS = crate::Reg<ephyris::EPHYRIS_SPEC>;
#[doc = "Ethernet PHY Raw Interrupt Status"]
pub mod ephyris;
#[doc = "EPHYIM register accessor: an alias for `Reg<EPHYIM_SPEC>`"]
pub type EPHYIM = crate::Reg<ephyim::EPHYIM_SPEC>;
#[doc = "Ethernet PHY Interrupt Mask"]
pub mod ephyim;
#[doc = "EPHYMISC register accessor: an alias for `Reg<EPHYMISC_SPEC>`"]
pub type EPHYMISC = crate::Reg<ephymisc::EPHYMISC_SPEC>;
#[doc = "Ethernet PHY Masked Interrupt Status and Clear"]
pub mod ephymisc;
