#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog Comparator Masked Interrupt Status"]
    pub acmis: crate::Reg<acmis::ACMIS_SPEC>,
    #[doc = "0x04 - Analog Comparator Raw Interrupt Status"]
    pub acris: crate::Reg<acris::ACRIS_SPEC>,
    #[doc = "0x08 - Analog Comparator Interrupt Enable"]
    pub acinten: crate::Reg<acinten::ACINTEN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Analog Comparator Reference Voltage Control"]
    pub acrefctl: crate::Reg<acrefctl::ACREFCTL_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - Analog Comparator Status 0"]
    pub acstat0: crate::Reg<acstat0::ACSTAT0_SPEC>,
    #[doc = "0x24 - Analog Comparator Control 0"]
    pub acctl0: crate::Reg<acctl0::ACCTL0_SPEC>,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - Analog Comparator Status 1"]
    pub acstat1: crate::Reg<acstat1::ACSTAT1_SPEC>,
    #[doc = "0x44 - Analog Comparator Control 1"]
    pub acctl1: crate::Reg<acctl1::ACCTL1_SPEC>,
    _reserved8: [u8; 0x18],
    #[doc = "0x60 - Analog Comparator Status 2"]
    pub acstat2: crate::Reg<acstat2::ACSTAT2_SPEC>,
    #[doc = "0x64 - Analog Comparator Control 2"]
    pub acctl2: crate::Reg<acctl2::ACCTL2_SPEC>,
    _reserved10: [u8; 0x0f58],
    #[doc = "0xfc0 - Analog Comparator Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
}
#[doc = "ACMIS register accessor: an alias for `Reg<ACMIS_SPEC>`"]
pub type ACMIS = crate::Reg<acmis::ACMIS_SPEC>;
#[doc = "Analog Comparator Masked Interrupt Status"]
pub mod acmis;
#[doc = "ACRIS register accessor: an alias for `Reg<ACRIS_SPEC>`"]
pub type ACRIS = crate::Reg<acris::ACRIS_SPEC>;
#[doc = "Analog Comparator Raw Interrupt Status"]
pub mod acris;
#[doc = "ACINTEN register accessor: an alias for `Reg<ACINTEN_SPEC>`"]
pub type ACINTEN = crate::Reg<acinten::ACINTEN_SPEC>;
#[doc = "Analog Comparator Interrupt Enable"]
pub mod acinten;
#[doc = "ACREFCTL register accessor: an alias for `Reg<ACREFCTL_SPEC>`"]
pub type ACREFCTL = crate::Reg<acrefctl::ACREFCTL_SPEC>;
#[doc = "Analog Comparator Reference Voltage Control"]
pub mod acrefctl;
#[doc = "ACSTAT0 register accessor: an alias for `Reg<ACSTAT0_SPEC>`"]
pub type ACSTAT0 = crate::Reg<acstat0::ACSTAT0_SPEC>;
#[doc = "Analog Comparator Status 0"]
pub mod acstat0;
#[doc = "ACCTL0 register accessor: an alias for `Reg<ACCTL0_SPEC>`"]
pub type ACCTL0 = crate::Reg<acctl0::ACCTL0_SPEC>;
#[doc = "Analog Comparator Control 0"]
pub mod acctl0;
#[doc = "ACSTAT1 register accessor: an alias for `Reg<ACSTAT1_SPEC>`"]
pub type ACSTAT1 = crate::Reg<acstat1::ACSTAT1_SPEC>;
#[doc = "Analog Comparator Status 1"]
pub mod acstat1;
#[doc = "ACCTL1 register accessor: an alias for `Reg<ACCTL1_SPEC>`"]
pub type ACCTL1 = crate::Reg<acctl1::ACCTL1_SPEC>;
#[doc = "Analog Comparator Control 1"]
pub mod acctl1;
#[doc = "ACSTAT2 register accessor: an alias for `Reg<ACSTAT2_SPEC>`"]
pub type ACSTAT2 = crate::Reg<acstat2::ACSTAT2_SPEC>;
#[doc = "Analog Comparator Status 2"]
pub mod acstat2;
#[doc = "ACCTL2 register accessor: an alias for `Reg<ACCTL2_SPEC>`"]
pub type ACCTL2 = crate::Reg<acctl2::ACCTL2_SPEC>;
#[doc = "Analog Comparator Control 2"]
pub mod acctl2;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "Analog Comparator Peripheral Properties"]
pub mod pp;
