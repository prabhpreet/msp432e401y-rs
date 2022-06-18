#[doc = "Register `HB16TIME` reader"]
pub struct R(crate::R<EPI_ALT16_HB16TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPI_ALT16_HB16TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPI_ALT16_HB16TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPI_ALT16_HB16TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB16TIME` writer"]
pub struct W(crate::W<EPI_ALT16_HB16TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPI_ALT16_HB16TIME_SPEC>;
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
impl From<crate::W<EPI_ALT16_HB16TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPI_ALT16_HB16TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_HB16TIME_RDWSM` reader - Read Wait State Minus One"]
pub type EPI_HB16TIME_RDWSM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16TIME_RDWSM` writer - Read Wait State Minus One"]
pub type EPI_HB16TIME_RDWSM_W<'a> = crate::BitWriter<'a, u32, EPI_ALT16_HB16TIME_SPEC, bool, 0>;
#[doc = "Field `EPI_HB16TIME_WRWSM` reader - Write Wait State Minus One"]
pub type EPI_HB16TIME_WRWSM_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16TIME_WRWSM` writer - Write Wait State Minus One"]
pub type EPI_HB16TIME_WRWSM_W<'a> = crate::BitWriter<'a, u32, EPI_ALT16_HB16TIME_SPEC, bool, 4>;
#[doc = "Field `EPI_HB16TIME_CAPWIDTH` reader - CS0n Inter-transfer Capture Width"]
pub type EPI_HB16TIME_CAPWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_HB16TIME_CAPWIDTH` writer - CS0n Inter-transfer Capture Width"]
pub type EPI_HB16TIME_CAPWIDTH_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALT16_HB16TIME_SPEC, u8, u8, 2, 12>;
#[doc = "PSRAM Row Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB16TIME_PSRAMSZ_A {
    #[doc = "0: No row size limitation"]
    EPI_HB16TIME_PSRAMSZ_0 = 0,
    #[doc = "1: 128 B"]
    EPI_HB16TIME_PSRAMSZ_128B = 1,
    #[doc = "2: 256 B"]
    EPI_HB16TIME_PSRAMSZ_256B = 2,
    #[doc = "3: 512 B"]
    EPI_HB16TIME_PSRAMSZ_512B = 3,
    #[doc = "4: 1024 B"]
    EPI_HB16TIME_PSRAMSZ_1KB = 4,
    #[doc = "5: 2048 B"]
    EPI_HB16TIME_PSRAMSZ_2KB = 5,
    #[doc = "6: 4096 B"]
    EPI_HB16TIME_PSRAMSZ_4KB = 6,
    #[doc = "7: 8192 B"]
    EPI_HB16TIME_PSRAMSZ_8KB = 7,
}
impl From<EPI_HB16TIME_PSRAMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB16TIME_PSRAMSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB16TIME_PSRAMSZ` reader - PSRAM Row Size"]
pub type EPI_HB16TIME_PSRAMSZ_R = crate::FieldReader<u8, EPI_HB16TIME_PSRAMSZ_A>;
impl EPI_HB16TIME_PSRAMSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB16TIME_PSRAMSZ_A {
        match self.bits {
            0 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_0,
            1 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_128B,
            2 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_256B,
            3 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_512B,
            4 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_1KB,
            5 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_2KB,
            6 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_4KB,
            7 => EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_8KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_0`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_0(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_0
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_128B`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_128b(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_128B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_256b(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_512B`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_512b(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_512B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_1KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_1kb(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_1KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_2KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_2kb(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_2KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_4KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_4kb(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_4KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME_PSRAMSZ_8KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time_psramsz_8kb(&self) -> bool {
        *self == EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_8KB
    }
}
#[doc = "Field `EPI_HB16TIME_PSRAMSZ` writer - PSRAM Row Size"]
pub type EPI_HB16TIME_PSRAMSZ_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALT16_HB16TIME_SPEC, u8, EPI_HB16TIME_PSRAMSZ_A, 3, 16>;
impl<'a> EPI_HB16TIME_PSRAMSZ_W<'a> {
    #[doc = "No row size limitation"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_0(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_0)
    }
    #[doc = "128 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_128b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_128B)
    }
    #[doc = "256 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_256b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_256B)
    }
    #[doc = "512 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_512b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_512B)
    }
    #[doc = "1024 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_1kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_1KB)
    }
    #[doc = "2048 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_2kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_2KB)
    }
    #[doc = "4096 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_4kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_4KB)
    }
    #[doc = "8192 B"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz_8kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME_PSRAMSZ_A::EPI_HB16TIME_PSRAMSZ_8KB)
    }
}
#[doc = "Field `EPI_HB16TIME_IRDYDLY` reader - CS0n Input Ready Delay"]
pub type EPI_HB16TIME_IRDYDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_HB16TIME_IRDYDLY` writer - CS0n Input Ready Delay"]
pub type EPI_HB16TIME_IRDYDLY_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALT16_HB16TIME_SPEC, u8, u8, 2, 24>;
impl R {
    #[doc = "Bit 0 - Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time_rdwsm(&self) -> EPI_HB16TIME_RDWSM_R {
        EPI_HB16TIME_RDWSM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time_wrwsm(&self) -> EPI_HB16TIME_WRWSM_R {
        EPI_HB16TIME_WRWSM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - CS0n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb16time_capwidth(&self) -> EPI_HB16TIME_CAPWIDTH_R {
        EPI_HB16TIME_CAPWIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz(&self) -> EPI_HB16TIME_PSRAMSZ_R {
        EPI_HB16TIME_PSRAMSZ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - CS0n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb16time_irdydly(&self) -> EPI_HB16TIME_IRDYDLY_R {
        EPI_HB16TIME_IRDYDLY_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time_rdwsm(&mut self) -> EPI_HB16TIME_RDWSM_W {
        EPI_HB16TIME_RDWSM_W::new(self)
    }
    #[doc = "Bit 4 - Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time_wrwsm(&mut self) -> EPI_HB16TIME_WRWSM_W {
        EPI_HB16TIME_WRWSM_W::new(self)
    }
    #[doc = "Bits 12:13 - CS0n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb16time_capwidth(&mut self) -> EPI_HB16TIME_CAPWIDTH_W {
        EPI_HB16TIME_CAPWIDTH_W::new(self)
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn epi_hb16time_psramsz(&mut self) -> EPI_HB16TIME_PSRAMSZ_W {
        EPI_HB16TIME_PSRAMSZ_W::new(self)
    }
    #[doc = "Bits 24:25 - CS0n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb16time_irdydly(&mut self) -> EPI_HB16TIME_IRDYDLY_W {
        EPI_HB16TIME_IRDYDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 16 Timing Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epi_alt16_hb16time](index.html) module"]
pub struct EPI_ALT16_HB16TIME_SPEC;
impl crate::RegisterSpec for EPI_ALT16_HB16TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epi_alt16_hb16time::R](R) reader structure"]
impl crate::Readable for EPI_ALT16_HB16TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epi_alt16_hb16time::W](W) writer structure"]
impl crate::Writable for EPI_ALT16_HB16TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB16TIME to value 0"]
impl crate::Resettable for EPI_ALT16_HB16TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
