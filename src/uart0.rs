#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved_1_rsr: [u8; 0x04],
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - UART Flag"]
    pub fr: crate::Reg<fr::FR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - UART IrDA Low-Power Register"]
    pub ilpr: crate::Reg<ilpr::ILPR_SPEC>,
    #[doc = "0x24 - UART Integer Baud-Rate Divisor"]
    pub ibrd: crate::Reg<ibrd::IBRD_SPEC>,
    #[doc = "0x28 - UART Fractional Baud-Rate Divisor"]
    pub fbrd: crate::Reg<fbrd::FBRD_SPEC>,
    #[doc = "0x2c - UART Line Control"]
    pub lcrh: crate::Reg<lcrh::LCRH_SPEC>,
    #[doc = "0x30 - UART Control"]
    pub ctl: crate::Reg<ctl::CTL_SPEC>,
    #[doc = "0x34 - UART Interrupt FIFO Level Select"]
    pub ifls: crate::Reg<ifls::IFLS_SPEC>,
    #[doc = "0x38 - UART Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x3c - UART Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x40 - UART Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x44 - UART Interrupt Clear"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x48 - UART DMA Control"]
    pub dmactl: crate::Reg<dmactl::DMACTL_SPEC>,
    _reserved14: [u8; 0x58],
    #[doc = "0xa4 - UART 9-Bit Self Address"]
    pub _9bitaddr: crate::Reg<_9bitaddr::_9BITADDR_SPEC>,
    #[doc = "0xa8 - UART 9-Bit Self Address Mask"]
    pub _9bitamask: crate::Reg<_9bitamask::_9BITAMASK_SPEC>,
    _reserved16: [u8; 0x0f14],
    #[doc = "0xfc0 - UART Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0xfc8 - UART Clock Configuration"]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn uart_alt_ecr(&self) -> &crate::Reg<uart_alt_ecr::UART_ALT_ECR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<uart_alt_ecr::UART_ALT_ECR_SPEC>)
        }
    }
    #[doc = "0x04 - UART Receive Status/Error Clear"]
    #[inline(always)]
    pub fn rsr(&self) -> &crate::Reg<rsr::RSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<rsr::RSR_SPEC>)
        }
    }
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART Data"]
pub mod dr;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "UART Receive Status/Error Clear"]
pub mod rsr;
#[doc = "UART_ALT_ECR register accessor: an alias for `Reg<UART_ALT_ECR_SPEC>`"]
pub type UART_ALT_ECR = crate::Reg<uart_alt_ecr::UART_ALT_ECR_SPEC>;
#[doc = "UART Receive Status/Error Clear"]
pub mod uart_alt_ecr;
#[doc = "FR register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "UART Flag"]
pub mod fr;
#[doc = "ILPR register accessor: an alias for `Reg<ILPR_SPEC>`"]
pub type ILPR = crate::Reg<ilpr::ILPR_SPEC>;
#[doc = "UART IrDA Low-Power Register"]
pub mod ilpr;
#[doc = "IBRD register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "UART Integer Baud-Rate Divisor"]
pub mod ibrd;
#[doc = "FBRD register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "UART Fractional Baud-Rate Divisor"]
pub mod fbrd;
#[doc = "LCRH register accessor: an alias for `Reg<LCRH_SPEC>`"]
pub type LCRH = crate::Reg<lcrh::LCRH_SPEC>;
#[doc = "UART Line Control"]
pub mod lcrh;
#[doc = "CTL register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "UART Control"]
pub mod ctl;
#[doc = "IFLS register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "UART Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "UART Interrupt Mask"]
pub mod im;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "UART Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "UART Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "UART Interrupt Clear"]
pub mod icr;
#[doc = "DMACTL register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "UART DMA Control"]
pub mod dmactl;
#[doc = "_9BITADDR register accessor: an alias for `Reg<_9BITADDR_SPEC>`"]
pub type _9BITADDR = crate::Reg<_9bitaddr::_9BITADDR_SPEC>;
#[doc = "UART 9-Bit Self Address"]
pub mod _9bitaddr;
#[doc = "_9BITAMASK register accessor: an alias for `Reg<_9BITAMASK_SPEC>`"]
pub type _9BITAMASK = crate::Reg<_9bitamask::_9BITAMASK_SPEC>;
#[doc = "UART 9-Bit Self Address Mask"]
pub mod _9bitamask;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "UART Peripheral Properties"]
pub mod pp;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "UART Clock Configuration"]
pub mod cc;
