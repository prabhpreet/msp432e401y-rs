#[doc = "Register `PPSCTRL` reader"]
pub struct R(crate::R<PPSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPSCTRL` writer"]
pub struct W(crate::W<PPSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PPSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_PPSCTRL_PPSCTRL_A {
    #[doc = "0: When the PPSEN0 bit = 0x0, the EN0PPS signal is 1 pulse of the PTP reference clock.(of width clk_ptp_i) every second"]
    EMAC_PPSCTRL_PPSCTRL_1HZ = 0,
    #[doc = "1: When the PPSEN0 bit = 0x0, the binary rollover is 2 Hz, and the digital rollover is 1 Hz"]
    EMAC_PPSCTRL_PPSCTRL_2HZ = 1,
    #[doc = "2: When the PPSEN0 bit = 0x0, the binary rollover is 4 Hz, and the digital rollover is 2 Hz"]
    EMAC_PPSCTRL_PPSCTRL_4HZ = 2,
    #[doc = "3: When thePPSEN0 bit = 0x0, the binary rollover is 8 Hz, and the digital rollover is 4 Hz,"]
    EMAC_PPSCTRL_PPSCTRL_8HZ = 3,
    #[doc = "4: When thePPSEN0 bit = 0x0, the binary rollover is 16 Hz, and the digital rollover is 8 Hz"]
    EMAC_PPSCTRL_PPSCTRL_16HZ = 4,
    #[doc = "5: When thePPSEN0 bit = 0x0, the binary rollover is 32 Hz, and the digital rollover is 16 Hz"]
    EMAC_PPSCTRL_PPSCTRL_32HZ = 5,
    #[doc = "6: When thePPSEN0 bit = 0x0, the binary rollover is 64 Hz, and the digital rollover is 32 Hz"]
    EMAC_PPSCTRL_PPSCTRL_64HZ = 6,
    #[doc = "7: When thePPSEN0 bit = 0x0, the binary rollover is 128 Hz, and the digital rollover is 64 Hz"]
    EMAC_PPSCTRL_PPSCTRL_128HZ = 7,
    #[doc = "8: When thePPSEN0 bit = 0x0, the binary rollover is 256 Hz, and the digital rollover is 128 Hz"]
    EMAC_PPSCTRL_PPSCTRL_256HZ = 8,
    #[doc = "9: When thePPSEN0 bit = 0x0, the binary rollover is 512 Hz, and the digital rollover is 256 Hz"]
    EMAC_PPSCTRL_PPSCTRL_512HZ = 9,
    #[doc = "10: When the PPSEN0 bit = 0x0, the binary rollover is 1.024 kHz, and the digital rollover is 512 Hz"]
    EMAC_PPSCTRL_PPSCTRL_1024HZ = 10,
    #[doc = "11: When thePPSEN0 bit = 0x0, the binary rollover is 2.048 kHz, and the digital rollover is 1.024 kHz"]
    EMAC_PPSCTRL_PPSCTRL_2048HZ = 11,
    #[doc = "12: When thePPSEN0 bit = 0x0, the binary rollover is 4.096 kHz, and the digital rollover is 2.048 kHz"]
    EMAC_PPSCTRL_PPSCTRL_4096HZ = 12,
    #[doc = "13: When thePPSEN0 bit = 0x0, the binary rollover is 8.192 kHz, and the digital rollover is 4.096 kHz"]
    EMAC_PPSCTRL_PPSCTRL_8192HZ = 13,
    #[doc = "14: When thePPSEN0 bit = 0x0, the binary rollover is 16.384 kHz, and the digital rollover is 8.092 kHz"]
    EMAC_PPSCTRL_PPSCTRL_16384HZ = 14,
    #[doc = "15: When thePPSEN0 bit = 0x0, the binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz"]
    EMAC_PPSCTRL_PPSCTRL_32768HZ = 15,
}
impl From<EMAC_PPSCTRL_PPSCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_PPSCTRL_PPSCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_PPSCTRL_PPSCTRL` reader - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
pub type EMAC_PPSCTRL_PPSCTRL_R = crate::FieldReader<u8, EMAC_PPSCTRL_PPSCTRL_A>;
impl EMAC_PPSCTRL_PPSCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_PPSCTRL_PPSCTRL_A {
        match self.bits {
            0 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1HZ,
            1 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2HZ,
            2 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4HZ,
            3 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8HZ,
            4 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16HZ,
            5 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32HZ,
            6 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_64HZ,
            7 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_128HZ,
            8 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_256HZ,
            9 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_512HZ,
            10 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1024HZ,
            11 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2048HZ,
            12 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4096HZ,
            13 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8192HZ,
            14 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16384HZ,
            15 => EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32768HZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_1HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_1hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_2HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_2hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_4HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_4hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_8HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_8hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_16HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_16hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_32HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_32hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_64HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_64hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_64HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_128HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_128hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_128HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_256HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_256hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_256HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_512HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_512hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_512HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_1024HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_1024hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1024HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_2048HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_2048hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2048HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_4096HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_4096hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4096HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_8192HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_8192hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8192HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_16384HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_16384hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16384HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_32768HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_32768hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32768HZ
    }
}
#[doc = "Field `EMAC_PPSCTRL_PPSCTRL` writer - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
pub type EMAC_PPSCTRL_PPSCTRL_W<'a> =
    crate::FieldWriterSafe<'a, u32, PPSCTRL_SPEC, u8, EMAC_PPSCTRL_PPSCTRL_A, 4, 0>;
impl<'a> EMAC_PPSCTRL_PPSCTRL_W<'a> {
    #[doc = "When the PPSEN0 bit = 0x0, the EN0PPS signal is 1 pulse of the PTP reference clock.(of width clk_ptp_i) every second"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_1hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 2 Hz, and the digital rollover is 1 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_2hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 4 Hz, and the digital rollover is 2 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_4hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8 Hz, and the digital rollover is 4 Hz,"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_8hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16 Hz, and the digital rollover is 8 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_16hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32 Hz, and the digital rollover is 16 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_32hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 64 Hz, and the digital rollover is 32 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_64hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_64HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 128 Hz, and the digital rollover is 64 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_128hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_128HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 256 Hz, and the digital rollover is 128 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_256hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_256HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 512 Hz, and the digital rollover is 256 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_512hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_512HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 1.024 kHz, and the digital rollover is 512 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_1024hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_1024HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 2.048 kHz, and the digital rollover is 1.024 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_2048hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_2048HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 4.096 kHz, and the digital rollover is 2.048 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_4096hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_4096HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8.192 kHz, and the digital rollover is 4.096 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_8192hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_8192HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16.384 kHz, and the digital rollover is 8.092 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_16384hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_16384HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_32768hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRL_A::EMAC_PPSCTRL_PPSCTRL_32768HZ)
    }
}
#[doc = "Field `EMAC_PPSCTRL_PPSEN0` reader - Flexible PPS Output Mode Enable"]
pub type EMAC_PPSCTRL_PPSEN0_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PPSCTRL_PPSEN0` writer - Flexible PPS Output Mode Enable"]
pub type EMAC_PPSCTRL_PPSEN0_W<'a> = crate::BitWriter<'a, u32, PPSCTRL_SPEC, bool, 4>;
#[doc = "Target Time Register Mode for PPS0 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_PPSCTRL_TRGMODS0_A {
    #[doc = "0: Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    EMAC_PPSCTRL_TRGMODS0_INTONLY = 0,
    #[doc = "2: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    EMAC_PPSCTRL_TRGMODS0_INTPPS0 = 2,
    #[doc = "3: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    EMAC_PPSCTRL_TRGMODS0_PPS0ONLY = 3,
}
impl From<EMAC_PPSCTRL_TRGMODS0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_PPSCTRL_TRGMODS0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_PPSCTRL_TRGMODS0` reader - Target Time Register Mode for PPS0 Output"]
pub type EMAC_PPSCTRL_TRGMODS0_R = crate::FieldReader<u8, EMAC_PPSCTRL_TRGMODS0_A>;
impl EMAC_PPSCTRL_TRGMODS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_PPSCTRL_TRGMODS0_A> {
        match self.bits {
            0 => Some(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTONLY),
            2 => Some(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTPPS0),
            3 => Some(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_INTONLY`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_intonly(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTONLY
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_INTPPS0`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_intpps0(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTPPS0
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_PPS0ONLY`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_pps0only(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY
    }
}
#[doc = "Field `EMAC_PPSCTRL_TRGMODS0` writer - Target Time Register Mode for PPS0 Output"]
pub type EMAC_PPSCTRL_TRGMODS0_W<'a> =
    crate::FieldWriter<'a, u32, PPSCTRL_SPEC, u8, EMAC_PPSCTRL_TRGMODS0_A, 2, 5>;
impl<'a> EMAC_PPSCTRL_TRGMODS0_W<'a> {
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_intonly(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTONLY)
    }
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_intpps0(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_INTPPS0)
    }
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_pps0only(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0_A::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY)
    }
}
impl R {
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl(&self) -> EMAC_PPSCTRL_PPSCTRL_R {
        EMAC_PPSCTRL_PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsen0(&self) -> EMAC_PPSCTRL_PPSEN0_R {
        EMAC_PPSCTRL_PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0(&self) -> EMAC_PPSCTRL_TRGMODS0_R {
        EMAC_PPSCTRL_TRGMODS0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl(&mut self) -> EMAC_PPSCTRL_PPSCTRL_W {
        EMAC_PPSCTRL_PPSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsen0(&mut self) -> EMAC_PPSCTRL_PPSEN0_W {
        EMAC_PPSCTRL_PPSEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0(&mut self) -> EMAC_PPSCTRL_TRGMODS0_W {
        EMAC_PPSCTRL_TRGMODS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PPS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppsctrl](index.html) module"]
pub struct PPSCTRL_SPEC;
impl crate::RegisterSpec for PPSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppsctrl::R](R) reader structure"]
impl crate::Readable for PPSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppsctrl::W](W) writer structure"]
impl crate::Writable for PPSCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPSCTRL to value 0"]
impl crate::Resettable for PPSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
