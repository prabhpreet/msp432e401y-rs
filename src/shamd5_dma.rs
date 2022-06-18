#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SHAMD5 DMA Interrupt Mask"]
    pub shamd5_dmaim: crate::Reg<shamd5_dmaim::SHAMD5_DMAIM_SPEC>,
    #[doc = "0x04 - SHAMD5 DMA Raw Interrupt Status"]
    pub shamd5_dmaris: crate::Reg<shamd5_dmaris::SHAMD5_DMARIS_SPEC>,
    #[doc = "0x08 - SHAMD5 DMA Masked Interrupt Status"]
    pub shamd5_dmamis: crate::Reg<shamd5_dmamis::SHAMD5_DMAMIS_SPEC>,
    #[doc = "0x0c - SHAMD5 DMA Interrupt Clear"]
    pub shamd5_dmaic: crate::Reg<shamd5_dmaic::SHAMD5_DMAIC_SPEC>,
}
#[doc = "SHAMD5_DMAIM register accessor: an alias for `Reg<SHAMD5_DMAIM_SPEC>`"]
pub type SHAMD5_DMAIM = crate::Reg<shamd5_dmaim::SHAMD5_DMAIM_SPEC>;
#[doc = "SHAMD5 DMA Interrupt Mask"]
pub mod shamd5_dmaim;
#[doc = "SHAMD5_DMARIS register accessor: an alias for `Reg<SHAMD5_DMARIS_SPEC>`"]
pub type SHAMD5_DMARIS = crate::Reg<shamd5_dmaris::SHAMD5_DMARIS_SPEC>;
#[doc = "SHAMD5 DMA Raw Interrupt Status"]
pub mod shamd5_dmaris;
#[doc = "SHAMD5_DMAMIS register accessor: an alias for `Reg<SHAMD5_DMAMIS_SPEC>`"]
pub type SHAMD5_DMAMIS = crate::Reg<shamd5_dmamis::SHAMD5_DMAMIS_SPEC>;
#[doc = "SHAMD5 DMA Masked Interrupt Status"]
pub mod shamd5_dmamis;
#[doc = "SHAMD5_DMAIC register accessor: an alias for `Reg<SHAMD5_DMAIC_SPEC>`"]
pub type SHAMD5_DMAIC = crate::Reg<shamd5_dmaic::SHAMD5_DMAIC_SPEC>;
#[doc = "SHAMD5 DMA Interrupt Clear"]
pub mod shamd5_dmaic;
