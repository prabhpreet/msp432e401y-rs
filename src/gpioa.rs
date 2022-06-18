#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x03fc],
    #[doc = "0x3fc - GPIO Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x400 - GPIO Direction"]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    #[doc = "0x404 - GPIO Interrupt Sense"]
    pub is: crate::Reg<is::IS_SPEC>,
    #[doc = "0x408 - GPIO Interrupt Both Edges"]
    pub ibe: crate::Reg<ibe::IBE_SPEC>,
    #[doc = "0x40c - GPIO Interrupt Event"]
    pub iev: crate::Reg<iev::IEV_SPEC>,
    #[doc = "0x410 - GPIO Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x414 - GPIO Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x418 - GPIO Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x41c - GPIO Interrupt Clear"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x420 - GPIO Alternate Function Select"]
    pub afsel: crate::Reg<afsel::AFSEL_SPEC>,
    _reserved10: [u8; 0xdc],
    #[doc = "0x500 - GPIO 2-mA Drive Select"]
    pub dr2r: crate::Reg<dr2r::DR2R_SPEC>,
    #[doc = "0x504 - GPIO 4-mA Drive Select"]
    pub dr4r: crate::Reg<dr4r::DR4R_SPEC>,
    #[doc = "0x508 - GPIO 8-mA Drive Select"]
    pub dr8r: crate::Reg<dr8r::DR8R_SPEC>,
    #[doc = "0x50c - GPIO Open Drain Select"]
    pub odr: crate::Reg<odr::ODR_SPEC>,
    #[doc = "0x510 - GPIO Pull-Up Select"]
    pub pur: crate::Reg<pur::PUR_SPEC>,
    #[doc = "0x514 - GPIO Pull-Down Select"]
    pub pdr: crate::Reg<pdr::PDR_SPEC>,
    #[doc = "0x518 - GPIO Slew Rate Control Select"]
    pub slr: crate::Reg<slr::SLR_SPEC>,
    #[doc = "0x51c - GPIO Digital Enable"]
    pub den: crate::Reg<den::DEN_SPEC>,
    #[doc = "0x520 - GPIO Lock"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x524 - GPIO Commit"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x528 - GPIO Analog Mode Select"]
    pub amsel: crate::Reg<amsel::AMSEL_SPEC>,
    #[doc = "0x52c - GPIO Port Control"]
    pub pctl: crate::Reg<pctl::PCTL_SPEC>,
    #[doc = "0x530 - GPIO ADC Control"]
    pub adcctl: crate::Reg<adcctl::ADCCTL_SPEC>,
    #[doc = "0x534 - GPIO DMA Control"]
    pub dmactl: crate::Reg<dmactl::DMACTL_SPEC>,
    #[doc = "0x538 - GPIO Select Interrupt"]
    pub si: crate::Reg<si::SI_SPEC>,
    #[doc = "0x53c - GPIO 12-mA Drive Select"]
    pub dr12r: crate::Reg<dr12r::DR12R_SPEC>,
    #[doc = "0x540 - GPIO Wake Pin Enable"]
    pub wakepen: crate::Reg<wakepen::WAKEPEN_SPEC>,
    #[doc = "0x544 - GPIO Wake Level"]
    pub wakelvl: crate::Reg<wakelvl::WAKELVL_SPEC>,
    #[doc = "0x548 - GPIO Wake Status"]
    pub wakestat: crate::Reg<wakestat::WAKESTAT_SPEC>,
    _reserved29: [u8; 0x0a74],
    #[doc = "0xfc0 - GPIO Peripheral Property"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - GPIO Peripheral Configuration"]
    pub pc: crate::Reg<pc::PC_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "GPIO Data"]
pub mod data;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "GPIO Direction"]
pub mod dir;
#[doc = "IS register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "GPIO Interrupt Sense"]
pub mod is;
#[doc = "IBE register accessor: an alias for `Reg<IBE_SPEC>`"]
pub type IBE = crate::Reg<ibe::IBE_SPEC>;
#[doc = "GPIO Interrupt Both Edges"]
pub mod ibe;
#[doc = "IEV register accessor: an alias for `Reg<IEV_SPEC>`"]
pub type IEV = crate::Reg<iev::IEV_SPEC>;
#[doc = "GPIO Interrupt Event"]
pub mod iev;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "GPIO Interrupt Mask"]
pub mod im;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "GPIO Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "GPIO Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "GPIO Interrupt Clear"]
pub mod icr;
#[doc = "AFSEL register accessor: an alias for `Reg<AFSEL_SPEC>`"]
pub type AFSEL = crate::Reg<afsel::AFSEL_SPEC>;
#[doc = "GPIO Alternate Function Select"]
pub mod afsel;
#[doc = "DR2R register accessor: an alias for `Reg<DR2R_SPEC>`"]
pub type DR2R = crate::Reg<dr2r::DR2R_SPEC>;
#[doc = "GPIO 2-mA Drive Select"]
pub mod dr2r;
#[doc = "DR4R register accessor: an alias for `Reg<DR4R_SPEC>`"]
pub type DR4R = crate::Reg<dr4r::DR4R_SPEC>;
#[doc = "GPIO 4-mA Drive Select"]
pub mod dr4r;
#[doc = "DR8R register accessor: an alias for `Reg<DR8R_SPEC>`"]
pub type DR8R = crate::Reg<dr8r::DR8R_SPEC>;
#[doc = "GPIO 8-mA Drive Select"]
pub mod dr8r;
#[doc = "ODR register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIO Open Drain Select"]
pub mod odr;
#[doc = "PUR register accessor: an alias for `Reg<PUR_SPEC>`"]
pub type PUR = crate::Reg<pur::PUR_SPEC>;
#[doc = "GPIO Pull-Up Select"]
pub mod pur;
#[doc = "PDR register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "GPIO Pull-Down Select"]
pub mod pdr;
#[doc = "SLR register accessor: an alias for `Reg<SLR_SPEC>`"]
pub type SLR = crate::Reg<slr::SLR_SPEC>;
#[doc = "GPIO Slew Rate Control Select"]
pub mod slr;
#[doc = "DEN register accessor: an alias for `Reg<DEN_SPEC>`"]
pub type DEN = crate::Reg<den::DEN_SPEC>;
#[doc = "GPIO Digital Enable"]
pub mod den;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "GPIO Lock"]
pub mod lock;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "GPIO Commit"]
pub mod cr;
#[doc = "AMSEL register accessor: an alias for `Reg<AMSEL_SPEC>`"]
pub type AMSEL = crate::Reg<amsel::AMSEL_SPEC>;
#[doc = "GPIO Analog Mode Select"]
pub mod amsel;
#[doc = "PCTL register accessor: an alias for `Reg<PCTL_SPEC>`"]
pub type PCTL = crate::Reg<pctl::PCTL_SPEC>;
#[doc = "GPIO Port Control"]
pub mod pctl;
#[doc = "ADCCTL register accessor: an alias for `Reg<ADCCTL_SPEC>`"]
pub type ADCCTL = crate::Reg<adcctl::ADCCTL_SPEC>;
#[doc = "GPIO ADC Control"]
pub mod adcctl;
#[doc = "DMACTL register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "GPIO DMA Control"]
pub mod dmactl;
#[doc = "SI register accessor: an alias for `Reg<SI_SPEC>`"]
pub type SI = crate::Reg<si::SI_SPEC>;
#[doc = "GPIO Select Interrupt"]
pub mod si;
#[doc = "DR12R register accessor: an alias for `Reg<DR12R_SPEC>`"]
pub type DR12R = crate::Reg<dr12r::DR12R_SPEC>;
#[doc = "GPIO 12-mA Drive Select"]
pub mod dr12r;
#[doc = "WAKEPEN register accessor: an alias for `Reg<WAKEPEN_SPEC>`"]
pub type WAKEPEN = crate::Reg<wakepen::WAKEPEN_SPEC>;
#[doc = "GPIO Wake Pin Enable"]
pub mod wakepen;
#[doc = "WAKELVL register accessor: an alias for `Reg<WAKELVL_SPEC>`"]
pub type WAKELVL = crate::Reg<wakelvl::WAKELVL_SPEC>;
#[doc = "GPIO Wake Level"]
pub mod wakelvl;
#[doc = "WAKESTAT register accessor: an alias for `Reg<WAKESTAT_SPEC>`"]
pub type WAKESTAT = crate::Reg<wakestat::WAKESTAT_SPEC>;
#[doc = "GPIO Wake Status"]
pub mod wakestat;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "GPIO Peripheral Property"]
pub mod pp;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "GPIO Peripheral Configuration"]
pub mod pc;
