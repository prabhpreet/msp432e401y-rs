#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x04 - DMA Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - DMA Channel Control Base Pointer"]
    pub ctlbase: crate::Reg<ctlbase::CTLBASE_SPEC>,
    #[doc = "0x0c - DMA Alternate Channel Control Base Pointer"]
    pub altbase: crate::Reg<altbase::ALTBASE_SPEC>,
    #[doc = "0x10 - DMA Channel Wait-on-Request Status"]
    pub waitstat: crate::Reg<waitstat::WAITSTAT_SPEC>,
    #[doc = "0x14 - DMA Channel Software Request"]
    pub swreq: crate::Reg<swreq::SWREQ_SPEC>,
    #[doc = "0x18 - DMA Channel Useburst Set"]
    pub useburstset: crate::Reg<useburstset::USEBURSTSET_SPEC>,
    #[doc = "0x1c - DMA Channel Useburst Clear"]
    pub useburstclr: crate::Reg<useburstclr::USEBURSTCLR_SPEC>,
    #[doc = "0x20 - DMA Channel Request Mask Set"]
    pub reqmaskset: crate::Reg<reqmaskset::REQMASKSET_SPEC>,
    #[doc = "0x24 - DMA Channel Request Mask Clear"]
    pub reqmaskclr: crate::Reg<reqmaskclr::REQMASKCLR_SPEC>,
    #[doc = "0x28 - DMA Channel Enable Set"]
    pub enaset: crate::Reg<enaset::ENASET_SPEC>,
    #[doc = "0x2c - DMA Channel Enable Clear"]
    pub enaclr: crate::Reg<enaclr::ENACLR_SPEC>,
    #[doc = "0x30 - DMA Channel Primary Alternate Set"]
    pub altset: crate::Reg<altset::ALTSET_SPEC>,
    #[doc = "0x34 - DMA Channel Primary Alternate Clear"]
    pub altclr: crate::Reg<altclr::ALTCLR_SPEC>,
    #[doc = "0x38 - DMA Channel Priority Set"]
    pub prioset: crate::Reg<prioset::PRIOSET_SPEC>,
    #[doc = "0x3c - DMA Channel Priority Clear"]
    pub prioclr: crate::Reg<prioclr::PRIOCLR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - DMA Bus Error Clear"]
    pub errclr: crate::Reg<errclr::ERRCLR_SPEC>,
    _reserved17: [u8; 0x04c0],
    #[doc = "0x510 - DMA Channel Map Select 0"]
    pub chmap0: crate::Reg<chmap0::CHMAP0_SPEC>,
    #[doc = "0x514 - DMA Channel Map Select 1"]
    pub chmap1: crate::Reg<chmap1::CHMAP1_SPEC>,
    #[doc = "0x518 - DMA Channel Map Select 2"]
    pub chmap2: crate::Reg<chmap2::CHMAP2_SPEC>,
    #[doc = "0x51c - DMA Channel Map Select 3"]
    pub chmap3: crate::Reg<chmap3::CHMAP3_SPEC>,
}
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "DMA Status"]
pub mod stat;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DMA Configuration"]
pub mod cfg;
#[doc = "CTLBASE register accessor: an alias for `Reg<CTLBASE_SPEC>`"]
pub type CTLBASE = crate::Reg<ctlbase::CTLBASE_SPEC>;
#[doc = "DMA Channel Control Base Pointer"]
pub mod ctlbase;
#[doc = "ALTBASE register accessor: an alias for `Reg<ALTBASE_SPEC>`"]
pub type ALTBASE = crate::Reg<altbase::ALTBASE_SPEC>;
#[doc = "DMA Alternate Channel Control Base Pointer"]
pub mod altbase;
#[doc = "WAITSTAT register accessor: an alias for `Reg<WAITSTAT_SPEC>`"]
pub type WAITSTAT = crate::Reg<waitstat::WAITSTAT_SPEC>;
#[doc = "DMA Channel Wait-on-Request Status"]
pub mod waitstat;
#[doc = "SWREQ register accessor: an alias for `Reg<SWREQ_SPEC>`"]
pub type SWREQ = crate::Reg<swreq::SWREQ_SPEC>;
#[doc = "DMA Channel Software Request"]
pub mod swreq;
#[doc = "USEBURSTSET register accessor: an alias for `Reg<USEBURSTSET_SPEC>`"]
pub type USEBURSTSET = crate::Reg<useburstset::USEBURSTSET_SPEC>;
#[doc = "DMA Channel Useburst Set"]
pub mod useburstset;
#[doc = "USEBURSTCLR register accessor: an alias for `Reg<USEBURSTCLR_SPEC>`"]
pub type USEBURSTCLR = crate::Reg<useburstclr::USEBURSTCLR_SPEC>;
#[doc = "DMA Channel Useburst Clear"]
pub mod useburstclr;
#[doc = "REQMASKSET register accessor: an alias for `Reg<REQMASKSET_SPEC>`"]
pub type REQMASKSET = crate::Reg<reqmaskset::REQMASKSET_SPEC>;
#[doc = "DMA Channel Request Mask Set"]
pub mod reqmaskset;
#[doc = "REQMASKCLR register accessor: an alias for `Reg<REQMASKCLR_SPEC>`"]
pub type REQMASKCLR = crate::Reg<reqmaskclr::REQMASKCLR_SPEC>;
#[doc = "DMA Channel Request Mask Clear"]
pub mod reqmaskclr;
#[doc = "ENASET register accessor: an alias for `Reg<ENASET_SPEC>`"]
pub type ENASET = crate::Reg<enaset::ENASET_SPEC>;
#[doc = "DMA Channel Enable Set"]
pub mod enaset;
#[doc = "ENACLR register accessor: an alias for `Reg<ENACLR_SPEC>`"]
pub type ENACLR = crate::Reg<enaclr::ENACLR_SPEC>;
#[doc = "DMA Channel Enable Clear"]
pub mod enaclr;
#[doc = "ALTSET register accessor: an alias for `Reg<ALTSET_SPEC>`"]
pub type ALTSET = crate::Reg<altset::ALTSET_SPEC>;
#[doc = "DMA Channel Primary Alternate Set"]
pub mod altset;
#[doc = "ALTCLR register accessor: an alias for `Reg<ALTCLR_SPEC>`"]
pub type ALTCLR = crate::Reg<altclr::ALTCLR_SPEC>;
#[doc = "DMA Channel Primary Alternate Clear"]
pub mod altclr;
#[doc = "PRIOSET register accessor: an alias for `Reg<PRIOSET_SPEC>`"]
pub type PRIOSET = crate::Reg<prioset::PRIOSET_SPEC>;
#[doc = "DMA Channel Priority Set"]
pub mod prioset;
#[doc = "PRIOCLR register accessor: an alias for `Reg<PRIOCLR_SPEC>`"]
pub type PRIOCLR = crate::Reg<prioclr::PRIOCLR_SPEC>;
#[doc = "DMA Channel Priority Clear"]
pub mod prioclr;
#[doc = "ERRCLR register accessor: an alias for `Reg<ERRCLR_SPEC>`"]
pub type ERRCLR = crate::Reg<errclr::ERRCLR_SPEC>;
#[doc = "DMA Bus Error Clear"]
pub mod errclr;
#[doc = "CHMAP0 register accessor: an alias for `Reg<CHMAP0_SPEC>`"]
pub type CHMAP0 = crate::Reg<chmap0::CHMAP0_SPEC>;
#[doc = "DMA Channel Map Select 0"]
pub mod chmap0;
#[doc = "CHMAP1 register accessor: an alias for `Reg<CHMAP1_SPEC>`"]
pub type CHMAP1 = crate::Reg<chmap1::CHMAP1_SPEC>;
#[doc = "DMA Channel Map Select 1"]
pub mod chmap1;
#[doc = "CHMAP2 register accessor: an alias for `Reg<CHMAP2_SPEC>`"]
pub type CHMAP2 = crate::Reg<chmap2::CHMAP2_SPEC>;
#[doc = "DMA Channel Map Select 2"]
pub mod chmap2;
#[doc = "CHMAP3 register accessor: an alias for `Reg<CHMAP3_SPEC>`"]
pub type CHMAP3 = crate::Reg<chmap3::CHMAP3_SPEC>;
#[doc = "DMA Channel Map Select 3"]
pub mod chmap3;
