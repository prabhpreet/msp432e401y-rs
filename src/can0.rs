#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x04 - CAN Status"]
    pub sts: crate::Reg<sts::STS_SPEC>,
    #[doc = "0x08 - CAN Error Counter"]
    pub err: crate::Reg<err::ERR_SPEC>,
    #[doc = "0x0c - CAN Bit Timing"]
    pub bit_: crate::Reg<bit_::BIT_SPEC>,
    #[doc = "0x10 - CAN Interrupt"]
    pub int: crate::Reg<int::INT_SPEC>,
    #[doc = "0x14 - CAN Test"]
    pub tst: crate::Reg<tst::TST_SPEC>,
    #[doc = "0x18 - CAN Baud Rate Prescaler Extension"]
    pub brpe: crate::Reg<brpe::BRPE_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - CAN IF1 Command Request"]
    pub if1crq: crate::Reg<if1crq::IF1CRQ_SPEC>,
    _reserved_8_if1cmsk: [u8; 0x04],
    #[doc = "0x28 - CAN IF1 Mask 1"]
    pub if1msk1: crate::Reg<if1msk1::IF1MSK1_SPEC>,
    #[doc = "0x2c - CAN IF1 Mask 2"]
    pub if1msk2: crate::Reg<if1msk2::IF1MSK2_SPEC>,
    #[doc = "0x30 - CAN IF1 Arbitration 1"]
    pub if1arb1: crate::Reg<if1arb1::IF1ARB1_SPEC>,
    #[doc = "0x34 - CAN IF1 Arbitration 2"]
    pub if1arb2: crate::Reg<if1arb2::IF1ARB2_SPEC>,
    #[doc = "0x38 - CAN IF1 Message Control"]
    pub if1mctl: crate::Reg<if1mctl::IF1MCTL_SPEC>,
    #[doc = "0x3c - CAN IF1 Data A1"]
    pub if1da1: crate::Reg<if1da1::IF1DA1_SPEC>,
    #[doc = "0x40 - CAN IF1 Data A2"]
    pub if1da2: crate::Reg<if1da2::IF1DA2_SPEC>,
    #[doc = "0x44 - CAN IF1 Data B1"]
    pub if1db1: crate::Reg<if1db1::IF1DB1_SPEC>,
    #[doc = "0x48 - CAN IF1 Data B2"]
    pub if1db2: crate::Reg<if1db2::IF1DB2_SPEC>,
    _reserved18: [u8; 0x34],
    #[doc = "0x80 - CAN IF2 Command Request"]
    pub if2crq: crate::Reg<if2crq::IF2CRQ_SPEC>,
    _reserved_19_if2cmsk: [u8; 0x04],
    #[doc = "0x88 - CAN IF2 Mask 1"]
    pub if2msk1: crate::Reg<if2msk1::IF2MSK1_SPEC>,
    #[doc = "0x8c - CAN IF2 Mask 2"]
    pub if2msk2: crate::Reg<if2msk2::IF2MSK2_SPEC>,
    #[doc = "0x90 - CAN IF2 Arbitration 1"]
    pub if2arb1: crate::Reg<if2arb1::IF2ARB1_SPEC>,
    #[doc = "0x94 - CAN IF2 Arbitration 2"]
    pub if2arb2: crate::Reg<if2arb2::IF2ARB2_SPEC>,
    #[doc = "0x98 - CAN IF2 Message Control"]
    pub if2mctl: crate::Reg<if2mctl::IF2MCTL_SPEC>,
    #[doc = "0x9c - CAN IF2 Data A1"]
    pub if2da1: crate::Reg<if2da1::IF2DA1_SPEC>,
    #[doc = "0xa0 - CAN IF2 Data A2"]
    pub if2da2: crate::Reg<if2da2::IF2DA2_SPEC>,
    #[doc = "0xa4 - CAN IF2 Data B1"]
    pub if2db1: crate::Reg<if2db1::IF2DB1_SPEC>,
    #[doc = "0xa8 - CAN IF2 Data B2"]
    pub if2db2: crate::Reg<if2db2::IF2DB2_SPEC>,
    _reserved29: [u8; 0x54],
    #[doc = "0x100 - CAN Transmission Request 1"]
    pub txrq1: crate::Reg<txrq1::TXRQ1_SPEC>,
    #[doc = "0x104 - CAN Transmission Request 2"]
    pub txrq2: crate::Reg<txrq2::TXRQ2_SPEC>,
    _reserved31: [u8; 0x18],
    #[doc = "0x120 - CAN New Data 1"]
    pub nwda1: crate::Reg<nwda1::NWDA1_SPEC>,
    #[doc = "0x124 - CAN New Data 2"]
    pub nwda2: crate::Reg<nwda2::NWDA2_SPEC>,
    _reserved33: [u8; 0x18],
    #[doc = "0x140 - CAN Message 1 Interrupt Pending"]
    pub msg1int: crate::Reg<msg1int::MSG1INT_SPEC>,
    #[doc = "0x144 - CAN Message 2 Interrupt Pending"]
    pub msg2int: crate::Reg<msg2int::MSG2INT_SPEC>,
    _reserved35: [u8; 0x18],
    #[doc = "0x160 - CAN Message 1 Valid"]
    pub msg1val: crate::Reg<msg1val::MSG1VAL_SPEC>,
    #[doc = "0x164 - CAN Message 2 Valid"]
    pub msg2val: crate::Reg<msg2val::MSG2VAL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x24 - CAN IF1 Command Mask"]
    #[inline(always)]
    pub fn can0_alt_if1cmsk(&self) -> &crate::Reg<can0_alt_if1cmsk::CAN0_ALT_IF1CMSK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<can0_alt_if1cmsk::CAN0_ALT_IF1CMSK_SPEC>)
        }
    }
    #[doc = "0x24 - CAN IF1 Command Mask"]
    #[inline(always)]
    pub fn if1cmsk(&self) -> &crate::Reg<if1cmsk::IF1CMSK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<if1cmsk::IF1CMSK_SPEC>)
        }
    }
    #[doc = "0x84 - CAN IF2 Command Mask"]
    #[inline(always)]
    pub fn can0_alt_if2cmsk(&self) -> &crate::Reg<can0_alt_if2cmsk::CAN0_ALT_IF2CMSK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const crate::Reg<can0_alt_if2cmsk::CAN0_ALT_IF2CMSK_SPEC>)
        }
    }
    #[doc = "0x84 - CAN IF2 Command Mask"]
    #[inline(always)]
    pub fn if2cmsk(&self) -> &crate::Reg<if2cmsk::IF2CMSK_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(132usize)
                as *const crate::Reg<if2cmsk::IF2CMSK_SPEC>)
        }
    }
}
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "CAN Control"]
pub mod ctl;
#[doc = "STS register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "CAN Status"]
pub mod sts;
#[doc = "ERR register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "CAN Error Counter"]
pub mod err;
#[doc = "BIT register accessor: an alias for `Reg<BIT_SPEC>`"]
pub type BIT = crate::Reg<bit_::BIT_SPEC>;
#[doc = "CAN Bit Timing"]
pub mod bit_;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "CAN Interrupt"]
pub mod int;
#[doc = "TST register accessor: an alias for `Reg<TST_SPEC>`"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "CAN Test"]
pub mod tst;
#[doc = "BRPE register accessor: an alias for `Reg<BRPE_SPEC>`"]
pub type BRPE = crate::Reg<brpe::BRPE_SPEC>;
#[doc = "CAN Baud Rate Prescaler Extension"]
pub mod brpe;
#[doc = "IF1CRQ register accessor: an alias for `Reg<IF1CRQ_SPEC>`"]
pub type IF1CRQ = crate::Reg<if1crq::IF1CRQ_SPEC>;
#[doc = "CAN IF1 Command Request"]
pub mod if1crq;
#[doc = "IF1CMSK register accessor: an alias for `Reg<IF1CMSK_SPEC>`"]
pub type IF1CMSK = crate::Reg<if1cmsk::IF1CMSK_SPEC>;
#[doc = "CAN IF1 Command Mask"]
pub mod if1cmsk;
#[doc = "CAN0_ALT_IF1CMSK register accessor: an alias for `Reg<CAN0_ALT_IF1CMSK_SPEC>`"]
pub type CAN0_ALT_IF1CMSK = crate::Reg<can0_alt_if1cmsk::CAN0_ALT_IF1CMSK_SPEC>;
#[doc = "CAN IF1 Command Mask"]
pub mod can0_alt_if1cmsk;
#[doc = "IF1MSK1 register accessor: an alias for `Reg<IF1MSK1_SPEC>`"]
pub type IF1MSK1 = crate::Reg<if1msk1::IF1MSK1_SPEC>;
#[doc = "CAN IF1 Mask 1"]
pub mod if1msk1;
#[doc = "IF1MSK2 register accessor: an alias for `Reg<IF1MSK2_SPEC>`"]
pub type IF1MSK2 = crate::Reg<if1msk2::IF1MSK2_SPEC>;
#[doc = "CAN IF1 Mask 2"]
pub mod if1msk2;
#[doc = "IF1ARB1 register accessor: an alias for `Reg<IF1ARB1_SPEC>`"]
pub type IF1ARB1 = crate::Reg<if1arb1::IF1ARB1_SPEC>;
#[doc = "CAN IF1 Arbitration 1"]
pub mod if1arb1;
#[doc = "IF1ARB2 register accessor: an alias for `Reg<IF1ARB2_SPEC>`"]
pub type IF1ARB2 = crate::Reg<if1arb2::IF1ARB2_SPEC>;
#[doc = "CAN IF1 Arbitration 2"]
pub mod if1arb2;
#[doc = "IF1MCTL register accessor: an alias for `Reg<IF1MCTL_SPEC>`"]
pub type IF1MCTL = crate::Reg<if1mctl::IF1MCTL_SPEC>;
#[doc = "CAN IF1 Message Control"]
pub mod if1mctl;
#[doc = "IF1DA1 register accessor: an alias for `Reg<IF1DA1_SPEC>`"]
pub type IF1DA1 = crate::Reg<if1da1::IF1DA1_SPEC>;
#[doc = "CAN IF1 Data A1"]
pub mod if1da1;
#[doc = "IF1DA2 register accessor: an alias for `Reg<IF1DA2_SPEC>`"]
pub type IF1DA2 = crate::Reg<if1da2::IF1DA2_SPEC>;
#[doc = "CAN IF1 Data A2"]
pub mod if1da2;
#[doc = "IF1DB1 register accessor: an alias for `Reg<IF1DB1_SPEC>`"]
pub type IF1DB1 = crate::Reg<if1db1::IF1DB1_SPEC>;
#[doc = "CAN IF1 Data B1"]
pub mod if1db1;
#[doc = "IF1DB2 register accessor: an alias for `Reg<IF1DB2_SPEC>`"]
pub type IF1DB2 = crate::Reg<if1db2::IF1DB2_SPEC>;
#[doc = "CAN IF1 Data B2"]
pub mod if1db2;
#[doc = "IF2CRQ register accessor: an alias for `Reg<IF2CRQ_SPEC>`"]
pub type IF2CRQ = crate::Reg<if2crq::IF2CRQ_SPEC>;
#[doc = "CAN IF2 Command Request"]
pub mod if2crq;
#[doc = "IF2CMSK register accessor: an alias for `Reg<IF2CMSK_SPEC>`"]
pub type IF2CMSK = crate::Reg<if2cmsk::IF2CMSK_SPEC>;
#[doc = "CAN IF2 Command Mask"]
pub mod if2cmsk;
#[doc = "CAN0_ALT_IF2CMSK register accessor: an alias for `Reg<CAN0_ALT_IF2CMSK_SPEC>`"]
pub type CAN0_ALT_IF2CMSK = crate::Reg<can0_alt_if2cmsk::CAN0_ALT_IF2CMSK_SPEC>;
#[doc = "CAN IF2 Command Mask"]
pub mod can0_alt_if2cmsk;
#[doc = "IF2MSK1 register accessor: an alias for `Reg<IF2MSK1_SPEC>`"]
pub type IF2MSK1 = crate::Reg<if2msk1::IF2MSK1_SPEC>;
#[doc = "CAN IF2 Mask 1"]
pub mod if2msk1;
#[doc = "IF2MSK2 register accessor: an alias for `Reg<IF2MSK2_SPEC>`"]
pub type IF2MSK2 = crate::Reg<if2msk2::IF2MSK2_SPEC>;
#[doc = "CAN IF2 Mask 2"]
pub mod if2msk2;
#[doc = "IF2ARB1 register accessor: an alias for `Reg<IF2ARB1_SPEC>`"]
pub type IF2ARB1 = crate::Reg<if2arb1::IF2ARB1_SPEC>;
#[doc = "CAN IF2 Arbitration 1"]
pub mod if2arb1;
#[doc = "IF2ARB2 register accessor: an alias for `Reg<IF2ARB2_SPEC>`"]
pub type IF2ARB2 = crate::Reg<if2arb2::IF2ARB2_SPEC>;
#[doc = "CAN IF2 Arbitration 2"]
pub mod if2arb2;
#[doc = "IF2MCTL register accessor: an alias for `Reg<IF2MCTL_SPEC>`"]
pub type IF2MCTL = crate::Reg<if2mctl::IF2MCTL_SPEC>;
#[doc = "CAN IF2 Message Control"]
pub mod if2mctl;
#[doc = "IF2DA1 register accessor: an alias for `Reg<IF2DA1_SPEC>`"]
pub type IF2DA1 = crate::Reg<if2da1::IF2DA1_SPEC>;
#[doc = "CAN IF2 Data A1"]
pub mod if2da1;
#[doc = "IF2DA2 register accessor: an alias for `Reg<IF2DA2_SPEC>`"]
pub type IF2DA2 = crate::Reg<if2da2::IF2DA2_SPEC>;
#[doc = "CAN IF2 Data A2"]
pub mod if2da2;
#[doc = "IF2DB1 register accessor: an alias for `Reg<IF2DB1_SPEC>`"]
pub type IF2DB1 = crate::Reg<if2db1::IF2DB1_SPEC>;
#[doc = "CAN IF2 Data B1"]
pub mod if2db1;
#[doc = "IF2DB2 register accessor: an alias for `Reg<IF2DB2_SPEC>`"]
pub type IF2DB2 = crate::Reg<if2db2::IF2DB2_SPEC>;
#[doc = "CAN IF2 Data B2"]
pub mod if2db2;
#[doc = "TXRQ1 register accessor: an alias for `Reg<TXRQ1_SPEC>`"]
pub type TXRQ1 = crate::Reg<txrq1::TXRQ1_SPEC>;
#[doc = "CAN Transmission Request 1"]
pub mod txrq1;
#[doc = "TXRQ2 register accessor: an alias for `Reg<TXRQ2_SPEC>`"]
pub type TXRQ2 = crate::Reg<txrq2::TXRQ2_SPEC>;
#[doc = "CAN Transmission Request 2"]
pub mod txrq2;
#[doc = "NWDA1 register accessor: an alias for `Reg<NWDA1_SPEC>`"]
pub type NWDA1 = crate::Reg<nwda1::NWDA1_SPEC>;
#[doc = "CAN New Data 1"]
pub mod nwda1;
#[doc = "NWDA2 register accessor: an alias for `Reg<NWDA2_SPEC>`"]
pub type NWDA2 = crate::Reg<nwda2::NWDA2_SPEC>;
#[doc = "CAN New Data 2"]
pub mod nwda2;
#[doc = "MSG1INT register accessor: an alias for `Reg<MSG1INT_SPEC>`"]
pub type MSG1INT = crate::Reg<msg1int::MSG1INT_SPEC>;
#[doc = "CAN Message 1 Interrupt Pending"]
pub mod msg1int;
#[doc = "MSG2INT register accessor: an alias for `Reg<MSG2INT_SPEC>`"]
pub type MSG2INT = crate::Reg<msg2int::MSG2INT_SPEC>;
#[doc = "CAN Message 2 Interrupt Pending"]
pub mod msg2int;
#[doc = "MSG1VAL register accessor: an alias for `Reg<MSG1VAL_SPEC>`"]
pub type MSG1VAL = crate::Reg<msg1val::MSG1VAL_SPEC>;
#[doc = "CAN Message 1 Valid"]
pub mod msg1val;
#[doc = "MSG2VAL register accessor: an alias for `Reg<MSG2VAL_SPEC>`"]
pub type MSG2VAL = crate::Reg<msg2val::MSG2VAL_SPEC>;
#[doc = "CAN Message 2 Valid"]
pub mod msg2val;
