#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cryptographic Modules Clock Gating Request"]
    pub ccmcgreq: crate::Reg<ccmcgreq::CCMCGREQ_SPEC>,
}
#[doc = "CCMCGREQ register accessor: an alias for `Reg<CCMCGREQ_SPEC>`"]
pub type CCMCGREQ = crate::Reg<ccmcgreq::CCMCGREQ_SPEC>;
#[doc = "Cryptographic Modules Clock Gating Request"]
pub mod ccmcgreq;
