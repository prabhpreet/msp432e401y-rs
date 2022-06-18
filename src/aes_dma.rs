#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES DMA Interrupt Mask"]
    pub aes_dmaim: crate::Reg<aes_dmaim::AES_DMAIM_SPEC>,
    #[doc = "0x04 - AES DMA Raw Interrupt Status"]
    pub aes_dmaris: crate::Reg<aes_dmaris::AES_DMARIS_SPEC>,
    #[doc = "0x08 - AES DMA Masked Interrupt Status"]
    pub aes_dmamis: crate::Reg<aes_dmamis::AES_DMAMIS_SPEC>,
    #[doc = "0x0c - AES DMA Interrupt Clear"]
    pub aes_dmaic: crate::Reg<aes_dmaic::AES_DMAIC_SPEC>,
}
#[doc = "AES_DMAIM register accessor: an alias for `Reg<AES_DMAIM_SPEC>`"]
pub type AES_DMAIM = crate::Reg<aes_dmaim::AES_DMAIM_SPEC>;
#[doc = "AES DMA Interrupt Mask"]
pub mod aes_dmaim;
#[doc = "AES_DMARIS register accessor: an alias for `Reg<AES_DMARIS_SPEC>`"]
pub type AES_DMARIS = crate::Reg<aes_dmaris::AES_DMARIS_SPEC>;
#[doc = "AES DMA Raw Interrupt Status"]
pub mod aes_dmaris;
#[doc = "AES_DMAMIS register accessor: an alias for `Reg<AES_DMAMIS_SPEC>`"]
pub type AES_DMAMIS = crate::Reg<aes_dmamis::AES_DMAMIS_SPEC>;
#[doc = "AES DMA Masked Interrupt Status"]
pub mod aes_dmamis;
#[doc = "AES_DMAIC register accessor: an alias for `Reg<AES_DMAIC_SPEC>`"]
pub type AES_DMAIC = crate::Reg<aes_dmaic::AES_DMAIC_SPEC>;
#[doc = "AES DMA Interrupt Clear"]
pub mod aes_dmaic;
