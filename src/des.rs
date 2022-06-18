#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DES Key 3 LSW for 192-Bit Key"]
    pub key3_l: crate::Reg<key3_l::KEY3_L_SPEC>,
    #[doc = "0x04 - DES Key 3 MSW for 192-Bit Key"]
    pub key3_h: crate::Reg<key3_h::KEY3_H_SPEC>,
    #[doc = "0x08 - DES Key 2 LSW for 128-Bit Key"]
    pub key2_l: crate::Reg<key2_l::KEY2_L_SPEC>,
    #[doc = "0x0c - DES Key 2 MSW for 128-Bit Key"]
    pub key2_h: crate::Reg<key2_h::KEY2_H_SPEC>,
    #[doc = "0x10 - DES Key 1 LSW for 64-Bit Key"]
    pub key1_l: crate::Reg<key1_l::KEY1_L_SPEC>,
    #[doc = "0x14 - DES Key 1 MSW for 64-Bit Key"]
    pub key1_h: crate::Reg<key1_h::KEY1_H_SPEC>,
    #[doc = "0x18 - DES Initialization Vector"]
    pub iv_l: crate::Reg<iv_l::IV_L_SPEC>,
    #[doc = "0x1c - DES Initialization Vector"]
    pub iv_h: crate::Reg<iv_h::IV_H_SPEC>,
    #[doc = "0x20 - DES Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x24 - DES Cryptographic Data Length"]
    pub length: crate::Reg<length::LENGTH_SPEC>,
    #[doc = "0x28 - DES LSW Data RW"]
    pub data_l: crate::Reg<data_l::DATA_L_SPEC>,
    #[doc = "0x2c - DES MSW Data RW"]
    pub data_h: crate::Reg<data_h::DATA_H_SPEC>,
    #[doc = "0x30 - DES Revision Number"]
    pub revision: crate::Reg<revision::REVISION_SPEC>,
    #[doc = "0x34 - DES System Configuration"]
    pub sysconfig: crate::Reg<sysconfig::SYSCONFIG_SPEC>,
    #[doc = "0x38 - DES System Status"]
    pub sysstatus: crate::Reg<sysstatus::SYSSTATUS_SPEC>,
    #[doc = "0x3c - DES Interrupt Status"]
    pub irqstatus: crate::Reg<irqstatus::IRQSTATUS_SPEC>,
    #[doc = "0x40 - DES Interrupt Enable"]
    pub irqenable: crate::Reg<irqenable::IRQENABLE_SPEC>,
    #[doc = "0x44 - DES Dirty Bits"]
    pub dirtybits: crate::Reg<dirtybits::DIRTYBITS_SPEC>,
}
#[doc = "KEY3_L register accessor: an alias for `Reg<KEY3_L_SPEC>`"]
pub type KEY3_L = crate::Reg<key3_l::KEY3_L_SPEC>;
#[doc = "DES Key 3 LSW for 192-Bit Key"]
pub mod key3_l;
#[doc = "KEY3_H register accessor: an alias for `Reg<KEY3_H_SPEC>`"]
pub type KEY3_H = crate::Reg<key3_h::KEY3_H_SPEC>;
#[doc = "DES Key 3 MSW for 192-Bit Key"]
pub mod key3_h;
#[doc = "KEY2_L register accessor: an alias for `Reg<KEY2_L_SPEC>`"]
pub type KEY2_L = crate::Reg<key2_l::KEY2_L_SPEC>;
#[doc = "DES Key 2 LSW for 128-Bit Key"]
pub mod key2_l;
#[doc = "KEY2_H register accessor: an alias for `Reg<KEY2_H_SPEC>`"]
pub type KEY2_H = crate::Reg<key2_h::KEY2_H_SPEC>;
#[doc = "DES Key 2 MSW for 128-Bit Key"]
pub mod key2_h;
#[doc = "KEY1_L register accessor: an alias for `Reg<KEY1_L_SPEC>`"]
pub type KEY1_L = crate::Reg<key1_l::KEY1_L_SPEC>;
#[doc = "DES Key 1 LSW for 64-Bit Key"]
pub mod key1_l;
#[doc = "KEY1_H register accessor: an alias for `Reg<KEY1_H_SPEC>`"]
pub type KEY1_H = crate::Reg<key1_h::KEY1_H_SPEC>;
#[doc = "DES Key 1 MSW for 64-Bit Key"]
pub mod key1_h;
#[doc = "IV_L register accessor: an alias for `Reg<IV_L_SPEC>`"]
pub type IV_L = crate::Reg<iv_l::IV_L_SPEC>;
#[doc = "DES Initialization Vector"]
pub mod iv_l;
#[doc = "IV_H register accessor: an alias for `Reg<IV_H_SPEC>`"]
pub type IV_H = crate::Reg<iv_h::IV_H_SPEC>;
#[doc = "DES Initialization Vector"]
pub mod iv_h;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "DES Control"]
pub mod ctrl;
#[doc = "LENGTH register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "DES Cryptographic Data Length"]
pub mod length;
#[doc = "DATA_L register accessor: an alias for `Reg<DATA_L_SPEC>`"]
pub type DATA_L = crate::Reg<data_l::DATA_L_SPEC>;
#[doc = "DES LSW Data RW"]
pub mod data_l;
#[doc = "DATA_H register accessor: an alias for `Reg<DATA_H_SPEC>`"]
pub type DATA_H = crate::Reg<data_h::DATA_H_SPEC>;
#[doc = "DES MSW Data RW"]
pub mod data_h;
#[doc = "REVISION register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "DES Revision Number"]
pub mod revision;
#[doc = "SYSCONFIG register accessor: an alias for `Reg<SYSCONFIG_SPEC>`"]
pub type SYSCONFIG = crate::Reg<sysconfig::SYSCONFIG_SPEC>;
#[doc = "DES System Configuration"]
pub mod sysconfig;
#[doc = "SYSSTATUS register accessor: an alias for `Reg<SYSSTATUS_SPEC>`"]
pub type SYSSTATUS = crate::Reg<sysstatus::SYSSTATUS_SPEC>;
#[doc = "DES System Status"]
pub mod sysstatus;
#[doc = "IRQSTATUS register accessor: an alias for `Reg<IRQSTATUS_SPEC>`"]
pub type IRQSTATUS = crate::Reg<irqstatus::IRQSTATUS_SPEC>;
#[doc = "DES Interrupt Status"]
pub mod irqstatus;
#[doc = "IRQENABLE register accessor: an alias for `Reg<IRQENABLE_SPEC>`"]
pub type IRQENABLE = crate::Reg<irqenable::IRQENABLE_SPEC>;
#[doc = "DES Interrupt Enable"]
pub mod irqenable;
#[doc = "DIRTYBITS register accessor: an alias for `Reg<DIRTYBITS_SPEC>`"]
pub type DIRTYBITS = crate::Reg<dirtybits::DIRTYBITS_SPEC>;
#[doc = "DES Dirty Bits"]
pub mod dirtybits;
