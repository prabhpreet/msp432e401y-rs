#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Master Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - PWM Time Base Sync"]
    pub sync: crate::Reg<sync::SYNC_SPEC>,
    #[doc = "0x08 - PWM Output Enable"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x0c - PWM Output Inversion"]
    pub invert: crate::Reg<invert::INVERT_SPEC>,
    #[doc = "0x10 - PWM Output Fault"]
    pub fault: crate::Reg<fault::FAULT_SPEC>,
    #[doc = "0x14 - PWM Interrupt Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x18 - PWM Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - PWM Interrupt Status and Clear"]
    pub isc: crate::Reg<isc::ISC_SPEC>,
    #[doc = "0x20 - PWM Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x24 - PWM Fault Condition Value"]
    pub faultval: crate::Reg<faultval::FAULTVAL_SPEC>,
    #[doc = "0x28 - PWM Enable Update"]
    pub enupd: crate::Reg<enupd::ENUPD_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - PWM0 Control"]
    pub _0_ctl: crate::Reg<_0_ctl::_0_CTL_SPEC>,
    #[doc = "0x44 - PWM0 Interrupt and Trigger Enable"]
    pub _0_inten: crate::Reg<_0_inten::_0_INTEN_SPEC>,
    #[doc = "0x48 - PWM0 Raw Interrupt Status"]
    pub _0_ris: crate::Reg<_0_ris::_0_RIS_SPEC>,
    #[doc = "0x4c - PWM0 Interrupt Status and Clear"]
    pub _0_isc: crate::Reg<_0_isc::_0_ISC_SPEC>,
    #[doc = "0x50 - PWM0 Load"]
    pub _0_load: crate::Reg<_0_load::_0_LOAD_SPEC>,
    #[doc = "0x54 - PWM0 Counter"]
    pub _0_count: crate::Reg<_0_count::_0_COUNT_SPEC>,
    #[doc = "0x58 - PWM0 Compare A"]
    pub _0_cmpa: crate::Reg<_0_cmpa::_0_CMPA_SPEC>,
    #[doc = "0x5c - PWM0 Compare B"]
    pub _0_cmpb: crate::Reg<_0_cmpb::_0_CMPB_SPEC>,
    #[doc = "0x60 - PWM0 Generator A Control"]
    pub _0_gena: crate::Reg<_0_gena::_0_GENA_SPEC>,
    #[doc = "0x64 - PWM0 Generator B Control"]
    pub _0_genb: crate::Reg<_0_genb::_0_GENB_SPEC>,
    #[doc = "0x68 - PWM0 Dead-Band Control"]
    pub _0_dbctl: crate::Reg<_0_dbctl::_0_DBCTL_SPEC>,
    #[doc = "0x6c - PWM0 Dead-Band Rising-Edge Delay"]
    pub _0_dbrise: crate::Reg<_0_dbrise::_0_DBRISE_SPEC>,
    #[doc = "0x70 - PWM0 Dead-Band Falling-Edge-Delay"]
    pub _0_dbfall: crate::Reg<_0_dbfall::_0_DBFALL_SPEC>,
    #[doc = "0x74 - PWM0 Fault Source 0"]
    pub _0_fltsrc0: crate::Reg<_0_fltsrc0::_0_FLTSRC0_SPEC>,
    #[doc = "0x78 - PWM0 Fault Source 1"]
    pub _0_fltsrc1: crate::Reg<_0_fltsrc1::_0_FLTSRC1_SPEC>,
    #[doc = "0x7c - PWM0 Minimum Fault Period"]
    pub _0_minfltper: crate::Reg<_0_minfltper::_0_MINFLTPER_SPEC>,
    #[doc = "0x80 - PWM1 Control"]
    pub _1_ctl: crate::Reg<_1_ctl::_1_CTL_SPEC>,
    #[doc = "0x84 - PWM1 Interrupt and Trigger Enable"]
    pub _1_inten: crate::Reg<_1_inten::_1_INTEN_SPEC>,
    #[doc = "0x88 - PWM1 Raw Interrupt Status"]
    pub _1_ris: crate::Reg<_1_ris::_1_RIS_SPEC>,
    #[doc = "0x8c - PWM1 Interrupt Status and Clear"]
    pub _1_isc: crate::Reg<_1_isc::_1_ISC_SPEC>,
    #[doc = "0x90 - PWM1 Load"]
    pub _1_load: crate::Reg<_1_load::_1_LOAD_SPEC>,
    #[doc = "0x94 - PWM1 Counter"]
    pub _1_count: crate::Reg<_1_count::_1_COUNT_SPEC>,
    #[doc = "0x98 - PWM1 Compare A"]
    pub _1_cmpa: crate::Reg<_1_cmpa::_1_CMPA_SPEC>,
    #[doc = "0x9c - PWM1 Compare B"]
    pub _1_cmpb: crate::Reg<_1_cmpb::_1_CMPB_SPEC>,
    #[doc = "0xa0 - PWM1 Generator A Control"]
    pub _1_gena: crate::Reg<_1_gena::_1_GENA_SPEC>,
    #[doc = "0xa4 - PWM1 Generator B Control"]
    pub _1_genb: crate::Reg<_1_genb::_1_GENB_SPEC>,
    #[doc = "0xa8 - PWM1 Dead-Band Control"]
    pub _1_dbctl: crate::Reg<_1_dbctl::_1_DBCTL_SPEC>,
    #[doc = "0xac - PWM1 Dead-Band Rising-Edge Delay"]
    pub _1_dbrise: crate::Reg<_1_dbrise::_1_DBRISE_SPEC>,
    #[doc = "0xb0 - PWM1 Dead-Band Falling-Edge-Delay"]
    pub _1_dbfall: crate::Reg<_1_dbfall::_1_DBFALL_SPEC>,
    #[doc = "0xb4 - PWM1 Fault Source 0"]
    pub _1_fltsrc0: crate::Reg<_1_fltsrc0::_1_FLTSRC0_SPEC>,
    #[doc = "0xb8 - PWM1 Fault Source 1"]
    pub _1_fltsrc1: crate::Reg<_1_fltsrc1::_1_FLTSRC1_SPEC>,
    #[doc = "0xbc - PWM1 Minimum Fault Period"]
    pub _1_minfltper: crate::Reg<_1_minfltper::_1_MINFLTPER_SPEC>,
    #[doc = "0xc0 - PWM2 Control"]
    pub _2_ctl: crate::Reg<_2_ctl::_2_CTL_SPEC>,
    #[doc = "0xc4 - PWM2 Interrupt and Trigger Enable"]
    pub _2_inten: crate::Reg<_2_inten::_2_INTEN_SPEC>,
    #[doc = "0xc8 - PWM2 Raw Interrupt Status"]
    pub _2_ris: crate::Reg<_2_ris::_2_RIS_SPEC>,
    #[doc = "0xcc - PWM2 Interrupt Status and Clear"]
    pub _2_isc: crate::Reg<_2_isc::_2_ISC_SPEC>,
    #[doc = "0xd0 - PWM2 Load"]
    pub _2_load: crate::Reg<_2_load::_2_LOAD_SPEC>,
    #[doc = "0xd4 - PWM2 Counter"]
    pub _2_count: crate::Reg<_2_count::_2_COUNT_SPEC>,
    #[doc = "0xd8 - PWM2 Compare A"]
    pub _2_cmpa: crate::Reg<_2_cmpa::_2_CMPA_SPEC>,
    #[doc = "0xdc - PWM2 Compare B"]
    pub _2_cmpb: crate::Reg<_2_cmpb::_2_CMPB_SPEC>,
    #[doc = "0xe0 - PWM2 Generator A Control"]
    pub _2_gena: crate::Reg<_2_gena::_2_GENA_SPEC>,
    #[doc = "0xe4 - PWM2 Generator B Control"]
    pub _2_genb: crate::Reg<_2_genb::_2_GENB_SPEC>,
    #[doc = "0xe8 - PWM2 Dead-Band Control"]
    pub _2_dbctl: crate::Reg<_2_dbctl::_2_DBCTL_SPEC>,
    #[doc = "0xec - PWM2 Dead-Band Rising-Edge Delay"]
    pub _2_dbrise: crate::Reg<_2_dbrise::_2_DBRISE_SPEC>,
    #[doc = "0xf0 - PWM2 Dead-Band Falling-Edge-Delay"]
    pub _2_dbfall: crate::Reg<_2_dbfall::_2_DBFALL_SPEC>,
    #[doc = "0xf4 - PWM2 Fault Source 0"]
    pub _2_fltsrc0: crate::Reg<_2_fltsrc0::_2_FLTSRC0_SPEC>,
    #[doc = "0xf8 - PWM2 Fault Source 1"]
    pub _2_fltsrc1: crate::Reg<_2_fltsrc1::_2_FLTSRC1_SPEC>,
    #[doc = "0xfc - PWM2 Minimum Fault Period"]
    pub _2_minfltper: crate::Reg<_2_minfltper::_2_MINFLTPER_SPEC>,
    #[doc = "0x100 - PWM3 Control"]
    pub _3_ctl: crate::Reg<_3_ctl::_3_CTL_SPEC>,
    #[doc = "0x104 - PWM3 Interrupt and Trigger Enable"]
    pub _3_inten: crate::Reg<_3_inten::_3_INTEN_SPEC>,
    #[doc = "0x108 - PWM3 Raw Interrupt Status"]
    pub _3_ris: crate::Reg<_3_ris::_3_RIS_SPEC>,
    #[doc = "0x10c - PWM3 Interrupt Status and Clear"]
    pub _3_isc: crate::Reg<_3_isc::_3_ISC_SPEC>,
    #[doc = "0x110 - PWM3 Load"]
    pub _3_load: crate::Reg<_3_load::_3_LOAD_SPEC>,
    #[doc = "0x114 - PWM3 Counter"]
    pub _3_count: crate::Reg<_3_count::_3_COUNT_SPEC>,
    #[doc = "0x118 - PWM3 Compare A"]
    pub _3_cmpa: crate::Reg<_3_cmpa::_3_CMPA_SPEC>,
    #[doc = "0x11c - PWM3 Compare B"]
    pub _3_cmpb: crate::Reg<_3_cmpb::_3_CMPB_SPEC>,
    #[doc = "0x120 - PWM3 Generator A Control"]
    pub _3_gena: crate::Reg<_3_gena::_3_GENA_SPEC>,
    #[doc = "0x124 - PWM3 Generator B Control"]
    pub _3_genb: crate::Reg<_3_genb::_3_GENB_SPEC>,
    #[doc = "0x128 - PWM3 Dead-Band Control"]
    pub _3_dbctl: crate::Reg<_3_dbctl::_3_DBCTL_SPEC>,
    #[doc = "0x12c - PWM3 Dead-Band Rising-Edge Delay"]
    pub _3_dbrise: crate::Reg<_3_dbrise::_3_DBRISE_SPEC>,
    #[doc = "0x130 - PWM3 Dead-Band Falling-Edge-Delay"]
    pub _3_dbfall: crate::Reg<_3_dbfall::_3_DBFALL_SPEC>,
    #[doc = "0x134 - PWM3 Fault Source 0"]
    pub _3_fltsrc0: crate::Reg<_3_fltsrc0::_3_FLTSRC0_SPEC>,
    #[doc = "0x138 - PWM3 Fault Source 1"]
    pub _3_fltsrc1: crate::Reg<_3_fltsrc1::_3_FLTSRC1_SPEC>,
    #[doc = "0x13c - PWM3 Minimum Fault Period"]
    pub _3_minfltper: crate::Reg<_3_minfltper::_3_MINFLTPER_SPEC>,
    _reserved75: [u8; 0x06c0],
    #[doc = "0x800 - PWM0 Fault Pin Logic Sense"]
    pub _0_fltsen: crate::Reg<_0_fltsen::_0_FLTSEN_SPEC>,
    #[doc = "0x804 - PWM0 Fault Status 0"]
    pub _0_fltstat0: crate::Reg<_0_fltstat0::_0_FLTSTAT0_SPEC>,
    #[doc = "0x808 - PWM0 Fault Status 1"]
    pub _0_fltstat1: crate::Reg<_0_fltstat1::_0_FLTSTAT1_SPEC>,
    _reserved78: [u8; 0x74],
    #[doc = "0x880 - PWM1 Fault Pin Logic Sense"]
    pub _1_fltsen: crate::Reg<_1_fltsen::_1_FLTSEN_SPEC>,
    #[doc = "0x884 - PWM1 Fault Status 0"]
    pub _1_fltstat0: crate::Reg<_1_fltstat0::_1_FLTSTAT0_SPEC>,
    #[doc = "0x888 - PWM1 Fault Status 1"]
    pub _1_fltstat1: crate::Reg<_1_fltstat1::_1_FLTSTAT1_SPEC>,
    _reserved81: [u8; 0x74],
    #[doc = "0x900 - PWM2 Fault Pin Logic Sense"]
    pub _2_fltsen: crate::Reg<_2_fltsen::_2_FLTSEN_SPEC>,
    #[doc = "0x904 - PWM2 Fault Status 0"]
    pub _2_fltstat0: crate::Reg<_2_fltstat0::_2_FLTSTAT0_SPEC>,
    #[doc = "0x908 - PWM2 Fault Status 1"]
    pub _2_fltstat1: crate::Reg<_2_fltstat1::_2_FLTSTAT1_SPEC>,
    _reserved84: [u8; 0x74],
    #[doc = "0x980 - PWM3 Fault Pin Logic Sense"]
    pub _3_fltsen: crate::Reg<_3_fltsen::_3_FLTSEN_SPEC>,
    #[doc = "0x984 - PWM3 Fault Status 0"]
    pub _3_fltstat0: crate::Reg<_3_fltstat0::_3_FLTSTAT0_SPEC>,
    #[doc = "0x988 - PWM3 Fault Status 1"]
    pub _3_fltstat1: crate::Reg<_3_fltstat1::_3_FLTSTAT1_SPEC>,
    _reserved87: [u8; 0x0634],
    #[doc = "0xfc0 - PWM Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    _reserved88: [u8; 0x04],
    #[doc = "0xfc8 - PWM Clock Configuration"]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "PWM Master Control"]
pub mod ctl;
#[doc = "SYNC register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "PWM Time Base Sync"]
pub mod sync;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "PWM Output Enable"]
pub mod enable;
#[doc = "INVERT register accessor: an alias for `Reg<INVERT_SPEC>`"]
pub type INVERT = crate::Reg<invert::INVERT_SPEC>;
#[doc = "PWM Output Inversion"]
pub mod invert;
#[doc = "FAULT register accessor: an alias for `Reg<FAULT_SPEC>`"]
pub type FAULT = crate::Reg<fault::FAULT_SPEC>;
#[doc = "PWM Output Fault"]
pub mod fault;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "PWM Interrupt Enable"]
pub mod inten;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "PWM Raw Interrupt Status"]
pub mod ris;
#[doc = "ISC register accessor: an alias for `Reg<ISC_SPEC>`"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "PWM Interrupt Status and Clear"]
pub mod isc;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "PWM Status"]
pub mod status;
#[doc = "FAULTVAL register accessor: an alias for `Reg<FAULTVAL_SPEC>`"]
pub type FAULTVAL = crate::Reg<faultval::FAULTVAL_SPEC>;
#[doc = "PWM Fault Condition Value"]
pub mod faultval;
#[doc = "ENUPD register accessor: an alias for `Reg<ENUPD_SPEC>`"]
pub type ENUPD = crate::Reg<enupd::ENUPD_SPEC>;
#[doc = "PWM Enable Update"]
pub mod enupd;
#[doc = "_0_CTL register accessor: an alias for `Reg<_0_CTL_SPEC>`"]
pub type _0_CTL = crate::Reg<_0_ctl::_0_CTL_SPEC>;
#[doc = "PWM0 Control"]
pub mod _0_ctl;
#[doc = "_0_INTEN register accessor: an alias for `Reg<_0_INTEN_SPEC>`"]
pub type _0_INTEN = crate::Reg<_0_inten::_0_INTEN_SPEC>;
#[doc = "PWM0 Interrupt and Trigger Enable"]
pub mod _0_inten;
#[doc = "_0_RIS register accessor: an alias for `Reg<_0_RIS_SPEC>`"]
pub type _0_RIS = crate::Reg<_0_ris::_0_RIS_SPEC>;
#[doc = "PWM0 Raw Interrupt Status"]
pub mod _0_ris;
#[doc = "_0_ISC register accessor: an alias for `Reg<_0_ISC_SPEC>`"]
pub type _0_ISC = crate::Reg<_0_isc::_0_ISC_SPEC>;
#[doc = "PWM0 Interrupt Status and Clear"]
pub mod _0_isc;
#[doc = "_0_LOAD register accessor: an alias for `Reg<_0_LOAD_SPEC>`"]
pub type _0_LOAD = crate::Reg<_0_load::_0_LOAD_SPEC>;
#[doc = "PWM0 Load"]
pub mod _0_load;
#[doc = "_0_COUNT register accessor: an alias for `Reg<_0_COUNT_SPEC>`"]
pub type _0_COUNT = crate::Reg<_0_count::_0_COUNT_SPEC>;
#[doc = "PWM0 Counter"]
pub mod _0_count;
#[doc = "_0_CMPA register accessor: an alias for `Reg<_0_CMPA_SPEC>`"]
pub type _0_CMPA = crate::Reg<_0_cmpa::_0_CMPA_SPEC>;
#[doc = "PWM0 Compare A"]
pub mod _0_cmpa;
#[doc = "_0_CMPB register accessor: an alias for `Reg<_0_CMPB_SPEC>`"]
pub type _0_CMPB = crate::Reg<_0_cmpb::_0_CMPB_SPEC>;
#[doc = "PWM0 Compare B"]
pub mod _0_cmpb;
#[doc = "_0_GENA register accessor: an alias for `Reg<_0_GENA_SPEC>`"]
pub type _0_GENA = crate::Reg<_0_gena::_0_GENA_SPEC>;
#[doc = "PWM0 Generator A Control"]
pub mod _0_gena;
#[doc = "_0_GENB register accessor: an alias for `Reg<_0_GENB_SPEC>`"]
pub type _0_GENB = crate::Reg<_0_genb::_0_GENB_SPEC>;
#[doc = "PWM0 Generator B Control"]
pub mod _0_genb;
#[doc = "_0_DBCTL register accessor: an alias for `Reg<_0_DBCTL_SPEC>`"]
pub type _0_DBCTL = crate::Reg<_0_dbctl::_0_DBCTL_SPEC>;
#[doc = "PWM0 Dead-Band Control"]
pub mod _0_dbctl;
#[doc = "_0_DBRISE register accessor: an alias for `Reg<_0_DBRISE_SPEC>`"]
pub type _0_DBRISE = crate::Reg<_0_dbrise::_0_DBRISE_SPEC>;
#[doc = "PWM0 Dead-Band Rising-Edge Delay"]
pub mod _0_dbrise;
#[doc = "_0_DBFALL register accessor: an alias for `Reg<_0_DBFALL_SPEC>`"]
pub type _0_DBFALL = crate::Reg<_0_dbfall::_0_DBFALL_SPEC>;
#[doc = "PWM0 Dead-Band Falling-Edge-Delay"]
pub mod _0_dbfall;
#[doc = "_0_FLTSRC0 register accessor: an alias for `Reg<_0_FLTSRC0_SPEC>`"]
pub type _0_FLTSRC0 = crate::Reg<_0_fltsrc0::_0_FLTSRC0_SPEC>;
#[doc = "PWM0 Fault Source 0"]
pub mod _0_fltsrc0;
#[doc = "_0_FLTSRC1 register accessor: an alias for `Reg<_0_FLTSRC1_SPEC>`"]
pub type _0_FLTSRC1 = crate::Reg<_0_fltsrc1::_0_FLTSRC1_SPEC>;
#[doc = "PWM0 Fault Source 1"]
pub mod _0_fltsrc1;
#[doc = "_0_MINFLTPER register accessor: an alias for `Reg<_0_MINFLTPER_SPEC>`"]
pub type _0_MINFLTPER = crate::Reg<_0_minfltper::_0_MINFLTPER_SPEC>;
#[doc = "PWM0 Minimum Fault Period"]
pub mod _0_minfltper;
#[doc = "_1_CTL register accessor: an alias for `Reg<_1_CTL_SPEC>`"]
pub type _1_CTL = crate::Reg<_1_ctl::_1_CTL_SPEC>;
#[doc = "PWM1 Control"]
pub mod _1_ctl;
#[doc = "_1_INTEN register accessor: an alias for `Reg<_1_INTEN_SPEC>`"]
pub type _1_INTEN = crate::Reg<_1_inten::_1_INTEN_SPEC>;
#[doc = "PWM1 Interrupt and Trigger Enable"]
pub mod _1_inten;
#[doc = "_1_RIS register accessor: an alias for `Reg<_1_RIS_SPEC>`"]
pub type _1_RIS = crate::Reg<_1_ris::_1_RIS_SPEC>;
#[doc = "PWM1 Raw Interrupt Status"]
pub mod _1_ris;
#[doc = "_1_ISC register accessor: an alias for `Reg<_1_ISC_SPEC>`"]
pub type _1_ISC = crate::Reg<_1_isc::_1_ISC_SPEC>;
#[doc = "PWM1 Interrupt Status and Clear"]
pub mod _1_isc;
#[doc = "_1_LOAD register accessor: an alias for `Reg<_1_LOAD_SPEC>`"]
pub type _1_LOAD = crate::Reg<_1_load::_1_LOAD_SPEC>;
#[doc = "PWM1 Load"]
pub mod _1_load;
#[doc = "_1_COUNT register accessor: an alias for `Reg<_1_COUNT_SPEC>`"]
pub type _1_COUNT = crate::Reg<_1_count::_1_COUNT_SPEC>;
#[doc = "PWM1 Counter"]
pub mod _1_count;
#[doc = "_1_CMPA register accessor: an alias for `Reg<_1_CMPA_SPEC>`"]
pub type _1_CMPA = crate::Reg<_1_cmpa::_1_CMPA_SPEC>;
#[doc = "PWM1 Compare A"]
pub mod _1_cmpa;
#[doc = "_1_CMPB register accessor: an alias for `Reg<_1_CMPB_SPEC>`"]
pub type _1_CMPB = crate::Reg<_1_cmpb::_1_CMPB_SPEC>;
#[doc = "PWM1 Compare B"]
pub mod _1_cmpb;
#[doc = "_1_GENA register accessor: an alias for `Reg<_1_GENA_SPEC>`"]
pub type _1_GENA = crate::Reg<_1_gena::_1_GENA_SPEC>;
#[doc = "PWM1 Generator A Control"]
pub mod _1_gena;
#[doc = "_1_GENB register accessor: an alias for `Reg<_1_GENB_SPEC>`"]
pub type _1_GENB = crate::Reg<_1_genb::_1_GENB_SPEC>;
#[doc = "PWM1 Generator B Control"]
pub mod _1_genb;
#[doc = "_1_DBCTL register accessor: an alias for `Reg<_1_DBCTL_SPEC>`"]
pub type _1_DBCTL = crate::Reg<_1_dbctl::_1_DBCTL_SPEC>;
#[doc = "PWM1 Dead-Band Control"]
pub mod _1_dbctl;
#[doc = "_1_DBRISE register accessor: an alias for `Reg<_1_DBRISE_SPEC>`"]
pub type _1_DBRISE = crate::Reg<_1_dbrise::_1_DBRISE_SPEC>;
#[doc = "PWM1 Dead-Band Rising-Edge Delay"]
pub mod _1_dbrise;
#[doc = "_1_DBFALL register accessor: an alias for `Reg<_1_DBFALL_SPEC>`"]
pub type _1_DBFALL = crate::Reg<_1_dbfall::_1_DBFALL_SPEC>;
#[doc = "PWM1 Dead-Band Falling-Edge-Delay"]
pub mod _1_dbfall;
#[doc = "_1_FLTSRC0 register accessor: an alias for `Reg<_1_FLTSRC0_SPEC>`"]
pub type _1_FLTSRC0 = crate::Reg<_1_fltsrc0::_1_FLTSRC0_SPEC>;
#[doc = "PWM1 Fault Source 0"]
pub mod _1_fltsrc0;
#[doc = "_1_FLTSRC1 register accessor: an alias for `Reg<_1_FLTSRC1_SPEC>`"]
pub type _1_FLTSRC1 = crate::Reg<_1_fltsrc1::_1_FLTSRC1_SPEC>;
#[doc = "PWM1 Fault Source 1"]
pub mod _1_fltsrc1;
#[doc = "_1_MINFLTPER register accessor: an alias for `Reg<_1_MINFLTPER_SPEC>`"]
pub type _1_MINFLTPER = crate::Reg<_1_minfltper::_1_MINFLTPER_SPEC>;
#[doc = "PWM1 Minimum Fault Period"]
pub mod _1_minfltper;
#[doc = "_2_CTL register accessor: an alias for `Reg<_2_CTL_SPEC>`"]
pub type _2_CTL = crate::Reg<_2_ctl::_2_CTL_SPEC>;
#[doc = "PWM2 Control"]
pub mod _2_ctl;
#[doc = "_2_INTEN register accessor: an alias for `Reg<_2_INTEN_SPEC>`"]
pub type _2_INTEN = crate::Reg<_2_inten::_2_INTEN_SPEC>;
#[doc = "PWM2 Interrupt and Trigger Enable"]
pub mod _2_inten;
#[doc = "_2_RIS register accessor: an alias for `Reg<_2_RIS_SPEC>`"]
pub type _2_RIS = crate::Reg<_2_ris::_2_RIS_SPEC>;
#[doc = "PWM2 Raw Interrupt Status"]
pub mod _2_ris;
#[doc = "_2_ISC register accessor: an alias for `Reg<_2_ISC_SPEC>`"]
pub type _2_ISC = crate::Reg<_2_isc::_2_ISC_SPEC>;
#[doc = "PWM2 Interrupt Status and Clear"]
pub mod _2_isc;
#[doc = "_2_LOAD register accessor: an alias for `Reg<_2_LOAD_SPEC>`"]
pub type _2_LOAD = crate::Reg<_2_load::_2_LOAD_SPEC>;
#[doc = "PWM2 Load"]
pub mod _2_load;
#[doc = "_2_COUNT register accessor: an alias for `Reg<_2_COUNT_SPEC>`"]
pub type _2_COUNT = crate::Reg<_2_count::_2_COUNT_SPEC>;
#[doc = "PWM2 Counter"]
pub mod _2_count;
#[doc = "_2_CMPA register accessor: an alias for `Reg<_2_CMPA_SPEC>`"]
pub type _2_CMPA = crate::Reg<_2_cmpa::_2_CMPA_SPEC>;
#[doc = "PWM2 Compare A"]
pub mod _2_cmpa;
#[doc = "_2_CMPB register accessor: an alias for `Reg<_2_CMPB_SPEC>`"]
pub type _2_CMPB = crate::Reg<_2_cmpb::_2_CMPB_SPEC>;
#[doc = "PWM2 Compare B"]
pub mod _2_cmpb;
#[doc = "_2_GENA register accessor: an alias for `Reg<_2_GENA_SPEC>`"]
pub type _2_GENA = crate::Reg<_2_gena::_2_GENA_SPEC>;
#[doc = "PWM2 Generator A Control"]
pub mod _2_gena;
#[doc = "_2_GENB register accessor: an alias for `Reg<_2_GENB_SPEC>`"]
pub type _2_GENB = crate::Reg<_2_genb::_2_GENB_SPEC>;
#[doc = "PWM2 Generator B Control"]
pub mod _2_genb;
#[doc = "_2_DBCTL register accessor: an alias for `Reg<_2_DBCTL_SPEC>`"]
pub type _2_DBCTL = crate::Reg<_2_dbctl::_2_DBCTL_SPEC>;
#[doc = "PWM2 Dead-Band Control"]
pub mod _2_dbctl;
#[doc = "_2_DBRISE register accessor: an alias for `Reg<_2_DBRISE_SPEC>`"]
pub type _2_DBRISE = crate::Reg<_2_dbrise::_2_DBRISE_SPEC>;
#[doc = "PWM2 Dead-Band Rising-Edge Delay"]
pub mod _2_dbrise;
#[doc = "_2_DBFALL register accessor: an alias for `Reg<_2_DBFALL_SPEC>`"]
pub type _2_DBFALL = crate::Reg<_2_dbfall::_2_DBFALL_SPEC>;
#[doc = "PWM2 Dead-Band Falling-Edge-Delay"]
pub mod _2_dbfall;
#[doc = "_2_FLTSRC0 register accessor: an alias for `Reg<_2_FLTSRC0_SPEC>`"]
pub type _2_FLTSRC0 = crate::Reg<_2_fltsrc0::_2_FLTSRC0_SPEC>;
#[doc = "PWM2 Fault Source 0"]
pub mod _2_fltsrc0;
#[doc = "_2_FLTSRC1 register accessor: an alias for `Reg<_2_FLTSRC1_SPEC>`"]
pub type _2_FLTSRC1 = crate::Reg<_2_fltsrc1::_2_FLTSRC1_SPEC>;
#[doc = "PWM2 Fault Source 1"]
pub mod _2_fltsrc1;
#[doc = "_2_MINFLTPER register accessor: an alias for `Reg<_2_MINFLTPER_SPEC>`"]
pub type _2_MINFLTPER = crate::Reg<_2_minfltper::_2_MINFLTPER_SPEC>;
#[doc = "PWM2 Minimum Fault Period"]
pub mod _2_minfltper;
#[doc = "_3_CTL register accessor: an alias for `Reg<_3_CTL_SPEC>`"]
pub type _3_CTL = crate::Reg<_3_ctl::_3_CTL_SPEC>;
#[doc = "PWM3 Control"]
pub mod _3_ctl;
#[doc = "_3_INTEN register accessor: an alias for `Reg<_3_INTEN_SPEC>`"]
pub type _3_INTEN = crate::Reg<_3_inten::_3_INTEN_SPEC>;
#[doc = "PWM3 Interrupt and Trigger Enable"]
pub mod _3_inten;
#[doc = "_3_RIS register accessor: an alias for `Reg<_3_RIS_SPEC>`"]
pub type _3_RIS = crate::Reg<_3_ris::_3_RIS_SPEC>;
#[doc = "PWM3 Raw Interrupt Status"]
pub mod _3_ris;
#[doc = "_3_ISC register accessor: an alias for `Reg<_3_ISC_SPEC>`"]
pub type _3_ISC = crate::Reg<_3_isc::_3_ISC_SPEC>;
#[doc = "PWM3 Interrupt Status and Clear"]
pub mod _3_isc;
#[doc = "_3_LOAD register accessor: an alias for `Reg<_3_LOAD_SPEC>`"]
pub type _3_LOAD = crate::Reg<_3_load::_3_LOAD_SPEC>;
#[doc = "PWM3 Load"]
pub mod _3_load;
#[doc = "_3_COUNT register accessor: an alias for `Reg<_3_COUNT_SPEC>`"]
pub type _3_COUNT = crate::Reg<_3_count::_3_COUNT_SPEC>;
#[doc = "PWM3 Counter"]
pub mod _3_count;
#[doc = "_3_CMPA register accessor: an alias for `Reg<_3_CMPA_SPEC>`"]
pub type _3_CMPA = crate::Reg<_3_cmpa::_3_CMPA_SPEC>;
#[doc = "PWM3 Compare A"]
pub mod _3_cmpa;
#[doc = "_3_CMPB register accessor: an alias for `Reg<_3_CMPB_SPEC>`"]
pub type _3_CMPB = crate::Reg<_3_cmpb::_3_CMPB_SPEC>;
#[doc = "PWM3 Compare B"]
pub mod _3_cmpb;
#[doc = "_3_GENA register accessor: an alias for `Reg<_3_GENA_SPEC>`"]
pub type _3_GENA = crate::Reg<_3_gena::_3_GENA_SPEC>;
#[doc = "PWM3 Generator A Control"]
pub mod _3_gena;
#[doc = "_3_GENB register accessor: an alias for `Reg<_3_GENB_SPEC>`"]
pub type _3_GENB = crate::Reg<_3_genb::_3_GENB_SPEC>;
#[doc = "PWM3 Generator B Control"]
pub mod _3_genb;
#[doc = "_3_DBCTL register accessor: an alias for `Reg<_3_DBCTL_SPEC>`"]
pub type _3_DBCTL = crate::Reg<_3_dbctl::_3_DBCTL_SPEC>;
#[doc = "PWM3 Dead-Band Control"]
pub mod _3_dbctl;
#[doc = "_3_DBRISE register accessor: an alias for `Reg<_3_DBRISE_SPEC>`"]
pub type _3_DBRISE = crate::Reg<_3_dbrise::_3_DBRISE_SPEC>;
#[doc = "PWM3 Dead-Band Rising-Edge Delay"]
pub mod _3_dbrise;
#[doc = "_3_DBFALL register accessor: an alias for `Reg<_3_DBFALL_SPEC>`"]
pub type _3_DBFALL = crate::Reg<_3_dbfall::_3_DBFALL_SPEC>;
#[doc = "PWM3 Dead-Band Falling-Edge-Delay"]
pub mod _3_dbfall;
#[doc = "_3_FLTSRC0 register accessor: an alias for `Reg<_3_FLTSRC0_SPEC>`"]
pub type _3_FLTSRC0 = crate::Reg<_3_fltsrc0::_3_FLTSRC0_SPEC>;
#[doc = "PWM3 Fault Source 0"]
pub mod _3_fltsrc0;
#[doc = "_3_FLTSRC1 register accessor: an alias for `Reg<_3_FLTSRC1_SPEC>`"]
pub type _3_FLTSRC1 = crate::Reg<_3_fltsrc1::_3_FLTSRC1_SPEC>;
#[doc = "PWM3 Fault Source 1"]
pub mod _3_fltsrc1;
#[doc = "_3_MINFLTPER register accessor: an alias for `Reg<_3_MINFLTPER_SPEC>`"]
pub type _3_MINFLTPER = crate::Reg<_3_minfltper::_3_MINFLTPER_SPEC>;
#[doc = "PWM3 Minimum Fault Period"]
pub mod _3_minfltper;
#[doc = "_0_FLTSEN register accessor: an alias for `Reg<_0_FLTSEN_SPEC>`"]
pub type _0_FLTSEN = crate::Reg<_0_fltsen::_0_FLTSEN_SPEC>;
#[doc = "PWM0 Fault Pin Logic Sense"]
pub mod _0_fltsen;
#[doc = "_0_FLTSTAT0 register accessor: an alias for `Reg<_0_FLTSTAT0_SPEC>`"]
pub type _0_FLTSTAT0 = crate::Reg<_0_fltstat0::_0_FLTSTAT0_SPEC>;
#[doc = "PWM0 Fault Status 0"]
pub mod _0_fltstat0;
#[doc = "_0_FLTSTAT1 register accessor: an alias for `Reg<_0_FLTSTAT1_SPEC>`"]
pub type _0_FLTSTAT1 = crate::Reg<_0_fltstat1::_0_FLTSTAT1_SPEC>;
#[doc = "PWM0 Fault Status 1"]
pub mod _0_fltstat1;
#[doc = "_1_FLTSEN register accessor: an alias for `Reg<_1_FLTSEN_SPEC>`"]
pub type _1_FLTSEN = crate::Reg<_1_fltsen::_1_FLTSEN_SPEC>;
#[doc = "PWM1 Fault Pin Logic Sense"]
pub mod _1_fltsen;
#[doc = "_1_FLTSTAT0 register accessor: an alias for `Reg<_1_FLTSTAT0_SPEC>`"]
pub type _1_FLTSTAT0 = crate::Reg<_1_fltstat0::_1_FLTSTAT0_SPEC>;
#[doc = "PWM1 Fault Status 0"]
pub mod _1_fltstat0;
#[doc = "_1_FLTSTAT1 register accessor: an alias for `Reg<_1_FLTSTAT1_SPEC>`"]
pub type _1_FLTSTAT1 = crate::Reg<_1_fltstat1::_1_FLTSTAT1_SPEC>;
#[doc = "PWM1 Fault Status 1"]
pub mod _1_fltstat1;
#[doc = "_2_FLTSEN register accessor: an alias for `Reg<_2_FLTSEN_SPEC>`"]
pub type _2_FLTSEN = crate::Reg<_2_fltsen::_2_FLTSEN_SPEC>;
#[doc = "PWM2 Fault Pin Logic Sense"]
pub mod _2_fltsen;
#[doc = "_2_FLTSTAT0 register accessor: an alias for `Reg<_2_FLTSTAT0_SPEC>`"]
pub type _2_FLTSTAT0 = crate::Reg<_2_fltstat0::_2_FLTSTAT0_SPEC>;
#[doc = "PWM2 Fault Status 0"]
pub mod _2_fltstat0;
#[doc = "_2_FLTSTAT1 register accessor: an alias for `Reg<_2_FLTSTAT1_SPEC>`"]
pub type _2_FLTSTAT1 = crate::Reg<_2_fltstat1::_2_FLTSTAT1_SPEC>;
#[doc = "PWM2 Fault Status 1"]
pub mod _2_fltstat1;
#[doc = "_3_FLTSEN register accessor: an alias for `Reg<_3_FLTSEN_SPEC>`"]
pub type _3_FLTSEN = crate::Reg<_3_fltsen::_3_FLTSEN_SPEC>;
#[doc = "PWM3 Fault Pin Logic Sense"]
pub mod _3_fltsen;
#[doc = "_3_FLTSTAT0 register accessor: an alias for `Reg<_3_FLTSTAT0_SPEC>`"]
pub type _3_FLTSTAT0 = crate::Reg<_3_fltstat0::_3_FLTSTAT0_SPEC>;
#[doc = "PWM3 Fault Status 0"]
pub mod _3_fltstat0;
#[doc = "_3_FLTSTAT1 register accessor: an alias for `Reg<_3_FLTSTAT1_SPEC>`"]
pub type _3_FLTSTAT1 = crate::Reg<_3_fltstat1::_3_FLTSTAT1_SPEC>;
#[doc = "PWM3 Fault Status 1"]
pub mod _3_fltstat1;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "PWM Peripheral Properties"]
pub mod pp;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "PWM Clock Configuration"]
pub mod cc;
