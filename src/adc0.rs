#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Active Sample Sequencer"]
    pub actss: crate::Reg<actss::ACTSS_SPEC>,
    #[doc = "0x04 - ADC Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x08 - ADC Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x0c - ADC Interrupt Status and Clear"]
    pub isc: crate::Reg<isc::ISC_SPEC>,
    #[doc = "0x10 - ADC Overflow Status"]
    pub ostat: crate::Reg<ostat::OSTAT_SPEC>,
    #[doc = "0x14 - ADC Event Multiplexer Select"]
    pub emux: crate::Reg<emux::EMUX_SPEC>,
    #[doc = "0x18 - ADC Underflow Status"]
    pub ustat: crate::Reg<ustat::USTAT_SPEC>,
    #[doc = "0x1c - ADC Trigger Source Select"]
    pub tssel: crate::Reg<tssel::TSSEL_SPEC>,
    #[doc = "0x20 - ADC Sample Sequencer Priority"]
    pub sspri: crate::Reg<sspri::SSPRI_SPEC>,
    #[doc = "0x24 - ADC Sample Phase Control"]
    pub spc: crate::Reg<spc::SPC_SPEC>,
    #[doc = "0x28 - ADC Processor Sample Sequence Initiate"]
    pub pssi: crate::Reg<pssi::PSSI_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - ADC Sample Averaging Control"]
    pub sac: crate::Reg<sac::SAC_SPEC>,
    #[doc = "0x34 - ADC Digital Comparator Interrupt Status and Clear"]
    pub dcisc: crate::Reg<dcisc::DCISC_SPEC>,
    #[doc = "0x38 - ADC Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - ADC Sample Sequence Input Multiplexer Select 0"]
    pub ssmux0: crate::Reg<ssmux0::SSMUX0_SPEC>,
    #[doc = "0x44 - ADC Sample Sequence Control 0"]
    pub ssctl0: crate::Reg<ssctl0::SSCTL0_SPEC>,
    #[doc = "0x48 - ADC Sample Sequence Result FIFO 0"]
    pub ssfifo0: crate::Reg<ssfifo0::SSFIFO0_SPEC>,
    #[doc = "0x4c - ADC Sample Sequence FIFO 0 Status"]
    pub ssfstat0: crate::Reg<ssfstat0::SSFSTAT0_SPEC>,
    #[doc = "0x50 - ADC Sample Sequence 0 Operation"]
    pub ssop0: crate::Reg<ssop0::SSOP0_SPEC>,
    #[doc = "0x54 - ADC Sample Sequence 0 Digital Comparator Select"]
    pub ssdc0: crate::Reg<ssdc0::SSDC0_SPEC>,
    #[doc = "0x58 - ADC Sample Sequence Extended Input Multiplexer Select 0"]
    pub ssemux0: crate::Reg<ssemux0::SSEMUX0_SPEC>,
    #[doc = "0x5c - ADC Sample Sequence 0 Sample and Hold Time"]
    pub sstsh0: crate::Reg<sstsh0::SSTSH0_SPEC>,
    #[doc = "0x60 - ADC Sample Sequence Input Multiplexer Select 1"]
    pub ssmux1: crate::Reg<ssmux1::SSMUX1_SPEC>,
    #[doc = "0x64 - ADC Sample Sequence Control 1"]
    pub ssctl1: crate::Reg<ssctl1::SSCTL1_SPEC>,
    #[doc = "0x68 - ADC Sample Sequence Result FIFO 1"]
    pub ssfifo1: crate::Reg<ssfifo1::SSFIFO1_SPEC>,
    #[doc = "0x6c - ADC Sample Sequence FIFO 1 Status"]
    pub ssfstat1: crate::Reg<ssfstat1::SSFSTAT1_SPEC>,
    #[doc = "0x70 - ADC Sample Sequence 1 Operation"]
    pub ssop1: crate::Reg<ssop1::SSOP1_SPEC>,
    #[doc = "0x74 - ADC Sample Sequence 1 Digital Comparator Select"]
    pub ssdc1: crate::Reg<ssdc1::SSDC1_SPEC>,
    #[doc = "0x78 - ADC Sample Sequence Extended Input Multiplexer Select 1"]
    pub ssemux1: crate::Reg<ssemux1::SSEMUX1_SPEC>,
    #[doc = "0x7c - ADC Sample Sequence 1 Sample and Hold Time"]
    pub sstsh1: crate::Reg<sstsh1::SSTSH1_SPEC>,
    #[doc = "0x80 - ADC Sample Sequence Input Multiplexer Select 2"]
    pub ssmux2: crate::Reg<ssmux2::SSMUX2_SPEC>,
    #[doc = "0x84 - ADC Sample Sequence Control 2"]
    pub ssctl2: crate::Reg<ssctl2::SSCTL2_SPEC>,
    #[doc = "0x88 - ADC Sample Sequence Result FIFO 2"]
    pub ssfifo2: crate::Reg<ssfifo2::SSFIFO2_SPEC>,
    #[doc = "0x8c - ADC Sample Sequence FIFO 2 Status"]
    pub ssfstat2: crate::Reg<ssfstat2::SSFSTAT2_SPEC>,
    #[doc = "0x90 - ADC Sample Sequence 2 Operation"]
    pub ssop2: crate::Reg<ssop2::SSOP2_SPEC>,
    #[doc = "0x94 - ADC Sample Sequence 2 Digital Comparator Select"]
    pub ssdc2: crate::Reg<ssdc2::SSDC2_SPEC>,
    #[doc = "0x98 - ADC Sample Sequence Extended Input Multiplexer Select 2"]
    pub ssemux2: crate::Reg<ssemux2::SSEMUX2_SPEC>,
    #[doc = "0x9c - ADC Sample Sequence 2 Sample and Hold Time"]
    pub sstsh2: crate::Reg<sstsh2::SSTSH2_SPEC>,
    #[doc = "0xa0 - ADC Sample Sequence Input Multiplexer Select 3"]
    pub ssmux3: crate::Reg<ssmux3::SSMUX3_SPEC>,
    #[doc = "0xa4 - ADC Sample Sequence Control 3"]
    pub ssctl3: crate::Reg<ssctl3::SSCTL3_SPEC>,
    #[doc = "0xa8 - ADC Sample Sequence Result FIFO 3"]
    pub ssfifo3: crate::Reg<ssfifo3::SSFIFO3_SPEC>,
    #[doc = "0xac - ADC Sample Sequence FIFO 3 Status"]
    pub ssfstat3: crate::Reg<ssfstat3::SSFSTAT3_SPEC>,
    #[doc = "0xb0 - ADC Sample Sequence 3 Operation"]
    pub ssop3: crate::Reg<ssop3::SSOP3_SPEC>,
    #[doc = "0xb4 - ADC Sample Sequence 3 Digital Comparator Select"]
    pub ssdc3: crate::Reg<ssdc3::SSDC3_SPEC>,
    #[doc = "0xb8 - ADC Sample Sequence Extended Input Multiplexer Select 3"]
    pub ssemux3: crate::Reg<ssemux3::SSEMUX3_SPEC>,
    #[doc = "0xbc - ADC Sample Sequence 3 Sample and Hold Time"]
    pub sstsh3: crate::Reg<sstsh3::SSTSH3_SPEC>,
    _reserved46: [u8; 0x0c40],
    #[doc = "0xd00 - ADC Digital Comparator Reset Initial Conditions"]
    pub dcric: crate::Reg<dcric::DCRIC_SPEC>,
    _reserved47: [u8; 0xfc],
    #[doc = "0xe00 - ADC Digital Comparator Control 0"]
    pub dcctl0: crate::Reg<dcctl0::DCCTL0_SPEC>,
    #[doc = "0xe04 - ADC Digital Comparator Control 1"]
    pub dcctl1: crate::Reg<dcctl1::DCCTL1_SPEC>,
    #[doc = "0xe08 - ADC Digital Comparator Control 2"]
    pub dcctl2: crate::Reg<dcctl2::DCCTL2_SPEC>,
    #[doc = "0xe0c - ADC Digital Comparator Control 3"]
    pub dcctl3: crate::Reg<dcctl3::DCCTL3_SPEC>,
    #[doc = "0xe10 - ADC Digital Comparator Control 4"]
    pub dcctl4: crate::Reg<dcctl4::DCCTL4_SPEC>,
    #[doc = "0xe14 - ADC Digital Comparator Control 5"]
    pub dcctl5: crate::Reg<dcctl5::DCCTL5_SPEC>,
    #[doc = "0xe18 - ADC Digital Comparator Control 6"]
    pub dcctl6: crate::Reg<dcctl6::DCCTL6_SPEC>,
    #[doc = "0xe1c - ADC Digital Comparator Control 7"]
    pub dcctl7: crate::Reg<dcctl7::DCCTL7_SPEC>,
    _reserved55: [u8; 0x20],
    #[doc = "0xe40 - ADC Digital Comparator Range 0"]
    pub dccmp0: crate::Reg<dccmp0::DCCMP0_SPEC>,
    #[doc = "0xe44 - ADC Digital Comparator Range 1"]
    pub dccmp1: crate::Reg<dccmp1::DCCMP1_SPEC>,
    #[doc = "0xe48 - ADC Digital Comparator Range 2"]
    pub dccmp2: crate::Reg<dccmp2::DCCMP2_SPEC>,
    #[doc = "0xe4c - ADC Digital Comparator Range 3"]
    pub dccmp3: crate::Reg<dccmp3::DCCMP3_SPEC>,
    #[doc = "0xe50 - ADC Digital Comparator Range 4"]
    pub dccmp4: crate::Reg<dccmp4::DCCMP4_SPEC>,
    #[doc = "0xe54 - ADC Digital Comparator Range 5"]
    pub dccmp5: crate::Reg<dccmp5::DCCMP5_SPEC>,
    #[doc = "0xe58 - ADC Digital Comparator Range 6"]
    pub dccmp6: crate::Reg<dccmp6::DCCMP6_SPEC>,
    #[doc = "0xe5c - ADC Digital Comparator Range 7"]
    pub dccmp7: crate::Reg<dccmp7::DCCMP7_SPEC>,
    _reserved63: [u8; 0x0160],
    #[doc = "0xfc0 - ADC Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - ADC Peripheral Configuration"]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0xfc8 - ADC Clock Configuration"]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
#[doc = "ACTSS register accessor: an alias for `Reg<ACTSS_SPEC>`"]
pub type ACTSS = crate::Reg<actss::ACTSS_SPEC>;
#[doc = "ADC Active Sample Sequencer"]
pub mod actss;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "ADC Raw Interrupt Status"]
pub mod ris;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "ADC Interrupt Mask"]
pub mod im;
#[doc = "ISC register accessor: an alias for `Reg<ISC_SPEC>`"]
pub type ISC = crate::Reg<isc::ISC_SPEC>;
#[doc = "ADC Interrupt Status and Clear"]
pub mod isc;
#[doc = "OSTAT register accessor: an alias for `Reg<OSTAT_SPEC>`"]
pub type OSTAT = crate::Reg<ostat::OSTAT_SPEC>;
#[doc = "ADC Overflow Status"]
pub mod ostat;
#[doc = "EMUX register accessor: an alias for `Reg<EMUX_SPEC>`"]
pub type EMUX = crate::Reg<emux::EMUX_SPEC>;
#[doc = "ADC Event Multiplexer Select"]
pub mod emux;
#[doc = "USTAT register accessor: an alias for `Reg<USTAT_SPEC>`"]
pub type USTAT = crate::Reg<ustat::USTAT_SPEC>;
#[doc = "ADC Underflow Status"]
pub mod ustat;
#[doc = "TSSEL register accessor: an alias for `Reg<TSSEL_SPEC>`"]
pub type TSSEL = crate::Reg<tssel::TSSEL_SPEC>;
#[doc = "ADC Trigger Source Select"]
pub mod tssel;
#[doc = "SSPRI register accessor: an alias for `Reg<SSPRI_SPEC>`"]
pub type SSPRI = crate::Reg<sspri::SSPRI_SPEC>;
#[doc = "ADC Sample Sequencer Priority"]
pub mod sspri;
#[doc = "SPC register accessor: an alias for `Reg<SPC_SPEC>`"]
pub type SPC = crate::Reg<spc::SPC_SPEC>;
#[doc = "ADC Sample Phase Control"]
pub mod spc;
#[doc = "PSSI register accessor: an alias for `Reg<PSSI_SPEC>`"]
pub type PSSI = crate::Reg<pssi::PSSI_SPEC>;
#[doc = "ADC Processor Sample Sequence Initiate"]
pub mod pssi;
#[doc = "SAC register accessor: an alias for `Reg<SAC_SPEC>`"]
pub type SAC = crate::Reg<sac::SAC_SPEC>;
#[doc = "ADC Sample Averaging Control"]
pub mod sac;
#[doc = "DCISC register accessor: an alias for `Reg<DCISC_SPEC>`"]
pub type DCISC = crate::Reg<dcisc::DCISC_SPEC>;
#[doc = "ADC Digital Comparator Interrupt Status and Clear"]
pub mod dcisc;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "ADC Control"]
pub mod ctl;
#[doc = "SSMUX0 register accessor: an alias for `Reg<SSMUX0_SPEC>`"]
pub type SSMUX0 = crate::Reg<ssmux0::SSMUX0_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub mod ssmux0;
#[doc = "SSCTL0 register accessor: an alias for `Reg<SSCTL0_SPEC>`"]
pub type SSCTL0 = crate::Reg<ssctl0::SSCTL0_SPEC>;
#[doc = "ADC Sample Sequence Control 0"]
pub mod ssctl0;
#[doc = "SSFIFO0 register accessor: an alias for `Reg<SSFIFO0_SPEC>`"]
pub type SSFIFO0 = crate::Reg<ssfifo0::SSFIFO0_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub mod ssfifo0;
#[doc = "SSFSTAT0 register accessor: an alias for `Reg<SSFSTAT0_SPEC>`"]
pub type SSFSTAT0 = crate::Reg<ssfstat0::SSFSTAT0_SPEC>;
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub mod ssfstat0;
#[doc = "SSOP0 register accessor: an alias for `Reg<SSOP0_SPEC>`"]
pub type SSOP0 = crate::Reg<ssop0::SSOP0_SPEC>;
#[doc = "ADC Sample Sequence 0 Operation"]
pub mod ssop0;
#[doc = "SSDC0 register accessor: an alias for `Reg<SSDC0_SPEC>`"]
pub type SSDC0 = crate::Reg<ssdc0::SSDC0_SPEC>;
#[doc = "ADC Sample Sequence 0 Digital Comparator Select"]
pub mod ssdc0;
#[doc = "SSEMUX0 register accessor: an alias for `Reg<SSEMUX0_SPEC>`"]
pub type SSEMUX0 = crate::Reg<ssemux0::SSEMUX0_SPEC>;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 0"]
pub mod ssemux0;
#[doc = "SSTSH0 register accessor: an alias for `Reg<SSTSH0_SPEC>`"]
pub type SSTSH0 = crate::Reg<sstsh0::SSTSH0_SPEC>;
#[doc = "ADC Sample Sequence 0 Sample and Hold Time"]
pub mod sstsh0;
#[doc = "SSMUX1 register accessor: an alias for `Reg<SSMUX1_SPEC>`"]
pub type SSMUX1 = crate::Reg<ssmux1::SSMUX1_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub mod ssmux1;
#[doc = "SSCTL1 register accessor: an alias for `Reg<SSCTL1_SPEC>`"]
pub type SSCTL1 = crate::Reg<ssctl1::SSCTL1_SPEC>;
#[doc = "ADC Sample Sequence Control 1"]
pub mod ssctl1;
#[doc = "SSFIFO1 register accessor: an alias for `Reg<SSFIFO1_SPEC>`"]
pub type SSFIFO1 = crate::Reg<ssfifo1::SSFIFO1_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub mod ssfifo1;
#[doc = "SSFSTAT1 register accessor: an alias for `Reg<SSFSTAT1_SPEC>`"]
pub type SSFSTAT1 = crate::Reg<ssfstat1::SSFSTAT1_SPEC>;
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub mod ssfstat1;
#[doc = "SSOP1 register accessor: an alias for `Reg<SSOP1_SPEC>`"]
pub type SSOP1 = crate::Reg<ssop1::SSOP1_SPEC>;
#[doc = "ADC Sample Sequence 1 Operation"]
pub mod ssop1;
#[doc = "SSDC1 register accessor: an alias for `Reg<SSDC1_SPEC>`"]
pub type SSDC1 = crate::Reg<ssdc1::SSDC1_SPEC>;
#[doc = "ADC Sample Sequence 1 Digital Comparator Select"]
pub mod ssdc1;
#[doc = "SSEMUX1 register accessor: an alias for `Reg<SSEMUX1_SPEC>`"]
pub type SSEMUX1 = crate::Reg<ssemux1::SSEMUX1_SPEC>;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 1"]
pub mod ssemux1;
#[doc = "SSTSH1 register accessor: an alias for `Reg<SSTSH1_SPEC>`"]
pub type SSTSH1 = crate::Reg<sstsh1::SSTSH1_SPEC>;
#[doc = "ADC Sample Sequence 1 Sample and Hold Time"]
pub mod sstsh1;
#[doc = "SSMUX2 register accessor: an alias for `Reg<SSMUX2_SPEC>`"]
pub type SSMUX2 = crate::Reg<ssmux2::SSMUX2_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub mod ssmux2;
#[doc = "SSCTL2 register accessor: an alias for `Reg<SSCTL2_SPEC>`"]
pub type SSCTL2 = crate::Reg<ssctl2::SSCTL2_SPEC>;
#[doc = "ADC Sample Sequence Control 2"]
pub mod ssctl2;
#[doc = "SSFIFO2 register accessor: an alias for `Reg<SSFIFO2_SPEC>`"]
pub type SSFIFO2 = crate::Reg<ssfifo2::SSFIFO2_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub mod ssfifo2;
#[doc = "SSFSTAT2 register accessor: an alias for `Reg<SSFSTAT2_SPEC>`"]
pub type SSFSTAT2 = crate::Reg<ssfstat2::SSFSTAT2_SPEC>;
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub mod ssfstat2;
#[doc = "SSOP2 register accessor: an alias for `Reg<SSOP2_SPEC>`"]
pub type SSOP2 = crate::Reg<ssop2::SSOP2_SPEC>;
#[doc = "ADC Sample Sequence 2 Operation"]
pub mod ssop2;
#[doc = "SSDC2 register accessor: an alias for `Reg<SSDC2_SPEC>`"]
pub type SSDC2 = crate::Reg<ssdc2::SSDC2_SPEC>;
#[doc = "ADC Sample Sequence 2 Digital Comparator Select"]
pub mod ssdc2;
#[doc = "SSEMUX2 register accessor: an alias for `Reg<SSEMUX2_SPEC>`"]
pub type SSEMUX2 = crate::Reg<ssemux2::SSEMUX2_SPEC>;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 2"]
pub mod ssemux2;
#[doc = "SSTSH2 register accessor: an alias for `Reg<SSTSH2_SPEC>`"]
pub type SSTSH2 = crate::Reg<sstsh2::SSTSH2_SPEC>;
#[doc = "ADC Sample Sequence 2 Sample and Hold Time"]
pub mod sstsh2;
#[doc = "SSMUX3 register accessor: an alias for `Reg<SSMUX3_SPEC>`"]
pub type SSMUX3 = crate::Reg<ssmux3::SSMUX3_SPEC>;
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub mod ssmux3;
#[doc = "SSCTL3 register accessor: an alias for `Reg<SSCTL3_SPEC>`"]
pub type SSCTL3 = crate::Reg<ssctl3::SSCTL3_SPEC>;
#[doc = "ADC Sample Sequence Control 3"]
pub mod ssctl3;
#[doc = "SSFIFO3 register accessor: an alias for `Reg<SSFIFO3_SPEC>`"]
pub type SSFIFO3 = crate::Reg<ssfifo3::SSFIFO3_SPEC>;
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub mod ssfifo3;
#[doc = "SSFSTAT3 register accessor: an alias for `Reg<SSFSTAT3_SPEC>`"]
pub type SSFSTAT3 = crate::Reg<ssfstat3::SSFSTAT3_SPEC>;
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub mod ssfstat3;
#[doc = "SSOP3 register accessor: an alias for `Reg<SSOP3_SPEC>`"]
pub type SSOP3 = crate::Reg<ssop3::SSOP3_SPEC>;
#[doc = "ADC Sample Sequence 3 Operation"]
pub mod ssop3;
#[doc = "SSDC3 register accessor: an alias for `Reg<SSDC3_SPEC>`"]
pub type SSDC3 = crate::Reg<ssdc3::SSDC3_SPEC>;
#[doc = "ADC Sample Sequence 3 Digital Comparator Select"]
pub mod ssdc3;
#[doc = "SSEMUX3 register accessor: an alias for `Reg<SSEMUX3_SPEC>`"]
pub type SSEMUX3 = crate::Reg<ssemux3::SSEMUX3_SPEC>;
#[doc = "ADC Sample Sequence Extended Input Multiplexer Select 3"]
pub mod ssemux3;
#[doc = "SSTSH3 register accessor: an alias for `Reg<SSTSH3_SPEC>`"]
pub type SSTSH3 = crate::Reg<sstsh3::SSTSH3_SPEC>;
#[doc = "ADC Sample Sequence 3 Sample and Hold Time"]
pub mod sstsh3;
#[doc = "DCRIC register accessor: an alias for `Reg<DCRIC_SPEC>`"]
pub type DCRIC = crate::Reg<dcric::DCRIC_SPEC>;
#[doc = "ADC Digital Comparator Reset Initial Conditions"]
pub mod dcric;
#[doc = "DCCTL0 register accessor: an alias for `Reg<DCCTL0_SPEC>`"]
pub type DCCTL0 = crate::Reg<dcctl0::DCCTL0_SPEC>;
#[doc = "ADC Digital Comparator Control 0"]
pub mod dcctl0;
#[doc = "DCCTL1 register accessor: an alias for `Reg<DCCTL1_SPEC>`"]
pub type DCCTL1 = crate::Reg<dcctl1::DCCTL1_SPEC>;
#[doc = "ADC Digital Comparator Control 1"]
pub mod dcctl1;
#[doc = "DCCTL2 register accessor: an alias for `Reg<DCCTL2_SPEC>`"]
pub type DCCTL2 = crate::Reg<dcctl2::DCCTL2_SPEC>;
#[doc = "ADC Digital Comparator Control 2"]
pub mod dcctl2;
#[doc = "DCCTL3 register accessor: an alias for `Reg<DCCTL3_SPEC>`"]
pub type DCCTL3 = crate::Reg<dcctl3::DCCTL3_SPEC>;
#[doc = "ADC Digital Comparator Control 3"]
pub mod dcctl3;
#[doc = "DCCTL4 register accessor: an alias for `Reg<DCCTL4_SPEC>`"]
pub type DCCTL4 = crate::Reg<dcctl4::DCCTL4_SPEC>;
#[doc = "ADC Digital Comparator Control 4"]
pub mod dcctl4;
#[doc = "DCCTL5 register accessor: an alias for `Reg<DCCTL5_SPEC>`"]
pub type DCCTL5 = crate::Reg<dcctl5::DCCTL5_SPEC>;
#[doc = "ADC Digital Comparator Control 5"]
pub mod dcctl5;
#[doc = "DCCTL6 register accessor: an alias for `Reg<DCCTL6_SPEC>`"]
pub type DCCTL6 = crate::Reg<dcctl6::DCCTL6_SPEC>;
#[doc = "ADC Digital Comparator Control 6"]
pub mod dcctl6;
#[doc = "DCCTL7 register accessor: an alias for `Reg<DCCTL7_SPEC>`"]
pub type DCCTL7 = crate::Reg<dcctl7::DCCTL7_SPEC>;
#[doc = "ADC Digital Comparator Control 7"]
pub mod dcctl7;
#[doc = "DCCMP0 register accessor: an alias for `Reg<DCCMP0_SPEC>`"]
pub type DCCMP0 = crate::Reg<dccmp0::DCCMP0_SPEC>;
#[doc = "ADC Digital Comparator Range 0"]
pub mod dccmp0;
#[doc = "DCCMP1 register accessor: an alias for `Reg<DCCMP1_SPEC>`"]
pub type DCCMP1 = crate::Reg<dccmp1::DCCMP1_SPEC>;
#[doc = "ADC Digital Comparator Range 1"]
pub mod dccmp1;
#[doc = "DCCMP2 register accessor: an alias for `Reg<DCCMP2_SPEC>`"]
pub type DCCMP2 = crate::Reg<dccmp2::DCCMP2_SPEC>;
#[doc = "ADC Digital Comparator Range 2"]
pub mod dccmp2;
#[doc = "DCCMP3 register accessor: an alias for `Reg<DCCMP3_SPEC>`"]
pub type DCCMP3 = crate::Reg<dccmp3::DCCMP3_SPEC>;
#[doc = "ADC Digital Comparator Range 3"]
pub mod dccmp3;
#[doc = "DCCMP4 register accessor: an alias for `Reg<DCCMP4_SPEC>`"]
pub type DCCMP4 = crate::Reg<dccmp4::DCCMP4_SPEC>;
#[doc = "ADC Digital Comparator Range 4"]
pub mod dccmp4;
#[doc = "DCCMP5 register accessor: an alias for `Reg<DCCMP5_SPEC>`"]
pub type DCCMP5 = crate::Reg<dccmp5::DCCMP5_SPEC>;
#[doc = "ADC Digital Comparator Range 5"]
pub mod dccmp5;
#[doc = "DCCMP6 register accessor: an alias for `Reg<DCCMP6_SPEC>`"]
pub type DCCMP6 = crate::Reg<dccmp6::DCCMP6_SPEC>;
#[doc = "ADC Digital Comparator Range 6"]
pub mod dccmp6;
#[doc = "DCCMP7 register accessor: an alias for `Reg<DCCMP7_SPEC>`"]
pub type DCCMP7 = crate::Reg<dccmp7::DCCMP7_SPEC>;
#[doc = "ADC Digital Comparator Range 7"]
pub mod dccmp7;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "ADC Peripheral Properties"]
pub mod pp;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "ADC Peripheral Configuration"]
pub mod pc;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "ADC Clock Configuration"]
pub mod cc;
