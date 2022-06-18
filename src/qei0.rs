#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QEI Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - QEI Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - QEI Position"]
    pub pos: crate::Reg<pos::POS_SPEC>,
    #[doc = "0x0c - QEI Maximum Position"]
    pub maxpos: crate::Reg<maxpos::MAXPOS_SPEC>,
    #[doc = "0x10 - QEI Timer Load"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x14 - QEI Timer"]
    pub time: crate::Reg<time::TIME_SPEC>,
    #[doc = "0x18 - QEI Velocity Counter"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x1c - QEI Velocity"]
    pub speed: crate::Reg<speed::SPEED_SPEC>,
    #[doc = "0x20 - QEI Interrupt Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x24 - QEI Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x28 - QEI Interrupt Status and Clear"]
    pub isc: crate::Reg<isc::ISC_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "QEI Control"]
pub mod ctl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "QEI Status"]
pub mod stat;
#[doc = "POS register accessor: an alias for `Reg<POS_SPEC>`"]
pub type POS = crate::Reg<pos::POS_SPEC>;
#[doc = "QEI Position"]
pub mod pos;
#[doc = "MAXPOS register accessor: an alias for `Reg<MAXPOS_SPEC>`"]
pub type MAXPOS = crate::Reg<maxpos::MAXPOS_SPEC>;
#[doc = "QEI Maximum Position"]
pub mod maxpos;
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "QEI Timer Load"]
pub mod load;
#[doc = "TIME register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "QEI Timer"]
pub mod time;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "QEI Velocity Counter"]
pub mod count;
#[doc = "SPEED register accessor: an alias for `Reg<SPEED_SPEC>`"]
pub type SPEED = crate::Reg<speed::SPEED_SPEC>;
#[doc = "QEI Velocity"]
pub mod speed;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "QEI Interrupt Enable"]
pub mod inten;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "QEI Raw Interrupt Status"]
pub mod ris;
#[doc = "ISC register accessor: an alias for `Reg<ISC_SPEC>`"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "QEI Interrupt Status and Clear"]
pub mod isc;
