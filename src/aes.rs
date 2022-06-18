#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Key 2_6"]
    pub key2_6: crate::Reg<key2_6::KEY2_6_SPEC>,
    #[doc = "0x04 - AES Key 2_7"]
    pub key2_7: crate::Reg<key2_7::KEY2_7_SPEC>,
    #[doc = "0x08 - AES Key 2_4"]
    pub key2_4: crate::Reg<key2_4::KEY2_4_SPEC>,
    #[doc = "0x0c - AES Key 2_5"]
    pub key2_5: crate::Reg<key2_5::KEY2_5_SPEC>,
    #[doc = "0x10 - AES Key 2_2"]
    pub key2_2: crate::Reg<key2_2::KEY2_2_SPEC>,
    #[doc = "0x14 - AES Key 2_3"]
    pub key2_3: crate::Reg<key2_3::KEY2_3_SPEC>,
    #[doc = "0x18 - AES Key 2_0"]
    pub key2_0: crate::Reg<key2_0::KEY2_0_SPEC>,
    #[doc = "0x1c - AES Key 2_1"]
    pub key2_1: crate::Reg<key2_1::KEY2_1_SPEC>,
    #[doc = "0x20 - AES Key 1_6"]
    pub key1_6: crate::Reg<key1_6::KEY1_6_SPEC>,
    #[doc = "0x24 - AES Key 1_7"]
    pub key1_7: crate::Reg<key1_7::KEY1_7_SPEC>,
    #[doc = "0x28 - AES Key 1_4"]
    pub key1_4: crate::Reg<key1_4::KEY1_4_SPEC>,
    #[doc = "0x2c - AES Key 1_5"]
    pub key1_5: crate::Reg<key1_5::KEY1_5_SPEC>,
    #[doc = "0x30 - AES Key 1_2"]
    pub key1_2: crate::Reg<key1_2::KEY1_2_SPEC>,
    #[doc = "0x34 - AES Key 1_3"]
    pub key1_3: crate::Reg<key1_3::KEY1_3_SPEC>,
    #[doc = "0x38 - AES Key 1_0"]
    pub key1_0: crate::Reg<key1_0::KEY1_0_SPEC>,
    #[doc = "0x3c - AES Key 1_1"]
    pub key1_1: crate::Reg<key1_1::KEY1_1_SPEC>,
    #[doc = "0x40 - AES Initialization Vector Input 0"]
    pub iv_in_0: crate::Reg<iv_in_0::IV_IN_0_SPEC>,
    #[doc = "0x44 - AES Initialization Vector Input 1"]
    pub iv_in_1: crate::Reg<iv_in_1::IV_IN_1_SPEC>,
    #[doc = "0x48 - AES Initialization Vector Input 2"]
    pub iv_in_2: crate::Reg<iv_in_2::IV_IN_2_SPEC>,
    #[doc = "0x4c - AES Initialization Vector Input 3"]
    pub iv_in_3: crate::Reg<iv_in_3::IV_IN_3_SPEC>,
    #[doc = "0x50 - AES Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x54 - AES Crypto Data Length 0"]
    pub c_length_0: crate::Reg<c_length_0::C_LENGTH_0_SPEC>,
    #[doc = "0x58 - AES Crypto Data Length 1"]
    pub c_length_1: crate::Reg<c_length_1::C_LENGTH_1_SPEC>,
    #[doc = "0x5c - AES Authentication Data Length"]
    pub auth_length: crate::Reg<auth_length::AUTH_LENGTH_SPEC>,
    #[doc = "0x60 - AES Data RW Plaintext/Ciphertext 0"]
    pub data_in_0: crate::Reg<data_in_0::DATA_IN_0_SPEC>,
    #[doc = "0x64 - AES Data RW Plaintext/Ciphertext 1"]
    pub data_in_1: crate::Reg<data_in_1::DATA_IN_1_SPEC>,
    #[doc = "0x68 - AES Data RW Plaintext/Ciphertext 2"]
    pub data_in_2: crate::Reg<data_in_2::DATA_IN_2_SPEC>,
    #[doc = "0x6c - AES Data RW Plaintext/Ciphertext 3"]
    pub data_in_3: crate::Reg<data_in_3::DATA_IN_3_SPEC>,
    #[doc = "0x70 - AES Hash Tag Out 0"]
    pub tag_out_0: crate::Reg<tag_out_0::TAG_OUT_0_SPEC>,
    #[doc = "0x74 - AES Hash Tag Out 1"]
    pub tag_out_1: crate::Reg<tag_out_1::TAG_OUT_1_SPEC>,
    #[doc = "0x78 - AES Hash Tag Out 2"]
    pub tag_out_2: crate::Reg<tag_out_2::TAG_OUT_2_SPEC>,
    #[doc = "0x7c - AES Hash Tag Out 3"]
    pub tag_out_3: crate::Reg<tag_out_3::TAG_OUT_3_SPEC>,
    #[doc = "0x80 - AES IP Revision Identifier"]
    pub revision: crate::Reg<revision::REVISION_SPEC>,
    #[doc = "0x84 - AES System Configuration"]
    pub sysconfig: crate::Reg<sysconfig::SYSCONFIG_SPEC>,
    #[doc = "0x88 - AES System Status"]
    pub sysstatus: crate::Reg<sysstatus::SYSSTATUS_SPEC>,
    #[doc = "0x8c - AES Interrupt Status"]
    pub irqstatus: crate::Reg<irqstatus::IRQSTATUS_SPEC>,
    #[doc = "0x90 - AES Interrupt Enable"]
    pub irqenable: crate::Reg<irqenable::IRQENABLE_SPEC>,
    #[doc = "0x94 - AES Dirty Bits"]
    pub dirtybits: crate::Reg<dirtybits::DIRTYBITS_SPEC>,
}
#[doc = "KEY2_6 register accessor: an alias for `Reg<KEY2_6_SPEC>`"]
pub type KEY2_6 = crate::Reg<key2_6::KEY2_6_SPEC>;
#[doc = "AES Key 2_6"]
pub mod key2_6;
#[doc = "KEY2_7 register accessor: an alias for `Reg<KEY2_7_SPEC>`"]
pub type KEY2_7 = crate::Reg<key2_7::KEY2_7_SPEC>;
#[doc = "AES Key 2_7"]
pub mod key2_7;
#[doc = "KEY2_4 register accessor: an alias for `Reg<KEY2_4_SPEC>`"]
pub type KEY2_4 = crate::Reg<key2_4::KEY2_4_SPEC>;
#[doc = "AES Key 2_4"]
pub mod key2_4;
#[doc = "KEY2_5 register accessor: an alias for `Reg<KEY2_5_SPEC>`"]
pub type KEY2_5 = crate::Reg<key2_5::KEY2_5_SPEC>;
#[doc = "AES Key 2_5"]
pub mod key2_5;
#[doc = "KEY2_2 register accessor: an alias for `Reg<KEY2_2_SPEC>`"]
pub type KEY2_2 = crate::Reg<key2_2::KEY2_2_SPEC>;
#[doc = "AES Key 2_2"]
pub mod key2_2;
#[doc = "KEY2_3 register accessor: an alias for `Reg<KEY2_3_SPEC>`"]
pub type KEY2_3 = crate::Reg<key2_3::KEY2_3_SPEC>;
#[doc = "AES Key 2_3"]
pub mod key2_3;
#[doc = "KEY2_0 register accessor: an alias for `Reg<KEY2_0_SPEC>`"]
pub type KEY2_0 = crate::Reg<key2_0::KEY2_0_SPEC>;
#[doc = "AES Key 2_0"]
pub mod key2_0;
#[doc = "KEY2_1 register accessor: an alias for `Reg<KEY2_1_SPEC>`"]
pub type KEY2_1 = crate::Reg<key2_1::KEY2_1_SPEC>;
#[doc = "AES Key 2_1"]
pub mod key2_1;
#[doc = "KEY1_6 register accessor: an alias for `Reg<KEY1_6_SPEC>`"]
pub type KEY1_6 = crate::Reg<key1_6::KEY1_6_SPEC>;
#[doc = "AES Key 1_6"]
pub mod key1_6;
#[doc = "KEY1_7 register accessor: an alias for `Reg<KEY1_7_SPEC>`"]
pub type KEY1_7 = crate::Reg<key1_7::KEY1_7_SPEC>;
#[doc = "AES Key 1_7"]
pub mod key1_7;
#[doc = "KEY1_4 register accessor: an alias for `Reg<KEY1_4_SPEC>`"]
pub type KEY1_4 = crate::Reg<key1_4::KEY1_4_SPEC>;
#[doc = "AES Key 1_4"]
pub mod key1_4;
#[doc = "KEY1_5 register accessor: an alias for `Reg<KEY1_5_SPEC>`"]
pub type KEY1_5 = crate::Reg<key1_5::KEY1_5_SPEC>;
#[doc = "AES Key 1_5"]
pub mod key1_5;
#[doc = "KEY1_2 register accessor: an alias for `Reg<KEY1_2_SPEC>`"]
pub type KEY1_2 = crate::Reg<key1_2::KEY1_2_SPEC>;
#[doc = "AES Key 1_2"]
pub mod key1_2;
#[doc = "KEY1_3 register accessor: an alias for `Reg<KEY1_3_SPEC>`"]
pub type KEY1_3 = crate::Reg<key1_3::KEY1_3_SPEC>;
#[doc = "AES Key 1_3"]
pub mod key1_3;
#[doc = "KEY1_0 register accessor: an alias for `Reg<KEY1_0_SPEC>`"]
pub type KEY1_0 = crate::Reg<key1_0::KEY1_0_SPEC>;
#[doc = "AES Key 1_0"]
pub mod key1_0;
#[doc = "KEY1_1 register accessor: an alias for `Reg<KEY1_1_SPEC>`"]
pub type KEY1_1 = crate::Reg<key1_1::KEY1_1_SPEC>;
#[doc = "AES Key 1_1"]
pub mod key1_1;
#[doc = "IV_IN_0 register accessor: an alias for `Reg<IV_IN_0_SPEC>`"]
pub type IV_IN_0 = crate::Reg<iv_in_0::IV_IN_0_SPEC>;
#[doc = "AES Initialization Vector Input 0"]
pub mod iv_in_0;
#[doc = "IV_IN_1 register accessor: an alias for `Reg<IV_IN_1_SPEC>`"]
pub type IV_IN_1 = crate::Reg<iv_in_1::IV_IN_1_SPEC>;
#[doc = "AES Initialization Vector Input 1"]
pub mod iv_in_1;
#[doc = "IV_IN_2 register accessor: an alias for `Reg<IV_IN_2_SPEC>`"]
pub type IV_IN_2 = crate::Reg<iv_in_2::IV_IN_2_SPEC>;
#[doc = "AES Initialization Vector Input 2"]
pub mod iv_in_2;
#[doc = "IV_IN_3 register accessor: an alias for `Reg<IV_IN_3_SPEC>`"]
pub type IV_IN_3 = crate::Reg<iv_in_3::IV_IN_3_SPEC>;
#[doc = "AES Initialization Vector Input 3"]
pub mod iv_in_3;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "AES Control"]
pub mod ctrl;
#[doc = "C_LENGTH_0 register accessor: an alias for `Reg<C_LENGTH_0_SPEC>`"]
pub type C_LENGTH_0 = crate::Reg<c_length_0::C_LENGTH_0_SPEC>;
#[doc = "AES Crypto Data Length 0"]
pub mod c_length_0;
#[doc = "C_LENGTH_1 register accessor: an alias for `Reg<C_LENGTH_1_SPEC>`"]
pub type C_LENGTH_1 = crate::Reg<c_length_1::C_LENGTH_1_SPEC>;
#[doc = "AES Crypto Data Length 1"]
pub mod c_length_1;
#[doc = "AUTH_LENGTH register accessor: an alias for `Reg<AUTH_LENGTH_SPEC>`"]
pub type AUTH_LENGTH = crate::Reg<auth_length::AUTH_LENGTH_SPEC>;
#[doc = "AES Authentication Data Length"]
pub mod auth_length;
#[doc = "DATA_IN_0 register accessor: an alias for `Reg<DATA_IN_0_SPEC>`"]
pub type DATA_IN_0 = crate::Reg<data_in_0::DATA_IN_0_SPEC>;
#[doc = "AES Data RW Plaintext/Ciphertext 0"]
pub mod data_in_0;
#[doc = "DATA_IN_1 register accessor: an alias for `Reg<DATA_IN_1_SPEC>`"]
pub type DATA_IN_1 = crate::Reg<data_in_1::DATA_IN_1_SPEC>;
#[doc = "AES Data RW Plaintext/Ciphertext 1"]
pub mod data_in_1;
#[doc = "DATA_IN_2 register accessor: an alias for `Reg<DATA_IN_2_SPEC>`"]
pub type DATA_IN_2 = crate::Reg<data_in_2::DATA_IN_2_SPEC>;
#[doc = "AES Data RW Plaintext/Ciphertext 2"]
pub mod data_in_2;
#[doc = "DATA_IN_3 register accessor: an alias for `Reg<DATA_IN_3_SPEC>`"]
pub type DATA_IN_3 = crate::Reg<data_in_3::DATA_IN_3_SPEC>;
#[doc = "AES Data RW Plaintext/Ciphertext 3"]
pub mod data_in_3;
#[doc = "TAG_OUT_0 register accessor: an alias for `Reg<TAG_OUT_0_SPEC>`"]
pub type TAG_OUT_0 = crate::Reg<tag_out_0::TAG_OUT_0_SPEC>;
#[doc = "AES Hash Tag Out 0"]
pub mod tag_out_0;
#[doc = "TAG_OUT_1 register accessor: an alias for `Reg<TAG_OUT_1_SPEC>`"]
pub type TAG_OUT_1 = crate::Reg<tag_out_1::TAG_OUT_1_SPEC>;
#[doc = "AES Hash Tag Out 1"]
pub mod tag_out_1;
#[doc = "TAG_OUT_2 register accessor: an alias for `Reg<TAG_OUT_2_SPEC>`"]
pub type TAG_OUT_2 = crate::Reg<tag_out_2::TAG_OUT_2_SPEC>;
#[doc = "AES Hash Tag Out 2"]
pub mod tag_out_2;
#[doc = "TAG_OUT_3 register accessor: an alias for `Reg<TAG_OUT_3_SPEC>`"]
pub type TAG_OUT_3 = crate::Reg<tag_out_3::TAG_OUT_3_SPEC>;
#[doc = "AES Hash Tag Out 3"]
pub mod tag_out_3;
#[doc = "REVISION register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "AES IP Revision Identifier"]
pub mod revision;
#[doc = "SYSCONFIG register accessor: an alias for `Reg<SYSCONFIG_SPEC>`"]
pub type SYSCONFIG = crate::Reg<sysconfig::SYSCONFIG_SPEC>;
#[doc = "AES System Configuration"]
pub mod sysconfig;
#[doc = "SYSSTATUS register accessor: an alias for `Reg<SYSSTATUS_SPEC>`"]
pub type SYSSTATUS = crate::Reg<sysstatus::SYSSTATUS_SPEC>;
#[doc = "AES System Status"]
pub mod sysstatus;
#[doc = "IRQSTATUS register accessor: an alias for `Reg<IRQSTATUS_SPEC>`"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "AES Interrupt Status"]
pub mod irqstatus;
#[doc = "IRQENABLE register accessor: an alias for `Reg<IRQENABLE_SPEC>`"]
pub type IRQENABLE = crate::Reg<irqenable::IRQENABLE_SPEC>;
#[doc = "AES Interrupt Enable"]
pub mod irqenable;
#[doc = "DIRTYBITS register accessor: an alias for `Reg<DIRTYBITS_SPEC>`"]
pub type DIRTYBITS = crate::Reg<dirtybits::DIRTYBITS_SPEC>;
#[doc = "AES Dirty Bits"]
pub mod dirtybits;
