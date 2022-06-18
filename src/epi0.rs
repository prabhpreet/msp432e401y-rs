#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EPI Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - EPI Main Baud Rate"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    #[doc = "0x08 - EPI Main Baud Rate"]
    pub baud2: crate::Reg<baud2::BAUD2_SPEC>,
    _reserved3: [u8; 0x04],
    _reserved_3_gpcfg: [u8; 0x04],
    _reserved_4_epi_alt: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - EPI Address Map"]
    pub addrmap: crate::Reg<addrmap::ADDRMAP_SPEC>,
    #[doc = "0x20 - EPI Read Size 0"]
    pub rsize0: crate::Reg<rsize0::RSIZE0_SPEC>,
    #[doc = "0x24 - EPI Read Address 0"]
    pub raddr0: crate::Reg<raddr0::RADDR0_SPEC>,
    #[doc = "0x28 - EPI Non-Blocking Read Data 0"]
    pub rpstd0: crate::Reg<rpstd0::RPSTD0_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - EPI Read Size 1"]
    pub rsize1: crate::Reg<rsize1::RSIZE1_SPEC>,
    #[doc = "0x34 - EPI Read Address 1"]
    pub raddr1: crate::Reg<raddr1::RADDR1_SPEC>,
    #[doc = "0x38 - EPI Non-Blocking Read Data 1"]
    pub rpstd1: crate::Reg<rpstd1::RPSTD1_SPEC>,
    _reserved12: [u8; 0x24],
    #[doc = "0x60 - EPI Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x6c - EPI Read FIFO Count"]
    pub rfifocnt: crate::Reg<rfifocnt::RFIFOCNT_SPEC>,
    #[doc = "0x70 - EPI Read FIFO"]
    pub readfifo0: crate::Reg<readfifo0::READFIFO0_SPEC>,
    #[doc = "0x74 - EPI Read FIFO Alias 1"]
    pub readfifo1: crate::Reg<readfifo1::READFIFO1_SPEC>,
    #[doc = "0x78 - EPI Read FIFO Alias 2"]
    pub readfifo2: crate::Reg<readfifo2::READFIFO2_SPEC>,
    #[doc = "0x7c - EPI Read FIFO Alias 3"]
    pub readfifo3: crate::Reg<readfifo3::READFIFO3_SPEC>,
    #[doc = "0x80 - EPI Read FIFO Alias 4"]
    pub readfifo4: crate::Reg<readfifo4::READFIFO4_SPEC>,
    #[doc = "0x84 - EPI Read FIFO Alias 5"]
    pub readfifo5: crate::Reg<readfifo5::READFIFO5_SPEC>,
    #[doc = "0x88 - EPI Read FIFO Alias 6"]
    pub readfifo6: crate::Reg<readfifo6::READFIFO6_SPEC>,
    #[doc = "0x8c - EPI Read FIFO Alias 7"]
    pub readfifo7: crate::Reg<readfifo7::READFIFO7_SPEC>,
    _reserved22: [u8; 0x0170],
    #[doc = "0x200 - EPI FIFO Level Selects"]
    pub fifolvl: crate::Reg<fifolvl::FIFOLVL_SPEC>,
    #[doc = "0x204 - EPI Write FIFO Count"]
    pub wfifocnt: crate::Reg<wfifocnt::WFIFOCNT_SPEC>,
    #[doc = "0x208 - EPI DMA Transmit Count"]
    pub dmatxcnt: crate::Reg<dmatxcnt::DMATXCNT_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x210 - EPI Interrupt Mask"]
    pub im: crate::Reg<im::IM_SPEC>,
    #[doc = "0x214 - EPI Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x218 - EPI Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x21c - EPI Error and Interrupt Status and Clear"]
    pub eisc: crate::Reg<eisc::EISC_SPEC>,
    _reserved29: [u8; 0xe8],
    _reserved_29_hb8cfg3: [u8; 0x04],
    _reserved_30_hb16cfg4: [u8; 0x04],
    _reserved_31_hb8time: [u8; 0x04],
    _reserved_32_hb8time2: [u8; 0x04],
    _reserved_33_hb16time3: [u8; 0x04],
    _reserved_34_hb16time4: [u8; 0x04],
    _reserved35: [u8; 0x40],
    #[doc = "0x360 - EPI Host-Bus PSRAM"]
    pub hbpsram: crate::Reg<hbpsram::HBPSRAM_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x10 - EPI Host-Bus 8 Configuration"]
    #[inline(always)]
    pub fn epi_alt8_hb8cfg(&self) -> &crate::Reg<epi_alt8_hb8cfg::EPI_ALT8_HB8CFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<epi_alt8_hb8cfg::EPI_ALT8_HB8CFG_SPEC>)
        }
    }
    #[doc = "0x10 - EPI SDRAM Configuration"]
    #[inline(always)]
    pub fn epi_altsd_sdramcfg(&self) -> &crate::Reg<epi_altsd_sdramcfg::EPI_ALTSD_SDRAMCFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<epi_altsd_sdramcfg::EPI_ALTSD_SDRAMCFG_SPEC>)
        }
    }
    #[doc = "0x10 - EPI General-Purpose Configuration"]
    #[inline(always)]
    pub fn gpcfg(&self) -> &crate::Reg<gpcfg::GPCFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<gpcfg::GPCFG_SPEC>)
        }
    }
    #[doc = "0x10 - EPI Host-Bus 16 Configuration"]
    #[inline(always)]
    pub fn epi_alt16_hb16cfg(&self) -> &crate::Reg<epi_alt16_hb16cfg::EPI_ALT16_HB16CFG_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<epi_alt16_hb16cfg::EPI_ALT16_HB16CFG_SPEC>)
        }
    }
    #[doc = "0x14 - EPI Host-Bus 16 Configuration 2"]
    #[inline(always)]
    pub fn epi_alt16_hb16cfg2(&self) -> &crate::Reg<epi_alt16_hb16cfg2::EPI_ALT16_HB16CFG2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<epi_alt16_hb16cfg2::EPI_ALT16_HB16CFG2_SPEC>)
        }
    }
    #[doc = "0x14 - EPI Host-Bus 8 Configuration 2"]
    #[inline(always)]
    pub fn epi_alt8_hb8cfg2(&self) -> &crate::Reg<epi_alt8_hb8cfg2::EPI_ALT8_HB8CFG2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<epi_alt8_hb8cfg2::EPI_ALT8_HB8CFG2_SPEC>)
        }
    }
    #[doc = "0x308 - EPI Host-Bus 16 Configuration 3"]
    #[inline(always)]
    pub fn epi_alt16_hb16cfg3(&self) -> &crate::Reg<epi_alt16_hb16cfg3::EPI_ALT16_HB16CFG3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(776usize)
                as *const crate::Reg<epi_alt16_hb16cfg3::EPI_ALT16_HB16CFG3_SPEC>)
        }
    }
    #[doc = "0x308 - EPI Host-Bus 8 Configuration 3"]
    #[inline(always)]
    pub fn hb8cfg3(&self) -> &crate::Reg<hb8cfg3::HB8CFG3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(776usize)
                as *const crate::Reg<hb8cfg3::HB8CFG3_SPEC>)
        }
    }
    #[doc = "0x30c - EPI Host-Bus 8 Configuration 4"]
    #[inline(always)]
    pub fn epi_alt8_hb8cfg4(&self) -> &crate::Reg<epi_alt8_hb8cfg4::EPI_ALT8_HB8CFG4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(780usize)
                as *const crate::Reg<epi_alt8_hb8cfg4::EPI_ALT8_HB8CFG4_SPEC>)
        }
    }
    #[doc = "0x30c - EPI Host-Bus 16 Configuration 4"]
    #[inline(always)]
    pub fn hb16cfg4(&self) -> &crate::Reg<hb16cfg4::HB16CFG4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(780usize)
                as *const crate::Reg<hb16cfg4::HB16CFG4_SPEC>)
        }
    }
    #[doc = "0x310 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn epi_alt16_hb16time(&self) -> &crate::Reg<epi_alt16_hb16time::EPI_ALT16_HB16TIME_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(784usize)
                as *const crate::Reg<epi_alt16_hb16time::EPI_ALT16_HB16TIME_SPEC>)
        }
    }
    #[doc = "0x310 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time(&self) -> &crate::Reg<hb8time::HB8TIME_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(784usize)
                as *const crate::Reg<hb8time::HB8TIME_SPEC>)
        }
    }
    #[doc = "0x314 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn epi_alt16_hb16time2(
        &self,
    ) -> &crate::Reg<epi_alt16_hb16time2::EPI_ALT16_HB16TIME2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(788usize)
                as *const crate::Reg<epi_alt16_hb16time2::EPI_ALT16_HB16TIME2_SPEC>)
        }
    }
    #[doc = "0x314 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn hb8time2(&self) -> &crate::Reg<hb8time2::HB8TIME2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(788usize)
                as *const crate::Reg<hb8time2::HB8TIME2_SPEC>)
        }
    }
    #[doc = "0x318 - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn epi_alt8_hb8time3(&self) -> &crate::Reg<epi_alt8_hb8time3::EPI_ALT8_HB8TIME3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(792usize)
                as *const crate::Reg<epi_alt8_hb8time3::EPI_ALT8_HB8TIME3_SPEC>)
        }
    }
    #[doc = "0x318 - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time3(&self) -> &crate::Reg<hb16time3::HB16TIME3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(792usize)
                as *const crate::Reg<hb16time3::HB16TIME3_SPEC>)
        }
    }
    #[doc = "0x31c - EPI Host-Bus 16 Timing Extension"]
    #[inline(always)]
    pub fn hb16time4(&self) -> &crate::Reg<hb16time4::HB16TIME4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(796usize)
                as *const crate::Reg<hb16time4::HB16TIME4_SPEC>)
        }
    }
    #[doc = "0x31c - EPI Host-Bus 8 Timing Extension"]
    #[inline(always)]
    pub fn epi_alt8_hb8time4(&self) -> &crate::Reg<epi_alt8_hb8time4::EPI_ALT8_HB8TIME4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(796usize)
                as *const crate::Reg<epi_alt8_hb8time4::EPI_ALT8_HB8TIME4_SPEC>)
        }
    }
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "EPI Configuration"]
pub mod cfg;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "EPI Main Baud Rate"]
pub mod baud;
#[doc = "BAUD2 register accessor: an alias for `Reg<BAUD2_SPEC>`"]
pub type BAUD2 = crate::Reg<baud2::BAUD2_SPEC>;
#[doc = "EPI Main Baud Rate"]
pub mod baud2;
#[doc = "EPI_ALT16_HB16CFG register accessor: an alias for `Reg<EPI_ALT16_HB16CFG_SPEC>`"]
pub type EPI_ALT16_HB16CFG = crate::Reg<epi_alt16_hb16cfg::EPI_ALT16_HB16CFG_SPEC>;
#[doc = "EPI Host-Bus 16 Configuration"]
pub mod epi_alt16_hb16cfg;
#[doc = "GPCFG register accessor: an alias for `Reg<GPCFG_SPEC>`"]
pub type GPCFG = crate::Reg<gpcfg::GPCFG_SPEC>;
#[doc = "EPI General-Purpose Configuration"]
pub mod gpcfg;
#[doc = "EPI_ALTSD_SDRAMCFG register accessor: an alias for `Reg<EPI_ALTSD_SDRAMCFG_SPEC>`"]
pub type EPI_ALTSD_SDRAMCFG = crate::Reg<epi_altsd_sdramcfg::EPI_ALTSD_SDRAMCFG_SPEC>;
#[doc = "EPI SDRAM Configuration"]
pub mod epi_altsd_sdramcfg;
#[doc = "EPI_ALT8_HB8CFG register accessor: an alias for `Reg<EPI_ALT8_HB8CFG_SPEC>`"]
pub type EPI_ALT8_HB8CFG = crate::Reg<epi_alt8_hb8cfg::EPI_ALT8_HB8CFG_SPEC>;
#[doc = "EPI Host-Bus 8 Configuration"]
pub mod epi_alt8_hb8cfg;
#[doc = "EPI_ALT8_HB8CFG2 register accessor: an alias for `Reg<EPI_ALT8_HB8CFG2_SPEC>`"]
pub type EPI_ALT8_HB8CFG2 = crate::Reg<epi_alt8_hb8cfg2::EPI_ALT8_HB8CFG2_SPEC>;
#[doc = "EPI Host-Bus 8 Configuration 2"]
pub mod epi_alt8_hb8cfg2;
#[doc = "EPI_ALT16_HB16CFG2 register accessor: an alias for `Reg<EPI_ALT16_HB16CFG2_SPEC>`"]
pub type EPI_ALT16_HB16CFG2 = crate::Reg<epi_alt16_hb16cfg2::EPI_ALT16_HB16CFG2_SPEC>;
#[doc = "EPI Host-Bus 16 Configuration 2"]
pub mod epi_alt16_hb16cfg2;
#[doc = "ADDRMAP register accessor: an alias for `Reg<ADDRMAP_SPEC>`"]
pub type ADDRMAP = crate::Reg<addrmap::ADDRMAP_SPEC>;
#[doc = "EPI Address Map"]
pub mod addrmap;
#[doc = "RSIZE0 register accessor: an alias for `Reg<RSIZE0_SPEC>`"]
pub type RSIZE0 = crate::Reg<rsize0::RSIZE0_SPEC>;
#[doc = "EPI Read Size 0"]
pub mod rsize0;
#[doc = "RADDR0 register accessor: an alias for `Reg<RADDR0_SPEC>`"]
pub type RADDR0 = crate::Reg<raddr0::RADDR0_SPEC>;
#[doc = "EPI Read Address 0"]
pub mod raddr0;
#[doc = "RPSTD0 register accessor: an alias for `Reg<RPSTD0_SPEC>`"]
pub type RPSTD0 = crate::Reg<rpstd0::RPSTD0_SPEC>;
#[doc = "EPI Non-Blocking Read Data 0"]
pub mod rpstd0;
#[doc = "RSIZE1 register accessor: an alias for `Reg<RSIZE1_SPEC>`"]
pub type RSIZE1 = crate::Reg<rsize1::RSIZE1_SPEC>;
#[doc = "EPI Read Size 1"]
pub mod rsize1;
#[doc = "RADDR1 register accessor: an alias for `Reg<RADDR1_SPEC>`"]
pub type RADDR1 = crate::Reg<raddr1::RADDR1_SPEC>;
#[doc = "EPI Read Address 1"]
pub mod raddr1;
#[doc = "RPSTD1 register accessor: an alias for `Reg<RPSTD1_SPEC>`"]
pub type RPSTD1 = crate::Reg<rpstd1::RPSTD1_SPEC>;
#[doc = "EPI Non-Blocking Read Data 1"]
pub mod rpstd1;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "EPI Status"]
pub mod stat;
#[doc = "RFIFOCNT register accessor: an alias for `Reg<RFIFOCNT_SPEC>`"]
pub type RFIFOCNT = crate::Reg<rfifocnt::RFIFOCNT_SPEC>;
#[doc = "EPI Read FIFO Count"]
pub mod rfifocnt;
#[doc = "READFIFO0 register accessor: an alias for `Reg<READFIFO0_SPEC>`"]
pub type READFIFO0 = crate::Reg<readfifo0::READFIFO0_SPEC>;
#[doc = "EPI Read FIFO"]
pub mod readfifo0;
#[doc = "READFIFO1 register accessor: an alias for `Reg<READFIFO1_SPEC>`"]
pub type READFIFO1 = crate::Reg<readfifo1::READFIFO1_SPEC>;
#[doc = "EPI Read FIFO Alias 1"]
pub mod readfifo1;
#[doc = "READFIFO2 register accessor: an alias for `Reg<READFIFO2_SPEC>`"]
pub type READFIFO2 = crate::Reg<readfifo2::READFIFO2_SPEC>;
#[doc = "EPI Read FIFO Alias 2"]
pub mod readfifo2;
#[doc = "READFIFO3 register accessor: an alias for `Reg<READFIFO3_SPEC>`"]
pub type READFIFO3 = crate::Reg<readfifo3::READFIFO3_SPEC>;
#[doc = "EPI Read FIFO Alias 3"]
pub mod readfifo3;
#[doc = "READFIFO4 register accessor: an alias for `Reg<READFIFO4_SPEC>`"]
pub type READFIFO4 = crate::Reg<readfifo4::READFIFO4_SPEC>;
#[doc = "EPI Read FIFO Alias 4"]
pub mod readfifo4;
#[doc = "READFIFO5 register accessor: an alias for `Reg<READFIFO5_SPEC>`"]
pub type READFIFO5 = crate::Reg<readfifo5::READFIFO5_SPEC>;
#[doc = "EPI Read FIFO Alias 5"]
pub mod readfifo5;
#[doc = "READFIFO6 register accessor: an alias for `Reg<READFIFO6_SPEC>`"]
pub type READFIFO6 = crate::Reg<readfifo6::READFIFO6_SPEC>;
#[doc = "EPI Read FIFO Alias 6"]
pub mod readfifo6;
#[doc = "READFIFO7 register accessor: an alias for `Reg<READFIFO7_SPEC>`"]
pub type READFIFO7 = crate::Reg<readfifo7::READFIFO7_SPEC>;
#[doc = "EPI Read FIFO Alias 7"]
pub mod readfifo7;
#[doc = "FIFOLVL register accessor: an alias for `Reg<FIFOLVL_SPEC>`"]
pub type FIFOLVL = crate::Reg<fifolvl::FIFOLVL_SPEC>;
#[doc = "EPI FIFO Level Selects"]
pub mod fifolvl;
#[doc = "WFIFOCNT register accessor: an alias for `Reg<WFIFOCNT_SPEC>`"]
pub type WFIFOCNT = crate::Reg<wfifocnt::WFIFOCNT_SPEC>;
#[doc = "EPI Write FIFO Count"]
pub mod wfifocnt;
#[doc = "DMATXCNT register accessor: an alias for `Reg<DMATXCNT_SPEC>`"]
pub type DMATXCNT = crate::Reg<dmatxcnt::DMATXCNT_SPEC>;
#[doc = "EPI DMA Transmit Count"]
pub mod dmatxcnt;
#[doc = "IM register accessor: an alias for `Reg<IM_SPEC>`"]
pub type IM = crate::Reg<im::IM_SPEC>;
#[doc = "EPI Interrupt Mask"]
pub mod im;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "EPI Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "EPI Masked Interrupt Status"]
pub mod mis;
#[doc = "EISC register accessor: an alias for `Reg<EISC_SPEC>`"]
pub type EISC = crate::Reg<eisc::EISC_SPEC>;
#[doc = "EPI Error and Interrupt Status and Clear"]
pub mod eisc;
#[doc = "HB8CFG3 register accessor: an alias for `Reg<HB8CFG3_SPEC>`"]
pub type HB8CFG3 = crate::Reg<hb8cfg3::HB8CFG3_SPEC>;
#[doc = "EPI Host-Bus 8 Configuration 3"]
pub mod hb8cfg3;
#[doc = "EPI_ALT16_HB16CFG3 register accessor: an alias for `Reg<EPI_ALT16_HB16CFG3_SPEC>`"]
pub type EPI_ALT16_HB16CFG3 = crate::Reg<epi_alt16_hb16cfg3::EPI_ALT16_HB16CFG3_SPEC>;
#[doc = "EPI Host-Bus 16 Configuration 3"]
pub mod epi_alt16_hb16cfg3;
#[doc = "HB16CFG4 register accessor: an alias for `Reg<HB16CFG4_SPEC>`"]
pub type HB16CFG4 = crate::Reg<hb16cfg4::HB16CFG4_SPEC>;
#[doc = "EPI Host-Bus 16 Configuration 4"]
pub mod hb16cfg4;
#[doc = "EPI_ALT8_HB8CFG4 register accessor: an alias for `Reg<EPI_ALT8_HB8CFG4_SPEC>`"]
pub type EPI_ALT8_HB8CFG4 = crate::Reg<epi_alt8_hb8cfg4::EPI_ALT8_HB8CFG4_SPEC>;
#[doc = "EPI Host-Bus 8 Configuration 4"]
pub mod epi_alt8_hb8cfg4;
#[doc = "HB8TIME register accessor: an alias for `Reg<HB8TIME_SPEC>`"]
pub type HB8TIME = crate::Reg<hb8time::HB8TIME_SPEC>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time;
#[doc = "EPI_ALT16_HB16TIME register accessor: an alias for `Reg<EPI_ALT16_HB16TIME_SPEC>`"]
pub type EPI_ALT16_HB16TIME = crate::Reg<epi_alt16_hb16time::EPI_ALT16_HB16TIME_SPEC>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod epi_alt16_hb16time;
#[doc = "HB8TIME2 register accessor: an alias for `Reg<HB8TIME2_SPEC>`"]
pub type HB8TIME2 = crate::Reg<hb8time2::HB8TIME2_SPEC>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod hb8time2;
#[doc = "EPI_ALT16_HB16TIME2 register accessor: an alias for `Reg<EPI_ALT16_HB16TIME2_SPEC>`"]
pub type EPI_ALT16_HB16TIME2 = crate::Reg<epi_alt16_hb16time2::EPI_ALT16_HB16TIME2_SPEC>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod epi_alt16_hb16time2;
#[doc = "HB16TIME3 register accessor: an alias for `Reg<HB16TIME3_SPEC>`"]
pub type HB16TIME3 = crate::Reg<hb16time3::HB16TIME3_SPEC>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time3;
#[doc = "EPI_ALT8_HB8TIME3 register accessor: an alias for `Reg<EPI_ALT8_HB8TIME3_SPEC>`"]
pub type EPI_ALT8_HB8TIME3 = crate::Reg<epi_alt8_hb8time3::EPI_ALT8_HB8TIME3_SPEC>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod epi_alt8_hb8time3;
#[doc = "EPI_ALT8_HB8TIME4 register accessor: an alias for `Reg<EPI_ALT8_HB8TIME4_SPEC>`"]
pub type EPI_ALT8_HB8TIME4 = crate::Reg<epi_alt8_hb8time4::EPI_ALT8_HB8TIME4_SPEC>;
#[doc = "EPI Host-Bus 8 Timing Extension"]
pub mod epi_alt8_hb8time4;
#[doc = "HB16TIME4 register accessor: an alias for `Reg<HB16TIME4_SPEC>`"]
pub type HB16TIME4 = crate::Reg<hb16time4::HB16TIME4_SPEC>;
#[doc = "EPI Host-Bus 16 Timing Extension"]
pub mod hb16time4;
#[doc = "HBPSRAM register accessor: an alias for `Reg<HBPSRAM_SPEC>`"]
pub type HBPSRAM = crate::Reg<hbpsram::HBPSRAM_SPEC>;
#[doc = "EPI Host-Bus PSRAM"]
pub mod hbpsram;
