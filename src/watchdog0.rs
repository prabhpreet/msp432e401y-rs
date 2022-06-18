#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Load"]
    pub load: crate::Reg<load::LOAD_SPEC>,
    #[doc = "0x04 - Watchdog Value"]
    pub value: crate::Reg<value::VALUE_SPEC>,
    #[doc = "0x08 - Watchdog Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x0c - Watchdog Interrupt Clear"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x10 - Watchdog Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x14 - Watchdog Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    _reserved6: [u8; 0x0400],
    #[doc = "0x418 - Watchdog Test"]
    pub test: crate::Reg<test::TEST_SPEC>,
    _reserved7: [u8; 0x07e4],
    #[doc = "0xc00 - Watchdog Lock"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "LOAD register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "Watchdog Load"]
pub mod load;
#[doc = "VALUE register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Watchdog Value"]
pub mod value;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Watchdog Control"]
pub mod ctl;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Watchdog Interrupt Clear"]
pub mod icr;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Watchdog Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Watchdog Masked Interrupt Status"]
pub mod mis;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Watchdog Test"]
pub mod test;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Watchdog Lock"]
pub mod lock;
