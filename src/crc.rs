#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - CRC SEED/Context"]
    pub crcseed: crate::Reg<crcseed::CRCSEED_SPEC>,
    #[doc = "0x14 - CRC Data Input"]
    pub crcdin: crate::Reg<crcdin::CRCDIN_SPEC>,
    #[doc = "0x18 - CRC Post Processing Result"]
    pub crcrsltpp: crate::Reg<crcrsltpp::CRCRSLTPP_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control"]
pub mod ctrl;
#[doc = "CRCSEED register accessor: an alias for `Reg<CRCSEED_SPEC>`"]
pub type CRCSEED = crate::Reg<crcseed::CRCSEED_SPEC>;
#[doc = "CRC SEED/Context"]
pub mod crcseed;
#[doc = "CRCDIN register accessor: an alias for `Reg<CRCDIN_SPEC>`"]
pub type CRCDIN = crate::Reg<crcdin::CRCDIN_SPEC>;
#[doc = "CRC Data Input"]
pub mod crcdin;
#[doc = "CRCRSLTPP register accessor: an alias for `Reg<CRCRSLTPP_SPEC>`"]
pub type CRCRSLTPP = crate::Reg<crcrsltpp::CRCRSLTPP_SPEC>;
#[doc = "CRC Post Processing Result"]
pub mod crcrsltpp;
