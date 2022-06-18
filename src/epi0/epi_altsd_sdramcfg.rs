#[doc = "Register `SDRAMCFG` reader"]
pub struct R(crate::R<EPI_ALTSD_SDRAMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPI_ALTSD_SDRAMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPI_ALTSD_SDRAMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPI_ALTSD_SDRAMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMCFG` writer"]
pub struct W(crate::W<EPI_ALTSD_SDRAMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPI_ALTSD_SDRAMCFG_SPEC>;
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
impl From<crate::W<EPI_ALTSD_SDRAMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPI_ALTSD_SDRAMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Size of SDRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_SDRAMCFG_SIZE_A {
    #[doc = "0: 64 megabits (8MB)"]
    EPI_SDRAMCFG_SIZE_8MB = 0,
    #[doc = "1: 128 megabits (16MB)"]
    EPI_SDRAMCFG_SIZE_16MB = 1,
    #[doc = "2: 256 megabits (32MB)"]
    EPI_SDRAMCFG_SIZE_32MB = 2,
    #[doc = "3: 512 megabits (64MB)"]
    EPI_SDRAMCFG_SIZE_64MB = 3,
}
impl From<EPI_SDRAMCFG_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_SDRAMCFG_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_SDRAMCFG_SIZE` reader - Size of SDRAM"]
pub type EPI_SDRAMCFG_SIZE_R = crate::FieldReader<u8, EPI_SDRAMCFG_SIZE_A>;
impl EPI_SDRAMCFG_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_SDRAMCFG_SIZE_A {
        match self.bits {
            0 => EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_8MB,
            1 => EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_16MB,
            2 => EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_32MB,
            3 => EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_64MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_8MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_8mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_8MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_16MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_16mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_32MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_32mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_32MB
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_SIZE_64MB`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_size_64mb(&self) -> bool {
        *self == EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_64MB
    }
}
#[doc = "Field `EPI_SDRAMCFG_SIZE` writer - Size of SDRAM"]
pub type EPI_SDRAMCFG_SIZE_W<'a> =
    crate::FieldWriterSafe<'a, u32, EPI_ALTSD_SDRAMCFG_SPEC, u8, EPI_SDRAMCFG_SIZE_A, 2, 0>;
impl<'a> EPI_SDRAMCFG_SIZE_W<'a> {
    #[doc = "64 megabits (8MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_8mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_8MB)
    }
    #[doc = "128 megabits (16MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_16mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_16MB)
    }
    #[doc = "256 megabits (32MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_32mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_32MB)
    }
    #[doc = "512 megabits (64MB)"]
    #[inline(always)]
    pub fn epi_sdramcfg_size_64mb(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_SIZE_A::EPI_SDRAMCFG_SIZE_64MB)
    }
}
#[doc = "Field `EPI_SDRAMCFG_SLEEP` reader - Sleep Mode"]
pub type EPI_SDRAMCFG_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `EPI_SDRAMCFG_SLEEP` writer - Sleep Mode"]
pub type EPI_SDRAMCFG_SLEEP_W<'a> = crate::BitWriter<'a, u32, EPI_ALTSD_SDRAMCFG_SPEC, bool, 9>;
#[doc = "Field `EPI_SDRAMCFG_RFSH` reader - Refresh Counter"]
pub type EPI_SDRAMCFG_RFSH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPI_SDRAMCFG_RFSH` writer - Refresh Counter"]
pub type EPI_SDRAMCFG_RFSH_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALTSD_SDRAMCFG_SPEC, u16, u16, 11, 16>;
#[doc = "EPI Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_SDRAMCFG_FREQ_A {
    #[doc = "0: 0 - 15 MHz"]
    EPI_SDRAMCFG_FREQ_NONE = 0,
    #[doc = "1: 15 - 30 MHz"]
    EPI_SDRAMCFG_FREQ_15MHZ = 1,
    #[doc = "2: 30 - 50 MHz"]
    EPI_SDRAMCFG_FREQ_30MHZ = 2,
}
impl From<EPI_SDRAMCFG_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_SDRAMCFG_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_SDRAMCFG_FREQ` reader - EPI Frequency Range"]
pub type EPI_SDRAMCFG_FREQ_R = crate::FieldReader<u8, EPI_SDRAMCFG_FREQ_A>;
impl EPI_SDRAMCFG_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_SDRAMCFG_FREQ_A> {
        match self.bits {
            0 => Some(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_NONE),
            1 => Some(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_15MHZ),
            2 => Some(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_30MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_NONE`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_none(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_15MHZ`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_15mhz(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_15MHZ
    }
    #[doc = "Checks if the value of the field is `EPI_SDRAMCFG_FREQ_30MHZ`"]
    #[inline(always)]
    pub fn is_epi_sdramcfg_freq_30mhz(&self) -> bool {
        *self == EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_30MHZ
    }
}
#[doc = "Field `EPI_SDRAMCFG_FREQ` writer - EPI Frequency Range"]
pub type EPI_SDRAMCFG_FREQ_W<'a> =
    crate::FieldWriter<'a, u32, EPI_ALTSD_SDRAMCFG_SPEC, u8, EPI_SDRAMCFG_FREQ_A, 2, 30>;
impl<'a> EPI_SDRAMCFG_FREQ_W<'a> {
    #[doc = "0 - 15 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_none(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_NONE)
    }
    #[doc = "15 - 30 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_15mhz(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_15MHZ)
    }
    #[doc = "30 - 50 MHz"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq_30mhz(self) -> &'a mut W {
        self.variant(EPI_SDRAMCFG_FREQ_A::EPI_SDRAMCFG_FREQ_30MHZ)
    }
}
impl R {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn epi_sdramcfg_size(&self) -> EPI_SDRAMCFG_SIZE_R {
        EPI_SDRAMCFG_SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn epi_sdramcfg_sleep(&self) -> EPI_SDRAMCFG_SLEEP_R {
        EPI_SDRAMCFG_SLEEP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn epi_sdramcfg_rfsh(&self) -> EPI_SDRAMCFG_RFSH_R {
        EPI_SDRAMCFG_RFSH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq(&self) -> EPI_SDRAMCFG_FREQ_R {
        EPI_SDRAMCFG_FREQ_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of SDRAM"]
    #[inline(always)]
    pub fn epi_sdramcfg_size(&mut self) -> EPI_SDRAMCFG_SIZE_W {
        EPI_SDRAMCFG_SIZE_W::new(self)
    }
    #[doc = "Bit 9 - Sleep Mode"]
    #[inline(always)]
    pub fn epi_sdramcfg_sleep(&mut self) -> EPI_SDRAMCFG_SLEEP_W {
        EPI_SDRAMCFG_SLEEP_W::new(self)
    }
    #[doc = "Bits 16:26 - Refresh Counter"]
    #[inline(always)]
    pub fn epi_sdramcfg_rfsh(&mut self) -> EPI_SDRAMCFG_RFSH_W {
        EPI_SDRAMCFG_RFSH_W::new(self)
    }
    #[doc = "Bits 30:31 - EPI Frequency Range"]
    #[inline(always)]
    pub fn epi_sdramcfg_freq(&mut self) -> EPI_SDRAMCFG_FREQ_W {
        EPI_SDRAMCFG_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI SDRAM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epi_altsd_sdramcfg](index.html) module"]
pub struct EPI_ALTSD_SDRAMCFG_SPEC;
impl crate::RegisterSpec for EPI_ALTSD_SDRAMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epi_altsd_sdramcfg::R](R) reader structure"]
impl crate::Readable for EPI_ALTSD_SDRAMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epi_altsd_sdramcfg::W](W) writer structure"]
impl crate::Writable for EPI_ALTSD_SDRAMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRAMCFG to value 0"]
impl crate::Resettable for EPI_ALTSD_SDRAMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
