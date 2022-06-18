#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Functional Address"]
    pub faddr: crate::Reg<faddr::FADDR_SPEC>,
    #[doc = "0x01 - USB Power"]
    pub power: crate::Reg<power::POWER_SPEC>,
    #[doc = "0x02 - USB Transmit Interrupt Status"]
    pub txis: crate::Reg<txis::TXIS_SPEC>,
    #[doc = "0x04 - USB Receive Interrupt Status"]
    pub rxis: crate::Reg<rxis::RXIS_SPEC>,
    #[doc = "0x06 - USB Transmit Interrupt Enable"]
    pub txie: crate::Reg<txie::TXIE_SPEC>,
    #[doc = "0x08 - USB Receive Interrupt Enable"]
    pub rxie: crate::Reg<rxie::RXIE_SPEC>,
    _reserved_6_is: [u8; 0x01],
    _reserved_7_ie: [u8; 0x01],
    #[doc = "0x0c - USB Frame Value"]
    pub frame: crate::Reg<frame::FRAME_SPEC>,
    #[doc = "0x0e - USB Endpoint Index"]
    pub epidx: crate::Reg<epidx::EPIDX_SPEC>,
    #[doc = "0x0f - USB Test Mode"]
    pub test: crate::Reg<test::TEST_SPEC>,
    _reserved11: [u8; 0x10],
    #[doc = "0x20 - USB FIFO Endpoint 0"]
    pub fifo0: crate::Reg<fifo0::FIFO0_SPEC>,
    #[doc = "0x24 - USB FIFO Endpoint 1"]
    pub fifo1: crate::Reg<fifo1::FIFO1_SPEC>,
    #[doc = "0x28 - USB FIFO Endpoint 2"]
    pub fifo2: crate::Reg<fifo2::FIFO2_SPEC>,
    #[doc = "0x2c - USB FIFO Endpoint 3"]
    pub fifo3: crate::Reg<fifo3::FIFO3_SPEC>,
    #[doc = "0x30 - USB FIFO Endpoint 4"]
    pub fifo4: crate::Reg<fifo4::FIFO4_SPEC>,
    #[doc = "0x34 - USB FIFO Endpoint 5"]
    pub fifo5: crate::Reg<fifo5::FIFO5_SPEC>,
    #[doc = "0x38 - USB FIFO Endpoint 6"]
    pub fifo6: crate::Reg<fifo6::FIFO6_SPEC>,
    #[doc = "0x3c - USB FIFO Endpoint 7"]
    pub fifo7: crate::Reg<fifo7::FIFO7_SPEC>,
    _reserved19: [u8; 0x20],
    #[doc = "0x60 - USB Device Control"]
    pub devctl: crate::Reg<devctl::DEVCTL_SPEC>,
    #[doc = "0x61 - USB Common Configuration"]
    pub cconf: crate::Reg<cconf::CCONF_SPEC>,
    #[doc = "0x62 - USB Transmit Dynamic FIFO Sizing"]
    pub txfifosz: crate::Reg<txfifosz::TXFIFOSZ_SPEC>,
    #[doc = "0x63 - USB Receive Dynamic FIFO Sizing"]
    pub rxfifosz: crate::Reg<rxfifosz::RXFIFOSZ_SPEC>,
    #[doc = "0x64 - USB Transmit FIFO Start Address"]
    pub txfifoadd: crate::Reg<txfifoadd::TXFIFOADD_SPEC>,
    #[doc = "0x66 - USB Receive FIFO Start Address"]
    pub rxfifoadd: crate::Reg<rxfifoadd::RXFIFOADD_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x70 - USB ULPI VBUS Control"]
    pub ulpivbusctl: crate::Reg<ulpivbusctl::ULPIVBUSCTL_SPEC>,
    _reserved26: [u8; 0x03],
    #[doc = "0x74 - USB ULPI Register Data"]
    pub ulpiregdata: crate::Reg<ulpiregdata::ULPIREGDATA_SPEC>,
    #[doc = "0x75 - USB ULPI Register Address"]
    pub ulpiregaddr: crate::Reg<ulpiregaddr::ULPIREGADDR_SPEC>,
    #[doc = "0x76 - USB ULPI Register Control"]
    pub ulpiregctl: crate::Reg<ulpiregctl::ULPIREGCTL_SPEC>,
    _reserved29: [u8; 0x01],
    #[doc = "0x78 - USB Endpoint Information"]
    pub epinfo: crate::Reg<epinfo::EPINFO_SPEC>,
    #[doc = "0x79 - USB RAM Information"]
    pub raminfo: crate::Reg<raminfo::RAMINFO_SPEC>,
    #[doc = "0x7a - USB Connect Timing"]
    pub contim: crate::Reg<contim::CONTIM_SPEC>,
    #[doc = "0x7b - USB OTG VBUS Pulse Timing"]
    pub vplen: crate::Reg<vplen::VPLEN_SPEC>,
    #[doc = "0x7c - USB High-Speed Last Transaction to End of Frame Timing"]
    pub hseof: crate::Reg<hseof::HSEOF_SPEC>,
    #[doc = "0x7d - USB Full-Speed Last Transaction to End of Frame Timing"]
    pub fseof: crate::Reg<fseof::FSEOF_SPEC>,
    #[doc = "0x7e - USB Low-Speed Last Transaction to End of Frame Timing"]
    pub lseof: crate::Reg<lseof::LSEOF_SPEC>,
    _reserved36: [u8; 0x01],
    #[doc = "0x80 - USB Transmit Functional Address Endpoint 0"]
    pub txfuncaddr0: crate::Reg<txfuncaddr0::TXFUNCADDR0_SPEC>,
    _reserved37: [u8; 0x01],
    #[doc = "0x82 - USB Transmit Hub Address Endpoint 0"]
    pub txhubaddr0: crate::Reg<txhubaddr0::TXHUBADDR0_SPEC>,
    #[doc = "0x83 - USB Transmit Hub Port Endpoint 0"]
    pub txhubport0: crate::Reg<txhubport0::TXHUBPORT0_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0x88 - USB Transmit Functional Address Endpoint 1"]
    pub txfuncaddr1: crate::Reg<txfuncaddr1::TXFUNCADDR1_SPEC>,
    _reserved40: [u8; 0x01],
    #[doc = "0x8a - USB Transmit Hub Address Endpoint 1"]
    pub txhubaddr1: crate::Reg<txhubaddr1::TXHUBADDR1_SPEC>,
    #[doc = "0x8b - USB Transmit Hub Port Endpoint 1"]
    pub txhubport1: crate::Reg<txhubport1::TXHUBPORT1_SPEC>,
    #[doc = "0x8c - USB Receive Functional Address Endpoint 1"]
    pub rxfuncaddr1: crate::Reg<rxfuncaddr1::RXFUNCADDR1_SPEC>,
    _reserved43: [u8; 0x01],
    #[doc = "0x8e - USB Receive Hub Address Endpoint 1"]
    pub rxhubaddr1: crate::Reg<rxhubaddr1::RXHUBADDR1_SPEC>,
    #[doc = "0x8f - USB Receive Hub Port Endpoint 1"]
    pub rxhubport1: crate::Reg<rxhubport1::RXHUBPORT1_SPEC>,
    #[doc = "0x90 - USB Transmit Functional Address Endpoint 2"]
    pub txfuncaddr2: crate::Reg<txfuncaddr2::TXFUNCADDR2_SPEC>,
    _reserved46: [u8; 0x01],
    #[doc = "0x92 - USB Transmit Hub Address Endpoint 2"]
    pub txhubaddr2: crate::Reg<txhubaddr2::TXHUBADDR2_SPEC>,
    #[doc = "0x93 - USB Transmit Hub Port Endpoint 2"]
    pub txhubport2: crate::Reg<txhubport2::TXHUBPORT2_SPEC>,
    #[doc = "0x94 - USB Receive Functional Address Endpoint 2"]
    pub rxfuncaddr2: crate::Reg<rxfuncaddr2::RXFUNCADDR2_SPEC>,
    _reserved49: [u8; 0x01],
    #[doc = "0x96 - USB Receive Hub Address Endpoint 2"]
    pub rxhubaddr2: crate::Reg<rxhubaddr2::RXHUBADDR2_SPEC>,
    #[doc = "0x97 - USB Receive Hub Port Endpoint 2"]
    pub rxhubport2: crate::Reg<rxhubport2::RXHUBPORT2_SPEC>,
    #[doc = "0x98 - USB Transmit Functional Address Endpoint 3"]
    pub txfuncaddr3: crate::Reg<txfuncaddr3::TXFUNCADDR3_SPEC>,
    _reserved52: [u8; 0x01],
    #[doc = "0x9a - USB Transmit Hub Address Endpoint 3"]
    pub txhubaddr3: crate::Reg<txhubaddr3::TXHUBADDR3_SPEC>,
    #[doc = "0x9b - USB Transmit Hub Port Endpoint 3"]
    pub txhubport3: crate::Reg<txhubport3::TXHUBPORT3_SPEC>,
    #[doc = "0x9c - USB Receive Functional Address Endpoint 3"]
    pub rxfuncaddr3: crate::Reg<rxfuncaddr3::RXFUNCADDR3_SPEC>,
    _reserved55: [u8; 0x01],
    #[doc = "0x9e - USB Receive Hub Address Endpoint 3"]
    pub rxhubaddr3: crate::Reg<rxhubaddr3::RXHUBADDR3_SPEC>,
    #[doc = "0x9f - USB Receive Hub Port Endpoint 3"]
    pub rxhubport3: crate::Reg<rxhubport3::RXHUBPORT3_SPEC>,
    #[doc = "0xa0 - USB Transmit Functional Address Endpoint 4"]
    pub txfuncaddr4: crate::Reg<txfuncaddr4::TXFUNCADDR4_SPEC>,
    _reserved58: [u8; 0x01],
    #[doc = "0xa2 - USB Transmit Hub Address Endpoint 4"]
    pub txhubaddr4: crate::Reg<txhubaddr4::TXHUBADDR4_SPEC>,
    #[doc = "0xa3 - USB Transmit Hub Port Endpoint 4"]
    pub txhubport4: crate::Reg<txhubport4::TXHUBPORT4_SPEC>,
    #[doc = "0xa4 - USB Receive Functional Address Endpoint 4"]
    pub rxfuncaddr4: crate::Reg<rxfuncaddr4::RXFUNCADDR4_SPEC>,
    _reserved61: [u8; 0x01],
    #[doc = "0xa6 - USB Receive Hub Address Endpoint 4"]
    pub rxhubaddr4: crate::Reg<rxhubaddr4::RXHUBADDR4_SPEC>,
    #[doc = "0xa7 - USB Receive Hub Port Endpoint 4"]
    pub rxhubport4: crate::Reg<rxhubport4::RXHUBPORT4_SPEC>,
    #[doc = "0xa8 - USB Transmit Functional Address Endpoint 5"]
    pub txfuncaddr5: crate::Reg<txfuncaddr5::TXFUNCADDR5_SPEC>,
    _reserved64: [u8; 0x01],
    #[doc = "0xaa - USB Transmit Hub Address Endpoint 5"]
    pub txhubaddr5: crate::Reg<txhubaddr5::TXHUBADDR5_SPEC>,
    #[doc = "0xab - USB Transmit Hub Port Endpoint 5"]
    pub txhubport5: crate::Reg<txhubport5::TXHUBPORT5_SPEC>,
    #[doc = "0xac - USB Receive Functional Address Endpoint 5"]
    pub rxfuncaddr5: crate::Reg<rxfuncaddr5::RXFUNCADDR5_SPEC>,
    _reserved67: [u8; 0x01],
    #[doc = "0xae - USB Receive Hub Address Endpoint 5"]
    pub rxhubaddr5: crate::Reg<rxhubaddr5::RXHUBADDR5_SPEC>,
    #[doc = "0xaf - USB Receive Hub Port Endpoint 5"]
    pub rxhubport5: crate::Reg<rxhubport5::RXHUBPORT5_SPEC>,
    #[doc = "0xb0 - USB Transmit Functional Address Endpoint 6"]
    pub txfuncaddr6: crate::Reg<txfuncaddr6::TXFUNCADDR6_SPEC>,
    _reserved70: [u8; 0x01],
    #[doc = "0xb2 - USB Transmit Hub Address Endpoint 6"]
    pub txhubaddr6: crate::Reg<txhubaddr6::TXHUBADDR6_SPEC>,
    #[doc = "0xb3 - USB Transmit Hub Port Endpoint 6"]
    pub txhubport6: crate::Reg<txhubport6::TXHUBPORT6_SPEC>,
    #[doc = "0xb4 - USB Receive Functional Address Endpoint 6"]
    pub rxfuncaddr6: crate::Reg<rxfuncaddr6::RXFUNCADDR6_SPEC>,
    _reserved73: [u8; 0x01],
    #[doc = "0xb6 - USB Receive Hub Address Endpoint 6"]
    pub rxhubaddr6: crate::Reg<rxhubaddr6::RXHUBADDR6_SPEC>,
    #[doc = "0xb7 - USB Receive Hub Port Endpoint 6"]
    pub rxhubport6: crate::Reg<rxhubport6::RXHUBPORT6_SPEC>,
    #[doc = "0xb8 - USB Transmit Functional Address Endpoint 7"]
    pub txfuncaddr7: crate::Reg<txfuncaddr7::TXFUNCADDR7_SPEC>,
    _reserved76: [u8; 0x01],
    #[doc = "0xba - USB Transmit Hub Address Endpoint 7"]
    pub txhubaddr7: crate::Reg<txhubaddr7::TXHUBADDR7_SPEC>,
    #[doc = "0xbb - USB Transmit Hub Port Endpoint 7"]
    pub txhubport7: crate::Reg<txhubport7::TXHUBPORT7_SPEC>,
    #[doc = "0xbc - USB Receive Functional Address Endpoint 7"]
    pub rxfuncaddr7: crate::Reg<rxfuncaddr7::RXFUNCADDR7_SPEC>,
    _reserved79: [u8; 0x01],
    #[doc = "0xbe - USB Receive Hub Address Endpoint 7"]
    pub rxhubaddr7: crate::Reg<rxhubaddr7::RXHUBADDR7_SPEC>,
    #[doc = "0xbf - USB Receive Hub Port Endpoint 7"]
    pub rxhubport7: crate::Reg<rxhubport7::RXHUBPORT7_SPEC>,
    _reserved81: [u8; 0x42],
    _reserved_81_csrl0: [u8; 0x01],
    #[doc = "0x103 - USB Control and Status Endpoint 0 High"]
    pub csrh0: crate::Reg<csrh0::CSRH0_SPEC>,
    _reserved83: [u8; 0x04],
    #[doc = "0x108 - USB Receive Byte Count Endpoint 0"]
    pub count0: crate::Reg<count0::COUNT0_SPEC>,
    _reserved84: [u8; 0x01],
    #[doc = "0x10a - USB Type Endpoint 0"]
    pub type0: crate::Reg<type0::TYPE0_SPEC>,
    #[doc = "0x10b - USB NAK Limit"]
    pub naklmt: crate::Reg<naklmt::NAKLMT_SPEC>,
    _reserved86: [u8; 0x04],
    #[doc = "0x110 - USB Maximum Transmit Data Endpoint 1"]
    pub txmaxp1: crate::Reg<txmaxp1::TXMAXP1_SPEC>,
    _reserved_87_txcsrl1: [u8; 0x01],
    #[doc = "0x113 - USB Transmit Control and Status Endpoint 1 High"]
    pub txcsrh1: crate::Reg<txcsrh1::TXCSRH1_SPEC>,
    #[doc = "0x114 - USB Maximum Receive Data Endpoint 1"]
    pub rxmaxp1: crate::Reg<rxmaxp1::RXMAXP1_SPEC>,
    _reserved_90_rxcsrl1: [u8; 0x01],
    _reserved_91_rxcsrh1: [u8; 0x01],
    #[doc = "0x118 - USB Receive Byte Count Endpoint 1"]
    pub rxcount1: crate::Reg<rxcount1::RXCOUNT1_SPEC>,
    #[doc = "0x11a - USB Host Transmit Configure Type Endpoint 1"]
    pub txtype1: crate::Reg<txtype1::TXTYPE1_SPEC>,
    _reserved_94_txinterval1: [u8; 0x01],
    #[doc = "0x11c - USB Host Configure Receive Type Endpoint 1"]
    pub rxtype1: crate::Reg<rxtype1::RXTYPE1_SPEC>,
    _reserved_96_rxinterval1: [u8; 0x01],
    _reserved97: [u8; 0x02],
    #[doc = "0x120 - USB Maximum Transmit Data Endpoint 2"]
    pub txmaxp2: crate::Reg<txmaxp2::TXMAXP2_SPEC>,
    _reserved_98_txcsrl2: [u8; 0x01],
    #[doc = "0x123 - USB Transmit Control and Status Endpoint 2 High"]
    pub txcsrh2: crate::Reg<txcsrh2::TXCSRH2_SPEC>,
    #[doc = "0x124 - USB Maximum Receive Data Endpoint 2"]
    pub rxmaxp2: crate::Reg<rxmaxp2::RXMAXP2_SPEC>,
    _reserved_101_rxcsrl2: [u8; 0x01],
    _reserved_102_rxcsrh2: [u8; 0x01],
    #[doc = "0x128 - USB Receive Byte Count Endpoint 2"]
    pub rxcount2: crate::Reg<rxcount2::RXCOUNT2_SPEC>,
    #[doc = "0x12a - USB Host Transmit Configure Type Endpoint 2"]
    pub txtype2: crate::Reg<txtype2::TXTYPE2_SPEC>,
    _reserved_105_txinterval2: [u8; 0x01],
    #[doc = "0x12c - USB Host Configure Receive Type Endpoint 2"]
    pub rxtype2: crate::Reg<rxtype2::RXTYPE2_SPEC>,
    _reserved_107_rxinterval2: [u8; 0x01],
    _reserved108: [u8; 0x02],
    #[doc = "0x130 - USB Maximum Transmit Data Endpoint 3"]
    pub txmaxp3: crate::Reg<txmaxp3::TXMAXP3_SPEC>,
    _reserved_109_txcsrl3: [u8; 0x01],
    #[doc = "0x133 - USB Transmit Control and Status Endpoint 3 High"]
    pub txcsrh3: crate::Reg<txcsrh3::TXCSRH3_SPEC>,
    #[doc = "0x134 - USB Maximum Receive Data Endpoint 3"]
    pub rxmaxp3: crate::Reg<rxmaxp3::RXMAXP3_SPEC>,
    _reserved_112_rxcsrl3: [u8; 0x01],
    _reserved_113_rxcsrh3: [u8; 0x01],
    #[doc = "0x138 - USB Receive Byte Count Endpoint 3"]
    pub rxcount3: crate::Reg<rxcount3::RXCOUNT3_SPEC>,
    #[doc = "0x13a - USB Host Transmit Configure Type Endpoint 3"]
    pub txtype3: crate::Reg<txtype3::TXTYPE3_SPEC>,
    _reserved_116_txinterval3: [u8; 0x01],
    #[doc = "0x13c - USB Host Configure Receive Type Endpoint 3"]
    pub rxtype3: crate::Reg<rxtype3::RXTYPE3_SPEC>,
    _reserved_118_rxinterval3: [u8; 0x01],
    _reserved119: [u8; 0x02],
    #[doc = "0x140 - USB Maximum Transmit Data Endpoint 4"]
    pub txmaxp4: crate::Reg<txmaxp4::TXMAXP4_SPEC>,
    _reserved_120_txcsrl4: [u8; 0x01],
    #[doc = "0x143 - USB Transmit Control and Status Endpoint 4 High"]
    pub txcsrh4: crate::Reg<txcsrh4::TXCSRH4_SPEC>,
    #[doc = "0x144 - USB Maximum Receive Data Endpoint 4"]
    pub rxmaxp4: crate::Reg<rxmaxp4::RXMAXP4_SPEC>,
    _reserved_123_rxcsrl4: [u8; 0x01],
    _reserved_124_rxcsrh4: [u8; 0x01],
    #[doc = "0x148 - USB Receive Byte Count Endpoint 4"]
    pub rxcount4: crate::Reg<rxcount4::RXCOUNT4_SPEC>,
    #[doc = "0x14a - USB Host Transmit Configure Type Endpoint 4"]
    pub txtype4: crate::Reg<txtype4::TXTYPE4_SPEC>,
    _reserved_127_txinterval4: [u8; 0x01],
    #[doc = "0x14c - USB Host Configure Receive Type Endpoint 4"]
    pub rxtype4: crate::Reg<rxtype4::RXTYPE4_SPEC>,
    _reserved_129_rxinterval4: [u8; 0x01],
    _reserved130: [u8; 0x02],
    #[doc = "0x150 - USB Maximum Transmit Data Endpoint 5"]
    pub txmaxp5: crate::Reg<txmaxp5::TXMAXP5_SPEC>,
    _reserved_131_txcsrl5: [u8; 0x01],
    #[doc = "0x153 - USB Transmit Control and Status Endpoint 5 High"]
    pub txcsrh5: crate::Reg<txcsrh5::TXCSRH5_SPEC>,
    #[doc = "0x154 - USB Maximum Receive Data Endpoint 5"]
    pub rxmaxp5: crate::Reg<rxmaxp5::RXMAXP5_SPEC>,
    _reserved_134_rxcsrl5: [u8; 0x01],
    _reserved_135_rxcsrh5: [u8; 0x01],
    #[doc = "0x158 - USB Receive Byte Count Endpoint 5"]
    pub rxcount5: crate::Reg<rxcount5::RXCOUNT5_SPEC>,
    #[doc = "0x15a - USB Host Transmit Configure Type Endpoint 5"]
    pub txtype5: crate::Reg<txtype5::TXTYPE5_SPEC>,
    _reserved_138_txinterval5: [u8; 0x01],
    #[doc = "0x15c - USB Host Configure Receive Type Endpoint 5"]
    pub rxtype5: crate::Reg<rxtype5::RXTYPE5_SPEC>,
    _reserved_140_rxinterval5: [u8; 0x01],
    _reserved141: [u8; 0x02],
    #[doc = "0x160 - USB Maximum Transmit Data Endpoint 6"]
    pub txmaxp6: crate::Reg<txmaxp6::TXMAXP6_SPEC>,
    _reserved_142_txcsrl6: [u8; 0x01],
    #[doc = "0x163 - USB Transmit Control and Status Endpoint 6 High"]
    pub txcsrh6: crate::Reg<txcsrh6::TXCSRH6_SPEC>,
    #[doc = "0x164 - USB Maximum Receive Data Endpoint 6"]
    pub rxmaxp6: crate::Reg<rxmaxp6::RXMAXP6_SPEC>,
    _reserved_145_rxcsrl6: [u8; 0x01],
    _reserved_146_rxcsrh6: [u8; 0x01],
    #[doc = "0x168 - USB Receive Byte Count Endpoint 6"]
    pub rxcount6: crate::Reg<rxcount6::RXCOUNT6_SPEC>,
    #[doc = "0x16a - USB Host Transmit Configure Type Endpoint 6"]
    pub txtype6: crate::Reg<txtype6::TXTYPE6_SPEC>,
    _reserved_149_txinterval6: [u8; 0x01],
    #[doc = "0x16c - USB Host Configure Receive Type Endpoint 6"]
    pub rxtype6: crate::Reg<rxtype6::RXTYPE6_SPEC>,
    _reserved_151_rxinterval6: [u8; 0x01],
    _reserved152: [u8; 0x02],
    #[doc = "0x170 - USB Maximum Transmit Data Endpoint 7"]
    pub txmaxp7: crate::Reg<txmaxp7::TXMAXP7_SPEC>,
    _reserved_153_txcsrl7: [u8; 0x01],
    #[doc = "0x173 - USB Transmit Control and Status Endpoint 7 High"]
    pub txcsrh7: crate::Reg<txcsrh7::TXCSRH7_SPEC>,
    #[doc = "0x174 - USB Maximum Receive Data Endpoint 7"]
    pub rxmaxp7: crate::Reg<rxmaxp7::RXMAXP7_SPEC>,
    _reserved_156_rxcsrl7: [u8; 0x01],
    _reserved_157_rxcsrh7: [u8; 0x01],
    #[doc = "0x178 - USB Receive Byte Count Endpoint 7"]
    pub rxcount7: crate::Reg<rxcount7::RXCOUNT7_SPEC>,
    #[doc = "0x17a - USB Host Transmit Configure Type Endpoint 7"]
    pub txtype7: crate::Reg<txtype7::TXTYPE7_SPEC>,
    _reserved_160_txinterval7: [u8; 0x01],
    #[doc = "0x17c - USB Host Configure Receive Type Endpoint 7"]
    pub rxtype7: crate::Reg<rxtype7::RXTYPE7_SPEC>,
    _reserved_162_rxinterval7: [u8; 0x01],
    _reserved163: [u8; 0x82],
    #[doc = "0x200 - USB DMA Interrupt"]
    pub dmaintr: crate::Reg<dmaintr::DMAINTR_SPEC>,
    _reserved164: [u8; 0x03],
    #[doc = "0x204 - USB DMA Control 0"]
    pub dmactl0: crate::Reg<dmactl0::DMACTL0_SPEC>,
    _reserved165: [u8; 0x02],
    #[doc = "0x208 - USB DMA Address 0"]
    pub dmaaddr0: crate::Reg<dmaaddr0::DMAADDR0_SPEC>,
    #[doc = "0x20c - USB DMA Count 0"]
    pub dmacount0: crate::Reg<dmacount0::DMACOUNT0_SPEC>,
    _reserved167: [u8; 0x04],
    #[doc = "0x214 - USB DMA Control 1"]
    pub dmactl1: crate::Reg<dmactl1::DMACTL1_SPEC>,
    _reserved168: [u8; 0x02],
    #[doc = "0x218 - USB DMA Address 1"]
    pub dmaaddr1: crate::Reg<dmaaddr1::DMAADDR1_SPEC>,
    #[doc = "0x21c - USB DMA Count 1"]
    pub dmacount1: crate::Reg<dmacount1::DMACOUNT1_SPEC>,
    _reserved170: [u8; 0x04],
    #[doc = "0x224 - USB DMA Control 2"]
    pub dmactl2: crate::Reg<dmactl2::DMACTL2_SPEC>,
    _reserved171: [u8; 0x02],
    #[doc = "0x228 - USB DMA Address 2"]
    pub dmaaddr2: crate::Reg<dmaaddr2::DMAADDR2_SPEC>,
    #[doc = "0x22c - USB DMA Count 2"]
    pub dmacount2: crate::Reg<dmacount2::DMACOUNT2_SPEC>,
    _reserved173: [u8; 0x04],
    #[doc = "0x234 - USB DMA Control 3"]
    pub dmactl3: crate::Reg<dmactl3::DMACTL3_SPEC>,
    _reserved174: [u8; 0x02],
    #[doc = "0x238 - USB DMA Address 3"]
    pub dmaaddr3: crate::Reg<dmaaddr3::DMAADDR3_SPEC>,
    #[doc = "0x23c - USB DMA Count 3"]
    pub dmacount3: crate::Reg<dmacount3::DMACOUNT3_SPEC>,
    _reserved176: [u8; 0x04],
    #[doc = "0x244 - USB DMA Control 4"]
    pub dmactl4: crate::Reg<dmactl4::DMACTL4_SPEC>,
    _reserved177: [u8; 0x02],
    #[doc = "0x248 - USB DMA Address 4"]
    pub dmaaddr4: crate::Reg<dmaaddr4::DMAADDR4_SPEC>,
    #[doc = "0x24c - USB DMA Count 4"]
    pub dmacount4: crate::Reg<dmacount4::DMACOUNT4_SPEC>,
    _reserved179: [u8; 0x04],
    #[doc = "0x254 - USB DMA Control 5"]
    pub dmactl5: crate::Reg<dmactl5::DMACTL5_SPEC>,
    _reserved180: [u8; 0x02],
    #[doc = "0x258 - USB DMA Address 5"]
    pub dmaaddr5: crate::Reg<dmaaddr5::DMAADDR5_SPEC>,
    #[doc = "0x25c - USB DMA Count 5"]
    pub dmacount5: crate::Reg<dmacount5::DMACOUNT5_SPEC>,
    _reserved182: [u8; 0x04],
    #[doc = "0x264 - USB DMA Control 6"]
    pub dmactl6: crate::Reg<dmactl6::DMACTL6_SPEC>,
    _reserved183: [u8; 0x02],
    #[doc = "0x268 - USB DMA Address 6"]
    pub dmaaddr6: crate::Reg<dmaaddr6::DMAADDR6_SPEC>,
    #[doc = "0x26c - USB DMA Count 6"]
    pub dmacount6: crate::Reg<dmacount6::DMACOUNT6_SPEC>,
    _reserved185: [u8; 0x04],
    #[doc = "0x274 - USB DMA Control 7"]
    pub dmactl7: crate::Reg<dmactl7::DMACTL7_SPEC>,
    _reserved186: [u8; 0x02],
    #[doc = "0x278 - USB DMA Address 7"]
    pub dmaaddr7: crate::Reg<dmaaddr7::DMAADDR7_SPEC>,
    #[doc = "0x27c - USB DMA Count 7"]
    pub dmacount7: crate::Reg<dmacount7::DMACOUNT7_SPEC>,
    _reserved188: [u8; 0x84],
    #[doc = "0x304 - USB Request Packet Count in Block Transfer Endpoint 1"]
    pub rqpktcount1: crate::Reg<rqpktcount1::RQPKTCOUNT1_SPEC>,
    _reserved189: [u8; 0x02],
    #[doc = "0x308 - USB Request Packet Count in Block Transfer Endpoint 2"]
    pub rqpktcount2: crate::Reg<rqpktcount2::RQPKTCOUNT2_SPEC>,
    _reserved190: [u8; 0x02],
    #[doc = "0x30c - USB Request Packet Count in Block Transfer Endpoint 3"]
    pub rqpktcount3: crate::Reg<rqpktcount3::RQPKTCOUNT3_SPEC>,
    _reserved191: [u8; 0x02],
    #[doc = "0x310 - USB Request Packet Count in Block Transfer Endpoint 4"]
    pub rqpktcount4: crate::Reg<rqpktcount4::RQPKTCOUNT4_SPEC>,
    _reserved192: [u8; 0x02],
    #[doc = "0x314 - USB Request Packet Count in Block Transfer Endpoint 5"]
    pub rqpktcount5: crate::Reg<rqpktcount5::RQPKTCOUNT5_SPEC>,
    _reserved193: [u8; 0x02],
    #[doc = "0x318 - USB Request Packet Count in Block Transfer Endpoint 6"]
    pub rqpktcount6: crate::Reg<rqpktcount6::RQPKTCOUNT6_SPEC>,
    _reserved194: [u8; 0x02],
    #[doc = "0x31c - USB Request Packet Count in Block Transfer Endpoint 7"]
    pub rqpktcount7: crate::Reg<rqpktcount7::RQPKTCOUNT7_SPEC>,
    _reserved195: [u8; 0x22],
    #[doc = "0x340 - USB Receive Double Packet Buffer Disable"]
    pub rxdpktbufdis: crate::Reg<rxdpktbufdis::RXDPKTBUFDIS_SPEC>,
    #[doc = "0x342 - USB Transmit Double Packet Buffer Disable"]
    pub txdpktbufdis: crate::Reg<txdpktbufdis::TXDPKTBUFDIS_SPEC>,
    #[doc = "0x344 - USB Chirp Timeout"]
    pub cto: crate::Reg<cto::CTO_SPEC>,
    #[doc = "0x346 - USB High Speed to UTM Operating Delay"]
    pub hhsrtn: crate::Reg<hhsrtn::HHSRTN_SPEC>,
    #[doc = "0x348 - USB High Speed Time-out Adder"]
    pub hsbt: crate::Reg<hsbt::HSBT_SPEC>,
    _reserved200: [u8; 0x16],
    #[doc = "0x360 - USB LPM Attributes"]
    pub lpmattr: crate::Reg<lpmattr::LPMATTR_SPEC>,
    #[doc = "0x362 - USB LPM Control"]
    pub lpmcntrl: crate::Reg<lpmcntrl::LPMCNTRL_SPEC>,
    #[doc = "0x363 - USB LPM Interrupt Mask"]
    pub lpmim: crate::Reg<lpmim::LPMIM_SPEC>,
    #[doc = "0x364 - USB LPM Raw Interrupt Status"]
    pub lpmris: crate::Reg<lpmris::LPMRIS_SPEC>,
    #[doc = "0x365 - USB LPM Function Address"]
    pub lpmfaddr: crate::Reg<lpmfaddr::LPMFADDR_SPEC>,
    _reserved205: [u8; 0x9a],
    #[doc = "0x400 - USB External Power Control"]
    pub epc: crate::Reg<epc::EPC_SPEC>,
    #[doc = "0x404 - USB External Power Control Raw Interrupt Status"]
    pub epcris: crate::Reg<epcris::EPCRIS_SPEC>,
    #[doc = "0x408 - USB External Power Control Interrupt Mask"]
    pub epcim: crate::Reg<epcim::EPCIM_SPEC>,
    #[doc = "0x40c - USB External Power Control Interrupt Status and Clear"]
    pub epcisc: crate::Reg<epcisc::EPCISC_SPEC>,
    #[doc = "0x410 - USB Device RESUME Raw Interrupt Status"]
    pub drris: crate::Reg<drris::DRRIS_SPEC>,
    #[doc = "0x414 - USB Device RESUME Interrupt Mask"]
    pub drim: crate::Reg<drim::DRIM_SPEC>,
    #[doc = "0x418 - USB Device RESUME Interrupt Status and Clear"]
    pub drisc: crate::Reg<drisc::DRISC_SPEC>,
    #[doc = "0x41c - USB General-Purpose Control and Status"]
    pub gpcs: crate::Reg<gpcs::GPCS_SPEC>,
    _reserved213: [u8; 0x10],
    #[doc = "0x430 - USB VBUS Droop Control"]
    pub vdc: crate::Reg<vdc::VDC_SPEC>,
    #[doc = "0x434 - USB VBUS Droop Control Raw Interrupt Status"]
    pub vdcris: crate::Reg<vdcris::VDCRIS_SPEC>,
    #[doc = "0x438 - USB VBUS Droop Control Interrupt Mask"]
    pub vdcim: crate::Reg<vdcim::VDCIM_SPEC>,
    #[doc = "0x43c - USB VBUS Droop Control Interrupt Status and Clear"]
    pub vdcisc: crate::Reg<vdcisc::VDCISC_SPEC>,
    _reserved217: [u8; 0x0b80],
    #[doc = "0xfc0 - USB Peripheral Properties"]
    pub pp: crate::Reg<pp::PP_SPEC>,
    #[doc = "0xfc4 - USB Peripheral Configuration"]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0xfc8 - USB Clock Configuration"]
    pub cc: crate::Reg<cc::CC_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x0a - USB General Interrupt Status"]
    #[inline(always)]
    pub fn usb0_alt_is(&self) -> &crate::Reg<usb0_alt_is::USB0_ALT_IS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(10usize)
                as *const crate::Reg<usb0_alt_is::USB0_ALT_IS_SPEC>)
        }
    }
    #[doc = "0x0a - USB General Interrupt Status"]
    #[inline(always)]
    pub fn is(&self) -> &crate::Reg<is::IS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(10usize) as *const crate::Reg<is::IS_SPEC>)
        }
    }
    #[doc = "0x0b - USB Interrupt Enable"]
    #[inline(always)]
    pub fn usb0_alt_ie(&self) -> &crate::Reg<usb0_alt_ie::USB0_ALT_IE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(11usize)
                as *const crate::Reg<usb0_alt_ie::USB0_ALT_IE_SPEC>)
        }
    }
    #[doc = "0x0b - USB Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> &crate::Reg<ie::IE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(11usize) as *const crate::Reg<ie::IE_SPEC>)
        }
    }
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    #[inline(always)]
    pub fn usb0_alt_csrl0(&self) -> &crate::Reg<usb0_alt_csrl0::USB0_ALT_CSRL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(258usize)
                as *const crate::Reg<usb0_alt_csrl0::USB0_ALT_CSRL0_SPEC>)
        }
    }
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    #[inline(always)]
    pub fn csrl0(&self) -> &crate::Reg<csrl0::CSRL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(258usize)
                as *const crate::Reg<csrl0::CSRL0_SPEC>)
        }
    }
    #[doc = "0x112 - USB Transmit Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl1(&self) -> &crate::Reg<usb0_alt_txcsrl1::USB0_ALT_TXCSRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(274usize)
                as *const crate::Reg<usb0_alt_txcsrl1::USB0_ALT_TXCSRL1_SPEC>)
        }
    }
    #[doc = "0x112 - USB Transmit Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub fn txcsrl1(&self) -> &crate::Reg<txcsrl1::TXCSRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(274usize)
                as *const crate::Reg<txcsrl1::TXCSRL1_SPEC>)
        }
    }
    #[doc = "0x116 - USB Receive Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl1(&self) -> &crate::Reg<usb0_alt_rxcsrl1::USB0_ALT_RXCSRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(278usize)
                as *const crate::Reg<usb0_alt_rxcsrl1::USB0_ALT_RXCSRL1_SPEC>)
        }
    }
    #[doc = "0x116 - USB Receive Control and Status Endpoint 1 Low"]
    #[inline(always)]
    pub fn rxcsrl1(&self) -> &crate::Reg<rxcsrl1::RXCSRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(278usize)
                as *const crate::Reg<rxcsrl1::RXCSRL1_SPEC>)
        }
    }
    #[doc = "0x117 - USB Receive Control and Status Endpoint 1 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh1(&self) -> &crate::Reg<usb0_alt_rxcsrh1::USB0_ALT_RXCSRH1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(279usize)
                as *const crate::Reg<usb0_alt_rxcsrh1::USB0_ALT_RXCSRH1_SPEC>)
        }
    }
    #[doc = "0x117 - USB Receive Control and Status Endpoint 1 High"]
    #[inline(always)]
    pub fn rxcsrh1(&self) -> &crate::Reg<rxcsrh1::RXCSRH1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(279usize)
                as *const crate::Reg<rxcsrh1::RXCSRH1_SPEC>)
        }
    }
    #[doc = "0x11b - USB Host Transmit Interval Endpoint 1"]
    #[inline(always)]
    pub fn usb0_alt_txinterval1(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval1::USB0_ALT_TXINTERVAL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(283usize)
                as *const crate::Reg<usb0_alt_txinterval1::USB0_ALT_TXINTERVAL1_SPEC>)
        }
    }
    #[doc = "0x11b - USB Host Transmit Interval Endpoint 1"]
    #[inline(always)]
    pub fn txinterval1(&self) -> &crate::Reg<txinterval1::TXINTERVAL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(283usize)
                as *const crate::Reg<txinterval1::TXINTERVAL1_SPEC>)
        }
    }
    #[doc = "0x11d - USB Host Receive Polling Interval Endpoint 1"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval1(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval1::USB0_ALT_RXINTERVAL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(285usize)
                as *const crate::Reg<usb0_alt_rxinterval1::USB0_ALT_RXINTERVAL1_SPEC>)
        }
    }
    #[doc = "0x11d - USB Host Receive Polling Interval Endpoint 1"]
    #[inline(always)]
    pub fn rxinterval1(&self) -> &crate::Reg<rxinterval1::RXINTERVAL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(285usize)
                as *const crate::Reg<rxinterval1::RXINTERVAL1_SPEC>)
        }
    }
    #[doc = "0x122 - USB Transmit Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl2(&self) -> &crate::Reg<usb0_alt_txcsrl2::USB0_ALT_TXCSRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(290usize)
                as *const crate::Reg<usb0_alt_txcsrl2::USB0_ALT_TXCSRL2_SPEC>)
        }
    }
    #[doc = "0x122 - USB Transmit Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub fn txcsrl2(&self) -> &crate::Reg<txcsrl2::TXCSRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(290usize)
                as *const crate::Reg<txcsrl2::TXCSRL2_SPEC>)
        }
    }
    #[doc = "0x126 - USB Receive Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl2(&self) -> &crate::Reg<usb0_alt_rxcsrl2::USB0_ALT_RXCSRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(294usize)
                as *const crate::Reg<usb0_alt_rxcsrl2::USB0_ALT_RXCSRL2_SPEC>)
        }
    }
    #[doc = "0x126 - USB Receive Control and Status Endpoint 2 Low"]
    #[inline(always)]
    pub fn rxcsrl2(&self) -> &crate::Reg<rxcsrl2::RXCSRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(294usize)
                as *const crate::Reg<rxcsrl2::RXCSRL2_SPEC>)
        }
    }
    #[doc = "0x127 - USB Receive Control and Status Endpoint 2 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh2(&self) -> &crate::Reg<usb0_alt_rxcsrh2::USB0_ALT_RXCSRH2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(295usize)
                as *const crate::Reg<usb0_alt_rxcsrh2::USB0_ALT_RXCSRH2_SPEC>)
        }
    }
    #[doc = "0x127 - USB Receive Control and Status Endpoint 2 High"]
    #[inline(always)]
    pub fn rxcsrh2(&self) -> &crate::Reg<rxcsrh2::RXCSRH2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(295usize)
                as *const crate::Reg<rxcsrh2::RXCSRH2_SPEC>)
        }
    }
    #[doc = "0x12b - USB Host Transmit Interval Endpoint 2"]
    #[inline(always)]
    pub fn usb0_alt_txinterval2(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval2::USB0_ALT_TXINTERVAL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(299usize)
                as *const crate::Reg<usb0_alt_txinterval2::USB0_ALT_TXINTERVAL2_SPEC>)
        }
    }
    #[doc = "0x12b - USB Host Transmit Interval Endpoint 2"]
    #[inline(always)]
    pub fn txinterval2(&self) -> &crate::Reg<txinterval2::TXINTERVAL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(299usize)
                as *const crate::Reg<txinterval2::TXINTERVAL2_SPEC>)
        }
    }
    #[doc = "0x12d - USB Host Receive Polling Interval Endpoint 2"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval2(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval2::USB0_ALT_RXINTERVAL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(301usize)
                as *const crate::Reg<usb0_alt_rxinterval2::USB0_ALT_RXINTERVAL2_SPEC>)
        }
    }
    #[doc = "0x12d - USB Host Receive Polling Interval Endpoint 2"]
    #[inline(always)]
    pub fn rxinterval2(&self) -> &crate::Reg<rxinterval2::RXINTERVAL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(301usize)
                as *const crate::Reg<rxinterval2::RXINTERVAL2_SPEC>)
        }
    }
    #[doc = "0x132 - USB Transmit Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl3(&self) -> &crate::Reg<usb0_alt_txcsrl3::USB0_ALT_TXCSRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(306usize)
                as *const crate::Reg<usb0_alt_txcsrl3::USB0_ALT_TXCSRL3_SPEC>)
        }
    }
    #[doc = "0x132 - USB Transmit Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub fn txcsrl3(&self) -> &crate::Reg<txcsrl3::TXCSRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(306usize)
                as *const crate::Reg<txcsrl3::TXCSRL3_SPEC>)
        }
    }
    #[doc = "0x136 - USB Receive Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl3(&self) -> &crate::Reg<usb0_alt_rxcsrl3::USB0_ALT_RXCSRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(310usize)
                as *const crate::Reg<usb0_alt_rxcsrl3::USB0_ALT_RXCSRL3_SPEC>)
        }
    }
    #[doc = "0x136 - USB Receive Control and Status Endpoint 3 Low"]
    #[inline(always)]
    pub fn rxcsrl3(&self) -> &crate::Reg<rxcsrl3::RXCSRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(310usize)
                as *const crate::Reg<rxcsrl3::RXCSRL3_SPEC>)
        }
    }
    #[doc = "0x137 - USB Receive Control and Status Endpoint 3 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh3(&self) -> &crate::Reg<usb0_alt_rxcsrh3::USB0_ALT_RXCSRH3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(311usize)
                as *const crate::Reg<usb0_alt_rxcsrh3::USB0_ALT_RXCSRH3_SPEC>)
        }
    }
    #[doc = "0x137 - USB Receive Control and Status Endpoint 3 High"]
    #[inline(always)]
    pub fn rxcsrh3(&self) -> &crate::Reg<rxcsrh3::RXCSRH3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(311usize)
                as *const crate::Reg<rxcsrh3::RXCSRH3_SPEC>)
        }
    }
    #[doc = "0x13b - USB Host Transmit Interval Endpoint 3"]
    #[inline(always)]
    pub fn usb0_alt_txinterval3(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval3::USB0_ALT_TXINTERVAL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(315usize)
                as *const crate::Reg<usb0_alt_txinterval3::USB0_ALT_TXINTERVAL3_SPEC>)
        }
    }
    #[doc = "0x13b - USB Host Transmit Interval Endpoint 3"]
    #[inline(always)]
    pub fn txinterval3(&self) -> &crate::Reg<txinterval3::TXINTERVAL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(315usize)
                as *const crate::Reg<txinterval3::TXINTERVAL3_SPEC>)
        }
    }
    #[doc = "0x13d - USB Host Receive Polling Interval Endpoint 3"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval3(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval3::USB0_ALT_RXINTERVAL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(317usize)
                as *const crate::Reg<usb0_alt_rxinterval3::USB0_ALT_RXINTERVAL3_SPEC>)
        }
    }
    #[doc = "0x13d - USB Host Receive Polling Interval Endpoint 3"]
    #[inline(always)]
    pub fn rxinterval3(&self) -> &crate::Reg<rxinterval3::RXINTERVAL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(317usize)
                as *const crate::Reg<rxinterval3::RXINTERVAL3_SPEC>)
        }
    }
    #[doc = "0x142 - USB Transmit Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl4(&self) -> &crate::Reg<usb0_alt_txcsrl4::USB0_ALT_TXCSRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(322usize)
                as *const crate::Reg<usb0_alt_txcsrl4::USB0_ALT_TXCSRL4_SPEC>)
        }
    }
    #[doc = "0x142 - USB Transmit Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub fn txcsrl4(&self) -> &crate::Reg<txcsrl4::TXCSRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(322usize)
                as *const crate::Reg<txcsrl4::TXCSRL4_SPEC>)
        }
    }
    #[doc = "0x146 - USB Receive Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl4(&self) -> &crate::Reg<usb0_alt_rxcsrl4::USB0_ALT_RXCSRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(326usize)
                as *const crate::Reg<usb0_alt_rxcsrl4::USB0_ALT_RXCSRL4_SPEC>)
        }
    }
    #[doc = "0x146 - USB Receive Control and Status Endpoint 4 Low"]
    #[inline(always)]
    pub fn rxcsrl4(&self) -> &crate::Reg<rxcsrl4::RXCSRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(326usize)
                as *const crate::Reg<rxcsrl4::RXCSRL4_SPEC>)
        }
    }
    #[doc = "0x147 - USB Receive Control and Status Endpoint 4 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh4(&self) -> &crate::Reg<usb0_alt_rxcsrh4::USB0_ALT_RXCSRH4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(327usize)
                as *const crate::Reg<usb0_alt_rxcsrh4::USB0_ALT_RXCSRH4_SPEC>)
        }
    }
    #[doc = "0x147 - USB Receive Control and Status Endpoint 4 High"]
    #[inline(always)]
    pub fn rxcsrh4(&self) -> &crate::Reg<rxcsrh4::RXCSRH4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(327usize)
                as *const crate::Reg<rxcsrh4::RXCSRH4_SPEC>)
        }
    }
    #[doc = "0x14b - USB Host Transmit Interval Endpoint 4"]
    #[inline(always)]
    pub fn usb0_alt_txinterval4(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval4::USB0_ALT_TXINTERVAL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(331usize)
                as *const crate::Reg<usb0_alt_txinterval4::USB0_ALT_TXINTERVAL4_SPEC>)
        }
    }
    #[doc = "0x14b - USB Host Transmit Interval Endpoint 4"]
    #[inline(always)]
    pub fn txinterval4(&self) -> &crate::Reg<txinterval4::TXINTERVAL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(331usize)
                as *const crate::Reg<txinterval4::TXINTERVAL4_SPEC>)
        }
    }
    #[doc = "0x14d - USB Host Receive Polling Interval Endpoint 4"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval4(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval4::USB0_ALT_RXINTERVAL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(333usize)
                as *const crate::Reg<usb0_alt_rxinterval4::USB0_ALT_RXINTERVAL4_SPEC>)
        }
    }
    #[doc = "0x14d - USB Host Receive Polling Interval Endpoint 4"]
    #[inline(always)]
    pub fn rxinterval4(&self) -> &crate::Reg<rxinterval4::RXINTERVAL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(333usize)
                as *const crate::Reg<rxinterval4::RXINTERVAL4_SPEC>)
        }
    }
    #[doc = "0x152 - USB Transmit Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl5(&self) -> &crate::Reg<usb0_alt_txcsrl5::USB0_ALT_TXCSRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(338usize)
                as *const crate::Reg<usb0_alt_txcsrl5::USB0_ALT_TXCSRL5_SPEC>)
        }
    }
    #[doc = "0x152 - USB Transmit Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub fn txcsrl5(&self) -> &crate::Reg<txcsrl5::TXCSRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(338usize)
                as *const crate::Reg<txcsrl5::TXCSRL5_SPEC>)
        }
    }
    #[doc = "0x156 - USB Receive Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl5(&self) -> &crate::Reg<usb0_alt_rxcsrl5::USB0_ALT_RXCSRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(342usize)
                as *const crate::Reg<usb0_alt_rxcsrl5::USB0_ALT_RXCSRL5_SPEC>)
        }
    }
    #[doc = "0x156 - USB Receive Control and Status Endpoint 5 Low"]
    #[inline(always)]
    pub fn rxcsrl5(&self) -> &crate::Reg<rxcsrl5::RXCSRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(342usize)
                as *const crate::Reg<rxcsrl5::RXCSRL5_SPEC>)
        }
    }
    #[doc = "0x157 - USB Receive Control and Status Endpoint 5 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh5(&self) -> &crate::Reg<usb0_alt_rxcsrh5::USB0_ALT_RXCSRH5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(343usize)
                as *const crate::Reg<usb0_alt_rxcsrh5::USB0_ALT_RXCSRH5_SPEC>)
        }
    }
    #[doc = "0x157 - USB Receive Control and Status Endpoint 5 High"]
    #[inline(always)]
    pub fn rxcsrh5(&self) -> &crate::Reg<rxcsrh5::RXCSRH5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(343usize)
                as *const crate::Reg<rxcsrh5::RXCSRH5_SPEC>)
        }
    }
    #[doc = "0x15b - USB Host Transmit Interval Endpoint 5"]
    #[inline(always)]
    pub fn usb0_alt_txinterval5(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval5::USB0_ALT_TXINTERVAL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(347usize)
                as *const crate::Reg<usb0_alt_txinterval5::USB0_ALT_TXINTERVAL5_SPEC>)
        }
    }
    #[doc = "0x15b - USB Host Transmit Interval Endpoint 5"]
    #[inline(always)]
    pub fn txinterval5(&self) -> &crate::Reg<txinterval5::TXINTERVAL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(347usize)
                as *const crate::Reg<txinterval5::TXINTERVAL5_SPEC>)
        }
    }
    #[doc = "0x15d - USB Host Receive Polling Interval Endpoint 5"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval5(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval5::USB0_ALT_RXINTERVAL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(349usize)
                as *const crate::Reg<usb0_alt_rxinterval5::USB0_ALT_RXINTERVAL5_SPEC>)
        }
    }
    #[doc = "0x15d - USB Host Receive Polling Interval Endpoint 5"]
    #[inline(always)]
    pub fn rxinterval5(&self) -> &crate::Reg<rxinterval5::RXINTERVAL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(349usize)
                as *const crate::Reg<rxinterval5::RXINTERVAL5_SPEC>)
        }
    }
    #[doc = "0x162 - USB Transmit Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl6(&self) -> &crate::Reg<usb0_alt_txcsrl6::USB0_ALT_TXCSRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(354usize)
                as *const crate::Reg<usb0_alt_txcsrl6::USB0_ALT_TXCSRL6_SPEC>)
        }
    }
    #[doc = "0x162 - USB Transmit Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub fn txcsrl6(&self) -> &crate::Reg<txcsrl6::TXCSRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(354usize)
                as *const crate::Reg<txcsrl6::TXCSRL6_SPEC>)
        }
    }
    #[doc = "0x166 - USB Receive Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl6(&self) -> &crate::Reg<usb0_alt_rxcsrl6::USB0_ALT_RXCSRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(358usize)
                as *const crate::Reg<usb0_alt_rxcsrl6::USB0_ALT_RXCSRL6_SPEC>)
        }
    }
    #[doc = "0x166 - USB Receive Control and Status Endpoint 6 Low"]
    #[inline(always)]
    pub fn rxcsrl6(&self) -> &crate::Reg<rxcsrl6::RXCSRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(358usize)
                as *const crate::Reg<rxcsrl6::RXCSRL6_SPEC>)
        }
    }
    #[doc = "0x167 - USB Receive Control and Status Endpoint 6 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh6(&self) -> &crate::Reg<usb0_alt_rxcsrh6::USB0_ALT_RXCSRH6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(359usize)
                as *const crate::Reg<usb0_alt_rxcsrh6::USB0_ALT_RXCSRH6_SPEC>)
        }
    }
    #[doc = "0x167 - USB Receive Control and Status Endpoint 6 High"]
    #[inline(always)]
    pub fn rxcsrh6(&self) -> &crate::Reg<rxcsrh6::RXCSRH6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(359usize)
                as *const crate::Reg<rxcsrh6::RXCSRH6_SPEC>)
        }
    }
    #[doc = "0x16b - USB Host Transmit Interval Endpoint 6"]
    #[inline(always)]
    pub fn usb0_alt_txinterval6(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval6::USB0_ALT_TXINTERVAL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(363usize)
                as *const crate::Reg<usb0_alt_txinterval6::USB0_ALT_TXINTERVAL6_SPEC>)
        }
    }
    #[doc = "0x16b - USB Host Transmit Interval Endpoint 6"]
    #[inline(always)]
    pub fn txinterval6(&self) -> &crate::Reg<txinterval6::TXINTERVAL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(363usize)
                as *const crate::Reg<txinterval6::TXINTERVAL6_SPEC>)
        }
    }
    #[doc = "0x16d - USB Host Receive Polling Interval Endpoint 6"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval6(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval6::USB0_ALT_RXINTERVAL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(365usize)
                as *const crate::Reg<usb0_alt_rxinterval6::USB0_ALT_RXINTERVAL6_SPEC>)
        }
    }
    #[doc = "0x16d - USB Host Receive Polling Interval Endpoint 6"]
    #[inline(always)]
    pub fn rxinterval6(&self) -> &crate::Reg<rxinterval6::RXINTERVAL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(365usize)
                as *const crate::Reg<rxinterval6::RXINTERVAL6_SPEC>)
        }
    }
    #[doc = "0x172 - USB Transmit Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub fn usb0_alt_txcsrl7(&self) -> &crate::Reg<usb0_alt_txcsrl7::USB0_ALT_TXCSRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(370usize)
                as *const crate::Reg<usb0_alt_txcsrl7::USB0_ALT_TXCSRL7_SPEC>)
        }
    }
    #[doc = "0x172 - USB Transmit Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub fn txcsrl7(&self) -> &crate::Reg<txcsrl7::TXCSRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(370usize)
                as *const crate::Reg<txcsrl7::TXCSRL7_SPEC>)
        }
    }
    #[doc = "0x176 - USB Receive Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrl7(&self) -> &crate::Reg<usb0_alt_rxcsrl7::USB0_ALT_RXCSRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(374usize)
                as *const crate::Reg<usb0_alt_rxcsrl7::USB0_ALT_RXCSRL7_SPEC>)
        }
    }
    #[doc = "0x176 - USB Receive Control and Status Endpoint 7 Low"]
    #[inline(always)]
    pub fn rxcsrl7(&self) -> &crate::Reg<rxcsrl7::RXCSRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(374usize)
                as *const crate::Reg<rxcsrl7::RXCSRL7_SPEC>)
        }
    }
    #[doc = "0x177 - USB Receive Control and Status Endpoint 7 High"]
    #[inline(always)]
    pub fn usb0_alt_rxcsrh7(&self) -> &crate::Reg<usb0_alt_rxcsrh7::USB0_ALT_RXCSRH7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(375usize)
                as *const crate::Reg<usb0_alt_rxcsrh7::USB0_ALT_RXCSRH7_SPEC>)
        }
    }
    #[doc = "0x177 - USB Receive Control and Status Endpoint 7 High"]
    #[inline(always)]
    pub fn rxcsrh7(&self) -> &crate::Reg<rxcsrh7::RXCSRH7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(375usize)
                as *const crate::Reg<rxcsrh7::RXCSRH7_SPEC>)
        }
    }
    #[doc = "0x17b - USB Host Transmit Interval Endpoint 7"]
    #[inline(always)]
    pub fn usb0_alt_txinterval7(
        &self,
    ) -> &crate::Reg<usb0_alt_txinterval7::USB0_ALT_TXINTERVAL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(379usize)
                as *const crate::Reg<usb0_alt_txinterval7::USB0_ALT_TXINTERVAL7_SPEC>)
        }
    }
    #[doc = "0x17b - USB Host Transmit Interval Endpoint 7"]
    #[inline(always)]
    pub fn txinterval7(&self) -> &crate::Reg<txinterval7::TXINTERVAL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(379usize)
                as *const crate::Reg<txinterval7::TXINTERVAL7_SPEC>)
        }
    }
    #[doc = "0x17d - USB Host Receive Polling Interval Endpoint 7"]
    #[inline(always)]
    pub fn usb0_alt_rxinterval7(
        &self,
    ) -> &crate::Reg<usb0_alt_rxinterval7::USB0_ALT_RXINTERVAL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(381usize)
                as *const crate::Reg<usb0_alt_rxinterval7::USB0_ALT_RXINTERVAL7_SPEC>)
        }
    }
    #[doc = "0x17d - USB Host Receive Polling Interval Endpoint 7"]
    #[inline(always)]
    pub fn rxinterval7(&self) -> &crate::Reg<rxinterval7::RXINTERVAL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(381usize)
                as *const crate::Reg<rxinterval7::RXINTERVAL7_SPEC>)
        }
    }
}
#[doc = "FADDR register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "USB Device Functional Address"]
pub mod faddr;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "USB Power"]
pub mod power;
#[doc = "TXIS register accessor: an alias for `Reg<TXIS_SPEC>`"]
pub type TXIS = crate::Reg<txis::TXIS_SPEC>;
#[doc = "USB Transmit Interrupt Status"]
pub mod txis;
#[doc = "RXIS register accessor: an alias for `Reg<RXIS_SPEC>`"]
pub type RXIS = crate::Reg<rxis::RXIS_SPEC>;
#[doc = "USB Receive Interrupt Status"]
pub mod rxis;
#[doc = "TXIE register accessor: an alias for `Reg<TXIE_SPEC>`"]
pub type TXIE = crate::Reg<txie::TXIE_SPEC>;
#[doc = "USB Transmit Interrupt Enable"]
pub mod txie;
#[doc = "RXIE register accessor: an alias for `Reg<RXIE_SPEC>`"]
pub type RXIE = crate::Reg<rxie::RXIE_SPEC>;
#[doc = "USB Receive Interrupt Enable"]
pub mod rxie;
#[doc = "IS register accessor: an alias for `Reg<IS_SPEC>`"]
pub type IS = crate::Reg<is::IS_SPEC>;
#[doc = "USB General Interrupt Status"]
pub mod is;
#[doc = "USB0_ALT_IS register accessor: an alias for `Reg<USB0_ALT_IS_SPEC>`"]
pub type USB0_ALT_IS = crate::Reg<usb0_alt_is::USB0_ALT_IS_SPEC>;
#[doc = "USB General Interrupt Status"]
pub mod usb0_alt_is;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "USB Interrupt Enable"]
pub mod ie;
#[doc = "USB0_ALT_IE register accessor: an alias for `Reg<USB0_ALT_IE_SPEC>`"]
pub type USB0_ALT_IE = crate::Reg<usb0_alt_ie::USB0_ALT_IE_SPEC>;
#[doc = "USB Interrupt Enable"]
pub mod usb0_alt_ie;
#[doc = "FRAME register accessor: an alias for `Reg<FRAME_SPEC>`"]
pub type FRAME = crate::Reg<frame::FRAME_SPEC>;
#[doc = "USB Frame Value"]
pub mod frame;
#[doc = "EPIDX register accessor: an alias for `Reg<EPIDX_SPEC>`"]
pub type EPIDX = crate::Reg<epidx::EPIDX_SPEC>;
#[doc = "USB Endpoint Index"]
pub mod epidx;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "USB Test Mode"]
pub mod test;
#[doc = "FIFO0 register accessor: an alias for `Reg<FIFO0_SPEC>`"]
pub type FIFO0 = crate::Reg<fifo0::FIFO0_SPEC>;
#[doc = "USB FIFO Endpoint 0"]
pub mod fifo0;
#[doc = "FIFO1 register accessor: an alias for `Reg<FIFO1_SPEC>`"]
pub type FIFO1 = crate::Reg<fifo1::FIFO1_SPEC>;
#[doc = "USB FIFO Endpoint 1"]
pub mod fifo1;
#[doc = "FIFO2 register accessor: an alias for `Reg<FIFO2_SPEC>`"]
pub type FIFO2 = crate::Reg<fifo2::FIFO2_SPEC>;
#[doc = "USB FIFO Endpoint 2"]
pub mod fifo2;
#[doc = "FIFO3 register accessor: an alias for `Reg<FIFO3_SPEC>`"]
pub type FIFO3 = crate::Reg<fifo3::FIFO3_SPEC>;
#[doc = "USB FIFO Endpoint 3"]
pub mod fifo3;
#[doc = "FIFO4 register accessor: an alias for `Reg<FIFO4_SPEC>`"]
pub type FIFO4 = crate::Reg<fifo4::FIFO4_SPEC>;
#[doc = "USB FIFO Endpoint 4"]
pub mod fifo4;
#[doc = "FIFO5 register accessor: an alias for `Reg<FIFO5_SPEC>`"]
pub type FIFO5 = crate::Reg<fifo5::FIFO5_SPEC>;
#[doc = "USB FIFO Endpoint 5"]
pub mod fifo5;
#[doc = "FIFO6 register accessor: an alias for `Reg<FIFO6_SPEC>`"]
pub type FIFO6 = crate::Reg<fifo6::FIFO6_SPEC>;
#[doc = "USB FIFO Endpoint 6"]
pub mod fifo6;
#[doc = "FIFO7 register accessor: an alias for `Reg<FIFO7_SPEC>`"]
pub type FIFO7 = crate::Reg<fifo7::FIFO7_SPEC>;
#[doc = "USB FIFO Endpoint 7"]
pub mod fifo7;
#[doc = "DEVCTL register accessor: an alias for `Reg<DEVCTL_SPEC>`"]
pub type DEVCTL = crate::Reg<devctl::DEVCTL_SPEC>;
#[doc = "USB Device Control"]
pub mod devctl;
#[doc = "CCONF register accessor: an alias for `Reg<CCONF_SPEC>`"]
pub type CCONF = crate::Reg<cconf::CCONF_SPEC>;
#[doc = "USB Common Configuration"]
pub mod cconf;
#[doc = "TXFIFOSZ register accessor: an alias for `Reg<TXFIFOSZ_SPEC>`"]
pub type TXFIFOSZ = crate::Reg<txfifosz::TXFIFOSZ_SPEC>;
#[doc = "USB Transmit Dynamic FIFO Sizing"]
pub mod txfifosz;
#[doc = "RXFIFOSZ register accessor: an alias for `Reg<RXFIFOSZ_SPEC>`"]
pub type RXFIFOSZ = crate::Reg<rxfifosz::RXFIFOSZ_SPEC>;
#[doc = "USB Receive Dynamic FIFO Sizing"]
pub mod rxfifosz;
#[doc = "TXFIFOADD register accessor: an alias for `Reg<TXFIFOADD_SPEC>`"]
pub type TXFIFOADD = crate::Reg<txfifoadd::TXFIFOADD_SPEC>;
#[doc = "USB Transmit FIFO Start Address"]
pub mod txfifoadd;
#[doc = "RXFIFOADD register accessor: an alias for `Reg<RXFIFOADD_SPEC>`"]
pub type RXFIFOADD = crate::Reg<rxfifoadd::RXFIFOADD_SPEC>;
#[doc = "USB Receive FIFO Start Address"]
pub mod rxfifoadd;
#[doc = "ULPIVBUSCTL register accessor: an alias for `Reg<ULPIVBUSCTL_SPEC>`"]
pub type ULPIVBUSCTL = crate::Reg<ulpivbusctl::ULPIVBUSCTL_SPEC>;
#[doc = "USB ULPI VBUS Control"]
pub mod ulpivbusctl;
#[doc = "ULPIREGDATA register accessor: an alias for `Reg<ULPIREGDATA_SPEC>`"]
pub type ULPIREGDATA = crate::Reg<ulpiregdata::ULPIREGDATA_SPEC>;
#[doc = "USB ULPI Register Data"]
pub mod ulpiregdata;
#[doc = "ULPIREGADDR register accessor: an alias for `Reg<ULPIREGADDR_SPEC>`"]
pub type ULPIREGADDR = crate::Reg<ulpiregaddr::ULPIREGADDR_SPEC>;
#[doc = "USB ULPI Register Address"]
pub mod ulpiregaddr;
#[doc = "ULPIREGCTL register accessor: an alias for `Reg<ULPIREGCTL_SPEC>`"]
pub type ULPIREGCTL = crate::Reg<ulpiregctl::ULPIREGCTL_SPEC>;
#[doc = "USB ULPI Register Control"]
pub mod ulpiregctl;
#[doc = "EPINFO register accessor: an alias for `Reg<EPINFO_SPEC>`"]
pub type EPINFO = crate::Reg<epinfo::EPINFO_SPEC>;
#[doc = "USB Endpoint Information"]
pub mod epinfo;
#[doc = "RAMINFO register accessor: an alias for `Reg<RAMINFO_SPEC>`"]
pub type RAMINFO = crate::Reg<raminfo::RAMINFO_SPEC>;
#[doc = "USB RAM Information"]
pub mod raminfo;
#[doc = "CONTIM register accessor: an alias for `Reg<CONTIM_SPEC>`"]
pub type CONTIM = crate::Reg<contim::CONTIM_SPEC>;
#[doc = "USB Connect Timing"]
pub mod contim;
#[doc = "VPLEN register accessor: an alias for `Reg<VPLEN_SPEC>`"]
pub type VPLEN = crate::Reg<vplen::VPLEN_SPEC>;
#[doc = "USB OTG VBUS Pulse Timing"]
pub mod vplen;
#[doc = "HSEOF register accessor: an alias for `Reg<HSEOF_SPEC>`"]
pub type HSEOF = crate::Reg<hseof::HSEOF_SPEC>;
#[doc = "USB High-Speed Last Transaction to End of Frame Timing"]
pub mod hseof;
#[doc = "FSEOF register accessor: an alias for `Reg<FSEOF_SPEC>`"]
pub type FSEOF = crate::Reg<fseof::FSEOF_SPEC>;
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing"]
pub mod fseof;
#[doc = "LSEOF register accessor: an alias for `Reg<LSEOF_SPEC>`"]
pub type LSEOF = crate::Reg<lseof::LSEOF_SPEC>;
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing"]
pub mod lseof;
#[doc = "TXFUNCADDR0 register accessor: an alias for `Reg<TXFUNCADDR0_SPEC>`"]
pub type TXFUNCADDR0 = crate::Reg<txfuncaddr0::TXFUNCADDR0_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 0"]
pub mod txfuncaddr0;
#[doc = "TXHUBADDR0 register accessor: an alias for `Reg<TXHUBADDR0_SPEC>`"]
pub type TXHUBADDR0 = crate::Reg<txhubaddr0::TXHUBADDR0_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 0"]
pub mod txhubaddr0;
#[doc = "TXHUBPORT0 register accessor: an alias for `Reg<TXHUBPORT0_SPEC>`"]
pub type TXHUBPORT0 = crate::Reg<txhubport0::TXHUBPORT0_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 0"]
pub mod txhubport0;
#[doc = "TXFUNCADDR1 register accessor: an alias for `Reg<TXFUNCADDR1_SPEC>`"]
pub type TXFUNCADDR1 = crate::Reg<txfuncaddr1::TXFUNCADDR1_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 1"]
pub mod txfuncaddr1;
#[doc = "TXHUBADDR1 register accessor: an alias for `Reg<TXHUBADDR1_SPEC>`"]
pub type TXHUBADDR1 = crate::Reg<txhubaddr1::TXHUBADDR1_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 1"]
pub mod txhubaddr1;
#[doc = "TXHUBPORT1 register accessor: an alias for `Reg<TXHUBPORT1_SPEC>`"]
pub type TXHUBPORT1 = crate::Reg<txhubport1::TXHUBPORT1_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 1"]
pub mod txhubport1;
#[doc = "RXFUNCADDR1 register accessor: an alias for `Reg<RXFUNCADDR1_SPEC>`"]
pub type RXFUNCADDR1 = crate::Reg<rxfuncaddr1::RXFUNCADDR1_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 1"]
pub mod rxfuncaddr1;
#[doc = "RXHUBADDR1 register accessor: an alias for `Reg<RXHUBADDR1_SPEC>`"]
pub type RXHUBADDR1 = crate::Reg<rxhubaddr1::RXHUBADDR1_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 1"]
pub mod rxhubaddr1;
#[doc = "RXHUBPORT1 register accessor: an alias for `Reg<RXHUBPORT1_SPEC>`"]
pub type RXHUBPORT1 = crate::Reg<rxhubport1::RXHUBPORT1_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 1"]
pub mod rxhubport1;
#[doc = "TXFUNCADDR2 register accessor: an alias for `Reg<TXFUNCADDR2_SPEC>`"]
pub type TXFUNCADDR2 = crate::Reg<txfuncaddr2::TXFUNCADDR2_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 2"]
pub mod txfuncaddr2;
#[doc = "TXHUBADDR2 register accessor: an alias for `Reg<TXHUBADDR2_SPEC>`"]
pub type TXHUBADDR2 = crate::Reg<txhubaddr2::TXHUBADDR2_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 2"]
pub mod txhubaddr2;
#[doc = "TXHUBPORT2 register accessor: an alias for `Reg<TXHUBPORT2_SPEC>`"]
pub type TXHUBPORT2 = crate::Reg<txhubport2::TXHUBPORT2_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 2"]
pub mod txhubport2;
#[doc = "RXFUNCADDR2 register accessor: an alias for `Reg<RXFUNCADDR2_SPEC>`"]
pub type RXFUNCADDR2 = crate::Reg<rxfuncaddr2::RXFUNCADDR2_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 2"]
pub mod rxfuncaddr2;
#[doc = "RXHUBADDR2 register accessor: an alias for `Reg<RXHUBADDR2_SPEC>`"]
pub type RXHUBADDR2 = crate::Reg<rxhubaddr2::RXHUBADDR2_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 2"]
pub mod rxhubaddr2;
#[doc = "RXHUBPORT2 register accessor: an alias for `Reg<RXHUBPORT2_SPEC>`"]
pub type RXHUBPORT2 = crate::Reg<rxhubport2::RXHUBPORT2_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 2"]
pub mod rxhubport2;
#[doc = "TXFUNCADDR3 register accessor: an alias for `Reg<TXFUNCADDR3_SPEC>`"]
pub type TXFUNCADDR3 = crate::Reg<txfuncaddr3::TXFUNCADDR3_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 3"]
pub mod txfuncaddr3;
#[doc = "TXHUBADDR3 register accessor: an alias for `Reg<TXHUBADDR3_SPEC>`"]
pub type TXHUBADDR3 = crate::Reg<txhubaddr3::TXHUBADDR3_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 3"]
pub mod txhubaddr3;
#[doc = "TXHUBPORT3 register accessor: an alias for `Reg<TXHUBPORT3_SPEC>`"]
pub type TXHUBPORT3 = crate::Reg<txhubport3::TXHUBPORT3_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 3"]
pub mod txhubport3;
#[doc = "RXFUNCADDR3 register accessor: an alias for `Reg<RXFUNCADDR3_SPEC>`"]
pub type RXFUNCADDR3 = crate::Reg<rxfuncaddr3::RXFUNCADDR3_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 3"]
pub mod rxfuncaddr3;
#[doc = "RXHUBADDR3 register accessor: an alias for `Reg<RXHUBADDR3_SPEC>`"]
pub type RXHUBADDR3 = crate::Reg<rxhubaddr3::RXHUBADDR3_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 3"]
pub mod rxhubaddr3;
#[doc = "RXHUBPORT3 register accessor: an alias for `Reg<RXHUBPORT3_SPEC>`"]
pub type RXHUBPORT3 = crate::Reg<rxhubport3::RXHUBPORT3_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 3"]
pub mod rxhubport3;
#[doc = "TXFUNCADDR4 register accessor: an alias for `Reg<TXFUNCADDR4_SPEC>`"]
pub type TXFUNCADDR4 = crate::Reg<txfuncaddr4::TXFUNCADDR4_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 4"]
pub mod txfuncaddr4;
#[doc = "TXHUBADDR4 register accessor: an alias for `Reg<TXHUBADDR4_SPEC>`"]
pub type TXHUBADDR4 = crate::Reg<txhubaddr4::TXHUBADDR4_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 4"]
pub mod txhubaddr4;
#[doc = "TXHUBPORT4 register accessor: an alias for `Reg<TXHUBPORT4_SPEC>`"]
pub type TXHUBPORT4 = crate::Reg<txhubport4::TXHUBPORT4_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 4"]
pub mod txhubport4;
#[doc = "RXFUNCADDR4 register accessor: an alias for `Reg<RXFUNCADDR4_SPEC>`"]
pub type RXFUNCADDR4 = crate::Reg<rxfuncaddr4::RXFUNCADDR4_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 4"]
pub mod rxfuncaddr4;
#[doc = "RXHUBADDR4 register accessor: an alias for `Reg<RXHUBADDR4_SPEC>`"]
pub type RXHUBADDR4 = crate::Reg<rxhubaddr4::RXHUBADDR4_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 4"]
pub mod rxhubaddr4;
#[doc = "RXHUBPORT4 register accessor: an alias for `Reg<RXHUBPORT4_SPEC>`"]
pub type RXHUBPORT4 = crate::Reg<rxhubport4::RXHUBPORT4_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 4"]
pub mod rxhubport4;
#[doc = "TXFUNCADDR5 register accessor: an alias for `Reg<TXFUNCADDR5_SPEC>`"]
pub type TXFUNCADDR5 = crate::Reg<txfuncaddr5::TXFUNCADDR5_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 5"]
pub mod txfuncaddr5;
#[doc = "TXHUBADDR5 register accessor: an alias for `Reg<TXHUBADDR5_SPEC>`"]
pub type TXHUBADDR5 = crate::Reg<txhubaddr5::TXHUBADDR5_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 5"]
pub mod txhubaddr5;
#[doc = "TXHUBPORT5 register accessor: an alias for `Reg<TXHUBPORT5_SPEC>`"]
pub type TXHUBPORT5 = crate::Reg<txhubport5::TXHUBPORT5_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 5"]
pub mod txhubport5;
#[doc = "RXFUNCADDR5 register accessor: an alias for `Reg<RXFUNCADDR5_SPEC>`"]
pub type RXFUNCADDR5 = crate::Reg<rxfuncaddr5::RXFUNCADDR5_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 5"]
pub mod rxfuncaddr5;
#[doc = "RXHUBADDR5 register accessor: an alias for `Reg<RXHUBADDR5_SPEC>`"]
pub type RXHUBADDR5 = crate::Reg<rxhubaddr5::RXHUBADDR5_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 5"]
pub mod rxhubaddr5;
#[doc = "RXHUBPORT5 register accessor: an alias for `Reg<RXHUBPORT5_SPEC>`"]
pub type RXHUBPORT5 = crate::Reg<rxhubport5::RXHUBPORT5_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 5"]
pub mod rxhubport5;
#[doc = "TXFUNCADDR6 register accessor: an alias for `Reg<TXFUNCADDR6_SPEC>`"]
pub type TXFUNCADDR6 = crate::Reg<txfuncaddr6::TXFUNCADDR6_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 6"]
pub mod txfuncaddr6;
#[doc = "TXHUBADDR6 register accessor: an alias for `Reg<TXHUBADDR6_SPEC>`"]
pub type TXHUBADDR6 = crate::Reg<txhubaddr6::TXHUBADDR6_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 6"]
pub mod txhubaddr6;
#[doc = "TXHUBPORT6 register accessor: an alias for `Reg<TXHUBPORT6_SPEC>`"]
pub type TXHUBPORT6 = crate::Reg<txhubport6::TXHUBPORT6_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 6"]
pub mod txhubport6;
#[doc = "RXFUNCADDR6 register accessor: an alias for `Reg<RXFUNCADDR6_SPEC>`"]
pub type RXFUNCADDR6 = crate::Reg<rxfuncaddr6::RXFUNCADDR6_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 6"]
pub mod rxfuncaddr6;
#[doc = "RXHUBADDR6 register accessor: an alias for `Reg<RXHUBADDR6_SPEC>`"]
pub type RXHUBADDR6 = crate::Reg<rxhubaddr6::RXHUBADDR6_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 6"]
pub mod rxhubaddr6;
#[doc = "RXHUBPORT6 register accessor: an alias for `Reg<RXHUBPORT6_SPEC>`"]
pub type RXHUBPORT6 = crate::Reg<rxhubport6::RXHUBPORT6_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 6"]
pub mod rxhubport6;
#[doc = "TXFUNCADDR7 register accessor: an alias for `Reg<TXFUNCADDR7_SPEC>`"]
pub type TXFUNCADDR7 = crate::Reg<txfuncaddr7::TXFUNCADDR7_SPEC>;
#[doc = "USB Transmit Functional Address Endpoint 7"]
pub mod txfuncaddr7;
#[doc = "TXHUBADDR7 register accessor: an alias for `Reg<TXHUBADDR7_SPEC>`"]
pub type TXHUBADDR7 = crate::Reg<txhubaddr7::TXHUBADDR7_SPEC>;
#[doc = "USB Transmit Hub Address Endpoint 7"]
pub mod txhubaddr7;
#[doc = "TXHUBPORT7 register accessor: an alias for `Reg<TXHUBPORT7_SPEC>`"]
pub type TXHUBPORT7 = crate::Reg<txhubport7::TXHUBPORT7_SPEC>;
#[doc = "USB Transmit Hub Port Endpoint 7"]
pub mod txhubport7;
#[doc = "RXFUNCADDR7 register accessor: an alias for `Reg<RXFUNCADDR7_SPEC>`"]
pub type RXFUNCADDR7 = crate::Reg<rxfuncaddr7::RXFUNCADDR7_SPEC>;
#[doc = "USB Receive Functional Address Endpoint 7"]
pub mod rxfuncaddr7;
#[doc = "RXHUBADDR7 register accessor: an alias for `Reg<RXHUBADDR7_SPEC>`"]
pub type RXHUBADDR7 = crate::Reg<rxhubaddr7::RXHUBADDR7_SPEC>;
#[doc = "USB Receive Hub Address Endpoint 7"]
pub mod rxhubaddr7;
#[doc = "RXHUBPORT7 register accessor: an alias for `Reg<RXHUBPORT7_SPEC>`"]
pub type RXHUBPORT7 = crate::Reg<rxhubport7::RXHUBPORT7_SPEC>;
#[doc = "USB Receive Hub Port Endpoint 7"]
pub mod rxhubport7;
#[doc = "CSRL0 register accessor: an alias for `Reg<CSRL0_SPEC>`"]
pub type CSRL0 = crate::Reg<csrl0::CSRL0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod csrl0;
#[doc = "USB0_ALT_CSRL0 register accessor: an alias for `Reg<USB0_ALT_CSRL0_SPEC>`"]
pub type USB0_ALT_CSRL0 = crate::Reg<usb0_alt_csrl0::USB0_ALT_CSRL0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod usb0_alt_csrl0;
#[doc = "CSRH0 register accessor: an alias for `Reg<CSRH0_SPEC>`"]
pub type CSRH0 = crate::Reg<csrh0::CSRH0_SPEC>;
#[doc = "USB Control and Status Endpoint 0 High"]
pub mod csrh0;
#[doc = "COUNT0 register accessor: an alias for `Reg<COUNT0_SPEC>`"]
pub type COUNT0 = crate::Reg<count0::COUNT0_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 0"]
pub mod count0;
#[doc = "TYPE0 register accessor: an alias for `Reg<TYPE0_SPEC>`"]
pub type TYPE0 = crate::Reg<type0::TYPE0_SPEC>;
#[doc = "USB Type Endpoint 0"]
pub mod type0;
#[doc = "NAKLMT register accessor: an alias for `Reg<NAKLMT_SPEC>`"]
pub type NAKLMT = crate::Reg<naklmt::NAKLMT_SPEC>;
#[doc = "USB NAK Limit"]
pub mod naklmt;
#[doc = "TXMAXP1 register accessor: an alias for `Reg<TXMAXP1_SPEC>`"]
pub type TXMAXP1 = crate::Reg<txmaxp1::TXMAXP1_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 1"]
pub mod txmaxp1;
#[doc = "TXCSRL1 register accessor: an alias for `Reg<TXCSRL1_SPEC>`"]
pub type TXCSRL1 = crate::Reg<txcsrl1::TXCSRL1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod txcsrl1;
#[doc = "USB0_ALT_TXCSRL1 register accessor: an alias for `Reg<USB0_ALT_TXCSRL1_SPEC>`"]
pub type USB0_ALT_TXCSRL1 = crate::Reg<usb0_alt_txcsrl1::USB0_ALT_TXCSRL1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod usb0_alt_txcsrl1;
#[doc = "TXCSRH1 register accessor: an alias for `Reg<TXCSRH1_SPEC>`"]
pub type TXCSRH1 = crate::Reg<txcsrh1::TXCSRH1_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 1 High"]
pub mod txcsrh1;
#[doc = "RXMAXP1 register accessor: an alias for `Reg<RXMAXP1_SPEC>`"]
pub type RXMAXP1 = crate::Reg<rxmaxp1::RXMAXP1_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 1"]
pub mod rxmaxp1;
#[doc = "RXCSRL1 register accessor: an alias for `Reg<RXCSRL1_SPEC>`"]
pub type RXCSRL1 = crate::Reg<rxcsrl1::RXCSRL1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod rxcsrl1;
#[doc = "USB0_ALT_RXCSRL1 register accessor: an alias for `Reg<USB0_ALT_RXCSRL1_SPEC>`"]
pub type USB0_ALT_RXCSRL1 = crate::Reg<usb0_alt_rxcsrl1::USB0_ALT_RXCSRL1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod usb0_alt_rxcsrl1;
#[doc = "RXCSRH1 register accessor: an alias for `Reg<RXCSRH1_SPEC>`"]
pub type RXCSRH1 = crate::Reg<rxcsrh1::RXCSRH1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod rxcsrh1;
#[doc = "USB0_ALT_RXCSRH1 register accessor: an alias for `Reg<USB0_ALT_RXCSRH1_SPEC>`"]
pub type USB0_ALT_RXCSRH1 = crate::Reg<usb0_alt_rxcsrh1::USB0_ALT_RXCSRH1_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod usb0_alt_rxcsrh1;
#[doc = "RXCOUNT1 register accessor: an alias for `Reg<RXCOUNT1_SPEC>`"]
pub type RXCOUNT1 = crate::Reg<rxcount1::RXCOUNT1_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 1"]
pub mod rxcount1;
#[doc = "TXTYPE1 register accessor: an alias for `Reg<TXTYPE1_SPEC>`"]
pub type TXTYPE1 = crate::Reg<txtype1::TXTYPE1_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 1"]
pub mod txtype1;
#[doc = "TXINTERVAL1 register accessor: an alias for `Reg<TXINTERVAL1_SPEC>`"]
pub type TXINTERVAL1 = crate::Reg<txinterval1::TXINTERVAL1_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod txinterval1;
#[doc = "USB0_ALT_TXINTERVAL1 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL1_SPEC>`"]
pub type USB0_ALT_TXINTERVAL1 = crate::Reg<usb0_alt_txinterval1::USB0_ALT_TXINTERVAL1_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod usb0_alt_txinterval1;
#[doc = "RXTYPE1 register accessor: an alias for `Reg<RXTYPE1_SPEC>`"]
pub type RXTYPE1 = crate::Reg<rxtype1::RXTYPE1_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 1"]
pub mod rxtype1;
#[doc = "RXINTERVAL1 register accessor: an alias for `Reg<RXINTERVAL1_SPEC>`"]
pub type RXINTERVAL1 = crate::Reg<rxinterval1::RXINTERVAL1_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod rxinterval1;
#[doc = "USB0_ALT_RXINTERVAL1 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL1_SPEC>`"]
pub type USB0_ALT_RXINTERVAL1 = crate::Reg<usb0_alt_rxinterval1::USB0_ALT_RXINTERVAL1_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod usb0_alt_rxinterval1;
#[doc = "TXMAXP2 register accessor: an alias for `Reg<TXMAXP2_SPEC>`"]
pub type TXMAXP2 = crate::Reg<txmaxp2::TXMAXP2_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 2"]
pub mod txmaxp2;
#[doc = "TXCSRL2 register accessor: an alias for `Reg<TXCSRL2_SPEC>`"]
pub type TXCSRL2 = crate::Reg<txcsrl2::TXCSRL2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod txcsrl2;
#[doc = "USB0_ALT_TXCSRL2 register accessor: an alias for `Reg<USB0_ALT_TXCSRL2_SPEC>`"]
pub type USB0_ALT_TXCSRL2 = crate::Reg<usb0_alt_txcsrl2::USB0_ALT_TXCSRL2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod usb0_alt_txcsrl2;
#[doc = "TXCSRH2 register accessor: an alias for `Reg<TXCSRH2_SPEC>`"]
pub type TXCSRH2 = crate::Reg<txcsrh2::TXCSRH2_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 2 High"]
pub mod txcsrh2;
#[doc = "RXMAXP2 register accessor: an alias for `Reg<RXMAXP2_SPEC>`"]
pub type RXMAXP2 = crate::Reg<rxmaxp2::RXMAXP2_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 2"]
pub mod rxmaxp2;
#[doc = "RXCSRL2 register accessor: an alias for `Reg<RXCSRL2_SPEC>`"]
pub type RXCSRL2 = crate::Reg<rxcsrl2::RXCSRL2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod rxcsrl2;
#[doc = "USB0_ALT_RXCSRL2 register accessor: an alias for `Reg<USB0_ALT_RXCSRL2_SPEC>`"]
pub type USB0_ALT_RXCSRL2 = crate::Reg<usb0_alt_rxcsrl2::USB0_ALT_RXCSRL2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod usb0_alt_rxcsrl2;
#[doc = "RXCSRH2 register accessor: an alias for `Reg<RXCSRH2_SPEC>`"]
pub type RXCSRH2 = crate::Reg<rxcsrh2::RXCSRH2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod rxcsrh2;
#[doc = "USB0_ALT_RXCSRH2 register accessor: an alias for `Reg<USB0_ALT_RXCSRH2_SPEC>`"]
pub type USB0_ALT_RXCSRH2 = crate::Reg<usb0_alt_rxcsrh2::USB0_ALT_RXCSRH2_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod usb0_alt_rxcsrh2;
#[doc = "RXCOUNT2 register accessor: an alias for `Reg<RXCOUNT2_SPEC>`"]
pub type RXCOUNT2 = crate::Reg<rxcount2::RXCOUNT2_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 2"]
pub mod rxcount2;
#[doc = "TXTYPE2 register accessor: an alias for `Reg<TXTYPE2_SPEC>`"]
pub type TXTYPE2 = crate::Reg<txtype2::TXTYPE2_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 2"]
pub mod txtype2;
#[doc = "TXINTERVAL2 register accessor: an alias for `Reg<TXINTERVAL2_SPEC>`"]
pub type TXINTERVAL2 = crate::Reg<txinterval2::TXINTERVAL2_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod txinterval2;
#[doc = "USB0_ALT_TXINTERVAL2 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL2_SPEC>`"]
pub type USB0_ALT_TXINTERVAL2 = crate::Reg<usb0_alt_txinterval2::USB0_ALT_TXINTERVAL2_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod usb0_alt_txinterval2;
#[doc = "RXTYPE2 register accessor: an alias for `Reg<RXTYPE2_SPEC>`"]
pub type RXTYPE2 = crate::Reg<rxtype2::RXTYPE2_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 2"]
pub mod rxtype2;
#[doc = "RXINTERVAL2 register accessor: an alias for `Reg<RXINTERVAL2_SPEC>`"]
pub type RXINTERVAL2 = crate::Reg<rxinterval2::RXINTERVAL2_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod rxinterval2;
#[doc = "USB0_ALT_RXINTERVAL2 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL2_SPEC>`"]
pub type USB0_ALT_RXINTERVAL2 = crate::Reg<usb0_alt_rxinterval2::USB0_ALT_RXINTERVAL2_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod usb0_alt_rxinterval2;
#[doc = "TXMAXP3 register accessor: an alias for `Reg<TXMAXP3_SPEC>`"]
pub type TXMAXP3 = crate::Reg<txmaxp3::TXMAXP3_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 3"]
pub mod txmaxp3;
#[doc = "TXCSRL3 register accessor: an alias for `Reg<TXCSRL3_SPEC>`"]
pub type TXCSRL3 = crate::Reg<txcsrl3::TXCSRL3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod txcsrl3;
#[doc = "USB0_ALT_TXCSRL3 register accessor: an alias for `Reg<USB0_ALT_TXCSRL3_SPEC>`"]
pub type USB0_ALT_TXCSRL3 = crate::Reg<usb0_alt_txcsrl3::USB0_ALT_TXCSRL3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod usb0_alt_txcsrl3;
#[doc = "TXCSRH3 register accessor: an alias for `Reg<TXCSRH3_SPEC>`"]
pub type TXCSRH3 = crate::Reg<txcsrh3::TXCSRH3_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 3 High"]
pub mod txcsrh3;
#[doc = "RXMAXP3 register accessor: an alias for `Reg<RXMAXP3_SPEC>`"]
pub type RXMAXP3 = crate::Reg<rxmaxp3::RXMAXP3_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 3"]
pub mod rxmaxp3;
#[doc = "RXCSRL3 register accessor: an alias for `Reg<RXCSRL3_SPEC>`"]
pub type RXCSRL3 = crate::Reg<rxcsrl3::RXCSRL3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod rxcsrl3;
#[doc = "USB0_ALT_RXCSRL3 register accessor: an alias for `Reg<USB0_ALT_RXCSRL3_SPEC>`"]
pub type USB0_ALT_RXCSRL3 = crate::Reg<usb0_alt_rxcsrl3::USB0_ALT_RXCSRL3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod usb0_alt_rxcsrl3;
#[doc = "RXCSRH3 register accessor: an alias for `Reg<RXCSRH3_SPEC>`"]
pub type RXCSRH3 = crate::Reg<rxcsrh3::RXCSRH3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod rxcsrh3;
#[doc = "USB0_ALT_RXCSRH3 register accessor: an alias for `Reg<USB0_ALT_RXCSRH3_SPEC>`"]
pub type USB0_ALT_RXCSRH3 = crate::Reg<usb0_alt_rxcsrh3::USB0_ALT_RXCSRH3_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod usb0_alt_rxcsrh3;
#[doc = "RXCOUNT3 register accessor: an alias for `Reg<RXCOUNT3_SPEC>`"]
pub type RXCOUNT3 = crate::Reg<rxcount3::RXCOUNT3_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 3"]
pub mod rxcount3;
#[doc = "TXTYPE3 register accessor: an alias for `Reg<TXTYPE3_SPEC>`"]
pub type TXTYPE3 = crate::Reg<txtype3::TXTYPE3_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 3"]
pub mod txtype3;
#[doc = "TXINTERVAL3 register accessor: an alias for `Reg<TXINTERVAL3_SPEC>`"]
pub type TXINTERVAL3 = crate::Reg<txinterval3::TXINTERVAL3_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod txinterval3;
#[doc = "USB0_ALT_TXINTERVAL3 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL3_SPEC>`"]
pub type USB0_ALT_TXINTERVAL3 = crate::Reg<usb0_alt_txinterval3::USB0_ALT_TXINTERVAL3_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod usb0_alt_txinterval3;
#[doc = "RXTYPE3 register accessor: an alias for `Reg<RXTYPE3_SPEC>`"]
pub type RXTYPE3 = crate::Reg<rxtype3::RXTYPE3_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 3"]
pub mod rxtype3;
#[doc = "RXINTERVAL3 register accessor: an alias for `Reg<RXINTERVAL3_SPEC>`"]
pub type RXINTERVAL3 = crate::Reg<rxinterval3::RXINTERVAL3_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod rxinterval3;
#[doc = "USB0_ALT_RXINTERVAL3 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL3_SPEC>`"]
pub type USB0_ALT_RXINTERVAL3 = crate::Reg<usb0_alt_rxinterval3::USB0_ALT_RXINTERVAL3_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod usb0_alt_rxinterval3;
#[doc = "TXMAXP4 register accessor: an alias for `Reg<TXMAXP4_SPEC>`"]
pub type TXMAXP4 = crate::Reg<txmaxp4::TXMAXP4_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 4"]
pub mod txmaxp4;
#[doc = "TXCSRL4 register accessor: an alias for `Reg<TXCSRL4_SPEC>`"]
pub type TXCSRL4 = crate::Reg<txcsrl4::TXCSRL4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod txcsrl4;
#[doc = "USB0_ALT_TXCSRL4 register accessor: an alias for `Reg<USB0_ALT_TXCSRL4_SPEC>`"]
pub type USB0_ALT_TXCSRL4 = crate::Reg<usb0_alt_txcsrl4::USB0_ALT_TXCSRL4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod usb0_alt_txcsrl4;
#[doc = "TXCSRH4 register accessor: an alias for `Reg<TXCSRH4_SPEC>`"]
pub type TXCSRH4 = crate::Reg<txcsrh4::TXCSRH4_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 4 High"]
pub mod txcsrh4;
#[doc = "RXMAXP4 register accessor: an alias for `Reg<RXMAXP4_SPEC>`"]
pub type RXMAXP4 = crate::Reg<rxmaxp4::RXMAXP4_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 4"]
pub mod rxmaxp4;
#[doc = "RXCSRL4 register accessor: an alias for `Reg<RXCSRL4_SPEC>`"]
pub type RXCSRL4 = crate::Reg<rxcsrl4::RXCSRL4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod rxcsrl4;
#[doc = "USB0_ALT_RXCSRL4 register accessor: an alias for `Reg<USB0_ALT_RXCSRL4_SPEC>`"]
pub type USB0_ALT_RXCSRL4 = crate::Reg<usb0_alt_rxcsrl4::USB0_ALT_RXCSRL4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod usb0_alt_rxcsrl4;
#[doc = "RXCSRH4 register accessor: an alias for `Reg<RXCSRH4_SPEC>`"]
pub type RXCSRH4 = crate::Reg<rxcsrh4::RXCSRH4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod rxcsrh4;
#[doc = "USB0_ALT_RXCSRH4 register accessor: an alias for `Reg<USB0_ALT_RXCSRH4_SPEC>`"]
pub type USB0_ALT_RXCSRH4 = crate::Reg<usb0_alt_rxcsrh4::USB0_ALT_RXCSRH4_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod usb0_alt_rxcsrh4;
#[doc = "RXCOUNT4 register accessor: an alias for `Reg<RXCOUNT4_SPEC>`"]
pub type RXCOUNT4 = crate::Reg<rxcount4::RXCOUNT4_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 4"]
pub mod rxcount4;
#[doc = "TXTYPE4 register accessor: an alias for `Reg<TXTYPE4_SPEC>`"]
pub type TXTYPE4 = crate::Reg<txtype4::TXTYPE4_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 4"]
pub mod txtype4;
#[doc = "TXINTERVAL4 register accessor: an alias for `Reg<TXINTERVAL4_SPEC>`"]
pub type TXINTERVAL4 = crate::Reg<txinterval4::TXINTERVAL4_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod txinterval4;
#[doc = "USB0_ALT_TXINTERVAL4 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL4_SPEC>`"]
pub type USB0_ALT_TXINTERVAL4 = crate::Reg<usb0_alt_txinterval4::USB0_ALT_TXINTERVAL4_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod usb0_alt_txinterval4;
#[doc = "RXTYPE4 register accessor: an alias for `Reg<RXTYPE4_SPEC>`"]
pub type RXTYPE4 = crate::Reg<rxtype4::RXTYPE4_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 4"]
pub mod rxtype4;
#[doc = "RXINTERVAL4 register accessor: an alias for `Reg<RXINTERVAL4_SPEC>`"]
pub type RXINTERVAL4 = crate::Reg<rxinterval4::RXINTERVAL4_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod rxinterval4;
#[doc = "USB0_ALT_RXINTERVAL4 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL4_SPEC>`"]
pub type USB0_ALT_RXINTERVAL4 = crate::Reg<usb0_alt_rxinterval4::USB0_ALT_RXINTERVAL4_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod usb0_alt_rxinterval4;
#[doc = "TXMAXP5 register accessor: an alias for `Reg<TXMAXP5_SPEC>`"]
pub type TXMAXP5 = crate::Reg<txmaxp5::TXMAXP5_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 5"]
pub mod txmaxp5;
#[doc = "TXCSRL5 register accessor: an alias for `Reg<TXCSRL5_SPEC>`"]
pub type TXCSRL5 = crate::Reg<txcsrl5::TXCSRL5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod txcsrl5;
#[doc = "USB0_ALT_TXCSRL5 register accessor: an alias for `Reg<USB0_ALT_TXCSRL5_SPEC>`"]
pub type USB0_ALT_TXCSRL5 = crate::Reg<usb0_alt_txcsrl5::USB0_ALT_TXCSRL5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod usb0_alt_txcsrl5;
#[doc = "TXCSRH5 register accessor: an alias for `Reg<TXCSRH5_SPEC>`"]
pub type TXCSRH5 = crate::Reg<txcsrh5::TXCSRH5_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 5 High"]
pub mod txcsrh5;
#[doc = "RXMAXP5 register accessor: an alias for `Reg<RXMAXP5_SPEC>`"]
pub type RXMAXP5 = crate::Reg<rxmaxp5::RXMAXP5_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 5"]
pub mod rxmaxp5;
#[doc = "RXCSRL5 register accessor: an alias for `Reg<RXCSRL5_SPEC>`"]
pub type RXCSRL5 = crate::Reg<rxcsrl5::RXCSRL5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod rxcsrl5;
#[doc = "USB0_ALT_RXCSRL5 register accessor: an alias for `Reg<USB0_ALT_RXCSRL5_SPEC>`"]
pub type USB0_ALT_RXCSRL5 = crate::Reg<usb0_alt_rxcsrl5::USB0_ALT_RXCSRL5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod usb0_alt_rxcsrl5;
#[doc = "RXCSRH5 register accessor: an alias for `Reg<RXCSRH5_SPEC>`"]
pub type RXCSRH5 = crate::Reg<rxcsrh5::RXCSRH5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod rxcsrh5;
#[doc = "USB0_ALT_RXCSRH5 register accessor: an alias for `Reg<USB0_ALT_RXCSRH5_SPEC>`"]
pub type USB0_ALT_RXCSRH5 = crate::Reg<usb0_alt_rxcsrh5::USB0_ALT_RXCSRH5_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod usb0_alt_rxcsrh5;
#[doc = "RXCOUNT5 register accessor: an alias for `Reg<RXCOUNT5_SPEC>`"]
pub type RXCOUNT5 = crate::Reg<rxcount5::RXCOUNT5_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 5"]
pub mod rxcount5;
#[doc = "TXTYPE5 register accessor: an alias for `Reg<TXTYPE5_SPEC>`"]
pub type TXTYPE5 = crate::Reg<txtype5::TXTYPE5_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 5"]
pub mod txtype5;
#[doc = "TXINTERVAL5 register accessor: an alias for `Reg<TXINTERVAL5_SPEC>`"]
pub type TXINTERVAL5 = crate::Reg<txinterval5::TXINTERVAL5_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod txinterval5;
#[doc = "USB0_ALT_TXINTERVAL5 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL5_SPEC>`"]
pub type USB0_ALT_TXINTERVAL5 = crate::Reg<usb0_alt_txinterval5::USB0_ALT_TXINTERVAL5_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod usb0_alt_txinterval5;
#[doc = "RXTYPE5 register accessor: an alias for `Reg<RXTYPE5_SPEC>`"]
pub type RXTYPE5 = crate::Reg<rxtype5::RXTYPE5_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 5"]
pub mod rxtype5;
#[doc = "RXINTERVAL5 register accessor: an alias for `Reg<RXINTERVAL5_SPEC>`"]
pub type RXINTERVAL5 = crate::Reg<rxinterval5::RXINTERVAL5_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod rxinterval5;
#[doc = "USB0_ALT_RXINTERVAL5 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL5_SPEC>`"]
pub type USB0_ALT_RXINTERVAL5 = crate::Reg<usb0_alt_rxinterval5::USB0_ALT_RXINTERVAL5_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod usb0_alt_rxinterval5;
#[doc = "TXMAXP6 register accessor: an alias for `Reg<TXMAXP6_SPEC>`"]
pub type TXMAXP6 = crate::Reg<txmaxp6::TXMAXP6_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 6"]
pub mod txmaxp6;
#[doc = "TXCSRL6 register accessor: an alias for `Reg<TXCSRL6_SPEC>`"]
pub type TXCSRL6 = crate::Reg<txcsrl6::TXCSRL6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod txcsrl6;
#[doc = "USB0_ALT_TXCSRL6 register accessor: an alias for `Reg<USB0_ALT_TXCSRL6_SPEC>`"]
pub type USB0_ALT_TXCSRL6 = crate::Reg<usb0_alt_txcsrl6::USB0_ALT_TXCSRL6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod usb0_alt_txcsrl6;
#[doc = "TXCSRH6 register accessor: an alias for `Reg<TXCSRH6_SPEC>`"]
pub type TXCSRH6 = crate::Reg<txcsrh6::TXCSRH6_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 6 High"]
pub mod txcsrh6;
#[doc = "RXMAXP6 register accessor: an alias for `Reg<RXMAXP6_SPEC>`"]
pub type RXMAXP6 = crate::Reg<rxmaxp6::RXMAXP6_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 6"]
pub mod rxmaxp6;
#[doc = "RXCSRL6 register accessor: an alias for `Reg<RXCSRL6_SPEC>`"]
pub type RXCSRL6 = crate::Reg<rxcsrl6::RXCSRL6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod rxcsrl6;
#[doc = "USB0_ALT_RXCSRL6 register accessor: an alias for `Reg<USB0_ALT_RXCSRL6_SPEC>`"]
pub type USB0_ALT_RXCSRL6 = crate::Reg<usb0_alt_rxcsrl6::USB0_ALT_RXCSRL6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod usb0_alt_rxcsrl6;
#[doc = "RXCSRH6 register accessor: an alias for `Reg<RXCSRH6_SPEC>`"]
pub type RXCSRH6 = crate::Reg<rxcsrh6::RXCSRH6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod rxcsrh6;
#[doc = "USB0_ALT_RXCSRH6 register accessor: an alias for `Reg<USB0_ALT_RXCSRH6_SPEC>`"]
pub type USB0_ALT_RXCSRH6 = crate::Reg<usb0_alt_rxcsrh6::USB0_ALT_RXCSRH6_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod usb0_alt_rxcsrh6;
#[doc = "RXCOUNT6 register accessor: an alias for `Reg<RXCOUNT6_SPEC>`"]
pub type RXCOUNT6 = crate::Reg<rxcount6::RXCOUNT6_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 6"]
pub mod rxcount6;
#[doc = "TXTYPE6 register accessor: an alias for `Reg<TXTYPE6_SPEC>`"]
pub type TXTYPE6 = crate::Reg<txtype6::TXTYPE6_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 6"]
pub mod txtype6;
#[doc = "TXINTERVAL6 register accessor: an alias for `Reg<TXINTERVAL6_SPEC>`"]
pub type TXINTERVAL6 = crate::Reg<txinterval6::TXINTERVAL6_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod txinterval6;
#[doc = "USB0_ALT_TXINTERVAL6 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL6_SPEC>`"]
pub type USB0_ALT_TXINTERVAL6 = crate::Reg<usb0_alt_txinterval6::USB0_ALT_TXINTERVAL6_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod usb0_alt_txinterval6;
#[doc = "RXTYPE6 register accessor: an alias for `Reg<RXTYPE6_SPEC>`"]
pub type RXTYPE6 = crate::Reg<rxtype6::RXTYPE6_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 6"]
pub mod rxtype6;
#[doc = "RXINTERVAL6 register accessor: an alias for `Reg<RXINTERVAL6_SPEC>`"]
pub type RXINTERVAL6 = crate::Reg<rxinterval6::RXINTERVAL6_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod rxinterval6;
#[doc = "USB0_ALT_RXINTERVAL6 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL6_SPEC>`"]
pub type USB0_ALT_RXINTERVAL6 = crate::Reg<usb0_alt_rxinterval6::USB0_ALT_RXINTERVAL6_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod usb0_alt_rxinterval6;
#[doc = "TXMAXP7 register accessor: an alias for `Reg<TXMAXP7_SPEC>`"]
pub type TXMAXP7 = crate::Reg<txmaxp7::TXMAXP7_SPEC>;
#[doc = "USB Maximum Transmit Data Endpoint 7"]
pub mod txmaxp7;
#[doc = "TXCSRL7 register accessor: an alias for `Reg<TXCSRL7_SPEC>`"]
pub type TXCSRL7 = crate::Reg<txcsrl7::TXCSRL7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod txcsrl7;
#[doc = "USB0_ALT_TXCSRL7 register accessor: an alias for `Reg<USB0_ALT_TXCSRL7_SPEC>`"]
pub type USB0_ALT_TXCSRL7 = crate::Reg<usb0_alt_txcsrl7::USB0_ALT_TXCSRL7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod usb0_alt_txcsrl7;
#[doc = "TXCSRH7 register accessor: an alias for `Reg<TXCSRH7_SPEC>`"]
pub type TXCSRH7 = crate::Reg<txcsrh7::TXCSRH7_SPEC>;
#[doc = "USB Transmit Control and Status Endpoint 7 High"]
pub mod txcsrh7;
#[doc = "RXMAXP7 register accessor: an alias for `Reg<RXMAXP7_SPEC>`"]
pub type RXMAXP7 = crate::Reg<rxmaxp7::RXMAXP7_SPEC>;
#[doc = "USB Maximum Receive Data Endpoint 7"]
pub mod rxmaxp7;
#[doc = "RXCSRL7 register accessor: an alias for `Reg<RXCSRL7_SPEC>`"]
pub type RXCSRL7 = crate::Reg<rxcsrl7::RXCSRL7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod rxcsrl7;
#[doc = "USB0_ALT_RXCSRL7 register accessor: an alias for `Reg<USB0_ALT_RXCSRL7_SPEC>`"]
pub type USB0_ALT_RXCSRL7 = crate::Reg<usb0_alt_rxcsrl7::USB0_ALT_RXCSRL7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod usb0_alt_rxcsrl7;
#[doc = "RXCSRH7 register accessor: an alias for `Reg<RXCSRH7_SPEC>`"]
pub type RXCSRH7 = crate::Reg<rxcsrh7::RXCSRH7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod rxcsrh7;
#[doc = "USB0_ALT_RXCSRH7 register accessor: an alias for `Reg<USB0_ALT_RXCSRH7_SPEC>`"]
pub type USB0_ALT_RXCSRH7 = crate::Reg<usb0_alt_rxcsrh7::USB0_ALT_RXCSRH7_SPEC>;
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod usb0_alt_rxcsrh7;
#[doc = "RXCOUNT7 register accessor: an alias for `Reg<RXCOUNT7_SPEC>`"]
pub type RXCOUNT7 = crate::Reg<rxcount7::RXCOUNT7_SPEC>;
#[doc = "USB Receive Byte Count Endpoint 7"]
pub mod rxcount7;
#[doc = "TXTYPE7 register accessor: an alias for `Reg<TXTYPE7_SPEC>`"]
pub type TXTYPE7 = crate::Reg<txtype7::TXTYPE7_SPEC>;
#[doc = "USB Host Transmit Configure Type Endpoint 7"]
pub mod txtype7;
#[doc = "TXINTERVAL7 register accessor: an alias for `Reg<TXINTERVAL7_SPEC>`"]
pub type TXINTERVAL7 = crate::Reg<txinterval7::TXINTERVAL7_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod txinterval7;
#[doc = "USB0_ALT_TXINTERVAL7 register accessor: an alias for `Reg<USB0_ALT_TXINTERVAL7_SPEC>`"]
pub type USB0_ALT_TXINTERVAL7 = crate::Reg<usb0_alt_txinterval7::USB0_ALT_TXINTERVAL7_SPEC>;
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod usb0_alt_txinterval7;
#[doc = "RXTYPE7 register accessor: an alias for `Reg<RXTYPE7_SPEC>`"]
pub type RXTYPE7 = crate::Reg<rxtype7::RXTYPE7_SPEC>;
#[doc = "USB Host Configure Receive Type Endpoint 7"]
pub mod rxtype7;
#[doc = "RXINTERVAL7 register accessor: an alias for `Reg<RXINTERVAL7_SPEC>`"]
pub type RXINTERVAL7 = crate::Reg<rxinterval7::RXINTERVAL7_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod rxinterval7;
#[doc = "USB0_ALT_RXINTERVAL7 register accessor: an alias for `Reg<USB0_ALT_RXINTERVAL7_SPEC>`"]
pub type USB0_ALT_RXINTERVAL7 = crate::Reg<usb0_alt_rxinterval7::USB0_ALT_RXINTERVAL7_SPEC>;
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod usb0_alt_rxinterval7;
#[doc = "DMAINTR register accessor: an alias for `Reg<DMAINTR_SPEC>`"]
pub type DMAINTR = crate::Reg<dmaintr::DMAINTR_SPEC>;
#[doc = "USB DMA Interrupt"]
pub mod dmaintr;
#[doc = "DMACTL0 register accessor: an alias for `Reg<DMACTL0_SPEC>`"]
pub type DMACTL0 = crate::Reg<dmactl0::DMACTL0_SPEC>;
#[doc = "USB DMA Control 0"]
pub mod dmactl0;
#[doc = "DMAADDR0 register accessor: an alias for `Reg<DMAADDR0_SPEC>`"]
pub type DMAADDR0 = crate::Reg<dmaaddr0::DMAADDR0_SPEC>;
#[doc = "USB DMA Address 0"]
pub mod dmaaddr0;
#[doc = "DMACOUNT0 register accessor: an alias for `Reg<DMACOUNT0_SPEC>`"]
pub type DMACOUNT0 = crate::Reg<dmacount0::DMACOUNT0_SPEC>;
#[doc = "USB DMA Count 0"]
pub mod dmacount0;
#[doc = "DMACTL1 register accessor: an alias for `Reg<DMACTL1_SPEC>`"]
pub type DMACTL1 = crate::Reg<dmactl1::DMACTL1_SPEC>;
#[doc = "USB DMA Control 1"]
pub mod dmactl1;
#[doc = "DMAADDR1 register accessor: an alias for `Reg<DMAADDR1_SPEC>`"]
pub type DMAADDR1 = crate::Reg<dmaaddr1::DMAADDR1_SPEC>;
#[doc = "USB DMA Address 1"]
pub mod dmaaddr1;
#[doc = "DMACOUNT1 register accessor: an alias for `Reg<DMACOUNT1_SPEC>`"]
pub type DMACOUNT1 = crate::Reg<dmacount1::DMACOUNT1_SPEC>;
#[doc = "USB DMA Count 1"]
pub mod dmacount1;
#[doc = "DMACTL2 register accessor: an alias for `Reg<DMACTL2_SPEC>`"]
pub type DMACTL2 = crate::Reg<dmactl2::DMACTL2_SPEC>;
#[doc = "USB DMA Control 2"]
pub mod dmactl2;
#[doc = "DMAADDR2 register accessor: an alias for `Reg<DMAADDR2_SPEC>`"]
pub type DMAADDR2 = crate::Reg<dmaaddr2::DMAADDR2_SPEC>;
#[doc = "USB DMA Address 2"]
pub mod dmaaddr2;
#[doc = "DMACOUNT2 register accessor: an alias for `Reg<DMACOUNT2_SPEC>`"]
pub type DMACOUNT2 = crate::Reg<dmacount2::DMACOUNT2_SPEC>;
#[doc = "USB DMA Count 2"]
pub mod dmacount2;
#[doc = "DMACTL3 register accessor: an alias for `Reg<DMACTL3_SPEC>`"]
pub type DMACTL3 = crate::Reg<dmactl3::DMACTL3_SPEC>;
#[doc = "USB DMA Control 3"]
pub mod dmactl3;
#[doc = "DMAADDR3 register accessor: an alias for `Reg<DMAADDR3_SPEC>`"]
pub type DMAADDR3 = crate::Reg<dmaaddr3::DMAADDR3_SPEC>;
#[doc = "USB DMA Address 3"]
pub mod dmaaddr3;
#[doc = "DMACOUNT3 register accessor: an alias for `Reg<DMACOUNT3_SPEC>`"]
pub type DMACOUNT3 = crate::Reg<dmacount3::DMACOUNT3_SPEC>;
#[doc = "USB DMA Count 3"]
pub mod dmacount3;
#[doc = "DMACTL4 register accessor: an alias for `Reg<DMACTL4_SPEC>`"]
pub type DMACTL4 = crate::Reg<dmactl4::DMACTL4_SPEC>;
#[doc = "USB DMA Control 4"]
pub mod dmactl4;
#[doc = "DMAADDR4 register accessor: an alias for `Reg<DMAADDR4_SPEC>`"]
pub type DMAADDR4 = crate::Reg<dmaaddr4::DMAADDR4_SPEC>;
#[doc = "USB DMA Address 4"]
pub mod dmaaddr4;
#[doc = "DMACOUNT4 register accessor: an alias for `Reg<DMACOUNT4_SPEC>`"]
pub type DMACOUNT4 = crate::Reg<dmacount4::DMACOUNT4_SPEC>;
#[doc = "USB DMA Count 4"]
pub mod dmacount4;
#[doc = "DMACTL5 register accessor: an alias for `Reg<DMACTL5_SPEC>`"]
pub type DMACTL5 = crate::Reg<dmactl5::DMACTL5_SPEC>;
#[doc = "USB DMA Control 5"]
pub mod dmactl5;
#[doc = "DMAADDR5 register accessor: an alias for `Reg<DMAADDR5_SPEC>`"]
pub type DMAADDR5 = crate::Reg<dmaaddr5::DMAADDR5_SPEC>;
#[doc = "USB DMA Address 5"]
pub mod dmaaddr5;
#[doc = "DMACOUNT5 register accessor: an alias for `Reg<DMACOUNT5_SPEC>`"]
pub type DMACOUNT5 = crate::Reg<dmacount5::DMACOUNT5_SPEC>;
#[doc = "USB DMA Count 5"]
pub mod dmacount5;
#[doc = "DMACTL6 register accessor: an alias for `Reg<DMACTL6_SPEC>`"]
pub type DMACTL6 = crate::Reg<dmactl6::DMACTL6_SPEC>;
#[doc = "USB DMA Control 6"]
pub mod dmactl6;
#[doc = "DMAADDR6 register accessor: an alias for `Reg<DMAADDR6_SPEC>`"]
pub type DMAADDR6 = crate::Reg<dmaaddr6::DMAADDR6_SPEC>;
#[doc = "USB DMA Address 6"]
pub mod dmaaddr6;
#[doc = "DMACOUNT6 register accessor: an alias for `Reg<DMACOUNT6_SPEC>`"]
pub type DMACOUNT6 = crate::Reg<dmacount6::DMACOUNT6_SPEC>;
#[doc = "USB DMA Count 6"]
pub mod dmacount6;
#[doc = "DMACTL7 register accessor: an alias for `Reg<DMACTL7_SPEC>`"]
pub type DMACTL7 = crate::Reg<dmactl7::DMACTL7_SPEC>;
#[doc = "USB DMA Control 7"]
pub mod dmactl7;
#[doc = "DMAADDR7 register accessor: an alias for `Reg<DMAADDR7_SPEC>`"]
pub type DMAADDR7 = crate::Reg<dmaaddr7::DMAADDR7_SPEC>;
#[doc = "USB DMA Address 7"]
pub mod dmaaddr7;
#[doc = "DMACOUNT7 register accessor: an alias for `Reg<DMACOUNT7_SPEC>`"]
pub type DMACOUNT7 = crate::Reg<dmacount7::DMACOUNT7_SPEC>;
#[doc = "USB DMA Count 7"]
pub mod dmacount7;
#[doc = "RQPKTCOUNT1 register accessor: an alias for `Reg<RQPKTCOUNT1_SPEC>`"]
pub type RQPKTCOUNT1 = crate::Reg<rqpktcount1::RQPKTCOUNT1_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1"]
pub mod rqpktcount1;
#[doc = "RQPKTCOUNT2 register accessor: an alias for `Reg<RQPKTCOUNT2_SPEC>`"]
pub type RQPKTCOUNT2 = crate::Reg<rqpktcount2::RQPKTCOUNT2_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2"]
pub mod rqpktcount2;
#[doc = "RQPKTCOUNT3 register accessor: an alias for `Reg<RQPKTCOUNT3_SPEC>`"]
pub type RQPKTCOUNT3 = crate::Reg<rqpktcount3::RQPKTCOUNT3_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3"]
pub mod rqpktcount3;
#[doc = "RQPKTCOUNT4 register accessor: an alias for `Reg<RQPKTCOUNT4_SPEC>`"]
pub type RQPKTCOUNT4 = crate::Reg<rqpktcount4::RQPKTCOUNT4_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4"]
pub mod rqpktcount4;
#[doc = "RQPKTCOUNT5 register accessor: an alias for `Reg<RQPKTCOUNT5_SPEC>`"]
pub type RQPKTCOUNT5 = crate::Reg<rqpktcount5::RQPKTCOUNT5_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5"]
pub mod rqpktcount5;
#[doc = "RQPKTCOUNT6 register accessor: an alias for `Reg<RQPKTCOUNT6_SPEC>`"]
pub type RQPKTCOUNT6 = crate::Reg<rqpktcount6::RQPKTCOUNT6_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6"]
pub mod rqpktcount6;
#[doc = "RQPKTCOUNT7 register accessor: an alias for `Reg<RQPKTCOUNT7_SPEC>`"]
pub type RQPKTCOUNT7 = crate::Reg<rqpktcount7::RQPKTCOUNT7_SPEC>;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7"]
pub mod rqpktcount7;
#[doc = "RXDPKTBUFDIS register accessor: an alias for `Reg<RXDPKTBUFDIS_SPEC>`"]
pub type RXDPKTBUFDIS = crate::Reg<rxdpktbufdis::RXDPKTBUFDIS_SPEC>;
#[doc = "USB Receive Double Packet Buffer Disable"]
pub mod rxdpktbufdis;
#[doc = "TXDPKTBUFDIS register accessor: an alias for `Reg<TXDPKTBUFDIS_SPEC>`"]
pub type TXDPKTBUFDIS = crate::Reg<txdpktbufdis::TXDPKTBUFDIS_SPEC>;
#[doc = "USB Transmit Double Packet Buffer Disable"]
pub mod txdpktbufdis;
#[doc = "CTO register accessor: an alias for `Reg<CTO_SPEC>`"]
pub type CTO = crate::Reg<cto::CTO_SPEC>;
#[doc = "USB Chirp Timeout"]
pub mod cto;
#[doc = "HHSRTN register accessor: an alias for `Reg<HHSRTN_SPEC>`"]
pub type HHSRTN = crate::Reg<hhsrtn::HHSRTN_SPEC>;
#[doc = "USB High Speed to UTM Operating Delay"]
pub mod hhsrtn;
#[doc = "HSBT register accessor: an alias for `Reg<HSBT_SPEC>`"]
pub type HSBT = crate::Reg<hsbt::HSBT_SPEC>;
#[doc = "USB High Speed Time-out Adder"]
pub mod hsbt;
#[doc = "LPMATTR register accessor: an alias for `Reg<LPMATTR_SPEC>`"]
pub type LPMATTR = crate::Reg<lpmattr::LPMATTR_SPEC>;
#[doc = "USB LPM Attributes"]
pub mod lpmattr;
#[doc = "LPMCNTRL register accessor: an alias for `Reg<LPMCNTRL_SPEC>`"]
pub type LPMCNTRL = crate::Reg<lpmcntrl::LPMCNTRL_SPEC>;
#[doc = "USB LPM Control"]
pub mod lpmcntrl;
#[doc = "LPMIM register accessor: an alias for `Reg<LPMIM_SPEC>`"]
pub type LPMIM = crate::Reg<lpmim::LPMIM_SPEC>;
#[doc = "USB LPM Interrupt Mask"]
pub mod lpmim;
#[doc = "LPMRIS register accessor: an alias for `Reg<LPMRIS_SPEC>`"]
pub type LPMRIS = crate::Reg<lpmris::LPMRIS_SPEC>;
#[doc = "USB LPM Raw Interrupt Status"]
pub mod lpmris;
#[doc = "LPMFADDR register accessor: an alias for `Reg<LPMFADDR_SPEC>`"]
pub type LPMFADDR = crate::Reg<lpmfaddr::LPMFADDR_SPEC>;
#[doc = "USB LPM Function Address"]
pub mod lpmfaddr;
#[doc = "EPC register accessor: an alias for `Reg<EPC_SPEC>`"]
pub type EPC = crate::Reg<epc::EPC_SPEC>;
#[doc = "USB External Power Control"]
pub mod epc;
#[doc = "EPCRIS register accessor: an alias for `Reg<EPCRIS_SPEC>`"]
pub type EPCRIS = crate::Reg<epcris::EPCRIS_SPEC>;
#[doc = "USB External Power Control Raw Interrupt Status"]
pub mod epcris;
#[doc = "EPCIM register accessor: an alias for `Reg<EPCIM_SPEC>`"]
pub type EPCIM = crate::Reg<epcim::EPCIM_SPEC>;
#[doc = "USB External Power Control Interrupt Mask"]
pub mod epcim;
#[doc = "EPCISC register accessor: an alias for `Reg<EPCISC_SPEC>`"]
pub type EPCISC = crate::Reg<epcisc::EPCISC_SPEC>;
#[doc = "USB External Power Control Interrupt Status and Clear"]
pub mod epcisc;
#[doc = "DRRIS register accessor: an alias for `Reg<DRRIS_SPEC>`"]
pub type DRRIS = crate::Reg<drris::DRRIS_SPEC>;
#[doc = "USB Device RESUME Raw Interrupt Status"]
pub mod drris;
#[doc = "DRIM register accessor: an alias for `Reg<DRIM_SPEC>`"]
pub type DRIM = crate::Reg<drim::DRIM_SPEC>;
#[doc = "USB Device RESUME Interrupt Mask"]
pub mod drim;
#[doc = "DRISC register accessor: an alias for `Reg<DRISC_SPEC>`"]
pub type DRISC = crate::Reg<drisc::DRISC_SPEC>;
#[doc = "USB Device RESUME Interrupt Status and Clear"]
pub mod drisc;
#[doc = "GPCS register accessor: an alias for `Reg<GPCS_SPEC>`"]
pub type GPCS = crate::Reg<gpcs::GPCS_SPEC>;
#[doc = "USB General-Purpose Control and Status"]
pub mod gpcs;
#[doc = "VDC register accessor: an alias for `Reg<VDC_SPEC>`"]
pub type VDC = crate::Reg<vdc::VDC_SPEC>;
#[doc = "USB VBUS Droop Control"]
pub mod vdc;
#[doc = "VDCRIS register accessor: an alias for `Reg<VDCRIS_SPEC>`"]
pub type VDCRIS = crate::Reg<vdcris::VDCRIS_SPEC>;
#[doc = "USB VBUS Droop Control Raw Interrupt Status"]
pub mod vdcris;
#[doc = "VDCIM register accessor: an alias for `Reg<VDCIM_SPEC>`"]
pub type VDCIM = crate::Reg<vdcim::VDCIM_SPEC>;
#[doc = "USB VBUS Droop Control Interrupt Mask"]
pub mod vdcim;
#[doc = "VDCISC register accessor: an alias for `Reg<VDCISC_SPEC>`"]
pub type VDCISC = crate::Reg<vdcisc::VDCISC_SPEC>;
#[doc = "USB VBUS Droop Control Interrupt Status and Clear"]
pub mod vdcisc;
#[doc = "PP register accessor: an alias for `Reg<PP_SPEC>`"]
pub type PP = crate::Reg<pp::PP_SPEC>;
#[doc = "USB Peripheral Properties"]
pub mod pp;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "USB Peripheral Configuration"]
pub mod pc;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "USB Clock Configuration"]
pub mod cc;
