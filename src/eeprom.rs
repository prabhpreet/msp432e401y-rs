#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Size Information"]
    pub eesize: crate::Reg<eesize::EESIZE_SPEC>,
    #[doc = "0x04 - EEPROM Current Block"]
    pub eeblock: crate::Reg<eeblock::EEBLOCK_SPEC>,
    #[doc = "0x08 - EEPROM Current Offset"]
    pub eeoffset: crate::Reg<eeoffset::EEOFFSET_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EEPROM Read-Write"]
    pub eerdwr: crate::Reg<eerdwr::EERDWR_SPEC>,
    #[doc = "0x14 - EEPROM Read-Write with Increment"]
    pub eerdwrinc: crate::Reg<eerdwrinc::EERDWRINC_SPEC>,
    #[doc = "0x18 - EEPROM Done Status"]
    pub eedone: crate::Reg<eedone::EEDONE_SPEC>,
    #[doc = "0x1c - EEPROM Support Control and Status"]
    pub eesupp: crate::Reg<eesupp::EESUPP_SPEC>,
    #[doc = "0x20 - EEPROM Unlock"]
    pub eeunlock: crate::Reg<eeunlock::EEUNLOCK_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - EEPROM Protection"]
    pub eeprot: crate::Reg<eeprot::EEPROT_SPEC>,
    #[doc = "0x34 - EEPROM Password"]
    pub eepass0: crate::Reg<eepass0::EEPASS0_SPEC>,
    #[doc = "0x38 - EEPROM Password"]
    pub eepass1: crate::Reg<eepass1::EEPASS1_SPEC>,
    #[doc = "0x3c - EEPROM Password"]
    pub eepass2: crate::Reg<eepass2::EEPASS2_SPEC>,
    #[doc = "0x40 - EEPROM Interrupt"]
    pub eeint: crate::Reg<eeint::EEINT_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x50 - EEPROM Block Hide 0"]
    pub eehide0: crate::Reg<eehide0::EEHIDE0_SPEC>,
    #[doc = "0x54 - EEPROM Block Hide 1"]
    pub eehide1: crate::Reg<eehide1::EEHIDE1_SPEC>,
    #[doc = "0x58 - EEPROM Block Hide 2"]
    pub eehide2: crate::Reg<eehide2::EEHIDE2_SPEC>,
    _reserved16: [u8; 0x24],
    #[doc = "0x80 - EEPROM Debug Mass Erase"]
    pub eedbgme: crate::Reg<eedbgme::EEDBGME_SPEC>,
    _reserved17: [u8; 0x0f3c],
    #[doc = "0xfc0 - EEPROM Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
}
#[doc = "EESIZE register accessor: an alias for `Reg<EESIZE_SPEC>`"]
pub type EESIZE = crate::Reg<eesize::EESIZE_SPEC>;
#[doc = "EEPROM Size Information"]
pub mod eesize;
#[doc = "EEBLOCK register accessor: an alias for `Reg<EEBLOCK_SPEC>`"]
pub type EEBLOCK = crate::Reg<eeblock::EEBLOCK_SPEC>;
#[doc = "EEPROM Current Block"]
pub mod eeblock;
#[doc = "EEOFFSET register accessor: an alias for `Reg<EEOFFSET_SPEC>`"]
pub type EEOFFSET = crate::Reg<eeoffset::EEOFFSET_SPEC>;
#[doc = "EEPROM Current Offset"]
pub mod eeoffset;
#[doc = "EERDWR register accessor: an alias for `Reg<EERDWR_SPEC>`"]
pub type EERDWR = crate::Reg<eerdwr::EERDWR_SPEC>;
#[doc = "EEPROM Read-Write"]
pub mod eerdwr;
#[doc = "EERDWRINC register accessor: an alias for `Reg<EERDWRINC_SPEC>`"]
pub type EERDWRINC = crate::Reg<eerdwrinc::EERDWRINC_SPEC>;
#[doc = "EEPROM Read-Write with Increment"]
pub mod eerdwrinc;
#[doc = "EEDONE register accessor: an alias for `Reg<EEDONE_SPEC>`"]
pub type EEDONE = crate::Reg<eedone::EEDONE_SPEC>;
#[doc = "EEPROM Done Status"]
pub mod eedone;
#[doc = "EESUPP register accessor: an alias for `Reg<EESUPP_SPEC>`"]
pub type EESUPP = crate::Reg<eesupp::EESUPP_SPEC>;
#[doc = "EEPROM Support Control and Status"]
pub mod eesupp;
#[doc = "EEUNLOCK register accessor: an alias for `Reg<EEUNLOCK_SPEC>`"]
pub type EEUNLOCK = crate::Reg<eeunlock::EEUNLOCK_SPEC>;
#[doc = "EEPROM Unlock"]
pub mod eeunlock;
#[doc = "EEPROT register accessor: an alias for `Reg<EEPROT_SPEC>`"]
pub type EEPROT = crate::Reg<eeprot::EEPROT_SPEC>;
#[doc = "EEPROM Protection"]
pub mod eeprot;
#[doc = "EEPASS0 register accessor: an alias for `Reg<EEPASS0_SPEC>`"]
pub type EEPASS0 = crate::Reg<eepass0::EEPASS0_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass0;
#[doc = "EEPASS1 register accessor: an alias for `Reg<EEPASS1_SPEC>`"]
pub type EEPASS1 = crate::Reg<eepass1::EEPASS1_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass1;
#[doc = "EEPASS2 register accessor: an alias for `Reg<EEPASS2_SPEC>`"]
pub type EEPASS2 = crate::Reg<eepass2::EEPASS2_SPEC>;
#[doc = "EEPROM Password"]
pub mod eepass2;
#[doc = "EEINT register accessor: an alias for `Reg<EEINT_SPEC>`"]
pub type EEINT = crate::Reg<eeint::EEINT_SPEC>;
#[doc = "EEPROM Interrupt"]
pub mod eeint;
#[doc = "EEHIDE0 register accessor: an alias for `Reg<EEHIDE0_SPEC>`"]
pub type EEHIDE0 = crate::Reg<eehide0::EEHIDE0_SPEC>;
#[doc = "EEPROM Block Hide 0"]
pub mod eehide0;
#[doc = "EEHIDE1 register accessor: an alias for `Reg<EEHIDE1_SPEC>`"]
pub type EEHIDE1 = crate::Reg<eehide1::EEHIDE1_SPEC>;
#[doc = "EEPROM Block Hide 1"]
pub mod eehide1;
#[doc = "EEHIDE2 register accessor: an alias for `Reg<EEHIDE2_SPEC>`"]
pub type EEHIDE2 = crate::Reg<eehide2::EEHIDE2_SPEC>;
#[doc = "EEPROM Block Hide 2"]
pub mod eehide2;
#[doc = "EEDBGME register accessor: an alias for `Reg<EEDBGME_SPEC>`"]
pub type EEDBGME = crate::Reg<eedbgme::EEDBGME_SPEC>;
#[doc = "EEPROM Debug Mass Erase"]
pub mod eedbgme;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "EEPROM Peripheral Properties"]
pub mod pp;
