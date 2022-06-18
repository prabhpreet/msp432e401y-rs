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
#[doc = "Field `AES_CTRL_OUTPUT_READY` reader - Output Ready Status"]
pub type AES_CTRL_OUTPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_OUTPUT_READY` writer - Output Ready Status"]
pub type AES_CTRL_OUTPUT_READY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `AES_CTRL_INPUT_READY` reader - Input Ready Status"]
pub type AES_CTRL_INPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_INPUT_READY` writer - Input Ready Status"]
pub type AES_CTRL_INPUT_READY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `AES_CTRL_DIRECTION` reader - Encryption/Decryption Selection"]
pub type AES_CTRL_DIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_DIRECTION` writer - Encryption/Decryption Selection"]
pub type AES_CTRL_DIRECTION_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AES_CTRL_KEY_SIZE_A {
    #[doc = "1: Key is 128 bits"]
    AES_CTRL_KEY_SIZE_128 = 1,
    #[doc = "2: Key is 192 bits"]
    AES_CTRL_KEY_SIZE_192 = 2,
    #[doc = "3: Key is 256 bits"]
    AES_CTRL_KEY_SIZE_256 = 3,
}
impl From<AES_CTRL_KEY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: AES_CTRL_KEY_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AES_CTRL_KEY_SIZE` reader - Key Size"]
pub type AES_CTRL_KEY_SIZE_R = crate::FieldReader<u8, AES_CTRL_KEY_SIZE_A>;
impl AES_CTRL_KEY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AES_CTRL_KEY_SIZE_A> {
        match self.bits {
            1 => Some(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_128),
            2 => Some(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_192),
            3 => Some(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_128`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_128(&self) -> bool {
        *self == AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_128
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_192`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_192(&self) -> bool {
        *self == AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_192
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_KEY_SIZE_256`"]
    #[inline(always)]
    pub fn is_aes_ctrl_key_size_256(&self) -> bool {
        *self == AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_256
    }
}
#[doc = "Field `AES_CTRL_KEY_SIZE` writer - Key Size"]
pub type AES_CTRL_KEY_SIZE_W<'a> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, AES_CTRL_KEY_SIZE_A, 2, 3>;
impl<'a> AES_CTRL_KEY_SIZE_W<'a> {
    #[doc = "Key is 128 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_128(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_128)
    }
    #[doc = "Key is 192 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_192(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_192)
    }
    #[doc = "Key is 256 bits"]
    #[inline(always)]
    pub fn aes_ctrl_key_size_256(self) -> &'a mut W {
        self.variant(AES_CTRL_KEY_SIZE_A::AES_CTRL_KEY_SIZE_256)
    }
}
#[doc = "Field `AES_CTRL_MODE` reader - ECB/CBC Mode"]
pub type AES_CTRL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_MODE` writer - ECB/CBC Mode"]
pub type AES_CTRL_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `AES_CTRL_CTR` reader - Counter Mode"]
pub type AES_CTRL_CTR_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_CTR` writer - Counter Mode"]
pub type AES_CTRL_CTR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "AES-CTR Mode Counter Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AES_CTRL_CTR_WIDTH_A {
    #[doc = "0: Counter is 32 bits"]
    AES_CTRL_CTR_WIDTH_32 = 0,
    #[doc = "1: Counter is 64 bits"]
    AES_CTRL_CTR_WIDTH_64 = 1,
    #[doc = "2: Counter is 96 bits"]
    AES_CTRL_CTR_WIDTH_96 = 2,
    #[doc = "3: Counter is 128 bits"]
    AES_CTRL_CTR_WIDTH_128 = 3,
}
impl From<AES_CTRL_CTR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: AES_CTRL_CTR_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AES_CTRL_CTR_WIDTH` reader - AES-CTR Mode Counter Width"]
pub type AES_CTRL_CTR_WIDTH_R = crate::FieldReader<u8, AES_CTRL_CTR_WIDTH_A>;
impl AES_CTRL_CTR_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_CTRL_CTR_WIDTH_A {
        match self.bits {
            0 => AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_32,
            1 => AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_64,
            2 => AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_96,
            3 => AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_32`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_32(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_32
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_64`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_64(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_64
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_96`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_96(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_96
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CTR_WIDTH_128`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ctr_width_128(&self) -> bool {
        *self == AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_128
    }
}
#[doc = "Field `AES_CTRL_CTR_WIDTH` writer - AES-CTR Mode Counter Width"]
pub type AES_CTRL_CTR_WIDTH_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, AES_CTRL_CTR_WIDTH_A, 2, 7>;
impl<'a> AES_CTRL_CTR_WIDTH_W<'a> {
    #[doc = "Counter is 32 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_32(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_32)
    }
    #[doc = "Counter is 64 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_64(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_64)
    }
    #[doc = "Counter is 96 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_96(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_96)
    }
    #[doc = "Counter is 128 bits"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width_128(self) -> &'a mut W {
        self.variant(AES_CTRL_CTR_WIDTH_A::AES_CTRL_CTR_WIDTH_128)
    }
}
#[doc = "Field `AES_CTRL_ICM` reader - AES Integer Counter Mode (ICM) Enable"]
pub type AES_CTRL_ICM_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_ICM` writer - AES Integer Counter Mode (ICM) Enable"]
pub type AES_CTRL_ICM_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `AES_CTRL_CFB` reader - Full block AES cipher feedback mode (CFB128) Enable"]
pub type AES_CTRL_CFB_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_CFB` writer - Full block AES cipher feedback mode (CFB128) Enable"]
pub type AES_CTRL_CFB_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
#[doc = "AES-XTS Operation Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AES_CTRL_XTS_A {
    #[doc = "0: No operation"]
    AES_CTRL_XTS_NOP = 0,
    #[doc = "1: Previous/intermediate tweak value and j loaded (value is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_TWEAKJL = 1,
    #[doc = "2: Key2, n and j are loaded (n is loaded via IV, j is loaded via the AAD length register)"]
    AES_CTRL_XTS_K2IJL = 2,
    #[doc = "3: Key2 and n are loaded; j=0 (n is loaded via IV)"]
    AES_CTRL_XTS_K2ILJ0 = 3,
}
impl From<AES_CTRL_XTS_A> for u8 {
    #[inline(always)]
    fn from(variant: AES_CTRL_XTS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AES_CTRL_XTS` reader - AES-XTS Operation Enabled"]
pub type AES_CTRL_XTS_R = crate::FieldReader<u8, AES_CTRL_XTS_A>;
impl AES_CTRL_XTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_CTRL_XTS_A {
        match self.bits {
            0 => AES_CTRL_XTS_A::AES_CTRL_XTS_NOP,
            1 => AES_CTRL_XTS_A::AES_CTRL_XTS_TWEAKJL,
            2 => AES_CTRL_XTS_A::AES_CTRL_XTS_K2IJL,
            3 => AES_CTRL_XTS_A::AES_CTRL_XTS_K2ILJ0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_NOP`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_nop(&self) -> bool {
        *self == AES_CTRL_XTS_A::AES_CTRL_XTS_NOP
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_TWEAKJL`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_tweakjl(&self) -> bool {
        *self == AES_CTRL_XTS_A::AES_CTRL_XTS_TWEAKJL
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_K2IJL`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_k2ijl(&self) -> bool {
        *self == AES_CTRL_XTS_A::AES_CTRL_XTS_K2IJL
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_XTS_K2ILJ0`"]
    #[inline(always)]
    pub fn is_aes_ctrl_xts_k2ilj0(&self) -> bool {
        *self == AES_CTRL_XTS_A::AES_CTRL_XTS_K2ILJ0
    }
}
#[doc = "Field `AES_CTRL_XTS` writer - AES-XTS Operation Enabled"]
pub type AES_CTRL_XTS_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, AES_CTRL_XTS_A, 2, 11>;
impl<'a> AES_CTRL_XTS_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn aes_ctrl_xts_nop(self) -> &'a mut W {
        self.variant(AES_CTRL_XTS_A::AES_CTRL_XTS_NOP)
    }
    #[doc = "Previous/intermediate tweak value and j loaded (value is loaded via IV, j is loaded via the AAD length register)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_tweakjl(self) -> &'a mut W {
        self.variant(AES_CTRL_XTS_A::AES_CTRL_XTS_TWEAKJL)
    }
    #[doc = "Key2, n and j are loaded (n is loaded via IV, j is loaded via the AAD length register)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_k2ijl(self) -> &'a mut W {
        self.variant(AES_CTRL_XTS_A::AES_CTRL_XTS_K2IJL)
    }
    #[doc = "Key2 and n are loaded; j=0 (n is loaded via IV)"]
    #[inline(always)]
    pub fn aes_ctrl_xts_k2ilj0(self) -> &'a mut W {
        self.variant(AES_CTRL_XTS_A::AES_CTRL_XTS_K2ILJ0)
    }
}
#[doc = "Field `AES_CTRL_F8` reader - AES f8 Mode Enable"]
pub type AES_CTRL_F8_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_F8` writer - AES f8 Mode Enable"]
pub type AES_CTRL_F8_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "Field `AES_CTRL_F9` reader - AES f9 Mode Enable"]
pub type AES_CTRL_F9_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_F9` writer - AES f9 Mode Enable"]
pub type AES_CTRL_F9_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 14>;
#[doc = "Field `AES_CTRL_CBCMAC` reader - AES-CBC MAC Enable"]
pub type AES_CTRL_CBCMAC_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_CBCMAC` writer - AES-CBC MAC Enable"]
pub type AES_CTRL_CBCMAC_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 15>;
#[doc = "AES-GCM Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AES_CTRL_GCM_A {
    #[doc = "0: No operation"]
    AES_CTRL_GCM_NOP = 0,
    #[doc = "1: GHASH with H loaded and Y0-encrypted forced to zero"]
    AES_CTRL_GCM_HLY0ZERO = 1,
    #[doc = "2: GHASH with H loaded and Y0-encrypted calculated internally"]
    AES_CTRL_GCM_HLY0CALC = 2,
    #[doc = "3: Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    AES_CTRL_GCM_HY0CALC = 3,
}
impl From<AES_CTRL_GCM_A> for u8 {
    #[inline(always)]
    fn from(variant: AES_CTRL_GCM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AES_CTRL_GCM` reader - AES-GCM Mode Enable"]
pub type AES_CTRL_GCM_R = crate::FieldReader<u8, AES_CTRL_GCM_A>;
impl AES_CTRL_GCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AES_CTRL_GCM_A {
        match self.bits {
            0 => AES_CTRL_GCM_A::AES_CTRL_GCM_NOP,
            1 => AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0ZERO,
            2 => AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0CALC,
            3 => AES_CTRL_GCM_A::AES_CTRL_GCM_HY0CALC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_NOP`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_nop(&self) -> bool {
        *self == AES_CTRL_GCM_A::AES_CTRL_GCM_NOP
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HLY0ZERO`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hly0zero(&self) -> bool {
        *self == AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0ZERO
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HLY0CALC`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hly0calc(&self) -> bool {
        *self == AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0CALC
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_GCM_HY0CALC`"]
    #[inline(always)]
    pub fn is_aes_ctrl_gcm_hy0calc(&self) -> bool {
        *self == AES_CTRL_GCM_A::AES_CTRL_GCM_HY0CALC
    }
}
#[doc = "Field `AES_CTRL_GCM` writer - AES-GCM Mode Enable"]
pub type AES_CTRL_GCM_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, AES_CTRL_GCM_A, 2, 16>;
impl<'a> AES_CTRL_GCM_W<'a> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_nop(self) -> &'a mut W {
        self.variant(AES_CTRL_GCM_A::AES_CTRL_GCM_NOP)
    }
    #[doc = "GHASH with H loaded and Y0-encrypted forced to zero"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hly0zero(self) -> &'a mut W {
        self.variant(AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0ZERO)
    }
    #[doc = "GHASH with H loaded and Y0-encrypted calculated internally"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hly0calc(self) -> &'a mut W {
        self.variant(AES_CTRL_GCM_A::AES_CTRL_GCM_HLY0CALC)
    }
    #[doc = "Autonomous GHASH (both H and Y0-encrypted calculated internally)"]
    #[inline(always)]
    pub fn aes_ctrl_gcm_hy0calc(self) -> &'a mut W {
        self.variant(AES_CTRL_GCM_A::AES_CTRL_GCM_HY0CALC)
    }
}
#[doc = "Field `AES_CTRL_CCM` reader - AES-CCM Mode Enable"]
pub type AES_CTRL_CCM_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_CCM` writer - AES-CCM Mode Enable"]
pub type AES_CTRL_CCM_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 18>;
#[doc = "L Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AES_CTRL_CCM_L_A {
    #[doc = "1: width = 2"]
    AES_CTRL_CCM_L_2 = 1,
    #[doc = "3: width = 4"]
    AES_CTRL_CCM_L_4 = 3,
    #[doc = "7: width = 8"]
    AES_CTRL_CCM_L_8 = 7,
}
impl From<AES_CTRL_CCM_L_A> for u8 {
    #[inline(always)]
    fn from(variant: AES_CTRL_CCM_L_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AES_CTRL_CCM_L` reader - L Value"]
pub type AES_CTRL_CCM_L_R = crate::FieldReader<u8, AES_CTRL_CCM_L_A>;
impl AES_CTRL_CCM_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AES_CTRL_CCM_L_A> {
        match self.bits {
            1 => Some(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_2),
            3 => Some(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_4),
            7 => Some(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_2`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_2(&self) -> bool {
        *self == AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_2
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_4`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_4(&self) -> bool {
        *self == AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_4
    }
    #[doc = "Checks if the value of the field is `AES_CTRL_CCM_L_8`"]
    #[inline(always)]
    pub fn is_aes_ctrl_ccm_l_8(&self) -> bool {
        *self == AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_8
    }
}
#[doc = "Field `AES_CTRL_CCM_L` writer - L Value"]
pub type AES_CTRL_CCM_L_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, AES_CTRL_CCM_L_A, 3, 19>;
impl<'a> AES_CTRL_CCM_L_W<'a> {
    #[doc = "width = 2"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_2(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_2)
    }
    #[doc = "width = 4"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_4(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_4)
    }
    #[doc = "width = 8"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l_8(self) -> &'a mut W {
        self.variant(AES_CTRL_CCM_L_A::AES_CTRL_CCM_L_8)
    }
}
#[doc = "Field `AES_CTRL_CCM_M` reader - Counter with CBC-MAC (CCM)"]
pub type AES_CTRL_CCM_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AES_CTRL_CCM_M` writer - Counter with CBC-MAC (CCM)"]
pub type AES_CTRL_CCM_M_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, 22>;
#[doc = "Field `AES_CTRL_SAVE_CONTEXT` reader - TAG or Result IV Save"]
pub type AES_CTRL_SAVE_CONTEXT_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_SAVE_CONTEXT` writer - TAG or Result IV Save"]
pub type AES_CTRL_SAVE_CONTEXT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 29>;
#[doc = "Field `AES_CTRL_SVCTXTRDY` reader - AES TAG/IV Block(s) Ready"]
pub type AES_CTRL_SVCTXTRDY_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_SVCTXTRDY` writer - AES TAG/IV Block(s) Ready"]
pub type AES_CTRL_SVCTXTRDY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `AES_CTRL_CTXTRDY` reader - Context Data Registers Ready"]
pub type AES_CTRL_CTXTRDY_R = crate::BitReader<bool>;
#[doc = "Field `AES_CTRL_CTXTRDY` writer - Context Data Registers Ready"]
pub type AES_CTRL_CTXTRDY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_output_ready(&self) -> AES_CTRL_OUTPUT_READY_R {
        AES_CTRL_OUTPUT_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_input_ready(&self) -> AES_CTRL_INPUT_READY_R {
        AES_CTRL_INPUT_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Encryption/Decryption Selection"]
    #[inline(always)]
    pub fn aes_ctrl_direction(&self) -> AES_CTRL_DIRECTION_R {
        AES_CTRL_DIRECTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Key Size"]
    #[inline(always)]
    pub fn aes_ctrl_key_size(&self) -> AES_CTRL_KEY_SIZE_R {
        AES_CTRL_KEY_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ECB/CBC Mode"]
    #[inline(always)]
    pub fn aes_ctrl_mode(&self) -> AES_CTRL_MODE_R {
        AES_CTRL_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Mode"]
    #[inline(always)]
    pub fn aes_ctrl_ctr(&self) -> AES_CTRL_CTR_R {
        AES_CTRL_CTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - AES-CTR Mode Counter Width"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width(&self) -> AES_CTRL_CTR_WIDTH_R {
        AES_CTRL_CTR_WIDTH_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - AES Integer Counter Mode (ICM) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_icm(&self) -> AES_CTRL_ICM_R {
        AES_CTRL_ICM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Full block AES cipher feedback mode (CFB128) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cfb(&self) -> AES_CTRL_CFB_R {
        AES_CTRL_CFB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - AES-XTS Operation Enabled"]
    #[inline(always)]
    pub fn aes_ctrl_xts(&self) -> AES_CTRL_XTS_R {
        AES_CTRL_XTS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - AES f8 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f8(&self) -> AES_CTRL_F8_R {
        AES_CTRL_F8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AES f9 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f9(&self) -> AES_CTRL_F9_R {
        AES_CTRL_F9_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AES-CBC MAC Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cbcmac(&self) -> AES_CTRL_CBCMAC_R {
        AES_CTRL_CBCMAC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - AES-GCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_gcm(&self) -> AES_CTRL_GCM_R {
        AES_CTRL_GCM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - AES-CCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_ccm(&self) -> AES_CTRL_CCM_R {
        AES_CTRL_CCM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - L Value"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l(&self) -> AES_CTRL_CCM_L_R {
        AES_CTRL_CCM_L_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Counter with CBC-MAC (CCM)"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_m(&self) -> AES_CTRL_CCM_M_R {
        AES_CTRL_CCM_M_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 29 - TAG or Result IV Save"]
    #[inline(always)]
    pub fn aes_ctrl_save_context(&self) -> AES_CTRL_SAVE_CONTEXT_R {
        AES_CTRL_SAVE_CONTEXT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AES TAG/IV Block(s) Ready"]
    #[inline(always)]
    pub fn aes_ctrl_svctxtrdy(&self) -> AES_CTRL_SVCTXTRDY_R {
        AES_CTRL_SVCTXTRDY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Context Data Registers Ready"]
    #[inline(always)]
    pub fn aes_ctrl_ctxtrdy(&self) -> AES_CTRL_CTXTRDY_R {
        AES_CTRL_CTXTRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_output_ready(&mut self) -> AES_CTRL_OUTPUT_READY_W {
        AES_CTRL_OUTPUT_READY_W::new(self)
    }
    #[doc = "Bit 1 - Input Ready Status"]
    #[inline(always)]
    pub fn aes_ctrl_input_ready(&mut self) -> AES_CTRL_INPUT_READY_W {
        AES_CTRL_INPUT_READY_W::new(self)
    }
    #[doc = "Bit 2 - Encryption/Decryption Selection"]
    #[inline(always)]
    pub fn aes_ctrl_direction(&mut self) -> AES_CTRL_DIRECTION_W {
        AES_CTRL_DIRECTION_W::new(self)
    }
    #[doc = "Bits 3:4 - Key Size"]
    #[inline(always)]
    pub fn aes_ctrl_key_size(&mut self) -> AES_CTRL_KEY_SIZE_W {
        AES_CTRL_KEY_SIZE_W::new(self)
    }
    #[doc = "Bit 5 - ECB/CBC Mode"]
    #[inline(always)]
    pub fn aes_ctrl_mode(&mut self) -> AES_CTRL_MODE_W {
        AES_CTRL_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Counter Mode"]
    #[inline(always)]
    pub fn aes_ctrl_ctr(&mut self) -> AES_CTRL_CTR_W {
        AES_CTRL_CTR_W::new(self)
    }
    #[doc = "Bits 7:8 - AES-CTR Mode Counter Width"]
    #[inline(always)]
    pub fn aes_ctrl_ctr_width(&mut self) -> AES_CTRL_CTR_WIDTH_W {
        AES_CTRL_CTR_WIDTH_W::new(self)
    }
    #[doc = "Bit 9 - AES Integer Counter Mode (ICM) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_icm(&mut self) -> AES_CTRL_ICM_W {
        AES_CTRL_ICM_W::new(self)
    }
    #[doc = "Bit 10 - Full block AES cipher feedback mode (CFB128) Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cfb(&mut self) -> AES_CTRL_CFB_W {
        AES_CTRL_CFB_W::new(self)
    }
    #[doc = "Bits 11:12 - AES-XTS Operation Enabled"]
    #[inline(always)]
    pub fn aes_ctrl_xts(&mut self) -> AES_CTRL_XTS_W {
        AES_CTRL_XTS_W::new(self)
    }
    #[doc = "Bit 13 - AES f8 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f8(&mut self) -> AES_CTRL_F8_W {
        AES_CTRL_F8_W::new(self)
    }
    #[doc = "Bit 14 - AES f9 Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_f9(&mut self) -> AES_CTRL_F9_W {
        AES_CTRL_F9_W::new(self)
    }
    #[doc = "Bit 15 - AES-CBC MAC Enable"]
    #[inline(always)]
    pub fn aes_ctrl_cbcmac(&mut self) -> AES_CTRL_CBCMAC_W {
        AES_CTRL_CBCMAC_W::new(self)
    }
    #[doc = "Bits 16:17 - AES-GCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_gcm(&mut self) -> AES_CTRL_GCM_W {
        AES_CTRL_GCM_W::new(self)
    }
    #[doc = "Bit 18 - AES-CCM Mode Enable"]
    #[inline(always)]
    pub fn aes_ctrl_ccm(&mut self) -> AES_CTRL_CCM_W {
        AES_CTRL_CCM_W::new(self)
    }
    #[doc = "Bits 19:21 - L Value"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_l(&mut self) -> AES_CTRL_CCM_L_W {
        AES_CTRL_CCM_L_W::new(self)
    }
    #[doc = "Bits 22:24 - Counter with CBC-MAC (CCM)"]
    #[inline(always)]
    pub fn aes_ctrl_ccm_m(&mut self) -> AES_CTRL_CCM_M_W {
        AES_CTRL_CCM_M_W::new(self)
    }
    #[doc = "Bit 29 - TAG or Result IV Save"]
    #[inline(always)]
    pub fn aes_ctrl_save_context(&mut self) -> AES_CTRL_SAVE_CONTEXT_W {
        AES_CTRL_SAVE_CONTEXT_W::new(self)
    }
    #[doc = "Bit 30 - AES TAG/IV Block(s) Ready"]
    #[inline(always)]
    pub fn aes_ctrl_svctxtrdy(&mut self) -> AES_CTRL_SVCTXTRDY_W {
        AES_CTRL_SVCTXTRDY_W::new(self)
    }
    #[doc = "Bit 31 - Context Data Registers Ready"]
    #[inline(always)]
    pub fn aes_ctrl_ctxtrdy(&mut self) -> AES_CTRL_CTXTRDY_W {
        AES_CTRL_CTXTRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
