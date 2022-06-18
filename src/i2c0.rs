#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Master Slave Address"]
    pub msa: crate::Reg<msa::MSA_SPEC>,
    _reserved_1_mcs: [u8; 0x04],
    #[doc = "0x08 - I2C Master Data"]
    pub mdr: crate::Reg<mdr::MDR_SPEC>,
    #[doc = "0x0c - I2C Master Timer Period"]
    pub mtpr: crate::Reg<mtpr::MTPR_SPEC>,
    #[doc = "0x10 - I2C Master Interrupt Mask"]
    pub mimr: crate::Reg<mimr::MIMR_SPEC>,
    #[doc = "0x14 - I2C Master Raw Interrupt Status"]
    pub mris: crate::Reg<mris::MRIS_SPEC>,
    #[doc = "0x18 - I2C Master Masked Interrupt Status"]
    pub mmis: crate::Reg<mmis::MMIS_SPEC>,
    #[doc = "0x1c - I2C Master Interrupt Clear"]
    pub micr: crate::Reg<micr::MICR_SPEC>,
    #[doc = "0x20 - I2C Master Configuration"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x24 - I2C Master Clock Low Timeout Count"]
    pub mclkocnt: crate::Reg<mclkocnt::MCLKOCNT_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - I2C Master Bus Monitor"]
    pub mbmon: crate::Reg<mbmon::MBMON_SPEC>,
    #[doc = "0x30 - I2C Master Burst Length"]
    pub mblen: crate::Reg<mblen::MBLEN_SPEC>,
    #[doc = "0x34 - I2C Master Burst Count"]
    pub mbcnt: crate::Reg<mbcnt::MBCNT_SPEC>,
    _reserved13: [u8; 0x07c8],
    #[doc = "0x800 - I2C Slave Own Address"]
    pub soar: crate::Reg<soar::SOAR_SPEC>,
    _reserved_14_scsr: [u8; 0x04],
    #[doc = "0x808 - I2C Slave Data"]
    pub sdr: crate::Reg<sdr::SDR_SPEC>,
    #[doc = "0x80c - I2C Slave Interrupt Mask"]
    pub simr: crate::Reg<simr::SIMR_SPEC>,
    #[doc = "0x810 - I2C Slave Raw Interrupt Status"]
    pub sris: crate::Reg<sris::SRIS_SPEC>,
    #[doc = "0x814 - I2C Slave Masked Interrupt Status"]
    pub smis: crate::Reg<smis::SMIS_SPEC>,
    #[doc = "0x818 - I2C Slave Interrupt Clear"]
    pub sicr: crate::Reg<sicr::SICR_SPEC>,
    #[doc = "0x81c - I2C Slave Own Address 2"]
    pub soar2: crate::Reg<soar2::SOAR2_SPEC>,
    #[doc = "0x820 - I2C Slave ACK Control"]
    pub sackctl: crate::Reg<sackctl::SACKCTL_SPEC>,
    _reserved22: [u8; 0x06dc],
    #[doc = "0xf00 - I2C FIFO Data"]
    pub fifodata: crate::Reg<fifodata::FIFODATA_SPEC>,
    #[doc = "0xf04 - I2C FIFO Control"]
    pub fifoctl: crate::Reg<fifoctl::FIFOCTL_SPEC>,
    #[doc = "0xf08 - I2C FIFO Status"]
    pub fifostatus: crate::Reg<fifostatus::FIFOSTATUS_SPEC>,
    _reserved25: [u8; 0xb4],
    #[doc = "0xfc0 - I2C Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - I2C Peripheral Configuration"]
    pub pc: crate::Reg<pc::PC_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn i2c0_alt_mcs(&self) -> &crate::Reg<i2c0_alt_mcs::I2C0_ALT_MCS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<i2c0_alt_mcs::I2C0_ALT_MCS_SPEC>)
        }
    }
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn mcs(&self) -> &crate::Reg<mcs::MCS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<mcs::MCS_SPEC>)
        }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn i2c0_alt_scsr(&self) -> &crate::Reg<i2c0_alt_scsr::I2C0_ALT_SCSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<i2c0_alt_scsr::I2C0_ALT_SCSR_SPEC>)
        }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn scsr(&self) -> &crate::Reg<scsr::SCSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2052usize)
                as *const crate::Reg<scsr::SCSR_SPEC>)
        }
    }
}
#[doc = "MSA register accessor: an alias for `Reg<MSA_SPEC>`"]
pub type MSA = crate::Reg<msa::MSA_SPEC>;
#[doc = "I2C Master Slave Address"]
pub mod msa;
#[doc = "MCS register accessor: an alias for `Reg<MCS_SPEC>`"]
pub type MCS = crate::Reg<mcs::MCS_SPEC>;
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C0_ALT_MCS register accessor: an alias for `Reg<I2C0_ALT_MCS_SPEC>`"]
pub type I2C0_ALT_MCS = crate::Reg<i2c0_alt_mcs::I2C0_ALT_MCS_SPEC>;
#[doc = "I2C Master Control/Status"]
pub mod i2c0_alt_mcs;
#[doc = "MDR register accessor: an alias for `Reg<MDR_SPEC>`"]
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
#[doc = "I2C Master Data"]
pub mod mdr;
#[doc = "MTPR register accessor: an alias for `Reg<MTPR_SPEC>`"]
pub type MTPR = crate::Reg<mtpr::MTPR_SPEC>;
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "MIMR register accessor: an alias for `Reg<MIMR_SPEC>`"]
pub type MIMR = crate::Reg<mimr::MIMR_SPEC>;
#[doc = "I2C Master Interrupt Mask"]
pub mod mimr;
#[doc = "MRIS register accessor: an alias for `Reg<MRIS_SPEC>`"]
pub type MRIS = crate::Reg<mris::MRIS_SPEC>;
#[doc = "I2C Master Raw Interrupt Status"]
pub mod mris;
#[doc = "MMIS register accessor: an alias for `Reg<MMIS_SPEC>`"]
pub type MMIS = crate::Reg<mmis::MMIS_SPEC>;
#[doc = "I2C Master Masked Interrupt Status"]
pub mod mmis;
#[doc = "MICR register accessor: an alias for `Reg<MICR_SPEC>`"]
pub type MICR = crate::Reg<micr::MICR_SPEC>;
#[doc = "I2C Master Interrupt Clear"]
pub mod micr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "I2C Master Configuration"]
pub mod mcr;
#[doc = "MCLKOCNT register accessor: an alias for `Reg<MCLKOCNT_SPEC>`"]
pub type MCLKOCNT = crate::Reg<mclkocnt::MCLKOCNT_SPEC>;
#[doc = "I2C Master Clock Low Timeout Count"]
pub mod mclkocnt;
#[doc = "MBMON register accessor: an alias for `Reg<MBMON_SPEC>`"]
pub type MBMON = crate::Reg<mbmon::MBMON_SPEC>;
#[doc = "I2C Master Bus Monitor"]
pub mod mbmon;
#[doc = "MBLEN register accessor: an alias for `Reg<MBLEN_SPEC>`"]
pub type MBLEN = crate::Reg<mblen::MBLEN_SPEC>;
#[doc = "I2C Master Burst Length"]
pub mod mblen;
#[doc = "MBCNT register accessor: an alias for `Reg<MBCNT_SPEC>`"]
pub type MBCNT = crate::Reg<mbcnt::MBCNT_SPEC>;
#[doc = "I2C Master Burst Count"]
pub mod mbcnt;
#[doc = "SOAR register accessor: an alias for `Reg<SOAR_SPEC>`"]
pub type SOAR = crate::Reg<soar::SOAR_SPEC>;
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "SCSR register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C0_ALT_SCSR register accessor: an alias for `Reg<I2C0_ALT_SCSR_SPEC>`"]
pub type I2C0_ALT_SCSR = crate::Reg<i2c0_alt_scsr::I2C0_ALT_SCSR_SPEC>;
#[doc = "I2C Slave Control/Status"]
pub mod i2c0_alt_scsr;
#[doc = "SDR register accessor: an alias for `Reg<SDR_SPEC>`"]
pub type SDR = crate::Reg<sdr::SDR_SPEC>;
#[doc = "I2C Slave Data"]
pub mod sdr;
#[doc = "SIMR register accessor: an alias for `Reg<SIMR_SPEC>`"]
pub type SIMR = crate::Reg<simr::SIMR_SPEC>;
#[doc = "I2C Slave Interrupt Mask"]
pub mod simr;
#[doc = "SRIS register accessor: an alias for `Reg<SRIS_SPEC>`"]
pub type SRIS = crate::Reg<sris::SRIS_SPEC>;
#[doc = "I2C Slave Raw Interrupt Status"]
pub mod sris;
#[doc = "SMIS register accessor: an alias for `Reg<SMIS_SPEC>`"]
pub type SMIS = crate::Reg<smis::SMIS_SPEC>;
#[doc = "I2C Slave Masked Interrupt Status"]
pub mod smis;
#[doc = "SICR register accessor: an alias for `Reg<SICR_SPEC>`"]
pub type SICR = crate::Reg<sicr::SICR_SPEC>;
#[doc = "I2C Slave Interrupt Clear"]
pub mod sicr;
#[doc = "SOAR2 register accessor: an alias for `Reg<SOAR2_SPEC>`"]
pub type SOAR2 = crate::Reg<soar2::SOAR2_SPEC>;
#[doc = "I2C Slave Own Address 2"]
pub mod soar2;
#[doc = "SACKCTL register accessor: an alias for `Reg<SACKCTL_SPEC>`"]
pub type SACKCTL = crate::Reg<sackctl::SACKCTL_SPEC>;
#[doc = "I2C Slave ACK Control"]
pub mod sackctl;
#[doc = "FIFODATA register accessor: an alias for `Reg<FIFODATA_SPEC>`"]
pub type FIFODATA = crate::Reg<fifodata::FIFODATA_SPEC>;
#[doc = "I2C FIFO Data"]
pub mod fifodata;
#[doc = "FIFOCTL register accessor: an alias for `Reg<FIFOCTL_SPEC>`"]
pub type FIFOCTL = crate::Reg<fifoctl::FIFOCTL_SPEC>;
#[doc = "I2C FIFO Control"]
pub mod fifoctl;
#[doc = "FIFOSTATUS register accessor: an alias for `Reg<FIFOSTATUS_SPEC>`"]
pub type FIFOSTATUS = crate::Reg<fifostatus::FIFOSTATUS_SPEC>;
#[doc = "I2C FIFO Status"]
pub mod fifostatus;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "I2C Peripheral Properties"]
pub mod pp;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "I2C Peripheral Configuration"]
pub mod pc;
