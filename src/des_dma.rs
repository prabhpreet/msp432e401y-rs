#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DES DMA Interrupt Mask"]
    pub des_dmaim: crate::Reg<des_dmaim::DES_DMAIM_SPEC>,
    #[doc = "0x04 - DES DMA Raw Interrupt Status"]
    pub des_dmaris: crate::Reg<des_dmaris::DES_DMARIS_SPEC>,
    #[doc = "0x08 - DES DMA Masked Interrupt Status"]
    pub des_dmamis: crate::Reg<des_dmamis::DES_DMAMIS_SPEC>,
    #[doc = "0x0c - DES DMA Interrupt Clear"]
    pub des_dmaic: crate::Reg<des_dmaic::DES_DMAIC_SPEC>,
}
#[doc = "DES_DMAIM register accessor: an alias for `Reg<DES_DMAIM_SPEC>`"]
pub type DES_DMAIM = crate::Reg<des_dmaim::DES_DMAIM_SPEC>;
#[doc = "DES DMA Interrupt Mask"]
pub mod des_dmaim;
#[doc = "DES_DMARIS register accessor: an alias for `Reg<DES_DMARIS_SPEC>`"]
pub type DES_DMARIS = crate::Reg<des_dmaris::DES_DMARIS_SPEC>;
#[doc = "DES DMA Raw Interrupt Status"]
pub mod des_dmaris;
#[doc = "DES_DMAMIS register accessor: an alias for `Reg<DES_DMAMIS_SPEC>`"]
pub type DES_DMAMIS = crate::Reg<des_dmamis::DES_DMAMIS_SPEC>;
#[doc = "DES DMA Masked Interrupt Status"]
pub mod des_dmamis;
#[doc = "DES_DMAIC register accessor: an alias for `Reg<DES_DMAIC_SPEC>`"]
pub type DES_DMAIC = crate::Reg<des_dmaic::DES_DMAIC_SPEC>;
#[doc = "DES DMA Interrupt Clear"]
pub mod des_dmaic;
