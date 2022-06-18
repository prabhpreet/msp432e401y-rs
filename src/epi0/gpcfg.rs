#[doc = "Register `GPCFG` reader"]
pub struct R(crate::R<GPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPCFG` writer"]
pub struct W(crate::W<GPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCFG_SPEC>;
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
impl From<crate::W<GPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Size of Data Bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_GPCFG_DSIZE_A {
    #[doc = "0: 8 Bits Wide (EPI0S0 to EPI0S7)"]
    EPI_GPCFG_DSIZE_4BIT = 0,
    #[doc = "1: 16 Bits Wide (EPI0S0 to EPI0S15)"]
    EPI_GPCFG_DSIZE_16BIT = 1,
    #[doc = "2: 24 Bits Wide (EPI0S0 to EPI0S23)"]
    EPI_GPCFG_DSIZE_24BIT = 2,
    #[doc = "3: 32 Bits Wide (EPI0S0 to EPI0S31)"]
    EPI_GPCFG_DSIZE_32BIT = 3,
}
impl From<EPI_GPCFG_DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_GPCFG_DSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_GPCFG_DSIZE` reader - Size of Data Bus"]
pub type EPI_GPCFG_DSIZE_R = crate::FieldReader<u8, EPI_GPCFG_DSIZE_A>;
impl EPI_GPCFG_DSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_GPCFG_DSIZE_A {
        match self.bits {
            0 => EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_4BIT,
            1 => EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_16BIT,
            2 => EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_24BIT,
            3 => EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_4BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_4bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_4BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_16BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_16bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_24BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_24bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_24BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_DSIZE_32BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_dsize_32bit(&self) -> bool {
        *self == EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_32BIT
    }
}
#[doc = "Field `EPI_GPCFG_DSIZE` writer - Size of Data Bus"]
pub type EPI_GPCFG_DSIZE_W<'a> =
    crate::FieldWriterSafe<'a, u32, GPCFG_SPEC, u8, EPI_GPCFG_DSIZE_A, 2, 0>;
impl<'a> EPI_GPCFG_DSIZE_W<'a> {
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_4bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_4BIT)
    }
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_16bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_16BIT)
    }
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_24bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_24BIT)
    }
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize_32bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_DSIZE_A::EPI_GPCFG_DSIZE_32BIT)
    }
}
#[doc = "Address Bus Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_GPCFG_ASIZE_A {
    #[doc = "0: No address"]
    EPI_GPCFG_ASIZE_NONE = 0,
    #[doc = "1: Up to 4 bits wide"]
    EPI_GPCFG_ASIZE_4BIT = 1,
    #[doc = "2: Up to 12 bits wide. This size cannot be used with 24-bit data"]
    EPI_GPCFG_ASIZE_12BIT = 2,
    #[doc = "3: Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    EPI_GPCFG_ASIZE_20BIT = 3,
}
impl From<EPI_GPCFG_ASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_GPCFG_ASIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_GPCFG_ASIZE` reader - Address Bus Size"]
pub type EPI_GPCFG_ASIZE_R = crate::FieldReader<u8, EPI_GPCFG_ASIZE_A>;
impl EPI_GPCFG_ASIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_GPCFG_ASIZE_A {
        match self.bits {
            0 => EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_NONE,
            1 => EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_4BIT,
            2 => EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_12BIT,
            3 => EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_20BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_NONE`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_none(&self) -> bool {
        *self == EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_4BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_4bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_4BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_12BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_12bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_12BIT
    }
    #[doc = "Checks if the value of the field is `EPI_GPCFG_ASIZE_20BIT`"]
    #[inline(always)]
    pub fn is_epi_gpcfg_asize_20bit(&self) -> bool {
        *self == EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_20BIT
    }
}
#[doc = "Field `EPI_GPCFG_ASIZE` writer - Address Bus Size"]
pub type EPI_GPCFG_ASIZE_W<'a> =
    crate::FieldWriterSafe<'a, u32, GPCFG_SPEC, u8, EPI_GPCFG_ASIZE_A, 2, 4>;
impl<'a> EPI_GPCFG_ASIZE_W<'a> {
    #[doc = "No address"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_none(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_NONE)
    }
    #[doc = "Up to 4 bits wide"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_4bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_4BIT)
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_12bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_12BIT)
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    #[inline(always)]
    pub fn epi_gpcfg_asize_20bit(self) -> &'a mut W {
        self.variant(EPI_GPCFG_ASIZE_A::EPI_GPCFG_ASIZE_20BIT)
    }
}
#[doc = "Field `EPI_GPCFG_WR2CYC` reader - 2-Cycle Writes"]
pub type EPI_GPCFG_WR2CYC_R = crate::BitReader<bool>;
#[doc = "Field `EPI_GPCFG_WR2CYC` writer - 2-Cycle Writes"]
pub type EPI_GPCFG_WR2CYC_W<'a> = crate::BitWriter<'a, u32, GPCFG_SPEC, bool, 19>;
#[doc = "Field `EPI_GPCFG_FRMCNT` reader - Frame Count"]
pub type EPI_GPCFG_FRMCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPI_GPCFG_FRMCNT` writer - Frame Count"]
pub type EPI_GPCFG_FRMCNT_W<'a> = crate::FieldWriter<'a, u32, GPCFG_SPEC, u8, u8, 4, 22>;
#[doc = "Field `EPI_GPCFG_FRM50` reader - 50/50 Frame"]
pub type EPI_GPCFG_FRM50_R = crate::BitReader<bool>;
#[doc = "Field `EPI_GPCFG_FRM50` writer - 50/50 Frame"]
pub type EPI_GPCFG_FRM50_W<'a> = crate::BitWriter<'a, u32, GPCFG_SPEC, bool, 26>;
#[doc = "Field `EPI_GPCFG_CLKGATE` reader - Clock Gated"]
pub type EPI_GPCFG_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `EPI_GPCFG_CLKGATE` writer - Clock Gated"]
pub type EPI_GPCFG_CLKGATE_W<'a> = crate::BitWriter<'a, u32, GPCFG_SPEC, bool, 30>;
#[doc = "Field `EPI_GPCFG_CLKPIN` reader - Clock Pin"]
pub type EPI_GPCFG_CLKPIN_R = crate::BitReader<bool>;
#[doc = "Field `EPI_GPCFG_CLKPIN` writer - Clock Pin"]
pub type EPI_GPCFG_CLKPIN_W<'a> = crate::BitWriter<'a, u32, GPCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize(&self) -> EPI_GPCFG_DSIZE_R {
        EPI_GPCFG_DSIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn epi_gpcfg_asize(&self) -> EPI_GPCFG_ASIZE_R {
        EPI_GPCFG_ASIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn epi_gpcfg_wr2cyc(&self) -> EPI_GPCFG_WR2CYC_R {
        EPI_GPCFG_WR2CYC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn epi_gpcfg_frmcnt(&self) -> EPI_GPCFG_FRMCNT_R {
        EPI_GPCFG_FRMCNT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn epi_gpcfg_frm50(&self) -> EPI_GPCFG_FRM50_R {
        EPI_GPCFG_FRM50_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn epi_gpcfg_clkgate(&self) -> EPI_GPCFG_CLKGATE_R {
        EPI_GPCFG_CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn epi_gpcfg_clkpin(&self) -> EPI_GPCFG_CLKPIN_R {
        EPI_GPCFG_CLKPIN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline(always)]
    pub fn epi_gpcfg_dsize(&mut self) -> EPI_GPCFG_DSIZE_W {
        EPI_GPCFG_DSIZE_W::new(self)
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline(always)]
    pub fn epi_gpcfg_asize(&mut self) -> EPI_GPCFG_ASIZE_W {
        EPI_GPCFG_ASIZE_W::new(self)
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline(always)]
    pub fn epi_gpcfg_wr2cyc(&mut self) -> EPI_GPCFG_WR2CYC_W {
        EPI_GPCFG_WR2CYC_W::new(self)
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline(always)]
    pub fn epi_gpcfg_frmcnt(&mut self) -> EPI_GPCFG_FRMCNT_W {
        EPI_GPCFG_FRMCNT_W::new(self)
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline(always)]
    pub fn epi_gpcfg_frm50(&mut self) -> EPI_GPCFG_FRM50_W {
        EPI_GPCFG_FRM50_W::new(self)
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline(always)]
    pub fn epi_gpcfg_clkgate(&mut self) -> EPI_GPCFG_CLKGATE_W {
        EPI_GPCFG_CLKGATE_W::new(self)
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline(always)]
    pub fn epi_gpcfg_clkpin(&mut self) -> EPI_GPCFG_CLKPIN_W {
        EPI_GPCFG_CLKPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI General-Purpose Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcfg](index.html) module"]
pub struct GPCFG_SPEC;
impl crate::RegisterSpec for GPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpcfg::R](R) reader structure"]
impl crate::Readable for GPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpcfg::W](W) writer structure"]
impl crate::Writable for GPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPCFG to value 0"]
impl crate::Resettable for GPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
