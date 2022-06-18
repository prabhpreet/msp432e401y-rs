#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: crate::Reg<did0::DID0_SPEC>,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: crate::Reg<did1::DID1_SPEC>,
    _reserved2: [u8; 0x30],
    #[doc = "0x38 - Power-Temp Brown Out Control"]
    pub ptboctl: crate::Reg<ptboctl::PTBOCTL_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x50 - Raw Interrupt Status"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x54 - Interrupt Mask Control"]
    pub imc: crate::Reg<imc::IMC_SPEC>,
    #[doc = "0x58 - Masked Interrupt Status and Clear"]
    pub misc: crate::Reg<misc::MISC_SPEC>,
    #[doc = "0x5c - Reset Cause"]
    pub resc: crate::Reg<resc::RESC_SPEC>,
    #[doc = "0x60 - Power-Temperature Cause"]
    pub pwrtc: crate::Reg<pwrtc::PWRTC_SPEC>,
    #[doc = "0x64 - NMI Cause Register"]
    pub nmic: crate::Reg<nmic::NMIC_SPEC>,
    _reserved9: [u8; 0x14],
    #[doc = "0x7c - Main Oscillator Control"]
    pub moscctl: crate::Reg<moscctl::MOSCCTL_SPEC>,
    _reserved10: [u8; 0x30],
    #[doc = "0xb0 - Run and Sleep Mode Configuration Register"]
    pub rsclkcfg: crate::Reg<rsclkcfg::RSCLKCFG_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0xc0 - Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
    pub memtim0: crate::Reg<memtim0::MEMTIM0_SPEC>,
    _reserved12: [u8; 0x74],
    #[doc = "0x138 - Alternate Clock Configuration"]
    pub altclkcfg: crate::Reg<altclkcfg::ALTCLKCFG_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x144 - Deep Sleep Clock Configuration Register"]
    pub dsclkcfg: crate::Reg<dsclkcfg::DSCLKCFG_SPEC>,
    #[doc = "0x148 - Divisor and Source Clock Configuration"]
    pub divsclk: crate::Reg<divsclk::DIVSCLK_SPEC>,
    #[doc = "0x14c - System Properties"]
    pub sysprop: crate::Reg<sysprop::SYSPROP_SPEC>,
    #[doc = "0x150 - Precision Internal Oscillator Calibration"]
    pub piosccal: crate::Reg<piosccal::PIOSCCAL_SPEC>,
    #[doc = "0x154 - Precision Internal Oscillator Statistics"]
    pub pioscstat: crate::Reg<pioscstat::PIOSCSTAT_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0x160 - PLL Frequency 0"]
    pub pllfreq0: crate::Reg<pllfreq0::PLLFREQ0_SPEC>,
    #[doc = "0x164 - PLL Frequency 1"]
    pub pllfreq1: crate::Reg<pllfreq1::PLLFREQ1_SPEC>,
    #[doc = "0x168 - PLL Status"]
    pub pllstat: crate::Reg<pllstat::PLLSTAT_SPEC>,
    _reserved21: [u8; 0x1c],
    #[doc = "0x188 - Sleep Power Configuration"]
    pub slppwrcfg: crate::Reg<slppwrcfg::SLPPWRCFG_SPEC>,
    #[doc = "0x18c - Deep-Sleep Power Configuration"]
    pub dslppwrcfg: crate::Reg<dslppwrcfg::DSLPPWRCFG_SPEC>,
    _reserved23: [u8; 0x10],
    #[doc = "0x1a0 - Non-Volatile Memory Information"]
    pub nvmstat: crate::Reg<nvmstat::NVMSTAT_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x1b4 - LDO Sleep Power Control"]
    pub ldospctl: crate::Reg<ldospctl::LDOSPCTL_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x1bc - LDO Deep-Sleep Power Control"]
    pub ldodpctl: crate::Reg<ldodpctl::LDODPCTL_SPEC>,
    _reserved26: [u8; 0x18],
    #[doc = "0x1d8 - Reset Behavior Control Register"]
    pub resbehavctl: crate::Reg<resbehavctl::RESBEHAVCTL_SPEC>,
    _reserved27: [u8; 0x18],
    #[doc = "0x1f4 - Hardware System Service Request"]
    pub hssr: crate::Reg<hssr::HSSR_SPEC>,
    _reserved28: [u8; 0x88],
    #[doc = "0x280 - USB Power Domain Status"]
    pub usbpds: crate::Reg<usbpds::USBPDS_SPEC>,
    #[doc = "0x284 - USB Memory Power Control"]
    pub usbmpc: crate::Reg<usbmpc::USBMPC_SPEC>,
    #[doc = "0x288 - Ethernet MAC Power Domain Status"]
    pub emacpds: crate::Reg<emacpds::EMACPDS_SPEC>,
    #[doc = "0x28c - Ethernet MAC Memory Power Control"]
    pub emacmpc: crate::Reg<emacmpc::EMACMPC_SPEC>,
    _reserved32: [u8; 0x70],
    #[doc = "0x300 - Watchdog Timer Peripheral Present"]
    pub ppwd: crate::Reg<ppwd::PPWD_SPEC>,
    #[doc = "0x304 - 16/32-Bit General-Purpose Timer Peripheral Present"]
    pub pptimer: crate::Reg<pptimer::PPTIMER_SPEC>,
    #[doc = "0x308 - General-Purpose Input/Output Peripheral Present"]
    pub ppgpio: crate::Reg<ppgpio::PPGPIO_SPEC>,
    #[doc = "0x30c - Micro Direct Memory Access Peripheral Present"]
    pub ppdma: crate::Reg<ppdma::PPDMA_SPEC>,
    #[doc = "0x310 - EPI Peripheral Present"]
    pub ppepi: crate::Reg<ppepi::PPEPI_SPEC>,
    #[doc = "0x314 - Hibernation Peripheral Present"]
    pub pphib: crate::Reg<pphib::PPHIB_SPEC>,
    #[doc = "0x318 - Universal Asynchronous Receiver/Transmitter Peripheral Present"]
    pub ppuart: crate::Reg<ppuart::PPUART_SPEC>,
    #[doc = "0x31c - Synchronous Serial Interface Peripheral Present"]
    pub ppssi: crate::Reg<ppssi::PPSSI_SPEC>,
    #[doc = "0x320 - Inter-Integrated Circuit Peripheral Present"]
    pub ppi2c: crate::Reg<ppi2c::PPI2C_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x328 - Universal Serial Bus Peripheral Present"]
    pub ppusb: crate::Reg<ppusb::PPUSB_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x330 - Ethernet PHY Peripheral Present"]
    pub ppephy: crate::Reg<ppephy::PPEPHY_SPEC>,
    #[doc = "0x334 - Controller Area Network Peripheral Present"]
    pub ppcan: crate::Reg<ppcan::PPCAN_SPEC>,
    #[doc = "0x338 - Analog-to-Digital Converter Peripheral Present"]
    pub ppadc: crate::Reg<ppadc::PPADC_SPEC>,
    #[doc = "0x33c - Analog Comparator Peripheral Present"]
    pub ppacmp: crate::Reg<ppacmp::PPACMP_SPEC>,
    #[doc = "0x340 - Pulse Width Modulator Peripheral Present"]
    pub pppwm: crate::Reg<pppwm::PPPWM_SPEC>,
    #[doc = "0x344 - Quadrature Encoder Interface Peripheral Present"]
    pub ppqei: crate::Reg<ppqei::PPQEI_SPEC>,
    _reserved48: [u8; 0x10],
    #[doc = "0x358 - EEPROM Peripheral Present"]
    pub ppeeprom: crate::Reg<ppeeprom::PPEEPROM_SPEC>,
    _reserved49: [u8; 0x18],
    #[doc = "0x374 - CRC and Cryptographic Modules Peripheral Present"]
    pub ppccm: crate::Reg<ppccm::PPCCM_SPEC>,
    _reserved50: [u8; 0x18],
    #[doc = "0x390 - LCD Peripheral Present"]
    pub pplcd: crate::Reg<pplcd::PPLCD_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x398 - 1-Wire Peripheral Present"]
    pub ppowire: crate::Reg<ppowire::PPOWIRE_SPEC>,
    #[doc = "0x39c - Ethernet MAC Peripheral Present"]
    pub ppemac: crate::Reg<ppemac::PPEMAC_SPEC>,
    _reserved53: [u8; 0x0160],
    #[doc = "0x500 - Watchdog Timer Software Reset"]
    pub srwd: crate::Reg<srwd::SRWD_SPEC>,
    #[doc = "0x504 - 16/32-Bit General-Purpose Timer Software Reset"]
    pub srtimer: crate::Reg<srtimer::SRTIMER_SPEC>,
    #[doc = "0x508 - General-Purpose Input/Output Software Reset"]
    pub srgpio: crate::Reg<srgpio::SRGPIO_SPEC>,
    #[doc = "0x50c - Micro Direct Memory Access Software Reset"]
    pub srdma: crate::Reg<srdma::SRDMA_SPEC>,
    #[doc = "0x510 - EPI Software Reset"]
    pub srepi: crate::Reg<srepi::SREPI_SPEC>,
    #[doc = "0x514 - Hibernation Software Reset"]
    pub srhib: crate::Reg<srhib::SRHIB_SPEC>,
    #[doc = "0x518 - Universal Asynchronous Receiver/Transmitter Software Reset"]
    pub sruart: crate::Reg<sruart::SRUART_SPEC>,
    #[doc = "0x51c - Synchronous Serial Interface Software Reset"]
    pub srssi: crate::Reg<srssi::SRSSI_SPEC>,
    #[doc = "0x520 - Inter-Integrated Circuit Software Reset"]
    pub sri2c: crate::Reg<sri2c::SRI2C_SPEC>,
    _reserved62: [u8; 0x04],
    #[doc = "0x528 - Universal Serial Bus Software Reset"]
    pub srusb: crate::Reg<srusb::SRUSB_SPEC>,
    _reserved63: [u8; 0x04],
    #[doc = "0x530 - Ethernet PHY Software Reset"]
    pub srephy: crate::Reg<srephy::SREPHY_SPEC>,
    #[doc = "0x534 - Controller Area Network Software Reset"]
    pub srcan: crate::Reg<srcan::SRCAN_SPEC>,
    #[doc = "0x538 - Analog-to-Digital Converter Software Reset"]
    pub sradc: crate::Reg<sradc::SRADC_SPEC>,
    #[doc = "0x53c - Analog Comparator Software Reset"]
    pub sracmp: crate::Reg<sracmp::SRACMP_SPEC>,
    #[doc = "0x540 - Pulse Width Modulator Software Reset"]
    pub srpwm: crate::Reg<srpwm::SRPWM_SPEC>,
    #[doc = "0x544 - Quadrature Encoder Interface Software Reset"]
    pub srqei: crate::Reg<srqei::SRQEI_SPEC>,
    _reserved69: [u8; 0x10],
    #[doc = "0x558 - EEPROM Software Reset"]
    pub sreeprom: crate::Reg<sreeprom::SREEPROM_SPEC>,
    _reserved70: [u8; 0x18],
    #[doc = "0x574 - CRC and Cryptographic Modules Software Reset"]
    pub srccm: crate::Reg<srccm::SRCCM_SPEC>,
    _reserved71: [u8; 0x24],
    #[doc = "0x59c - Ethernet MAC Software Reset"]
    pub sremac: crate::Reg<sremac::SREMAC_SPEC>,
    _reserved72: [u8; 0x60],
    #[doc = "0x600 - Watchdog Timer Run Mode Clock Gating Control"]
    pub rcgcwd: crate::Reg<rcgcwd::RCGCWD_SPEC>,
    #[doc = "0x604 - 16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
    pub rcgctimer: crate::Reg<rcgctimer::RCGCTIMER_SPEC>,
    #[doc = "0x608 - General-Purpose Input/Output Run Mode Clock Gating Control"]
    pub rcgcgpio: crate::Reg<rcgcgpio::RCGCGPIO_SPEC>,
    #[doc = "0x60c - Micro Direct Memory Access Run Mode Clock Gating Control"]
    pub rcgcdma: crate::Reg<rcgcdma::RCGCDMA_SPEC>,
    #[doc = "0x610 - EPI Run Mode Clock Gating Control"]
    pub rcgcepi: crate::Reg<rcgcepi::RCGCEPI_SPEC>,
    #[doc = "0x614 - Hibernation Run Mode Clock Gating Control"]
    pub rcgchib: crate::Reg<rcgchib::RCGCHIB_SPEC>,
    #[doc = "0x618 - Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
    pub rcgcuart: crate::Reg<rcgcuart::RCGCUART_SPEC>,
    #[doc = "0x61c - Synchronous Serial Interface Run Mode Clock Gating Control"]
    pub rcgcssi: crate::Reg<rcgcssi::RCGCSSI_SPEC>,
    #[doc = "0x620 - Inter-Integrated Circuit Run Mode Clock Gating Control"]
    pub rcgci2c: crate::Reg<rcgci2c::RCGCI2C_SPEC>,
    _reserved81: [u8; 0x04],
    #[doc = "0x628 - Universal Serial Bus Run Mode Clock Gating Control"]
    pub rcgcusb: crate::Reg<rcgcusb::RCGCUSB_SPEC>,
    _reserved82: [u8; 0x04],
    #[doc = "0x630 - Ethernet PHY Run Mode Clock Gating Control"]
    pub rcgcephy: crate::Reg<rcgcephy::RCGCEPHY_SPEC>,
    #[doc = "0x634 - Controller Area Network Run Mode Clock Gating Control"]
    pub rcgccan: crate::Reg<rcgccan::RCGCCAN_SPEC>,
    #[doc = "0x638 - Analog-to-Digital Converter Run Mode Clock Gating Control"]
    pub rcgcadc: crate::Reg<rcgcadc::RCGCADC_SPEC>,
    #[doc = "0x63c - Analog Comparator Run Mode Clock Gating Control"]
    pub rcgcacmp: crate::Reg<rcgcacmp::RCGCACMP_SPEC>,
    #[doc = "0x640 - Pulse Width Modulator Run Mode Clock Gating Control"]
    pub rcgcpwm: crate::Reg<rcgcpwm::RCGCPWM_SPEC>,
    #[doc = "0x644 - Quadrature Encoder Interface Run Mode Clock Gating Control"]
    pub rcgcqei: crate::Reg<rcgcqei::RCGCQEI_SPEC>,
    _reserved88: [u8; 0x10],
    #[doc = "0x658 - EEPROM Run Mode Clock Gating Control"]
    pub rcgceeprom: crate::Reg<rcgceeprom::RCGCEEPROM_SPEC>,
    _reserved89: [u8; 0x18],
    #[doc = "0x674 - CRC and Cryptographic Modules Run Mode Clock Gating Control"]
    pub rcgcccm: crate::Reg<rcgcccm::RCGCCCM_SPEC>,
    _reserved90: [u8; 0x24],
    #[doc = "0x69c - Ethernet MAC Run Mode Clock Gating Control"]
    pub rcgcemac: crate::Reg<rcgcemac::RCGCEMAC_SPEC>,
    _reserved91: [u8; 0x60],
    #[doc = "0x700 - Watchdog Timer Sleep Mode Clock Gating Control"]
    pub scgcwd: crate::Reg<scgcwd::SCGCWD_SPEC>,
    #[doc = "0x704 - 16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
    pub scgctimer: crate::Reg<scgctimer::SCGCTIMER_SPEC>,
    #[doc = "0x708 - General-Purpose Input/Output Sleep Mode Clock Gating Control"]
    pub scgcgpio: crate::Reg<scgcgpio::SCGCGPIO_SPEC>,
    #[doc = "0x70c - Micro Direct Memory Access Sleep Mode Clock Gating Control"]
    pub scgcdma: crate::Reg<scgcdma::SCGCDMA_SPEC>,
    #[doc = "0x710 - EPI Sleep Mode Clock Gating Control"]
    pub scgcepi: crate::Reg<scgcepi::SCGCEPI_SPEC>,
    #[doc = "0x714 - Hibernation Sleep Mode Clock Gating Control"]
    pub scgchib: crate::Reg<scgchib::SCGCHIB_SPEC>,
    #[doc = "0x718 - Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
    pub scgcuart: crate::Reg<scgcuart::SCGCUART_SPEC>,
    #[doc = "0x71c - Synchronous Serial Interface Sleep Mode Clock Gating Control"]
    pub scgcssi: crate::Reg<scgcssi::SCGCSSI_SPEC>,
    #[doc = "0x720 - Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
    pub scgci2c: crate::Reg<scgci2c::SCGCI2C_SPEC>,
    _reserved100: [u8; 0x04],
    #[doc = "0x728 - Universal Serial Bus Sleep Mode Clock Gating Control"]
    pub scgcusb: crate::Reg<scgcusb::SCGCUSB_SPEC>,
    _reserved101: [u8; 0x04],
    #[doc = "0x730 - Ethernet PHY Sleep Mode Clock Gating Control"]
    pub scgcephy: crate::Reg<scgcephy::SCGCEPHY_SPEC>,
    #[doc = "0x734 - Controller Area Network Sleep Mode Clock Gating Control"]
    pub scgccan: crate::Reg<scgccan::SCGCCAN_SPEC>,
    #[doc = "0x738 - Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
    pub scgcadc: crate::Reg<scgcadc::SCGCADC_SPEC>,
    #[doc = "0x73c - Analog Comparator Sleep Mode Clock Gating Control"]
    pub scgcacmp: crate::Reg<scgcacmp::SCGCACMP_SPEC>,
    #[doc = "0x740 - Pulse Width Modulator Sleep Mode Clock Gating Control"]
    pub scgcpwm: crate::Reg<scgcpwm::SCGCPWM_SPEC>,
    #[doc = "0x744 - Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
    pub scgcqei: crate::Reg<scgcqei::SCGCQEI_SPEC>,
    _reserved107: [u8; 0x10],
    #[doc = "0x758 - EEPROM Sleep Mode Clock Gating Control"]
    pub scgceeprom: crate::Reg<scgceeprom::SCGCEEPROM_SPEC>,
    _reserved108: [u8; 0x18],
    #[doc = "0x774 - CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
    pub scgcccm: crate::Reg<scgcccm::SCGCCCM_SPEC>,
    _reserved109: [u8; 0x24],
    #[doc = "0x79c - Ethernet MAC Sleep Mode Clock Gating Control"]
    pub scgcemac: crate::Reg<scgcemac::SCGCEMAC_SPEC>,
    _reserved110: [u8; 0x60],
    #[doc = "0x800 - Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgcwd: crate::Reg<dcgcwd::DCGCWD_SPEC>,
    #[doc = "0x804 - 16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
    pub dcgctimer: crate::Reg<dcgctimer::DCGCTIMER_SPEC>,
    #[doc = "0x808 - General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
    pub dcgcgpio: crate::Reg<dcgcgpio::DCGCGPIO_SPEC>,
    #[doc = "0x80c - Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
    pub dcgcdma: crate::Reg<dcgcdma::DCGCDMA_SPEC>,
    #[doc = "0x810 - EPI Deep-Sleep Mode Clock Gating Control"]
    pub dcgcepi: crate::Reg<dcgcepi::DCGCEPI_SPEC>,
    #[doc = "0x814 - Hibernation Deep-Sleep Mode Clock Gating Control"]
    pub dcgchib: crate::Reg<dcgchib::DCGCHIB_SPEC>,
    #[doc = "0x818 - Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcuart: crate::Reg<dcgcuart::DCGCUART_SPEC>,
    #[doc = "0x81c - Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcssi: crate::Reg<dcgcssi::DCGCSSI_SPEC>,
    #[doc = "0x820 - Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
    pub dcgci2c: crate::Reg<dcgci2c::DCGCI2C_SPEC>,
    _reserved119: [u8; 0x04],
    #[doc = "0x828 - Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
    pub dcgcusb: crate::Reg<dcgcusb::DCGCUSB_SPEC>,
    _reserved120: [u8; 0x04],
    #[doc = "0x830 - Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
    pub dcgcephy: crate::Reg<dcgcephy::DCGCEPHY_SPEC>,
    #[doc = "0x834 - Controller Area Network Deep-Sleep Mode Clock Gating Control"]
    pub dcgccan: crate::Reg<dcgccan::DCGCCAN_SPEC>,
    #[doc = "0x838 - Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
    pub dcgcadc: crate::Reg<dcgcadc::DCGCADC_SPEC>,
    #[doc = "0x83c - Analog Comparator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcacmp: crate::Reg<dcgcacmp::DCGCACMP_SPEC>,
    #[doc = "0x840 - Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
    pub dcgcpwm: crate::Reg<dcgcpwm::DCGCPWM_SPEC>,
    #[doc = "0x844 - Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
    pub dcgcqei: crate::Reg<dcgcqei::DCGCQEI_SPEC>,
    _reserved126: [u8; 0x10],
    #[doc = "0x858 - EEPROM Deep-Sleep Mode Clock Gating Control"]
    pub dcgceeprom: crate::Reg<dcgceeprom::DCGCEEPROM_SPEC>,
    _reserved127: [u8; 0x18],
    #[doc = "0x874 - CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
    pub dcgcccm: crate::Reg<dcgcccm::DCGCCCM_SPEC>,
    _reserved128: [u8; 0x24],
    #[doc = "0x89c - Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
    pub dcgcemac: crate::Reg<dcgcemac::DCGCEMAC_SPEC>,
    _reserved129: [u8; 0x60],
    #[doc = "0x900 - Watchdog Timer Power Control"]
    pub pcwd: crate::Reg<pcwd::PCWD_SPEC>,
    #[doc = "0x904 - 16/32-Bit General-Purpose Timer Power Control"]
    pub pctimer: crate::Reg<pctimer::PCTIMER_SPEC>,
    #[doc = "0x908 - General-Purpose Input/Output Power Control"]
    pub pcgpio: crate::Reg<pcgpio::PCGPIO_SPEC>,
    #[doc = "0x90c - Micro Direct Memory Access Power Control"]
    pub pcdma: crate::Reg<pcdma::PCDMA_SPEC>,
    #[doc = "0x910 - External Peripheral Interface Power Control"]
    pub pcepi: crate::Reg<pcepi::PCEPI_SPEC>,
    #[doc = "0x914 - Hibernation Power Control"]
    pub pchib: crate::Reg<pchib::PCHIB_SPEC>,
    #[doc = "0x918 - Universal Asynchronous Receiver/Transmitter Power Control"]
    pub pcuart: crate::Reg<pcuart::PCUART_SPEC>,
    #[doc = "0x91c - Synchronous Serial Interface Power Control"]
    pub pcssi: crate::Reg<pcssi::PCSSI_SPEC>,
    #[doc = "0x920 - Inter-Integrated Circuit Power Control"]
    pub pci2c: crate::Reg<pci2c::PCI2C_SPEC>,
    _reserved138: [u8; 0x04],
    #[doc = "0x928 - Universal Serial Bus Power Control"]
    pub pcusb: crate::Reg<pcusb::PCUSB_SPEC>,
    _reserved139: [u8; 0x04],
    #[doc = "0x930 - Ethernet PHY Power Control"]
    pub pcephy: crate::Reg<pcephy::PCEPHY_SPEC>,
    #[doc = "0x934 - Controller Area Network Power Control"]
    pub pccan: crate::Reg<pccan::PCCAN_SPEC>,
    #[doc = "0x938 - Analog-to-Digital Converter Power Control"]
    pub pcadc: crate::Reg<pcadc::PCADC_SPEC>,
    #[doc = "0x93c - Analog Comparator Power Control"]
    pub pcacmp: crate::Reg<pcacmp::PCACMP_SPEC>,
    #[doc = "0x940 - Pulse Width Modulator Power Control"]
    pub pcpwm: crate::Reg<pcpwm::PCPWM_SPEC>,
    #[doc = "0x944 - Quadrature Encoder Interface Power Control"]
    pub pcqei: crate::Reg<pcqei::PCQEI_SPEC>,
    _reserved145: [u8; 0x10],
    #[doc = "0x958 - EEPROM Power Control"]
    pub pceeprom: crate::Reg<pceeprom::PCEEPROM_SPEC>,
    _reserved146: [u8; 0x18],
    #[doc = "0x974 - CRC and Cryptographic Modules Power Control"]
    pub pcccm: crate::Reg<pcccm::PCCCM_SPEC>,
    _reserved147: [u8; 0x24],
    #[doc = "0x99c - Ethernet MAC Power Control"]
    pub pcemac: crate::Reg<pcemac::PCEMAC_SPEC>,
    _reserved148: [u8; 0x60],
    #[doc = "0xa00 - Watchdog Timer Peripheral Ready"]
    pub prwd: crate::Reg<prwd::PRWD_SPEC>,
    #[doc = "0xa04 - 16/32-Bit General-Purpose Timer Peripheral Ready"]
    pub prtimer: crate::Reg<prtimer::PRTIMER_SPEC>,
    #[doc = "0xa08 - General-Purpose Input/Output Peripheral Ready"]
    pub prgpio: crate::Reg<prgpio::PRGPIO_SPEC>,
    #[doc = "0xa0c - Micro Direct Memory Access Peripheral Ready"]
    pub prdma: crate::Reg<prdma::PRDMA_SPEC>,
    #[doc = "0xa10 - EPI Peripheral Ready"]
    pub prepi: crate::Reg<prepi::PREPI_SPEC>,
    #[doc = "0xa14 - Hibernation Peripheral Ready"]
    pub prhib: crate::Reg<prhib::PRHIB_SPEC>,
    #[doc = "0xa18 - Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
    pub pruart: crate::Reg<pruart::PRUART_SPEC>,
    #[doc = "0xa1c - Synchronous Serial Interface Peripheral Ready"]
    pub prssi: crate::Reg<prssi::PRSSI_SPEC>,
    #[doc = "0xa20 - Inter-Integrated Circuit Peripheral Ready"]
    pub pri2c: crate::Reg<pri2c::PRI2C_SPEC>,
    _reserved157: [u8; 0x04],
    #[doc = "0xa28 - Universal Serial Bus Peripheral Ready"]
    pub prusb: crate::Reg<prusb::PRUSB_SPEC>,
    _reserved158: [u8; 0x04],
    #[doc = "0xa30 - Ethernet PHY Peripheral Ready"]
    pub prephy: crate::Reg<prephy::PREPHY_SPEC>,
    #[doc = "0xa34 - Controller Area Network Peripheral Ready"]
    pub prcan: crate::Reg<prcan::PRCAN_SPEC>,
    #[doc = "0xa38 - Analog-to-Digital Converter Peripheral Ready"]
    pub pradc: crate::Reg<pradc::PRADC_SPEC>,
    #[doc = "0xa3c - Analog Comparator Peripheral Ready"]
    pub pracmp: crate::Reg<pracmp::PRACMP_SPEC>,
    #[doc = "0xa40 - Pulse Width Modulator Peripheral Ready"]
    pub prpwm: crate::Reg<prpwm::PRPWM_SPEC>,
    #[doc = "0xa44 - Quadrature Encoder Interface Peripheral Ready"]
    pub prqei: crate::Reg<prqei::PRQEI_SPEC>,
    _reserved164: [u8; 0x10],
    #[doc = "0xa58 - EEPROM Peripheral Ready"]
    pub preeprom: crate::Reg<preeprom::PREEPROM_SPEC>,
    _reserved165: [u8; 0x18],
    #[doc = "0xa74 - CRC and Cryptographic Modules Peripheral Ready"]
    pub prccm: crate::Reg<prccm::PRCCM_SPEC>,
    _reserved166: [u8; 0x24],
    #[doc = "0xa9c - Ethernet MAC Peripheral Ready"]
    pub premac: crate::Reg<premac::PREMAC_SPEC>,
    _reserved167: [u8; 0x0480],
    #[doc = "0xf20 - Unique ID 0"]
    pub uniqueid0: crate::Reg<uniqueid0::UNIQUEID0_SPEC>,
    #[doc = "0xf24 - Unique ID 1"]
    pub uniqueid1: crate::Reg<uniqueid1::UNIQUEID1_SPEC>,
    #[doc = "0xf28 - Unique ID 2"]
    pub uniqueid2: crate::Reg<uniqueid2::UNIQUEID2_SPEC>,
    #[doc = "0xf2c - Unique ID 3"]
    pub uniqueid3: crate::Reg<uniqueid3::UNIQUEID3_SPEC>,
}
#[doc = "DID0 register accessor: an alias for `Reg<DID0_SPEC>`"]
pub type DID0 = crate::Reg<did0::DID0_SPEC>;
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "DID1 register accessor: an alias for `Reg<DID1_SPEC>`"]
pub type DID1 = crate::Reg<did1::DID1_SPEC>;
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "PTBOCTL register accessor: an alias for `Reg<PTBOCTL_SPEC>`"]
pub type PTBOCTL = crate::Reg<ptboctl::PTBOCTL_SPEC>;
#[doc = "Power-Temp Brown Out Control"]
pub mod ptboctl;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "IMC register accessor: an alias for `Reg<IMC_SPEC>`"]
pub type IMC = crate::Reg<imc::IMC_SPEC>;
#[doc = "Interrupt Mask Control"]
pub mod imc;
#[doc = "MISC register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Masked Interrupt Status and Clear"]
pub mod misc;
#[doc = "RESC register accessor: an alias for `Reg<RESC_SPEC>`"]
pub type RESC = crate::Reg<resc::RESC_SPEC>;
#[doc = "Reset Cause"]
pub mod resc;
#[doc = "PWRTC register accessor: an alias for `Reg<PWRTC_SPEC>`"]
pub type PWRTC = crate::Reg<pwrtc::PWRTC_SPEC>;
#[doc = "Power-Temperature Cause"]
pub mod pwrtc;
#[doc = "NMIC register accessor: an alias for `Reg<NMIC_SPEC>`"]
pub type NMIC = crate::Reg<nmic::NMIC_SPEC>;
#[doc = "NMI Cause Register"]
pub mod nmic;
#[doc = "MOSCCTL register accessor: an alias for `Reg<MOSCCTL_SPEC>`"]
pub type MOSCCTL = crate::Reg<moscctl::MOSCCTL_SPEC>;
#[doc = "Main Oscillator Control"]
pub mod moscctl;
#[doc = "RSCLKCFG register accessor: an alias for `Reg<RSCLKCFG_SPEC>`"]
pub type RSCLKCFG = crate::Reg<rsclkcfg::RSCLKCFG_SPEC>;
#[doc = "Run and Sleep Mode Configuration Register"]
pub mod rsclkcfg;
#[doc = "MEMTIM0 register accessor: an alias for `Reg<MEMTIM0_SPEC>`"]
pub type MEMTIM0 = crate::Reg<memtim0::MEMTIM0_SPEC>;
#[doc = "Memory Timing Parameter Register 0 for Main Flash and EEPROM"]
pub mod memtim0;
#[doc = "ALTCLKCFG register accessor: an alias for `Reg<ALTCLKCFG_SPEC>`"]
pub type ALTCLKCFG = crate::Reg<altclkcfg::ALTCLKCFG_SPEC>;
#[doc = "Alternate Clock Configuration"]
pub mod altclkcfg;
#[doc = "DSCLKCFG register accessor: an alias for `Reg<DSCLKCFG_SPEC>`"]
pub type DSCLKCFG = crate::Reg<dsclkcfg::DSCLKCFG_SPEC>;
#[doc = "Deep Sleep Clock Configuration Register"]
pub mod dsclkcfg;
#[doc = "DIVSCLK register accessor: an alias for `Reg<DIVSCLK_SPEC>`"]
pub type DIVSCLK = crate::Reg<divsclk::DIVSCLK_SPEC>;
#[doc = "Divisor and Source Clock Configuration"]
pub mod divsclk;
#[doc = "SYSPROP register accessor: an alias for `Reg<SYSPROP_SPEC>`"]
pub type SYSPROP = crate::Reg<sysprop::SYSPROP_SPEC>;
#[doc = "System Properties"]
pub mod sysprop;
#[doc = "PIOSCCAL register accessor: an alias for `Reg<PIOSCCAL_SPEC>`"]
pub type PIOSCCAL = crate::Reg<piosccal::PIOSCCAL_SPEC>;
#[doc = "Precision Internal Oscillator Calibration"]
pub mod piosccal;
#[doc = "PIOSCSTAT register accessor: an alias for `Reg<PIOSCSTAT_SPEC>`"]
pub type PIOSCSTAT = crate::Reg<pioscstat::PIOSCSTAT_SPEC>;
#[doc = "Precision Internal Oscillator Statistics"]
pub mod pioscstat;
#[doc = "PLLFREQ0 register accessor: an alias for `Reg<PLLFREQ0_SPEC>`"]
pub type PLLFREQ0 = crate::Reg<pllfreq0::PLLFREQ0_SPEC>;
#[doc = "PLL Frequency 0"]
pub mod pllfreq0;
#[doc = "PLLFREQ1 register accessor: an alias for `Reg<PLLFREQ1_SPEC>`"]
pub type PLLFREQ1 = crate::Reg<pllfreq1::PLLFREQ1_SPEC>;
#[doc = "PLL Frequency 1"]
pub mod pllfreq1;
#[doc = "PLLSTAT register accessor: an alias for `Reg<PLLSTAT_SPEC>`"]
pub type PLLSTAT = crate::Reg<pllstat::PLLSTAT_SPEC>;
#[doc = "PLL Status"]
pub mod pllstat;
#[doc = "SLPPWRCFG register accessor: an alias for `Reg<SLPPWRCFG_SPEC>`"]
pub type SLPPWRCFG = crate::Reg<slppwrcfg::SLPPWRCFG_SPEC>;
#[doc = "Sleep Power Configuration"]
pub mod slppwrcfg;
#[doc = "DSLPPWRCFG register accessor: an alias for `Reg<DSLPPWRCFG_SPEC>`"]
pub type DSLPPWRCFG = crate::Reg<dslppwrcfg::DSLPPWRCFG_SPEC>;
#[doc = "Deep-Sleep Power Configuration"]
pub mod dslppwrcfg;
#[doc = "NVMSTAT register accessor: an alias for `Reg<NVMSTAT_SPEC>`"]
pub type NVMSTAT = crate::Reg<nvmstat::NVMSTAT_SPEC>;
#[doc = "Non-Volatile Memory Information"]
pub mod nvmstat;
#[doc = "LDOSPCTL register accessor: an alias for `Reg<LDOSPCTL_SPEC>`"]
pub type LDOSPCTL = crate::Reg<ldospctl::LDOSPCTL_SPEC>;
#[doc = "LDO Sleep Power Control"]
pub mod ldospctl;
#[doc = "LDODPCTL register accessor: an alias for `Reg<LDODPCTL_SPEC>`"]
pub type LDODPCTL = crate::Reg<ldodpctl::LDODPCTL_SPEC>;
#[doc = "LDO Deep-Sleep Power Control"]
pub mod ldodpctl;
#[doc = "RESBEHAVCTL register accessor: an alias for `Reg<RESBEHAVCTL_SPEC>`"]
pub type RESBEHAVCTL = crate::Reg<resbehavctl::RESBEHAVCTL_SPEC>;
#[doc = "Reset Behavior Control Register"]
pub mod resbehavctl;
#[doc = "HSSR register accessor: an alias for `Reg<HSSR_SPEC>`"]
pub type HSSR = crate::Reg<hssr::HSSR_SPEC>;
#[doc = "Hardware System Service Request"]
pub mod hssr;
#[doc = "USBPDS register accessor: an alias for `Reg<USBPDS_SPEC>`"]
pub type USBPDS = crate::Reg<usbpds::USBPDS_SPEC>;
#[doc = "USB Power Domain Status"]
pub mod usbpds;
#[doc = "USBMPC register accessor: an alias for `Reg<USBMPC_SPEC>`"]
pub type USBMPC = crate::Reg<usbmpc::USBMPC_SPEC>;
#[doc = "USB Memory Power Control"]
pub mod usbmpc;
#[doc = "EMACPDS register accessor: an alias for `Reg<EMACPDS_SPEC>`"]
pub type EMACPDS = crate::Reg<emacpds::EMACPDS_SPEC>;
#[doc = "Ethernet MAC Power Domain Status"]
pub mod emacpds;
#[doc = "EMACMPC register accessor: an alias for `Reg<EMACMPC_SPEC>`"]
pub type EMACMPC = crate::Reg<emacmpc::EMACMPC_SPEC>;
#[doc = "Ethernet MAC Memory Power Control"]
pub mod emacmpc;
#[doc = "PPWD register accessor: an alias for `Reg<PPWD_SPEC>`"]
pub type PPWD = crate::Reg<ppwd::PPWD_SPEC>;
#[doc = "Watchdog Timer Peripheral Present"]
pub mod ppwd;
#[doc = "PPTIMER register accessor: an alias for `Reg<PPTIMER_SPEC>`"]
pub type PPTIMER = crate::Reg<pptimer::PPTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Present"]
pub mod pptimer;
#[doc = "PPGPIO register accessor: an alias for `Reg<PPGPIO_SPEC>`"]
pub type PPGPIO = crate::Reg<ppgpio::PPGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Peripheral Present"]
pub mod ppgpio;
#[doc = "PPDMA register accessor: an alias for `Reg<PPDMA_SPEC>`"]
pub type PPDMA = crate::Reg<ppdma::PPDMA_SPEC>;
#[doc = "Micro Direct Memory Access Peripheral Present"]
pub mod ppdma;
#[doc = "PPEPI register accessor: an alias for `Reg<PPEPI_SPEC>`"]
pub type PPEPI = crate::Reg<ppepi::PPEPI_SPEC>;
#[doc = "EPI Peripheral Present"]
pub mod ppepi;
#[doc = "PPHIB register accessor: an alias for `Reg<PPHIB_SPEC>`"]
pub type PPHIB = crate::Reg<pphib::PPHIB_SPEC>;
#[doc = "Hibernation Peripheral Present"]
pub mod pphib;
#[doc = "PPUART register accessor: an alias for `Reg<PPUART_SPEC>`"]
pub type PPUART = crate::Reg<ppuart::PPUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Present"]
pub mod ppuart;
#[doc = "PPSSI register accessor: an alias for `Reg<PPSSI_SPEC>`"]
pub type PPSSI = crate::Reg<ppssi::PPSSI_SPEC>;
#[doc = "Synchronous Serial Interface Peripheral Present"]
pub mod ppssi;
#[doc = "PPI2C register accessor: an alias for `Reg<PPI2C_SPEC>`"]
pub type PPI2C = crate::Reg<ppi2c::PPI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Peripheral Present"]
pub mod ppi2c;
#[doc = "PPUSB register accessor: an alias for `Reg<PPUSB_SPEC>`"]
pub type PPUSB = crate::Reg<ppusb::PPUSB_SPEC>;
#[doc = "Universal Serial Bus Peripheral Present"]
pub mod ppusb;
#[doc = "PPEPHY register accessor: an alias for `Reg<PPEPHY_SPEC>`"]
pub type PPEPHY = crate::Reg<ppephy::PPEPHY_SPEC>;
#[doc = "Ethernet PHY Peripheral Present"]
pub mod ppephy;
#[doc = "PPCAN register accessor: an alias for `Reg<PPCAN_SPEC>`"]
pub type PPCAN = crate::Reg<ppcan::PPCAN_SPEC>;
#[doc = "Controller Area Network Peripheral Present"]
pub mod ppcan;
#[doc = "PPADC register accessor: an alias for `Reg<PPADC_SPEC>`"]
pub type PPADC = crate::Reg<ppadc::PPADC_SPEC>;
#[doc = "Analog-to-Digital Converter Peripheral Present"]
pub mod ppadc;
#[doc = "PPACMP register accessor: an alias for `Reg<PPACMP_SPEC>`"]
pub type PPACMP = crate::Reg<ppacmp::PPACMP_SPEC>;
#[doc = "Analog Comparator Peripheral Present"]
pub mod ppacmp;
#[doc = "PPPWM register accessor: an alias for `Reg<PPPWM_SPEC>`"]
pub type PPPWM = crate::Reg<pppwm::PPPWM_SPEC>;
#[doc = "Pulse Width Modulator Peripheral Present"]
pub mod pppwm;
#[doc = "PPQEI register accessor: an alias for `Reg<PPQEI_SPEC>`"]
pub type PPQEI = crate::Reg<ppqei::PPQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Peripheral Present"]
pub mod ppqei;
#[doc = "PPEEPROM register accessor: an alias for `Reg<PPEEPROM_SPEC>`"]
pub type PPEEPROM = crate::Reg<ppeeprom::PPEEPROM_SPEC>;
#[doc = "EEPROM Peripheral Present"]
pub mod ppeeprom;
#[doc = "PPCCM register accessor: an alias for `Reg<PPCCM_SPEC>`"]
pub type PPCCM = crate::Reg<ppccm::PPCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Peripheral Present"]
pub mod ppccm;
#[doc = "PPLCD register accessor: an alias for `Reg<PPLCD_SPEC>`"]
pub type PPLCD = crate::Reg<pplcd::PPLCD_SPEC>;
#[doc = "LCD Peripheral Present"]
pub mod pplcd;
#[doc = "PPOWIRE register accessor: an alias for `Reg<PPOWIRE_SPEC>`"]
pub type PPOWIRE = crate::Reg<ppowire::PPOWIRE_SPEC>;
#[doc = "1-Wire Peripheral Present"]
pub mod ppowire;
#[doc = "PPEMAC register accessor: an alias for `Reg<PPEMAC_SPEC>`"]
pub type PPEMAC = crate::Reg<ppemac::PPEMAC_SPEC>;
#[doc = "Ethernet MAC Peripheral Present"]
pub mod ppemac;
#[doc = "SRWD register accessor: an alias for `Reg<SRWD_SPEC>`"]
pub type SRWD = crate::Reg<srwd::SRWD_SPEC>;
#[doc = "Watchdog Timer Software Reset"]
pub mod srwd;
#[doc = "SRTIMER register accessor: an alias for `Reg<SRTIMER_SPEC>`"]
pub type SRTIMER = crate::Reg<srtimer::SRTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Software Reset"]
pub mod srtimer;
#[doc = "SRGPIO register accessor: an alias for `Reg<SRGPIO_SPEC>`"]
pub type SRGPIO = crate::Reg<srgpio::SRGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Software Reset"]
pub mod srgpio;
#[doc = "SRDMA register accessor: an alias for `Reg<SRDMA_SPEC>`"]
pub type SRDMA = crate::Reg<srdma::SRDMA_SPEC>;
#[doc = "Micro Direct Memory Access Software Reset"]
pub mod srdma;
#[doc = "SREPI register accessor: an alias for `Reg<SREPI_SPEC>`"]
pub type SREPI = crate::Reg<srepi::SREPI_SPEC>;
#[doc = "EPI Software Reset"]
pub mod srepi;
#[doc = "SRHIB register accessor: an alias for `Reg<SRHIB_SPEC>`"]
pub type SRHIB = crate::Reg<srhib::SRHIB_SPEC>;
#[doc = "Hibernation Software Reset"]
pub mod srhib;
#[doc = "SRUART register accessor: an alias for `Reg<SRUART_SPEC>`"]
pub type SRUART = crate::Reg<sruart::SRUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Software Reset"]
pub mod sruart;
#[doc = "SRSSI register accessor: an alias for `Reg<SRSSI_SPEC>`"]
pub type SRSSI = crate::Reg<srssi::SRSSI_SPEC>;
#[doc = "Synchronous Serial Interface Software Reset"]
pub mod srssi;
#[doc = "SRI2C register accessor: an alias for `Reg<SRI2C_SPEC>`"]
pub type SRI2C = crate::Reg<sri2c::SRI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Software Reset"]
pub mod sri2c;
#[doc = "SRUSB register accessor: an alias for `Reg<SRUSB_SPEC>`"]
pub type SRUSB = crate::Reg<srusb::SRUSB_SPEC>;
#[doc = "Universal Serial Bus Software Reset"]
pub mod srusb;
#[doc = "SREPHY register accessor: an alias for `Reg<SREPHY_SPEC>`"]
pub type SREPHY = crate::Reg<srephy::SREPHY_SPEC>;
#[doc = "Ethernet PHY Software Reset"]
pub mod srephy;
#[doc = "SRCAN register accessor: an alias for `Reg<SRCAN_SPEC>`"]
pub type SRCAN = crate::Reg<srcan::SRCAN_SPEC>;
#[doc = "Controller Area Network Software Reset"]
pub mod srcan;
#[doc = "SRADC register accessor: an alias for `Reg<SRADC_SPEC>`"]
pub type SRADC = crate::Reg<sradc::SRADC_SPEC>;
#[doc = "Analog-to-Digital Converter Software Reset"]
pub mod sradc;
#[doc = "SRACMP register accessor: an alias for `Reg<SRACMP_SPEC>`"]
pub type SRACMP = crate::Reg<sracmp::SRACMP_SPEC>;
#[doc = "Analog Comparator Software Reset"]
pub mod sracmp;
#[doc = "SRPWM register accessor: an alias for `Reg<SRPWM_SPEC>`"]
pub type SRPWM = crate::Reg<srpwm::SRPWM_SPEC>;
#[doc = "Pulse Width Modulator Software Reset"]
pub mod srpwm;
#[doc = "SRQEI register accessor: an alias for `Reg<SRQEI_SPEC>`"]
pub type SRQEI = crate::Reg<srqei::SRQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Software Reset"]
pub mod srqei;
#[doc = "SREEPROM register accessor: an alias for `Reg<SREEPROM_SPEC>`"]
pub type SREEPROM = crate::Reg<sreeprom::SREEPROM_SPEC>;
#[doc = "EEPROM Software Reset"]
pub mod sreeprom;
#[doc = "SRCCM register accessor: an alias for `Reg<SRCCM_SPEC>`"]
pub type SRCCM = crate::Reg<srccm::SRCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Software Reset"]
pub mod srccm;
#[doc = "SREMAC register accessor: an alias for `Reg<SREMAC_SPEC>`"]
pub type SREMAC = crate::Reg<sremac::SREMAC_SPEC>;
#[doc = "Ethernet MAC Software Reset"]
pub mod sremac;
#[doc = "RCGCWD register accessor: an alias for `Reg<RCGCWD_SPEC>`"]
pub type RCGCWD = crate::Reg<rcgcwd::RCGCWD_SPEC>;
#[doc = "Watchdog Timer Run Mode Clock Gating Control"]
pub mod rcgcwd;
#[doc = "RCGCTIMER register accessor: an alias for `Reg<RCGCTIMER_SPEC>`"]
pub type RCGCTIMER = crate::Reg<rcgctimer::RCGCTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Run Mode Clock Gating Control"]
pub mod rcgctimer;
#[doc = "RCGCGPIO register accessor: an alias for `Reg<RCGCGPIO_SPEC>`"]
pub type RCGCGPIO = crate::Reg<rcgcgpio::RCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Run Mode Clock Gating Control"]
pub mod rcgcgpio;
#[doc = "RCGCDMA register accessor: an alias for `Reg<RCGCDMA_SPEC>`"]
pub type RCGCDMA = crate::Reg<rcgcdma::RCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Run Mode Clock Gating Control"]
pub mod rcgcdma;
#[doc = "RCGCEPI register accessor: an alias for `Reg<RCGCEPI_SPEC>`"]
pub type RCGCEPI = crate::Reg<rcgcepi::RCGCEPI_SPEC>;
#[doc = "EPI Run Mode Clock Gating Control"]
pub mod rcgcepi;
#[doc = "RCGCHIB register accessor: an alias for `Reg<RCGCHIB_SPEC>`"]
pub type RCGCHIB = crate::Reg<rcgchib::RCGCHIB_SPEC>;
#[doc = "Hibernation Run Mode Clock Gating Control"]
pub mod rcgchib;
#[doc = "RCGCUART register accessor: an alias for `Reg<RCGCUART_SPEC>`"]
pub type RCGCUART = crate::Reg<rcgcuart::RCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Run Mode Clock Gating Control"]
pub mod rcgcuart;
#[doc = "RCGCSSI register accessor: an alias for `Reg<RCGCSSI_SPEC>`"]
pub type RCGCSSI = crate::Reg<rcgcssi::RCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Run Mode Clock Gating Control"]
pub mod rcgcssi;
#[doc = "RCGCI2C register accessor: an alias for `Reg<RCGCI2C_SPEC>`"]
pub type RCGCI2C = crate::Reg<rcgci2c::RCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Run Mode Clock Gating Control"]
pub mod rcgci2c;
#[doc = "RCGCUSB register accessor: an alias for `Reg<RCGCUSB_SPEC>`"]
pub type RCGCUSB = crate::Reg<rcgcusb::RCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Run Mode Clock Gating Control"]
pub mod rcgcusb;
#[doc = "RCGCEPHY register accessor: an alias for `Reg<RCGCEPHY_SPEC>`"]
pub type RCGCEPHY = crate::Reg<rcgcephy::RCGCEPHY_SPEC>;
#[doc = "Ethernet PHY Run Mode Clock Gating Control"]
pub mod rcgcephy;
#[doc = "RCGCCAN register accessor: an alias for `Reg<RCGCCAN_SPEC>`"]
pub type RCGCCAN = crate::Reg<rcgccan::RCGCCAN_SPEC>;
#[doc = "Controller Area Network Run Mode Clock Gating Control"]
pub mod rcgccan;
#[doc = "RCGCADC register accessor: an alias for `Reg<RCGCADC_SPEC>`"]
pub type RCGCADC = crate::Reg<rcgcadc::RCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Run Mode Clock Gating Control"]
pub mod rcgcadc;
#[doc = "RCGCACMP register accessor: an alias for `Reg<RCGCACMP_SPEC>`"]
pub type RCGCACMP = crate::Reg<rcgcacmp::RCGCACMP_SPEC>;
#[doc = "Analog Comparator Run Mode Clock Gating Control"]
pub mod rcgcacmp;
#[doc = "RCGCPWM register accessor: an alias for `Reg<RCGCPWM_SPEC>`"]
pub type RCGCPWM = crate::Reg<rcgcpwm::RCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Run Mode Clock Gating Control"]
pub mod rcgcpwm;
#[doc = "RCGCQEI register accessor: an alias for `Reg<RCGCQEI_SPEC>`"]
pub type RCGCQEI = crate::Reg<rcgcqei::RCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Run Mode Clock Gating Control"]
pub mod rcgcqei;
#[doc = "RCGCEEPROM register accessor: an alias for `Reg<RCGCEEPROM_SPEC>`"]
pub type RCGCEEPROM = crate::Reg<rcgceeprom::RCGCEEPROM_SPEC>;
#[doc = "EEPROM Run Mode Clock Gating Control"]
pub mod rcgceeprom;
#[doc = "RCGCCCM register accessor: an alias for `Reg<RCGCCCM_SPEC>`"]
pub type RCGCCCM = crate::Reg<rcgcccm::RCGCCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Run Mode Clock Gating Control"]
pub mod rcgcccm;
#[doc = "RCGCEMAC register accessor: an alias for `Reg<RCGCEMAC_SPEC>`"]
pub type RCGCEMAC = crate::Reg<rcgcemac::RCGCEMAC_SPEC>;
#[doc = "Ethernet MAC Run Mode Clock Gating Control"]
pub mod rcgcemac;
#[doc = "SCGCWD register accessor: an alias for `Reg<SCGCWD_SPEC>`"]
pub type SCGCWD = crate::Reg<scgcwd::SCGCWD_SPEC>;
#[doc = "Watchdog Timer Sleep Mode Clock Gating Control"]
pub mod scgcwd;
#[doc = "SCGCTIMER register accessor: an alias for `Reg<SCGCTIMER_SPEC>`"]
pub type SCGCTIMER = crate::Reg<scgctimer::SCGCTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Sleep Mode Clock Gating Control"]
pub mod scgctimer;
#[doc = "SCGCGPIO register accessor: an alias for `Reg<SCGCGPIO_SPEC>`"]
pub type SCGCGPIO = crate::Reg<scgcgpio::SCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Sleep Mode Clock Gating Control"]
pub mod scgcgpio;
#[doc = "SCGCDMA register accessor: an alias for `Reg<SCGCDMA_SPEC>`"]
pub type SCGCDMA = crate::Reg<scgcdma::SCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Sleep Mode Clock Gating Control"]
pub mod scgcdma;
#[doc = "SCGCEPI register accessor: an alias for `Reg<SCGCEPI_SPEC>`"]
pub type SCGCEPI = crate::Reg<scgcepi::SCGCEPI_SPEC>;
#[doc = "EPI Sleep Mode Clock Gating Control"]
pub mod scgcepi;
#[doc = "SCGCHIB register accessor: an alias for `Reg<SCGCHIB_SPEC>`"]
pub type SCGCHIB = crate::Reg<scgchib::SCGCHIB_SPEC>;
#[doc = "Hibernation Sleep Mode Clock Gating Control"]
pub mod scgchib;
#[doc = "SCGCUART register accessor: an alias for `Reg<SCGCUART_SPEC>`"]
pub type SCGCUART = crate::Reg<scgcuart::SCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Sleep Mode Clock Gating Control"]
pub mod scgcuart;
#[doc = "SCGCSSI register accessor: an alias for `Reg<SCGCSSI_SPEC>`"]
pub type SCGCSSI = crate::Reg<scgcssi::SCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Sleep Mode Clock Gating Control"]
pub mod scgcssi;
#[doc = "SCGCI2C register accessor: an alias for `Reg<SCGCI2C_SPEC>`"]
pub type SCGCI2C = crate::Reg<scgci2c::SCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Sleep Mode Clock Gating Control"]
pub mod scgci2c;
#[doc = "SCGCUSB register accessor: an alias for `Reg<SCGCUSB_SPEC>`"]
pub type SCGCUSB = crate::Reg<scgcusb::SCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control"]
pub mod scgcusb;
#[doc = "SCGCEPHY register accessor: an alias for `Reg<SCGCEPHY_SPEC>`"]
pub type SCGCEPHY = crate::Reg<scgcephy::SCGCEPHY_SPEC>;
#[doc = "Ethernet PHY Sleep Mode Clock Gating Control"]
pub mod scgcephy;
#[doc = "SCGCCAN register accessor: an alias for `Reg<SCGCCAN_SPEC>`"]
pub type SCGCCAN = crate::Reg<scgccan::SCGCCAN_SPEC>;
#[doc = "Controller Area Network Sleep Mode Clock Gating Control"]
pub mod scgccan;
#[doc = "SCGCADC register accessor: an alias for `Reg<SCGCADC_SPEC>`"]
pub type SCGCADC = crate::Reg<scgcadc::SCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control"]
pub mod scgcadc;
#[doc = "SCGCACMP register accessor: an alias for `Reg<SCGCACMP_SPEC>`"]
pub type SCGCACMP = crate::Reg<scgcacmp::SCGCACMP_SPEC>;
#[doc = "Analog Comparator Sleep Mode Clock Gating Control"]
pub mod scgcacmp;
#[doc = "SCGCPWM register accessor: an alias for `Reg<SCGCPWM_SPEC>`"]
pub type SCGCPWM = crate::Reg<scgcpwm::SCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Sleep Mode Clock Gating Control"]
pub mod scgcpwm;
#[doc = "SCGCQEI register accessor: an alias for `Reg<SCGCQEI_SPEC>`"]
pub type SCGCQEI = crate::Reg<scgcqei::SCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Sleep Mode Clock Gating Control"]
pub mod scgcqei;
#[doc = "SCGCEEPROM register accessor: an alias for `Reg<SCGCEEPROM_SPEC>`"]
pub type SCGCEEPROM = crate::Reg<scgceeprom::SCGCEEPROM_SPEC>;
#[doc = "EEPROM Sleep Mode Clock Gating Control"]
pub mod scgceeprom;
#[doc = "SCGCCCM register accessor: an alias for `Reg<SCGCCCM_SPEC>`"]
pub type SCGCCCM = crate::Reg<scgcccm::SCGCCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Sleep Mode Clock Gating Control"]
pub mod scgcccm;
#[doc = "SCGCEMAC register accessor: an alias for `Reg<SCGCEMAC_SPEC>`"]
pub type SCGCEMAC = crate::Reg<scgcemac::SCGCEMAC_SPEC>;
#[doc = "Ethernet MAC Sleep Mode Clock Gating Control"]
pub mod scgcemac;
#[doc = "DCGCWD register accessor: an alias for `Reg<DCGCWD_SPEC>`"]
pub type DCGCWD = crate::Reg<dcgcwd::DCGCWD_SPEC>;
#[doc = "Watchdog Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcwd;
#[doc = "DCGCTIMER register accessor: an alias for `Reg<DCGCTIMER_SPEC>`"]
pub type DCGCTIMER = crate::Reg<dcgctimer::DCGCTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Deep-Sleep Mode Clock Gating Control"]
pub mod dcgctimer;
#[doc = "DCGCGPIO register accessor: an alias for `Reg<DCGCGPIO_SPEC>`"]
pub type DCGCGPIO = crate::Reg<dcgcgpio::DCGCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcgpio;
#[doc = "DCGCDMA register accessor: an alias for `Reg<DCGCDMA_SPEC>`"]
pub type DCGCDMA = crate::Reg<dcgcdma::DCGCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcdma;
#[doc = "DCGCEPI register accessor: an alias for `Reg<DCGCEPI_SPEC>`"]
pub type DCGCEPI = crate::Reg<dcgcepi::DCGCEPI_SPEC>;
#[doc = "EPI Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcepi;
#[doc = "DCGCHIB register accessor: an alias for `Reg<DCGCHIB_SPEC>`"]
pub type DCGCHIB = crate::Reg<dcgchib::DCGCHIB_SPEC>;
#[doc = "Hibernation Deep-Sleep Mode Clock Gating Control"]
pub mod dcgchib;
#[doc = "DCGCUART register accessor: an alias for `Reg<DCGCUART_SPEC>`"]
pub type DCGCUART = crate::Reg<dcgcuart::DCGCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcuart;
#[doc = "DCGCSSI register accessor: an alias for `Reg<DCGCSSI_SPEC>`"]
pub type DCGCSSI = crate::Reg<dcgcssi::DCGCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcssi;
#[doc = "DCGCI2C register accessor: an alias for `Reg<DCGCI2C_SPEC>`"]
pub type DCGCI2C = crate::Reg<dcgci2c::DCGCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Deep-Sleep Mode Clock Gating Control"]
pub mod dcgci2c;
#[doc = "DCGCUSB register accessor: an alias for `Reg<DCGCUSB_SPEC>`"]
pub type DCGCUSB = crate::Reg<dcgcusb::DCGCUSB_SPEC>;
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcusb;
#[doc = "DCGCEPHY register accessor: an alias for `Reg<DCGCEPHY_SPEC>`"]
pub type DCGCEPHY = crate::Reg<dcgcephy::DCGCEPHY_SPEC>;
#[doc = "Ethernet PHY Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcephy;
#[doc = "DCGCCAN register accessor: an alias for `Reg<DCGCCAN_SPEC>`"]
pub type DCGCCAN = crate::Reg<dcgccan::DCGCCAN_SPEC>;
#[doc = "Controller Area Network Deep-Sleep Mode Clock Gating Control"]
pub mod dcgccan;
#[doc = "DCGCADC register accessor: an alias for `Reg<DCGCADC_SPEC>`"]
pub type DCGCADC = crate::Reg<dcgcadc::DCGCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcadc;
#[doc = "DCGCACMP register accessor: an alias for `Reg<DCGCACMP_SPEC>`"]
pub type DCGCACMP = crate::Reg<dcgcacmp::DCGCACMP_SPEC>;
#[doc = "Analog Comparator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcacmp;
#[doc = "DCGCPWM register accessor: an alias for `Reg<DCGCPWM_SPEC>`"]
pub type DCGCPWM = crate::Reg<dcgcpwm::DCGCPWM_SPEC>;
#[doc = "Pulse Width Modulator Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcpwm;
#[doc = "DCGCQEI register accessor: an alias for `Reg<DCGCQEI_SPEC>`"]
pub type DCGCQEI = crate::Reg<dcgcqei::DCGCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcqei;
#[doc = "DCGCEEPROM register accessor: an alias for `Reg<DCGCEEPROM_SPEC>`"]
pub type DCGCEEPROM = crate::Reg<dcgceeprom::DCGCEEPROM_SPEC>;
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control"]
pub mod dcgceeprom;
#[doc = "DCGCCCM register accessor: an alias for `Reg<DCGCCCM_SPEC>`"]
pub type DCGCCCM = crate::Reg<dcgcccm::DCGCCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcccm;
#[doc = "DCGCEMAC register accessor: an alias for `Reg<DCGCEMAC_SPEC>`"]
pub type DCGCEMAC = crate::Reg<dcgcemac::DCGCEMAC_SPEC>;
#[doc = "Ethernet MAC Deep-Sleep Mode Clock Gating Control"]
pub mod dcgcemac;
#[doc = "PCWD register accessor: an alias for `Reg<PCWD_SPEC>`"]
pub type PCWD = crate::Reg<pcwd::PCWD_SPEC>;
#[doc = "Watchdog Timer Power Control"]
pub mod pcwd;
#[doc = "PCTIMER register accessor: an alias for `Reg<PCTIMER_SPEC>`"]
pub type PCTIMER = crate::Reg<pctimer::PCTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Power Control"]
pub mod pctimer;
#[doc = "PCGPIO register accessor: an alias for `Reg<PCGPIO_SPEC>`"]
pub type PCGPIO = crate::Reg<pcgpio::PCGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Power Control"]
pub mod pcgpio;
#[doc = "PCDMA register accessor: an alias for `Reg<PCDMA_SPEC>`"]
pub type PCDMA = crate::Reg<pcdma::PCDMA_SPEC>;
#[doc = "Micro Direct Memory Access Power Control"]
pub mod pcdma;
#[doc = "PCEPI register accessor: an alias for `Reg<PCEPI_SPEC>`"]
pub type PCEPI = crate::Reg<pcepi::PCEPI_SPEC>;
#[doc = "External Peripheral Interface Power Control"]
pub mod pcepi;
#[doc = "PCHIB register accessor: an alias for `Reg<PCHIB_SPEC>`"]
pub type PCHIB = crate::Reg<pchib::PCHIB_SPEC>;
#[doc = "Hibernation Power Control"]
pub mod pchib;
#[doc = "PCUART register accessor: an alias for `Reg<PCUART_SPEC>`"]
pub type PCUART = crate::Reg<pcuart::PCUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Power Control"]
pub mod pcuart;
#[doc = "PCSSI register accessor: an alias for `Reg<PCSSI_SPEC>`"]
pub type PCSSI = crate::Reg<pcssi::PCSSI_SPEC>;
#[doc = "Synchronous Serial Interface Power Control"]
pub mod pcssi;
#[doc = "PCI2C register accessor: an alias for `Reg<PCI2C_SPEC>`"]
pub type PCI2C = crate::Reg<pci2c::PCI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Power Control"]
pub mod pci2c;
#[doc = "PCUSB register accessor: an alias for `Reg<PCUSB_SPEC>`"]
pub type PCUSB = crate::Reg<pcusb::PCUSB_SPEC>;
#[doc = "Universal Serial Bus Power Control"]
pub mod pcusb;
#[doc = "PCEPHY register accessor: an alias for `Reg<PCEPHY_SPEC>`"]
pub type PCEPHY = crate::Reg<pcephy::PCEPHY_SPEC>;
#[doc = "Ethernet PHY Power Control"]
pub mod pcephy;
#[doc = "PCCAN register accessor: an alias for `Reg<PCCAN_SPEC>`"]
pub type PCCAN = crate::Reg<pccan::PCCAN_SPEC>;
#[doc = "Controller Area Network Power Control"]
pub mod pccan;
#[doc = "PCADC register accessor: an alias for `Reg<PCADC_SPEC>`"]
pub type PCADC = crate::Reg<pcadc::PCADC_SPEC>;
#[doc = "Analog-to-Digital Converter Power Control"]
pub mod pcadc;
#[doc = "PCACMP register accessor: an alias for `Reg<PCACMP_SPEC>`"]
pub type PCACMP = crate::Reg<pcacmp::PCACMP_SPEC>;
#[doc = "Analog Comparator Power Control"]
pub mod pcacmp;
#[doc = "PCPWM register accessor: an alias for `Reg<PCPWM_SPEC>`"]
pub type PCPWM = crate::Reg<pcpwm::PCPWM_SPEC>;
#[doc = "Pulse Width Modulator Power Control"]
pub mod pcpwm;
#[doc = "PCQEI register accessor: an alias for `Reg<PCQEI_SPEC>`"]
pub type PCQEI = crate::Reg<pcqei::PCQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Power Control"]
pub mod pcqei;
#[doc = "PCEEPROM register accessor: an alias for `Reg<PCEEPROM_SPEC>`"]
pub type PCEEPROM = crate::Reg<pceeprom::PCEEPROM_SPEC>;
#[doc = "EEPROM Power Control"]
pub mod pceeprom;
#[doc = "PCCCM register accessor: an alias for `Reg<PCCCM_SPEC>`"]
pub type PCCCM = crate::Reg<pcccm::PCCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Power Control"]
pub mod pcccm;
#[doc = "PCEMAC register accessor: an alias for `Reg<PCEMAC_SPEC>`"]
pub type PCEMAC = crate::Reg<pcemac::PCEMAC_SPEC>;
#[doc = "Ethernet MAC Power Control"]
pub mod pcemac;
#[doc = "PRWD register accessor: an alias for `Reg<PRWD_SPEC>`"]
pub type PRWD = crate::Reg<prwd::PRWD_SPEC>;
#[doc = "Watchdog Timer Peripheral Ready"]
pub mod prwd;
#[doc = "PRTIMER register accessor: an alias for `Reg<PRTIMER_SPEC>`"]
pub type PRTIMER = crate::Reg<prtimer::PRTIMER_SPEC>;
#[doc = "16/32-Bit General-Purpose Timer Peripheral Ready"]
pub mod prtimer;
#[doc = "PRGPIO register accessor: an alias for `Reg<PRGPIO_SPEC>`"]
pub type PRGPIO = crate::Reg<prgpio::PRGPIO_SPEC>;
#[doc = "General-Purpose Input/Output Peripheral Ready"]
pub mod prgpio;
#[doc = "PRDMA register accessor: an alias for `Reg<PRDMA_SPEC>`"]
pub type PRDMA = crate::Reg<prdma::PRDMA_SPEC>;
#[doc = "Micro Direct Memory Access Peripheral Ready"]
pub mod prdma;
#[doc = "PREPI register accessor: an alias for `Reg<PREPI_SPEC>`"]
pub type PREPI = crate::Reg<prepi::PREPI_SPEC>;
#[doc = "EPI Peripheral Ready"]
pub mod prepi;
#[doc = "PRHIB register accessor: an alias for `Reg<PRHIB_SPEC>`"]
pub type PRHIB = crate::Reg<prhib::PRHIB_SPEC>;
#[doc = "Hibernation Peripheral Ready"]
pub mod prhib;
#[doc = "PRUART register accessor: an alias for `Reg<PRUART_SPEC>`"]
pub type PRUART = crate::Reg<pruart::PRUART_SPEC>;
#[doc = "Universal Asynchronous Receiver/Transmitter Peripheral Ready"]
pub mod pruart;
#[doc = "PRSSI register accessor: an alias for `Reg<PRSSI_SPEC>`"]
pub type PRSSI = crate::Reg<prssi::PRSSI_SPEC>;
#[doc = "Synchronous Serial Interface Peripheral Ready"]
pub mod prssi;
#[doc = "PRI2C register accessor: an alias for `Reg<PRI2C_SPEC>`"]
pub type PRI2C = crate::Reg<pri2c::PRI2C_SPEC>;
#[doc = "Inter-Integrated Circuit Peripheral Ready"]
pub mod pri2c;
#[doc = "PRUSB register accessor: an alias for `Reg<PRUSB_SPEC>`"]
pub type PRUSB = crate::Reg<prusb::PRUSB_SPEC>;
#[doc = "Universal Serial Bus Peripheral Ready"]
pub mod prusb;
#[doc = "PREPHY register accessor: an alias for `Reg<PREPHY_SPEC>`"]
pub type PREPHY = crate::Reg<prephy::PREPHY_SPEC>;
#[doc = "Ethernet PHY Peripheral Ready"]
pub mod prephy;
#[doc = "PRCAN register accessor: an alias for `Reg<PRCAN_SPEC>`"]
pub type PRCAN = crate::Reg<prcan::PRCAN_SPEC>;
#[doc = "Controller Area Network Peripheral Ready"]
pub mod prcan;
#[doc = "PRADC register accessor: an alias for `Reg<PRADC_SPEC>`"]
pub type PRADC = crate::Reg<pradc::PRADC_SPEC>;
#[doc = "Analog-to-Digital Converter Peripheral Ready"]
pub mod pradc;
#[doc = "PRACMP register accessor: an alias for `Reg<PRACMP_SPEC>`"]
pub type PRACMP = crate::Reg<pracmp::PRACMP_SPEC>;
#[doc = "Analog Comparator Peripheral Ready"]
pub mod pracmp;
#[doc = "PRPWM register accessor: an alias for `Reg<PRPWM_SPEC>`"]
pub type PRPWM = crate::Reg<prpwm::PRPWM_SPEC>;
#[doc = "Pulse Width Modulator Peripheral Ready"]
pub mod prpwm;
#[doc = "PRQEI register accessor: an alias for `Reg<PRQEI_SPEC>`"]
pub type PRQEI = crate::Reg<prqei::PRQEI_SPEC>;
#[doc = "Quadrature Encoder Interface Peripheral Ready"]
pub mod prqei;
#[doc = "PREEPROM register accessor: an alias for `Reg<PREEPROM_SPEC>`"]
pub type PREEPROM = crate::Reg<preeprom::PREEPROM_SPEC>;
#[doc = "EEPROM Peripheral Ready"]
pub mod preeprom;
#[doc = "PRCCM register accessor: an alias for `Reg<PRCCM_SPEC>`"]
pub type PRCCM = crate::Reg<prccm::PRCCM_SPEC>;
#[doc = "CRC and Cryptographic Modules Peripheral Ready"]
pub mod prccm;
#[doc = "PREMAC register accessor: an alias for `Reg<PREMAC_SPEC>`"]
pub type PREMAC = crate::Reg<premac::PREMAC_SPEC>;
#[doc = "Ethernet MAC Peripheral Ready"]
pub mod premac;
#[doc = "UNIQUEID0 register accessor: an alias for `Reg<UNIQUEID0_SPEC>`"]
pub type UNIQUEID0 = crate::Reg<uniqueid0::UNIQUEID0_SPEC>;
#[doc = "Unique ID 0"]
pub mod uniqueid0;
#[doc = "UNIQUEID1 register accessor: an alias for `Reg<UNIQUEID1_SPEC>`"]
pub type UNIQUEID1 = crate::Reg<uniqueid1::UNIQUEID1_SPEC>;
#[doc = "Unique ID 1"]
pub mod uniqueid1;
#[doc = "UNIQUEID2 register accessor: an alias for `Reg<UNIQUEID2_SPEC>`"]
pub type UNIQUEID2 = crate::Reg<uniqueid2::UNIQUEID2_SPEC>;
#[doc = "Unique ID 2"]
pub mod uniqueid2;
#[doc = "UNIQUEID3 register accessor: an alias for `Reg<UNIQUEID3_SPEC>`"]
pub type UNIQUEID3 = crate::Reg<uniqueid3::UNIQUEID3_SPEC>;
#[doc = "Unique ID 3"]
pub mod uniqueid3;
