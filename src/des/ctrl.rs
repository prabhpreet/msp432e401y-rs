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
#[doc = "Field `DES_CTRL_OUTPUT_READY` reader - When 1, Data decrypted/encrypted ready"]
pub type DES_CTRL_OUTPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `DES_CTRL_OUTPUT_READY` writer - When 1, Data decrypted/encrypted ready"]
pub type DES_CTRL_OUTPUT_READY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `DES_CTRL_INPUT_READY` reader - When 1, ready to encrypt/decrypt data"]
pub type DES_CTRL_INPUT_READY_R = crate::BitReader<bool>;
#[doc = "Field `DES_CTRL_INPUT_READY` writer - When 1, ready to encrypt/decrypt data"]
pub type DES_CTRL_INPUT_READY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `DES_CTRL_DIRECTION` reader - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
pub type DES_CTRL_DIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `DES_CTRL_DIRECTION` writer - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
pub type DES_CTRL_DIRECTION_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `DES_CTRL_TDES` reader - Select DES or triple DES encryption/decryption"]
pub type DES_CTRL_TDES_R = crate::BitReader<bool>;
#[doc = "Field `DES_CTRL_TDES` writer - Select DES or triple DES encryption/decryption"]
pub type DES_CTRL_TDES_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `DES_CTRL_MODE` reader - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
pub type DES_CTRL_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DES_CTRL_MODE` writer - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
pub type DES_CTRL_MODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DES_CTRL_CONTEXT` reader - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
pub type DES_CTRL_CONTEXT_R = crate::BitReader<bool>;
#[doc = "Field `DES_CTRL_CONTEXT` writer - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
pub type DES_CTRL_CONTEXT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - When 1, Data decrypted/encrypted ready"]
    #[inline(always)]
    pub fn des_ctrl_output_ready(&self) -> DES_CTRL_OUTPUT_READY_R {
        DES_CTRL_OUTPUT_READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, ready to encrypt/decrypt data"]
    #[inline(always)]
    pub fn des_ctrl_input_ready(&self) -> DES_CTRL_INPUT_READY_R {
        DES_CTRL_INPUT_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
    #[inline(always)]
    pub fn des_ctrl_direction(&self) -> DES_CTRL_DIRECTION_R {
        DES_CTRL_DIRECTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select DES or triple DES encryption/decryption"]
    #[inline(always)]
    pub fn des_ctrl_tdes(&self) -> DES_CTRL_TDES_R {
        DES_CTRL_TDES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
    #[inline(always)]
    pub fn des_ctrl_mode(&self) -> DES_CTRL_MODE_R {
        DES_CTRL_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
    #[inline(always)]
    pub fn des_ctrl_context(&self) -> DES_CTRL_CONTEXT_R {
        DES_CTRL_CONTEXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, Data decrypted/encrypted ready"]
    #[inline(always)]
    pub fn des_ctrl_output_ready(&mut self) -> DES_CTRL_OUTPUT_READY_W {
        DES_CTRL_OUTPUT_READY_W::new(self)
    }
    #[doc = "Bit 1 - When 1, ready to encrypt/decrypt data"]
    #[inline(always)]
    pub fn des_ctrl_input_ready(&mut self) -> DES_CTRL_INPUT_READY_W {
        DES_CTRL_INPUT_READY_W::new(self)
    }
    #[doc = "Bit 2 - Select encryption/decryption 0x0: decryption is selected0x1: Encryption is selected"]
    #[inline(always)]
    pub fn des_ctrl_direction(&mut self) -> DES_CTRL_DIRECTION_W {
        DES_CTRL_DIRECTION_W::new(self)
    }
    #[doc = "Bit 3 - Select DES or triple DES encryption/decryption"]
    #[inline(always)]
    pub fn des_ctrl_tdes(&mut self) -> DES_CTRL_TDES_W {
        DES_CTRL_TDES_W::new(self)
    }
    #[doc = "Bits 4:5 - Select CBC, ECB or CFB mode0x0: ECB mode0x1: CBC mode0x2: CFB mode0x3: reserved"]
    #[inline(always)]
    pub fn des_ctrl_mode(&mut self) -> DES_CTRL_MODE_W {
        DES_CTRL_MODE_W::new(self)
    }
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context"]
    #[inline(always)]
    pub fn des_ctrl_context(&mut self) -> DES_CTRL_CONTEXT_W {
        DES_CTRL_CONTEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DES Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
