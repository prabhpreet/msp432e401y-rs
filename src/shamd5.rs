#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHA Outer Digest A"]
    pub odigest_a: crate::Reg<odigest_a::ODIGEST_A_SPEC>,
    #[doc = "0x04 - SHA Outer Digest B"]
    pub odigest_b: crate::Reg<odigest_b::ODIGEST_B_SPEC>,
    #[doc = "0x08 - SHA Outer Digest C"]
    pub odigest_c: crate::Reg<odigest_c::ODIGEST_C_SPEC>,
    #[doc = "0x0c - SHA Outer Digest D"]
    pub odigest_d: crate::Reg<odigest_d::ODIGEST_D_SPEC>,
    #[doc = "0x10 - SHA Outer Digest E"]
    pub odigest_e: crate::Reg<odigest_e::ODIGEST_E_SPEC>,
    #[doc = "0x14 - SHA Outer Digest F"]
    pub odigest_f: crate::Reg<odigest_f::ODIGEST_F_SPEC>,
    #[doc = "0x18 - SHA Outer Digest G"]
    pub odigest_g: crate::Reg<odigest_g::ODIGEST_G_SPEC>,
    #[doc = "0x1c - SHA Outer Digest H"]
    pub odigest_h: crate::Reg<odigest_h::ODIGEST_H_SPEC>,
    #[doc = "0x20 - SHA Inner Digest A"]
    pub idigest_a: crate::Reg<idigest_a::IDIGEST_A_SPEC>,
    #[doc = "0x24 - SHA Inner Digest B"]
    pub idigest_b: crate::Reg<idigest_b::IDIGEST_B_SPEC>,
    #[doc = "0x28 - SHA Inner Digest C"]
    pub idigest_c: crate::Reg<idigest_c::IDIGEST_C_SPEC>,
    #[doc = "0x2c - SHA Inner Digest D"]
    pub idigest_d: crate::Reg<idigest_d::IDIGEST_D_SPEC>,
    #[doc = "0x30 - SHA Inner Digest E"]
    pub idigest_e: crate::Reg<idigest_e::IDIGEST_E_SPEC>,
    #[doc = "0x34 - SHA Inner Digest F"]
    pub idigest_f: crate::Reg<idigest_f::IDIGEST_F_SPEC>,
    #[doc = "0x38 - SHA Inner Digest G"]
    pub idigest_g: crate::Reg<idigest_g::IDIGEST_G_SPEC>,
    #[doc = "0x3c - SHA Inner Digest H"]
    pub idigest_h: crate::Reg<idigest_h::IDIGEST_H_SPEC>,
    #[doc = "0x40 - SHA Digest Count"]
    pub digest_count: crate::Reg<digest_count::DIGEST_COUNT_SPEC>,
    #[doc = "0x44 - SHA Mode"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x48 - SHA Length"]
    pub length: crate::Reg<length::LENGTH_SPEC>,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - SHA Data 0 Input"]
    pub data_0_in: crate::Reg<data_0_in::DATA_0_IN_SPEC>,
    #[doc = "0x84 - SHA Data 1 Input"]
    pub data_1_in: crate::Reg<data_1_in::DATA_1_IN_SPEC>,
    #[doc = "0x88 - SHA Data 2 Input"]
    pub data_2_in: crate::Reg<data_2_in::DATA_2_IN_SPEC>,
    #[doc = "0x8c - SHA Data 3 Input"]
    pub data_3_in: crate::Reg<data_3_in::DATA_3_IN_SPEC>,
    #[doc = "0x90 - SHA Data 4 Input"]
    pub data_4_in: crate::Reg<data_4_in::DATA_4_IN_SPEC>,
    #[doc = "0x94 - SHA Data 5 Input"]
    pub data_5_in: crate::Reg<data_5_in::DATA_5_IN_SPEC>,
    #[doc = "0x98 - SHA Data 6 Input"]
    pub data_6_in: crate::Reg<data_6_in::DATA_6_IN_SPEC>,
    #[doc = "0x9c - SHA Data 7 Input"]
    pub data_7_in: crate::Reg<data_7_in::DATA_7_IN_SPEC>,
    #[doc = "0xa0 - SHA Data 8 Input"]
    pub data_8_in: crate::Reg<data_8_in::DATA_8_IN_SPEC>,
    #[doc = "0xa4 - SHA Data 9 Input"]
    pub data_9_in: crate::Reg<data_9_in::DATA_9_IN_SPEC>,
    #[doc = "0xa8 - SHA Data 10 Input"]
    pub data_10_in: crate::Reg<data_10_in::DATA_10_IN_SPEC>,
    #[doc = "0xac - SHA Data 11 Input"]
    pub data_11_in: crate::Reg<data_11_in::DATA_11_IN_SPEC>,
    #[doc = "0xb0 - SHA Data 12 Input"]
    pub data_12_in: crate::Reg<data_12_in::DATA_12_IN_SPEC>,
    #[doc = "0xb4 - SHA Data 13 Input"]
    pub data_13_in: crate::Reg<data_13_in::DATA_13_IN_SPEC>,
    #[doc = "0xb8 - SHA Data 14 Input"]
    pub data_14_in: crate::Reg<data_14_in::DATA_14_IN_SPEC>,
    #[doc = "0xbc - SHA Data 15 Input"]
    pub data_15_in: crate::Reg<data_15_in::DATA_15_IN_SPEC>,
    _reserved35: [u8; 0x40],
    #[doc = "0x100 - SHA Revision"]
    pub revision: crate::Reg<revision::REVISION_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x110 - SHA System Configuration"]
    pub sysconfig: crate::Reg<sysconfig::SYSCONFIG_SPEC>,
    #[doc = "0x114 - SHA System Status"]
    pub sysstatus: crate::Reg<sysstatus::SYSSTATUS_SPEC>,
    #[doc = "0x118 - SHA Interrupt Status"]
    pub irqstatus: crate::Reg<irqstatus::IRQSTATUS_SPEC>,
    #[doc = "0x11c - SHA Interrupt Enable"]
    pub irqenable: crate::Reg<irqenable::IRQENABLE_SPEC>,
}
#[doc = "ODIGEST_A register accessor: an alias for `Reg<ODIGEST_A_SPEC>`"]
pub type ODIGEST_A = crate::Reg<odigest_a::ODIGEST_A_SPEC>;
#[doc = "SHA Outer Digest A"]
pub mod odigest_a;
#[doc = "ODIGEST_B register accessor: an alias for `Reg<ODIGEST_B_SPEC>`"]
pub type ODIGEST_B = crate::Reg<odigest_b::ODIGEST_B_SPEC>;
#[doc = "SHA Outer Digest B"]
pub mod odigest_b;
#[doc = "ODIGEST_C register accessor: an alias for `Reg<ODIGEST_C_SPEC>`"]
pub type ODIGEST_C = crate::Reg<odigest_c::ODIGEST_C_SPEC>;
#[doc = "SHA Outer Digest C"]
pub mod odigest_c;
#[doc = "ODIGEST_D register accessor: an alias for `Reg<ODIGEST_D_SPEC>`"]
pub type ODIGEST_D = crate::Reg<odigest_d::ODIGEST_D_SPEC>;
#[doc = "SHA Outer Digest D"]
pub mod odigest_d;
#[doc = "ODIGEST_E register accessor: an alias for `Reg<ODIGEST_E_SPEC>`"]
pub type ODIGEST_E = crate::Reg<odigest_e::ODIGEST_E_SPEC>;
#[doc = "SHA Outer Digest E"]
pub mod odigest_e;
#[doc = "ODIGEST_F register accessor: an alias for `Reg<ODIGEST_F_SPEC>`"]
pub type ODIGEST_F = crate::Reg<odigest_f::ODIGEST_F_SPEC>;
#[doc = "SHA Outer Digest F"]
pub mod odigest_f;
#[doc = "ODIGEST_G register accessor: an alias for `Reg<ODIGEST_G_SPEC>`"]
pub type ODIGEST_G = crate::Reg<odigest_g::ODIGEST_G_SPEC>;
#[doc = "SHA Outer Digest G"]
pub mod odigest_g;
#[doc = "ODIGEST_H register accessor: an alias for `Reg<ODIGEST_H_SPEC>`"]
pub type ODIGEST_H = crate::Reg<odigest_h::ODIGEST_H_SPEC>;
#[doc = "SHA Outer Digest H"]
pub mod odigest_h;
#[doc = "IDIGEST_A register accessor: an alias for `Reg<IDIGEST_A_SPEC>`"]
pub type IDIGEST_A = crate::Reg<idigest_a::IDIGEST_A_SPEC>;
#[doc = "SHA Inner Digest A"]
pub mod idigest_a;
#[doc = "IDIGEST_B register accessor: an alias for `Reg<IDIGEST_B_SPEC>`"]
pub type IDIGEST_B = crate::Reg<idigest_b::IDIGEST_B_SPEC>;
#[doc = "SHA Inner Digest B"]
pub mod idigest_b;
#[doc = "IDIGEST_C register accessor: an alias for `Reg<IDIGEST_C_SPEC>`"]
pub type IDIGEST_C = crate::Reg<idigest_c::IDIGEST_C_SPEC>;
#[doc = "SHA Inner Digest C"]
pub mod idigest_c;
#[doc = "IDIGEST_D register accessor: an alias for `Reg<IDIGEST_D_SPEC>`"]
pub type IDIGEST_D = crate::Reg<idigest_d::IDIGEST_D_SPEC>;
#[doc = "SHA Inner Digest D"]
pub mod idigest_d;
#[doc = "IDIGEST_E register accessor: an alias for `Reg<IDIGEST_E_SPEC>`"]
pub type IDIGEST_E = crate::Reg<idigest_e::IDIGEST_E_SPEC>;
#[doc = "SHA Inner Digest E"]
pub mod idigest_e;
#[doc = "IDIGEST_F register accessor: an alias for `Reg<IDIGEST_F_SPEC>`"]
pub type IDIGEST_F = crate::Reg<idigest_f::IDIGEST_F_SPEC>;
#[doc = "SHA Inner Digest F"]
pub mod idigest_f;
#[doc = "IDIGEST_G register accessor: an alias for `Reg<IDIGEST_G_SPEC>`"]
pub type IDIGEST_G = crate::Reg<idigest_g::IDIGEST_G_SPEC>;
#[doc = "SHA Inner Digest G"]
pub mod idigest_g;
#[doc = "IDIGEST_H register accessor: an alias for `Reg<IDIGEST_H_SPEC>`"]
pub type IDIGEST_H = crate::Reg<idigest_h::IDIGEST_H_SPEC>;
#[doc = "SHA Inner Digest H"]
pub mod idigest_h;
#[doc = "DIGEST_COUNT register accessor: an alias for `Reg<DIGEST_COUNT_SPEC>`"]
pub type DIGEST_COUNT = crate::Reg<digest_count::DIGEST_COUNT_SPEC>;
#[doc = "SHA Digest Count"]
pub mod digest_count;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "SHA Mode"]
pub mod mode;
#[doc = "LENGTH register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "SHA Length"]
pub mod length;
#[doc = "DATA_0_IN register accessor: an alias for `Reg<DATA_0_IN_SPEC>`"]
pub type DATA_0_IN = crate::Reg<data_0_in::DATA_0_IN_SPEC>;
#[doc = "SHA Data 0 Input"]
pub mod data_0_in;
#[doc = "DATA_1_IN register accessor: an alias for `Reg<DATA_1_IN_SPEC>`"]
pub type DATA_1_IN = crate::Reg<data_1_in::DATA_1_IN_SPEC>;
#[doc = "SHA Data 1 Input"]
pub mod data_1_in;
#[doc = "DATA_2_IN register accessor: an alias for `Reg<DATA_2_IN_SPEC>`"]
pub type DATA_2_IN = crate::Reg<data_2_in::DATA_2_IN_SPEC>;
#[doc = "SHA Data 2 Input"]
pub mod data_2_in;
#[doc = "DATA_3_IN register accessor: an alias for `Reg<DATA_3_IN_SPEC>`"]
pub type DATA_3_IN = crate::Reg<data_3_in::DATA_3_IN_SPEC>;
#[doc = "SHA Data 3 Input"]
pub mod data_3_in;
#[doc = "DATA_4_IN register accessor: an alias for `Reg<DATA_4_IN_SPEC>`"]
pub type DATA_4_IN = crate::Reg<data_4_in::DATA_4_IN_SPEC>;
#[doc = "SHA Data 4 Input"]
pub mod data_4_in;
#[doc = "DATA_5_IN register accessor: an alias for `Reg<DATA_5_IN_SPEC>`"]
pub type DATA_5_IN = crate::Reg<data_5_in::DATA_5_IN_SPEC>;
#[doc = "SHA Data 5 Input"]
pub mod data_5_in;
#[doc = "DATA_6_IN register accessor: an alias for `Reg<DATA_6_IN_SPEC>`"]
pub type DATA_6_IN = crate::Reg<data_6_in::DATA_6_IN_SPEC>;
#[doc = "SHA Data 6 Input"]
pub mod data_6_in;
#[doc = "DATA_7_IN register accessor: an alias for `Reg<DATA_7_IN_SPEC>`"]
pub type DATA_7_IN = crate::Reg<data_7_in::DATA_7_IN_SPEC>;
#[doc = "SHA Data 7 Input"]
pub mod data_7_in;
#[doc = "DATA_8_IN register accessor: an alias for `Reg<DATA_8_IN_SPEC>`"]
pub type DATA_8_IN = crate::Reg<data_8_in::DATA_8_IN_SPEC>;
#[doc = "SHA Data 8 Input"]
pub mod data_8_in;
#[doc = "DATA_9_IN register accessor: an alias for `Reg<DATA_9_IN_SPEC>`"]
pub type DATA_9_IN = crate::Reg<data_9_in::DATA_9_IN_SPEC>;
#[doc = "SHA Data 9 Input"]
pub mod data_9_in;
#[doc = "DATA_10_IN register accessor: an alias for `Reg<DATA_10_IN_SPEC>`"]
pub type DATA_10_IN = crate::Reg<data_10_in::DATA_10_IN_SPEC>;
#[doc = "SHA Data 10 Input"]
pub mod data_10_in;
#[doc = "DATA_11_IN register accessor: an alias for `Reg<DATA_11_IN_SPEC>`"]
pub type DATA_11_IN = crate::Reg<data_11_in::DATA_11_IN_SPEC>;
#[doc = "SHA Data 11 Input"]
pub mod data_11_in;
#[doc = "DATA_12_IN register accessor: an alias for `Reg<DATA_12_IN_SPEC>`"]
pub type DATA_12_IN = crate::Reg<data_12_in::DATA_12_IN_SPEC>;
#[doc = "SHA Data 12 Input"]
pub mod data_12_in;
#[doc = "DATA_13_IN register accessor: an alias for `Reg<DATA_13_IN_SPEC>`"]
pub type DATA_13_IN = crate::Reg<data_13_in::DATA_13_IN_SPEC>;
#[doc = "SHA Data 13 Input"]
pub mod data_13_in;
#[doc = "DATA_14_IN register accessor: an alias for `Reg<DATA_14_IN_SPEC>`"]
pub type DATA_14_IN = crate::Reg<data_14_in::DATA_14_IN_SPEC>;
#[doc = "SHA Data 14 Input"]
pub mod data_14_in;
#[doc = "DATA_15_IN register accessor: an alias for `Reg<DATA_15_IN_SPEC>`"]
pub type DATA_15_IN = crate::Reg<data_15_in::DATA_15_IN_SPEC>;
#[doc = "SHA Data 15 Input"]
pub mod data_15_in;
#[doc = "REVISION register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "SHA Revision"]
pub mod revision;
#[doc = "SYSCONFIG register accessor: an alias for `Reg<SYSCONFIG_SPEC>`"]
pub type SYSCONFIG = crate::Reg<sysconfig::SYSCONFIG_SPEC>;
#[doc = "SHA System Configuration"]
pub mod sysconfig;
#[doc = "SYSSTATUS register accessor: an alias for `Reg<SYSSTATUS_SPEC>`"]
pub type SYSSTATUS = crate::Reg<sysstatus::SYSSTATUS_SPEC>;
#[doc = "SHA System Status"]
pub mod sysstatus;
#[doc = "IRQSTATUS register accessor: an alias for `Reg<IRQSTATUS_SPEC>`"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "SHA Interrupt Status"]
pub mod irqstatus;
#[doc = "IRQENABLE register accessor: an alias for `Reg<IRQENABLE_SPEC>`"]
pub type IRQENABLE = crate::Reg<irqenable::IRQENABLE_SPEC>;
#[doc = "SHA Interrupt Enable"]
pub mod irqenable;
