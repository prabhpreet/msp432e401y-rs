#[doc = "Register `DID1` reader"]
pub struct R(crate::R<DID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DID1` writer"]
pub struct W(crate::W<DID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DID1_SPEC>;
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
impl From<crate::W<DID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Qualification Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_QUAL_A {
    #[doc = "0: Engineering Sample (unqualified)"]
    SYSCTL_DID1_QUAL_ES = 0,
    #[doc = "1: Pilot Production (unqualified)"]
    SYSCTL_DID1_QUAL_PP = 1,
    #[doc = "2: Fully Qualified"]
    SYSCTL_DID1_QUAL_FQ = 2,
}
impl From<SYSCTL_DID1_QUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_QUAL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID1_QUAL` reader - Qualification Status"]
pub type SYSCTL_DID1_QUAL_R = crate::FieldReader<u8, SYSCTL_DID1_QUAL_A>;
impl SYSCTL_DID1_QUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID1_QUAL_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES),
            1 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP),
            2 => Some(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_ES`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_es(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_PP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_pp(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_QUAL_FQ`"]
    #[inline(always)]
    pub fn is_sysctl_did1_qual_fq(&self) -> bool {
        *self == SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ
    }
}
#[doc = "Field `SYSCTL_DID1_QUAL` writer - Qualification Status"]
pub type SYSCTL_DID1_QUAL_W<'a> =
    crate::FieldWriter<'a, u32, DID1_SPEC, u8, SYSCTL_DID1_QUAL_A, 2, 0>;
impl<'a> SYSCTL_DID1_QUAL_W<'a> {
    #[doc = "Engineering Sample (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_es(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_ES)
    }
    #[doc = "Pilot Production (unqualified)"]
    #[inline(always)]
    pub fn sysctl_did1_qual_pp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_PP)
    }
    #[doc = "Fully Qualified"]
    #[inline(always)]
    pub fn sysctl_did1_qual_fq(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_QUAL_A::SYSCTL_DID1_QUAL_FQ)
    }
}
#[doc = "Field `SYSCTL_DID1_ROHS` reader - RoHS-Compliance"]
pub type SYSCTL_DID1_ROHS_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DID1_ROHS` writer - RoHS-Compliance"]
pub type SYSCTL_DID1_ROHS_W<'a> = crate::BitWriter<'a, u32, DID1_SPEC, bool, 2>;
#[doc = "Package Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PKG_A {
    #[doc = "1: QFP package"]
    SYSCTL_DID1_PKG_QFP = 1,
    #[doc = "2: BGA package"]
    SYSCTL_DID1_PKG_BGA = 2,
}
impl From<SYSCTL_DID1_PKG_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PKG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID1_PKG` reader - Package Type"]
pub type SYSCTL_DID1_PKG_R = crate::FieldReader<u8, SYSCTL_DID1_PKG_A>;
impl SYSCTL_DID1_PKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID1_PKG_A> {
        match self.bits {
            1 => Some(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP),
            2 => Some(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_QFP`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_qfp(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PKG_BGA`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pkg_bga(&self) -> bool {
        *self == SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA
    }
}
#[doc = "Field `SYSCTL_DID1_PKG` writer - Package Type"]
pub type SYSCTL_DID1_PKG_W<'a> =
    crate::FieldWriter<'a, u32, DID1_SPEC, u8, SYSCTL_DID1_PKG_A, 2, 3>;
impl<'a> SYSCTL_DID1_PKG_W<'a> {
    #[doc = "QFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_qfp(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_QFP)
    }
    #[doc = "BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pkg_bga(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PKG_A::SYSCTL_DID1_PKG_BGA)
    }
}
#[doc = "Temperature Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_TEMP_A {
    #[doc = "0: Commercial temperature range"]
    SYSCTL_DID1_TEMP_C = 0,
    #[doc = "1: Industrial temperature range"]
    SYSCTL_DID1_TEMP_I = 1,
    #[doc = "2: Extended temperature range"]
    SYSCTL_DID1_TEMP_E = 2,
}
impl From<SYSCTL_DID1_TEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_TEMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID1_TEMP` reader - Temperature Range"]
pub type SYSCTL_DID1_TEMP_R = crate::FieldReader<u8, SYSCTL_DID1_TEMP_A>;
impl SYSCTL_DID1_TEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID1_TEMP_A> {
        match self.bits {
            0 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C),
            1 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I),
            2 => Some(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_C`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_c(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_I`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_i(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_TEMP_E`"]
    #[inline(always)]
    pub fn is_sysctl_did1_temp_e(&self) -> bool {
        *self == SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E
    }
}
#[doc = "Field `SYSCTL_DID1_TEMP` writer - Temperature Range"]
pub type SYSCTL_DID1_TEMP_W<'a> =
    crate::FieldWriter<'a, u32, DID1_SPEC, u8, SYSCTL_DID1_TEMP_A, 3, 5>;
impl<'a> SYSCTL_DID1_TEMP_W<'a> {
    #[doc = "Commercial temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_c(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_C)
    }
    #[doc = "Industrial temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_i(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_I)
    }
    #[doc = "Extended temperature range"]
    #[inline(always)]
    pub fn sysctl_did1_temp_e(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_TEMP_A::SYSCTL_DID1_TEMP_E)
    }
}
#[doc = "Package Pin Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID1_PINCNT_A {
    #[doc = "6: 128-pin TQFP package"]
    SYSCTL_DID1_PINCNT_128 = 6,
    #[doc = "7: 212-pin BGA package"]
    SYSCTL_DID1_PINCNT_212 = 7,
}
impl From<SYSCTL_DID1_PINCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID1_PINCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID1_PINCNT` reader - Package Pin Count"]
pub type SYSCTL_DID1_PINCNT_R = crate::FieldReader<u8, SYSCTL_DID1_PINCNT_A>;
impl SYSCTL_DID1_PINCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID1_PINCNT_A> {
        match self.bits {
            6 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_128),
            7 => Some(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_212),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PINCNT_128`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_128(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_128
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID1_PINCNT_212`"]
    #[inline(always)]
    pub fn is_sysctl_did1_pincnt_212(&self) -> bool {
        *self == SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_212
    }
}
#[doc = "Field `SYSCTL_DID1_PINCNT` writer - Package Pin Count"]
pub type SYSCTL_DID1_PINCNT_W<'a> =
    crate::FieldWriter<'a, u32, DID1_SPEC, u8, SYSCTL_DID1_PINCNT_A, 3, 13>;
impl<'a> SYSCTL_DID1_PINCNT_W<'a> {
    #[doc = "128-pin TQFP package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_128(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_128)
    }
    #[doc = "212-pin BGA package"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt_212(self) -> &'a mut W {
        self.variant(SYSCTL_DID1_PINCNT_A::SYSCTL_DID1_PINCNT_212)
    }
}
#[doc = "Field `SYSCTL_DID1_PRTNO` reader - Part Number"]
pub type SYSCTL_DID1_PRTNO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_DID1_PRTNO` writer - Part Number"]
pub type SYSCTL_DID1_PRTNO_W<'a> = crate::FieldWriter<'a, u32, DID1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `SYSCTL_DID1_FAM` reader - Family"]
pub type SYSCTL_DID1_FAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_DID1_FAM` writer - Family"]
pub type SYSCTL_DID1_FAM_W<'a> = crate::FieldWriter<'a, u32, DID1_SPEC, u8, u8, 4, 24>;
#[doc = "Field `SYSCTL_DID1_VER` reader - DID1 Version"]
pub type SYSCTL_DID1_VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_DID1_VER` writer - DID1 Version"]
pub type SYSCTL_DID1_VER_W<'a> = crate::FieldWriter<'a, u32, DID1_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&self) -> SYSCTL_DID1_QUAL_R {
        SYSCTL_DID1_QUAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&self) -> SYSCTL_DID1_ROHS_R {
        SYSCTL_DID1_ROHS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&self) -> SYSCTL_DID1_PKG_R {
        SYSCTL_DID1_PKG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&self) -> SYSCTL_DID1_TEMP_R {
        SYSCTL_DID1_TEMP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&self) -> SYSCTL_DID1_PINCNT_R {
        SYSCTL_DID1_PINCNT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&self) -> SYSCTL_DID1_PRTNO_R {
        SYSCTL_DID1_PRTNO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&self) -> SYSCTL_DID1_FAM_R {
        SYSCTL_DID1_FAM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&self) -> SYSCTL_DID1_VER_R {
        SYSCTL_DID1_VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Qualification Status"]
    #[inline(always)]
    pub fn sysctl_did1_qual(&mut self) -> SYSCTL_DID1_QUAL_W {
        SYSCTL_DID1_QUAL_W::new(self)
    }
    #[doc = "Bit 2 - RoHS-Compliance"]
    #[inline(always)]
    pub fn sysctl_did1_rohs(&mut self) -> SYSCTL_DID1_ROHS_W {
        SYSCTL_DID1_ROHS_W::new(self)
    }
    #[doc = "Bits 3:4 - Package Type"]
    #[inline(always)]
    pub fn sysctl_did1_pkg(&mut self) -> SYSCTL_DID1_PKG_W {
        SYSCTL_DID1_PKG_W::new(self)
    }
    #[doc = "Bits 5:7 - Temperature Range"]
    #[inline(always)]
    pub fn sysctl_did1_temp(&mut self) -> SYSCTL_DID1_TEMP_W {
        SYSCTL_DID1_TEMP_W::new(self)
    }
    #[doc = "Bits 13:15 - Package Pin Count"]
    #[inline(always)]
    pub fn sysctl_did1_pincnt(&mut self) -> SYSCTL_DID1_PINCNT_W {
        SYSCTL_DID1_PINCNT_W::new(self)
    }
    #[doc = "Bits 16:23 - Part Number"]
    #[inline(always)]
    pub fn sysctl_did1_prtno(&mut self) -> SYSCTL_DID1_PRTNO_W {
        SYSCTL_DID1_PRTNO_W::new(self)
    }
    #[doc = "Bits 24:27 - Family"]
    #[inline(always)]
    pub fn sysctl_did1_fam(&mut self) -> SYSCTL_DID1_FAM_W {
        SYSCTL_DID1_FAM_W::new(self)
    }
    #[doc = "Bits 28:31 - DID1 Version"]
    #[inline(always)]
    pub fn sysctl_did1_ver(&mut self) -> SYSCTL_DID1_VER_W {
        SYSCTL_DID1_VER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did1](index.html) module"]
pub struct DID1_SPEC;
impl crate::RegisterSpec for DID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [did1::R](R) reader structure"]
impl crate::Readable for DID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [did1::W](W) writer structure"]
impl crate::Writable for DID1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DID1 to value 0"]
impl crate::Resettable for DID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
