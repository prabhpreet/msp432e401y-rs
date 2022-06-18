#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_CTRL_TYPE_A {
    #[doc = "0: Polynomial 0x8005"]
    CRC_CTRL_TYPE_P8055 = 0,
    #[doc = "1: Polynomial 0x1021"]
    CRC_CTRL_TYPE_P1021 = 1,
    #[doc = "2: Polynomial 0x4C11DB7"]
    CRC_CTRL_TYPE_P4C11DB7 = 2,
    #[doc = "3: Polynomial 0x1EDC6F41"]
    CRC_CTRL_TYPE_P1EDC6F41 = 3,
    #[doc = "8: TCP checksum"]
    CRC_CTRL_TYPE_TCPCHKSUM = 8,
}
impl From<CRC_CTRL_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_CTRL_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_CTRL_TYPE` reader - Operation Type"]
pub type CRC_CTRL_TYPE_R = crate::FieldReader<u8, CRC_CTRL_TYPE_A>;
impl CRC_CTRL_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_CTRL_TYPE_A> {
        match self.bits {
            0 => Some(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P8055),
            1 => Some(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1021),
            2 => Some(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P4C11DB7),
            3 => Some(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1EDC6F41),
            8 => Some(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_TCPCHKSUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P8055`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p8055(&self) -> bool {
        *self == CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P8055
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P1021`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p1021(&self) -> bool {
        *self == CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1021
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P4C11DB7`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p4c11db7(&self) -> bool {
        *self == CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P4C11DB7
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_P1EDC6F41`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_p1edc6f41(&self) -> bool {
        *self == CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1EDC6F41
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_TYPE_TCPCHKSUM`"]
    #[inline(always)]
    pub fn is_crc_ctrl_type_tcpchksum(&self) -> bool {
        *self == CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_TCPCHKSUM
    }
}
#[doc = "Field `CRC_CTRL_TYPE` writer - Operation Type"]
pub type CRC_CTRL_TYPE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CRC_CTRL_TYPE_A, 4, 0>;
impl<'a> CRC_CTRL_TYPE_W<'a> {
    #[doc = "Polynomial 0x8005"]
    #[inline(always)]
    pub fn crc_ctrl_type_p8055(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P8055)
    }
    #[doc = "Polynomial 0x1021"]
    #[inline(always)]
    pub fn crc_ctrl_type_p1021(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1021)
    }
    #[doc = "Polynomial 0x4C11DB7"]
    #[inline(always)]
    pub fn crc_ctrl_type_p4c11db7(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P4C11DB7)
    }
    #[doc = "Polynomial 0x1EDC6F41"]
    #[inline(always)]
    pub fn crc_ctrl_type_p1edc6f41(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_P1EDC6F41)
    }
    #[doc = "TCP checksum"]
    #[inline(always)]
    pub fn crc_ctrl_type_tcpchksum(self) -> &'a mut W {
        self.variant(CRC_CTRL_TYPE_A::CRC_CTRL_TYPE_TCPCHKSUM)
    }
}
#[doc = "Endian Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_CTRL_ENDIAN_A {
    #[doc = "0: Configuration unchanged. (B3, B2, B1, B0)"]
    CRC_CTRL_ENDIAN_SBHW = 0,
    #[doc = "1: Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    CRC_CTRL_ENDIAN_SHW = 1,
    #[doc = "2: Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    CRC_CTRL_ENDIAN_SHWNB = 2,
    #[doc = "3: Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    CRC_CTRL_ENDIAN_SBSW = 3,
}
impl From<CRC_CTRL_ENDIAN_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_CTRL_ENDIAN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_CTRL_ENDIAN` reader - Endian Control"]
pub type CRC_CTRL_ENDIAN_R = crate::FieldReader<u8, CRC_CTRL_ENDIAN_A>;
impl CRC_CTRL_ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_CTRL_ENDIAN_A {
        match self.bits {
            0 => CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBHW,
            1 => CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHW,
            2 => CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHWNB,
            3 => CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBSW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SBHW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_sbhw(&self) -> bool {
        *self == CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBHW
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SHW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_shw(&self) -> bool {
        *self == CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHW
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SHWNB`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_shwnb(&self) -> bool {
        *self == CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHWNB
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_ENDIAN_SBSW`"]
    #[inline(always)]
    pub fn is_crc_ctrl_endian_sbsw(&self) -> bool {
        *self == CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBSW
    }
}
#[doc = "Field `CRC_CTRL_ENDIAN` writer - Endian Control"]
pub type CRC_CTRL_ENDIAN_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CRC_CTRL_ENDIAN_A, 2, 4>;
impl<'a> CRC_CTRL_ENDIAN_W<'a> {
    #[doc = "Configuration unchanged. (B3, B2, B1, B0)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_sbhw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBHW)
    }
    #[doc = "Bytes are swapped in half-words but half-words are not swapped (B2, B3, B0, B1)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_shw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHW)
    }
    #[doc = "Half-words are swapped but bytes are not swapped in half-word. (B1, B0, B3, B2)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_shwnb(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SHWNB)
    }
    #[doc = "Bytes are swapped in half-words and half-words are swapped. (B0, B1, B2, B3)"]
    #[inline(always)]
    pub fn crc_ctrl_endian_sbsw(self) -> &'a mut W {
        self.variant(CRC_CTRL_ENDIAN_A::CRC_CTRL_ENDIAN_SBSW)
    }
}
#[doc = "Field `CRC_CTRL_BR` reader - Bit reverse enable"]
pub type CRC_CTRL_BR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_CTRL_BR` writer - Bit reverse enable"]
pub type CRC_CTRL_BR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `CRC_CTRL_OBR` reader - Output Reverse Enable"]
pub type CRC_CTRL_OBR_R = crate::BitReader<bool>;
#[doc = "Field `CRC_CTRL_OBR` writer - Output Reverse Enable"]
pub type CRC_CTRL_OBR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `CRC_CTRL_RESINV` reader - Result Inverse Enable"]
pub type CRC_CTRL_RESINV_R = crate::BitReader<bool>;
#[doc = "Field `CRC_CTRL_RESINV` writer - Result Inverse Enable"]
pub type CRC_CTRL_RESINV_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `CRC_CTRL_SIZE` reader - Input Data Size"]
pub type CRC_CTRL_SIZE_R = crate::BitReader<bool>;
#[doc = "Field `CRC_CTRL_SIZE` writer - Input Data Size"]
pub type CRC_CTRL_SIZE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
#[doc = "CRC Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_CTRL_INIT_A {
    #[doc = "0: Use the CRCSEED register context as the starting value"]
    CRC_CTRL_INIT_SEED = 0,
    #[doc = "2: Initialize to all '0s'"]
    CRC_CTRL_INIT_0 = 2,
    #[doc = "3: Initialize to all '1s'"]
    CRC_CTRL_INIT_1 = 3,
}
impl From<CRC_CTRL_INIT_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_CTRL_INIT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRC_CTRL_INIT` reader - CRC Initialization"]
pub type CRC_CTRL_INIT_R = crate::FieldReader<u8, CRC_CTRL_INIT_A>;
impl CRC_CTRL_INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_CTRL_INIT_A> {
        match self.bits {
            0 => Some(CRC_CTRL_INIT_A::CRC_CTRL_INIT_SEED),
            2 => Some(CRC_CTRL_INIT_A::CRC_CTRL_INIT_0),
            3 => Some(CRC_CTRL_INIT_A::CRC_CTRL_INIT_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_SEED`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_seed(&self) -> bool {
        *self == CRC_CTRL_INIT_A::CRC_CTRL_INIT_SEED
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_0`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_0(&self) -> bool {
        *self == CRC_CTRL_INIT_A::CRC_CTRL_INIT_0
    }
    #[doc = "Checks if the value of the field is `CRC_CTRL_INIT_1`"]
    #[inline(always)]
    pub fn is_crc_ctrl_init_1(&self) -> bool {
        *self == CRC_CTRL_INIT_A::CRC_CTRL_INIT_1
    }
}
#[doc = "Field `CRC_CTRL_INIT` writer - CRC Initialization"]
pub type CRC_CTRL_INIT_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CRC_CTRL_INIT_A, 2, 13>;
impl<'a> CRC_CTRL_INIT_W<'a> {
    #[doc = "Use the CRCSEED register context as the starting value"]
    #[inline(always)]
    pub fn crc_ctrl_init_seed(self) -> &'a mut W {
        self.variant(CRC_CTRL_INIT_A::CRC_CTRL_INIT_SEED)
    }
    #[doc = "Initialize to all '0s'"]
    #[inline(always)]
    pub fn crc_ctrl_init_0(self) -> &'a mut W {
        self.variant(CRC_CTRL_INIT_A::CRC_CTRL_INIT_0)
    }
    #[doc = "Initialize to all '1s'"]
    #[inline(always)]
    pub fn crc_ctrl_init_1(self) -> &'a mut W {
        self.variant(CRC_CTRL_INIT_A::CRC_CTRL_INIT_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn crc_ctrl_type(&self) -> CRC_CTRL_TYPE_R {
        CRC_CTRL_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn crc_ctrl_endian(&self) -> CRC_CTRL_ENDIAN_R {
        CRC_CTRL_ENDIAN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn crc_ctrl_br(&self) -> CRC_CTRL_BR_R {
        CRC_CTRL_BR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_obr(&self) -> CRC_CTRL_OBR_R {
        CRC_CTRL_OBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_resinv(&self) -> CRC_CTRL_RESINV_R {
        CRC_CTRL_RESINV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn crc_ctrl_size(&self) -> CRC_CTRL_SIZE_R {
        CRC_CTRL_SIZE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn crc_ctrl_init(&self) -> CRC_CTRL_INIT_R {
        CRC_CTRL_INIT_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operation Type"]
    #[inline(always)]
    pub fn crc_ctrl_type(&mut self) -> CRC_CTRL_TYPE_W {
        CRC_CTRL_TYPE_W::new(self)
    }
    #[doc = "Bits 4:5 - Endian Control"]
    #[inline(always)]
    pub fn crc_ctrl_endian(&mut self) -> CRC_CTRL_ENDIAN_W {
        CRC_CTRL_ENDIAN_W::new(self)
    }
    #[doc = "Bit 7 - Bit reverse enable"]
    #[inline(always)]
    pub fn crc_ctrl_br(&mut self) -> CRC_CTRL_BR_W {
        CRC_CTRL_BR_W::new(self)
    }
    #[doc = "Bit 8 - Output Reverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_obr(&mut self) -> CRC_CTRL_OBR_W {
        CRC_CTRL_OBR_W::new(self)
    }
    #[doc = "Bit 9 - Result Inverse Enable"]
    #[inline(always)]
    pub fn crc_ctrl_resinv(&mut self) -> CRC_CTRL_RESINV_W {
        CRC_CTRL_RESINV_W::new(self)
    }
    #[doc = "Bit 12 - Input Data Size"]
    #[inline(always)]
    pub fn crc_ctrl_size(&mut self) -> CRC_CTRL_SIZE_W {
        CRC_CTRL_SIZE_W::new(self)
    }
    #[doc = "Bits 13:14 - CRC Initialization"]
    #[inline(always)]
    pub fn crc_ctrl_init(&mut self) -> CRC_CTRL_INIT_W {
        CRC_CTRL_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
