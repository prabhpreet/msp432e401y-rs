#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Memory Address"]
    pub fma: crate::Reg<fma::FMA_SPEC>,
    #[doc = "0x04 - Flash Memory Data"]
    pub fmd: crate::Reg<fmd::FMD_SPEC>,
    #[doc = "0x08 - Flash Memory Control"]
    pub fmc: crate::Reg<fmc::FMC_SPEC>,
    #[doc = "0x0c - Flash Controller Raw Interrupt Status"]
    pub fcris: crate::Reg<fcris::FCRIS_SPEC>,
    #[doc = "0x10 - Flash Controller Interrupt Mask"]
    pub fcim: crate::Reg<fcim::FCIM_SPEC>,
    #[doc = "0x14 - Flash Controller Masked Interrupt Status and Clear"]
    pub fcmisc: crate::Reg<fcmisc::FCMISC_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Flash Memory Control 2"]
    pub fmc2: crate::Reg<fmc2::FMC2_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x30 - Flash Write Buffer Valid"]
    pub fwbval: crate::Reg<fwbval::FWBVAL_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x3c - Flash Program/Erase Key"]
    pub flpekey: crate::Reg<flpekey::FLPEKEY_SPEC>,
    _reserved9: [u8; 0xc0],
    #[doc = "0x100..0x180 - Flash Write Buffer n"]
    pub fwbn: [crate::Reg<fwbn::FWBN_SPEC>; 32],
    _reserved10: [u8; 0x0e40],
    #[doc = "0xfc0 - Flash Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - SRAM Size"]
    pub ssize: crate::Reg<ssize::SSIZE_SPEC>,
    #[doc = "0xfc8 - Flash Configuration Register"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0xfcc - ROM Software Map"]
    pub romswmap: crate::Reg<romswmap::ROMSWMAP_SPEC>,
    #[doc = "0xfd0 - Flash DMA Address Size"]
    pub dmasz: crate::Reg<dmasz::DMASZ_SPEC>,
    #[doc = "0xfd4 - Flash DMA Starting Address"]
    pub dmast: crate::Reg<dmast::DMAST_SPEC>,
    _reserved16: [u8; 0xfc],
    #[doc = "0x10d4 - Reset Vector Pointer"]
    pub rvp: crate::Reg<rvp::RVP_SPEC>,
    _reserved17: [u8; 0xf8],
    #[doc = "0x11d0 - Boot Configuration"]
    pub bootcfg: crate::Reg<bootcfg::BOOTCFG_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: crate::Reg<userreg0::USERREG0_SPEC>,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: crate::Reg<userreg1::USERREG1_SPEC>,
    #[doc = "0x11e8 - User Register 2"]
    pub userreg2: crate::Reg<userreg2::USERREG2_SPEC>,
    #[doc = "0x11ec - User Register 3"]
    pub userreg3: crate::Reg<userreg3::USERREG3_SPEC>,
    _reserved22: [u8; 0x10],
    #[doc = "0x1200 - Flash Memory Protection Read Enable 0"]
    pub fmpre0: crate::Reg<fmpre0::FMPRE0_SPEC>,
    #[doc = "0x1204 - Flash Memory Protection Read Enable 1"]
    pub fmpre1: crate::Reg<fmpre1::FMPRE1_SPEC>,
    #[doc = "0x1208 - Flash Memory Protection Read Enable 2"]
    pub fmpre2: crate::Reg<fmpre2::FMPRE2_SPEC>,
    #[doc = "0x120c - Flash Memory Protection Read Enable 3"]
    pub fmpre3: crate::Reg<fmpre3::FMPRE3_SPEC>,
    #[doc = "0x1210 - Flash Memory Protection Read Enable 4"]
    pub fmpre4: crate::Reg<fmpre4::FMPRE4_SPEC>,
    #[doc = "0x1214 - Flash Memory Protection Read Enable 5"]
    pub fmpre5: crate::Reg<fmpre5::FMPRE5_SPEC>,
    #[doc = "0x1218 - Flash Memory Protection Read Enable 6"]
    pub fmpre6: crate::Reg<fmpre6::FMPRE6_SPEC>,
    #[doc = "0x121c - Flash Memory Protection Read Enable 7"]
    pub fmpre7: crate::Reg<fmpre7::FMPRE7_SPEC>,
    #[doc = "0x1220 - Flash Memory Protection Read Enable 8"]
    pub fmpre8: crate::Reg<fmpre8::FMPRE8_SPEC>,
    #[doc = "0x1224 - Flash Memory Protection Read Enable 9"]
    pub fmpre9: crate::Reg<fmpre9::FMPRE9_SPEC>,
    #[doc = "0x1228 - Flash Memory Protection Read Enable 10"]
    pub fmpre10: crate::Reg<fmpre10::FMPRE10_SPEC>,
    #[doc = "0x122c - Flash Memory Protection Read Enable 11"]
    pub fmpre11: crate::Reg<fmpre11::FMPRE11_SPEC>,
    #[doc = "0x1230 - Flash Memory Protection Read Enable 12"]
    pub fmpre12: crate::Reg<fmpre12::FMPRE12_SPEC>,
    #[doc = "0x1234 - Flash Memory Protection Read Enable 13"]
    pub fmpre13: crate::Reg<fmpre13::FMPRE13_SPEC>,
    #[doc = "0x1238 - Flash Memory Protection Read Enable 14"]
    pub fmpre14: crate::Reg<fmpre14::FMPRE14_SPEC>,
    #[doc = "0x123c - Flash Memory Protection Read Enable 15"]
    pub fmpre15: crate::Reg<fmpre15::FMPRE15_SPEC>,
    _reserved38: [u8; 0x01c0],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: crate::Reg<fmppe0::FMPPE0_SPEC>,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: crate::Reg<fmppe1::FMPPE1_SPEC>,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: crate::Reg<fmppe2::FMPPE2_SPEC>,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: crate::Reg<fmppe3::FMPPE3_SPEC>,
    #[doc = "0x1410 - Flash Memory Protection Program Enable 4"]
    pub fmppe4: crate::Reg<fmppe4::FMPPE4_SPEC>,
    #[doc = "0x1414 - Flash Memory Protection Program Enable 5"]
    pub fmppe5: crate::Reg<fmppe5::FMPPE5_SPEC>,
    #[doc = "0x1418 - Flash Memory Protection Program Enable 6"]
    pub fmppe6: crate::Reg<fmppe6::FMPPE6_SPEC>,
    #[doc = "0x141c - Flash Memory Protection Program Enable 7"]
    pub fmppe7: crate::Reg<fmppe7::FMPPE7_SPEC>,
    #[doc = "0x1420 - Flash Memory Protection Program Enable 8"]
    pub fmppe8: crate::Reg<fmppe8::FMPPE8_SPEC>,
    #[doc = "0x1424 - Flash Memory Protection Program Enable 9"]
    pub fmppe9: crate::Reg<fmppe9::FMPPE9_SPEC>,
    #[doc = "0x1428 - Flash Memory Protection Program Enable 10"]
    pub fmppe10: crate::Reg<fmppe10::FMPPE10_SPEC>,
    #[doc = "0x142c - Flash Memory Protection Program Enable 11"]
    pub fmppe11: crate::Reg<fmppe11::FMPPE11_SPEC>,
    #[doc = "0x1430 - Flash Memory Protection Program Enable 12"]
    pub fmppe12: crate::Reg<fmppe12::FMPPE12_SPEC>,
    #[doc = "0x1434 - Flash Memory Protection Program Enable 13"]
    pub fmppe13: crate::Reg<fmppe13::FMPPE13_SPEC>,
    #[doc = "0x1438 - Flash Memory Protection Program Enable 14"]
    pub fmppe14: crate::Reg<fmppe14::FMPPE14_SPEC>,
    #[doc = "0x143c - Flash Memory Protection Program Enable 15"]
    pub fmppe15: crate::Reg<fmppe15::FMPPE15_SPEC>,
}
#[doc = "FMA register accessor: an alias for `Reg<FMA_SPEC>`"]
pub type FMA = crate::Reg<fma::FMA_SPEC>;
#[doc = "Flash Memory Address"]
pub mod fma;
#[doc = "FMD register accessor: an alias for `Reg<FMD_SPEC>`"]
pub type FMD = crate::Reg<fmd::FMD_SPEC>;
#[doc = "Flash Memory Data"]
pub mod fmd;
#[doc = "FMC register accessor: an alias for `Reg<FMC_SPEC>`"]
pub type FMC = crate::Reg<fmc::FMC_SPEC>;
#[doc = "Flash Memory Control"]
pub mod fmc;
#[doc = "FCRIS register accessor: an alias for `Reg<FCRIS_SPEC>`"]
pub type FCRIS = crate::Reg<fcris::FCRIS_SPEC>;
#[doc = "Flash Controller Raw Interrupt Status"]
pub mod fcris;
#[doc = "FCIM register accessor: an alias for `Reg<FCIM_SPEC>`"]
pub type FCIM = crate::Reg<fcim::FCIM_SPEC>;
#[doc = "Flash Controller Interrupt Mask"]
pub mod fcim;
#[doc = "FCMISC register accessor: an alias for `Reg<FCMISC_SPEC>`"]
pub type FCMISC = crate::Reg<fcmisc::FCMISC_SPEC>;
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub mod fcmisc;
#[doc = "FMC2 register accessor: an alias for `Reg<FMC2_SPEC>`"]
pub type FMC2 = crate::Reg<fmc2::FMC2_SPEC>;
#[doc = "Flash Memory Control 2"]
pub mod fmc2;
#[doc = "FWBVAL register accessor: an alias for `Reg<FWBVAL_SPEC>`"]
pub type FWBVAL = crate::Reg<fwbval::FWBVAL_SPEC>;
#[doc = "Flash Write Buffer Valid"]
pub mod fwbval;
#[doc = "FLPEKEY register accessor: an alias for `Reg<FLPEKEY_SPEC>`"]
pub type FLPEKEY = crate::Reg<flpekey::FLPEKEY_SPEC>;
#[doc = "Flash Program/Erase Key"]
pub mod flpekey;
#[doc = "FWBN register accessor: an alias for `Reg<FWBN_SPEC>`"]
pub type FWBN = crate::Reg<fwbn::FWBN_SPEC>;
#[doc = "Flash Write Buffer n"]
pub mod fwbn;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "Flash Peripheral Properties"]
pub mod pp;
#[doc = "SSIZE register accessor: an alias for `Reg<SSIZE_SPEC>`"]
pub type SSIZE = crate::Reg<ssize::SSIZE_SPEC>;
#[doc = "SRAM Size"]
pub mod ssize;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod conf;
#[doc = "ROMSWMAP register accessor: an alias for `Reg<ROMSWMAP_SPEC>`"]
pub type ROMSWMAP = crate::Reg<romswmap::ROMSWMAP_SPEC>;
#[doc = "ROM Software Map"]
pub mod romswmap;
#[doc = "DMASZ register accessor: an alias for `Reg<DMASZ_SPEC>`"]
pub type DMASZ = crate::Reg<dmasz::DMASZ_SPEC>;
#[doc = "Flash DMA Address Size"]
pub mod dmasz;
#[doc = "DMAST register accessor: an alias for `Reg<DMAST_SPEC>`"]
pub type DMAST = crate::Reg<dmast::DMAST_SPEC>;
#[doc = "Flash DMA Starting Address"]
pub mod dmast;
#[doc = "RVP register accessor: an alias for `Reg<RVP_SPEC>`"]
pub type RVP = crate::Reg<rvp::RVP_SPEC>;
#[doc = "Reset Vector Pointer"]
pub mod rvp;
#[doc = "BOOTCFG register accessor: an alias for `Reg<BOOTCFG_SPEC>`"]
pub type BOOTCFG = crate::Reg<bootcfg::BOOTCFG_SPEC>;
#[doc = "Boot Configuration"]
pub mod bootcfg;
#[doc = "USERREG0 register accessor: an alias for `Reg<USERREG0_SPEC>`"]
pub type USERREG0 = crate::Reg<userreg0::USERREG0_SPEC>;
#[doc = "User Register 0"]
pub mod userreg0;
#[doc = "USERREG1 register accessor: an alias for `Reg<USERREG1_SPEC>`"]
pub type USERREG1 = crate::Reg<userreg1::USERREG1_SPEC>;
#[doc = "User Register 1"]
pub mod userreg1;
#[doc = "USERREG2 register accessor: an alias for `Reg<USERREG2_SPEC>`"]
pub type USERREG2 = crate::Reg<userreg2::USERREG2_SPEC>;
#[doc = "User Register 2"]
pub mod userreg2;
#[doc = "USERREG3 register accessor: an alias for `Reg<USERREG3_SPEC>`"]
pub type USERREG3 = crate::Reg<userreg3::USERREG3_SPEC>;
#[doc = "User Register 3"]
pub mod userreg3;
#[doc = "FMPRE0 register accessor: an alias for `Reg<FMPRE0_SPEC>`"]
pub type FMPRE0 = crate::Reg<fmpre0::FMPRE0_SPEC>;
#[doc = "Flash Memory Protection Read Enable 0"]
pub mod fmpre0;
#[doc = "FMPRE1 register accessor: an alias for `Reg<FMPRE1_SPEC>`"]
pub type FMPRE1 = crate::Reg<fmpre1::FMPRE1_SPEC>;
#[doc = "Flash Memory Protection Read Enable 1"]
pub mod fmpre1;
#[doc = "FMPRE2 register accessor: an alias for `Reg<FMPRE2_SPEC>`"]
pub type FMPRE2 = crate::Reg<fmpre2::FMPRE2_SPEC>;
#[doc = "Flash Memory Protection Read Enable 2"]
pub mod fmpre2;
#[doc = "FMPRE3 register accessor: an alias for `Reg<FMPRE3_SPEC>`"]
pub type FMPRE3 = crate::Reg<fmpre3::FMPRE3_SPEC>;
#[doc = "Flash Memory Protection Read Enable 3"]
pub mod fmpre3;
#[doc = "FMPRE4 register accessor: an alias for `Reg<FMPRE4_SPEC>`"]
pub type FMPRE4 = crate::Reg<fmpre4::FMPRE4_SPEC>;
#[doc = "Flash Memory Protection Read Enable 4"]
pub mod fmpre4;
#[doc = "FMPRE5 register accessor: an alias for `Reg<FMPRE5_SPEC>`"]
pub type FMPRE5 = crate::Reg<fmpre5::FMPRE5_SPEC>;
#[doc = "Flash Memory Protection Read Enable 5"]
pub mod fmpre5;
#[doc = "FMPRE6 register accessor: an alias for `Reg<FMPRE6_SPEC>`"]
pub type FMPRE6 = crate::Reg<fmpre6::FMPRE6_SPEC>;
#[doc = "Flash Memory Protection Read Enable 6"]
pub mod fmpre6;
#[doc = "FMPRE7 register accessor: an alias for `Reg<FMPRE7_SPEC>`"]
pub type FMPRE7 = crate::Reg<fmpre7::FMPRE7_SPEC>;
#[doc = "Flash Memory Protection Read Enable 7"]
pub mod fmpre7;
#[doc = "FMPRE8 register accessor: an alias for `Reg<FMPRE8_SPEC>`"]
pub type FMPRE8 = crate::Reg<fmpre8::FMPRE8_SPEC>;
#[doc = "Flash Memory Protection Read Enable 8"]
pub mod fmpre8;
#[doc = "FMPRE9 register accessor: an alias for `Reg<FMPRE9_SPEC>`"]
pub type FMPRE9 = crate::Reg<fmpre9::FMPRE9_SPEC>;
#[doc = "Flash Memory Protection Read Enable 9"]
pub mod fmpre9;
#[doc = "FMPRE10 register accessor: an alias for `Reg<FMPRE10_SPEC>`"]
pub type FMPRE10 = crate::Reg<fmpre10::FMPRE10_SPEC>;
#[doc = "Flash Memory Protection Read Enable 10"]
pub mod fmpre10;
#[doc = "FMPRE11 register accessor: an alias for `Reg<FMPRE11_SPEC>`"]
pub type FMPRE11 = crate::Reg<fmpre11::FMPRE11_SPEC>;
#[doc = "Flash Memory Protection Read Enable 11"]
pub mod fmpre11;
#[doc = "FMPRE12 register accessor: an alias for `Reg<FMPRE12_SPEC>`"]
pub type FMPRE12 = crate::Reg<fmpre12::FMPRE12_SPEC>;
#[doc = "Flash Memory Protection Read Enable 12"]
pub mod fmpre12;
#[doc = "FMPRE13 register accessor: an alias for `Reg<FMPRE13_SPEC>`"]
pub type FMPRE13 = crate::Reg<fmpre13::FMPRE13_SPEC>;
#[doc = "Flash Memory Protection Read Enable 13"]
pub mod fmpre13;
#[doc = "FMPRE14 register accessor: an alias for `Reg<FMPRE14_SPEC>`"]
pub type FMPRE14 = crate::Reg<fmpre14::FMPRE14_SPEC>;
#[doc = "Flash Memory Protection Read Enable 14"]
pub mod fmpre14;
#[doc = "FMPRE15 register accessor: an alias for `Reg<FMPRE15_SPEC>`"]
pub type FMPRE15 = crate::Reg<fmpre15::FMPRE15_SPEC>;
#[doc = "Flash Memory Protection Read Enable 15"]
pub mod fmpre15;
#[doc = "FMPPE0 register accessor: an alias for `Reg<FMPPE0_SPEC>`"]
pub type FMPPE0 = crate::Reg<fmppe0::FMPPE0_SPEC>;
#[doc = "Flash Memory Protection Program Enable 0"]
pub mod fmppe0;
#[doc = "FMPPE1 register accessor: an alias for `Reg<FMPPE1_SPEC>`"]
pub type FMPPE1 = crate::Reg<fmppe1::FMPPE1_SPEC>;
#[doc = "Flash Memory Protection Program Enable 1"]
pub mod fmppe1;
#[doc = "FMPPE2 register accessor: an alias for `Reg<FMPPE2_SPEC>`"]
pub type FMPPE2 = crate::Reg<fmppe2::FMPPE2_SPEC>;
#[doc = "Flash Memory Protection Program Enable 2"]
pub mod fmppe2;
#[doc = "FMPPE3 register accessor: an alias for `Reg<FMPPE3_SPEC>`"]
pub type FMPPE3 = crate::Reg<fmppe3::FMPPE3_SPEC>;
#[doc = "Flash Memory Protection Program Enable 3"]
pub mod fmppe3;
#[doc = "FMPPE4 register accessor: an alias for `Reg<FMPPE4_SPEC>`"]
pub type FMPPE4 = crate::Reg<fmppe4::FMPPE4_SPEC>;
#[doc = "Flash Memory Protection Program Enable 4"]
pub mod fmppe4;
#[doc = "FMPPE5 register accessor: an alias for `Reg<FMPPE5_SPEC>`"]
pub type FMPPE5 = crate::Reg<fmppe5::FMPPE5_SPEC>;
#[doc = "Flash Memory Protection Program Enable 5"]
pub mod fmppe5;
#[doc = "FMPPE6 register accessor: an alias for `Reg<FMPPE6_SPEC>`"]
pub type FMPPE6 = crate::Reg<fmppe6::FMPPE6_SPEC>;
#[doc = "Flash Memory Protection Program Enable 6"]
pub mod fmppe6;
#[doc = "FMPPE7 register accessor: an alias for `Reg<FMPPE7_SPEC>`"]
pub type FMPPE7 = crate::Reg<fmppe7::FMPPE7_SPEC>;
#[doc = "Flash Memory Protection Program Enable 7"]
pub mod fmppe7;
#[doc = "FMPPE8 register accessor: an alias for `Reg<FMPPE8_SPEC>`"]
pub type FMPPE8 = crate::Reg<fmppe8::FMPPE8_SPEC>;
#[doc = "Flash Memory Protection Program Enable 8"]
pub mod fmppe8;
#[doc = "FMPPE9 register accessor: an alias for `Reg<FMPPE9_SPEC>`"]
pub type FMPPE9 = crate::Reg<fmppe9::FMPPE9_SPEC>;
#[doc = "Flash Memory Protection Program Enable 9"]
pub mod fmppe9;
#[doc = "FMPPE10 register accessor: an alias for `Reg<FMPPE10_SPEC>`"]
pub type FMPPE10 = crate::Reg<fmppe10::FMPPE10_SPEC>;
#[doc = "Flash Memory Protection Program Enable 10"]
pub mod fmppe10;
#[doc = "FMPPE11 register accessor: an alias for `Reg<FMPPE11_SPEC>`"]
pub type FMPPE11 = crate::Reg<fmppe11::FMPPE11_SPEC>;
#[doc = "Flash Memory Protection Program Enable 11"]
pub mod fmppe11;
#[doc = "FMPPE12 register accessor: an alias for `Reg<FMPPE12_SPEC>`"]
pub type FMPPE12 = crate::Reg<fmppe12::FMPPE12_SPEC>;
#[doc = "Flash Memory Protection Program Enable 12"]
pub mod fmppe12;
#[doc = "FMPPE13 register accessor: an alias for `Reg<FMPPE13_SPEC>`"]
pub type FMPPE13 = crate::Reg<fmppe13::FMPPE13_SPEC>;
#[doc = "Flash Memory Protection Program Enable 13"]
pub mod fmppe13;
#[doc = "FMPPE14 register accessor: an alias for `Reg<FMPPE14_SPEC>`"]
pub type FMPPE14 = crate::Reg<fmppe14::FMPPE14_SPEC>;
#[doc = "Flash Memory Protection Program Enable 14"]
pub mod fmppe14;
#[doc = "FMPPE15 register accessor: an alias for `Reg<FMPPE15_SPEC>`"]
pub type FMPPE15 = crate::Reg<fmppe15::FMPPE15_SPEC>;
#[doc = "Flash Memory Protection Program Enable 15"]
pub mod fmppe15;
