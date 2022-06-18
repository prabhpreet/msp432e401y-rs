#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Exception Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x04 - System Exception Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x08 - System Exception Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x0c - System Exception Interrupt Clear"]
    pub ic: crate::Reg<ic::IC_SPEC>,
}
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "System Exception Raw Interrupt Status"]
pub mod ris;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "System Exception Interrupt Mask"]
pub mod im;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "System Exception Masked Interrupt Status"]
pub mod mis;
#[doc = "IC register accessor: an alias for `Reg<IC_SPEC>`"]
pub type IC = crate::Reg<ic::IC_SPEC>;
#[doc = "System Exception Interrupt Clear"]
pub mod ic;
