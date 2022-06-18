#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Hash Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAMD5_MODE_ALGO_A {
    #[doc = "0: MD5"]
    SHAMD5_MODE_ALGO_MD5 = 0,
    #[doc = "2: SHA-1"]
    SHAMD5_MODE_ALGO_SHA1 = 2,
    #[doc = "4: SHA-224"]
    SHAMD5_MODE_ALGO_SHA224 = 4,
    #[doc = "6: SHA-256"]
    SHAMD5_MODE_ALGO_SHA256 = 6,
}
impl From<SHAMD5_MODE_ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAMD5_MODE_ALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SHAMD5_MODE_ALGO` reader - Hash Algorithm"]
pub type SHAMD5_MODE_ALGO_R = crate::FieldReader<u8, SHAMD5_MODE_ALGO_A>;
impl SHAMD5_MODE_ALGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SHAMD5_MODE_ALGO_A> {
        match self.bits {
            0 => Some(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_MD5),
            2 => Some(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA1),
            4 => Some(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA224),
            6 => Some(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_MD5`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_md5(&self) -> bool {
        *self == SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_MD5
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA1`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha1(&self) -> bool {
        *self == SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA1
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA224`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha224(&self) -> bool {
        *self == SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA224
    }
    #[doc = "Checks if the value of the field is `SHAMD5_MODE_ALGO_SHA256`"]
    #[inline(always)]
    pub fn is_shamd5_mode_algo_sha256(&self) -> bool {
        *self == SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA256
    }
}
#[doc = "Field `SHAMD5_MODE_ALGO` writer - Hash Algorithm"]
pub type SHAMD5_MODE_ALGO_W<'a> =
    crate::FieldWriter<'a, u32, MODE_SPEC, u8, SHAMD5_MODE_ALGO_A, 3, 0>;
impl<'a> SHAMD5_MODE_ALGO_W<'a> {
    #[doc = "MD5"]
    #[inline(always)]
    pub fn shamd5_mode_algo_md5(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_MD5)
    }
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha1(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA1)
    }
    #[doc = "SHA-224"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha224(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA224)
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn shamd5_mode_algo_sha256(self) -> &'a mut W {
        self.variant(SHAMD5_MODE_ALGO_A::SHAMD5_MODE_ALGO_SHA256)
    }
}
#[doc = "Field `SHAMD5_MODE_ALGO_CONSTANT` reader - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
pub type SHAMD5_MODE_ALGO_CONSTANT_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_MODE_ALGO_CONSTANT` writer - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
pub type SHAMD5_MODE_ALGO_CONSTANT_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, bool, 3>;
#[doc = "Field `SHAMD5_MODE_CLOSE_HASH` reader - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
pub type SHAMD5_MODE_CLOSE_HASH_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_MODE_CLOSE_HASH` writer - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
pub type SHAMD5_MODE_CLOSE_HASH_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, bool, 4>;
#[doc = "Field `SHAMD5_MODE_HMAC_KEY_PROC` reader - HMAC Key Processing Enable"]
pub type SHAMD5_MODE_HMAC_KEY_PROC_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_MODE_HMAC_KEY_PROC` writer - HMAC Key Processing Enable"]
pub type SHAMD5_MODE_HMAC_KEY_PROC_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, bool, 5>;
#[doc = "Field `SHAMD5_MODE_HMAC_OUTER_HASH` reader - HMAC Outer Hash Processing Enable"]
pub type SHAMD5_MODE_HMAC_OUTER_HASH_R = crate::BitReader<bool>;
#[doc = "Field `SHAMD5_MODE_HMAC_OUTER_HASH` writer - HMAC Outer Hash Processing Enable"]
pub type SHAMD5_MODE_HMAC_OUTER_HASH_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:2 - Hash Algorithm"]
    #[inline(always)]
    pub fn shamd5_mode_algo(&self) -> SHAMD5_MODE_ALGO_R {
        SHAMD5_MODE_ALGO_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
    #[inline(always)]
    pub fn shamd5_mode_algo_constant(&self) -> SHAMD5_MODE_ALGO_CONSTANT_R {
        SHAMD5_MODE_ALGO_CONSTANT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
    #[inline(always)]
    pub fn shamd5_mode_close_hash(&self) -> SHAMD5_MODE_CLOSE_HASH_R {
        SHAMD5_MODE_CLOSE_HASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HMAC Key Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_key_proc(&self) -> SHAMD5_MODE_HMAC_KEY_PROC_R {
        SHAMD5_MODE_HMAC_KEY_PROC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - HMAC Outer Hash Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_outer_hash(&self) -> SHAMD5_MODE_HMAC_OUTER_HASH_R {
        SHAMD5_MODE_HMAC_OUTER_HASH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Hash Algorithm"]
    #[inline(always)]
    pub fn shamd5_mode_algo(&mut self) -> SHAMD5_MODE_ALGO_W {
        SHAMD5_MODE_ALGO_W::new(self)
    }
    #[doc = "Bit 3 - The initial digest register will be overwritten with the algorithm constants for the selected algorithm when hashing and the initial digest count register will be reset to 0"]
    #[inline(always)]
    pub fn shamd5_mode_algo_constant(&mut self) -> SHAMD5_MODE_ALGO_CONSTANT_W {
        SHAMD5_MODE_ALGO_CONSTANT_W::new(self)
    }
    #[doc = "Bit 4 - Performs the padding, the Hash/HMAC will be 'closed' at the end of the block, as per MD5/SHA-1/SHA-2 specification"]
    #[inline(always)]
    pub fn shamd5_mode_close_hash(&mut self) -> SHAMD5_MODE_CLOSE_HASH_W {
        SHAMD5_MODE_CLOSE_HASH_W::new(self)
    }
    #[doc = "Bit 5 - HMAC Key Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_key_proc(&mut self) -> SHAMD5_MODE_HMAC_KEY_PROC_W {
        SHAMD5_MODE_HMAC_KEY_PROC_W::new(self)
    }
    #[doc = "Bit 7 - HMAC Outer Hash Processing Enable"]
    #[inline(always)]
    pub fn shamd5_mode_hmac_outer_hash(&mut self) -> SHAMD5_MODE_HMAC_OUTER_HASH_W {
        SHAMD5_MODE_HMAC_OUTER_HASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
