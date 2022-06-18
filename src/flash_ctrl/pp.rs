#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PP_SIZE` reader - Flash Size"]
pub type FLASH_PP_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_PP_SIZE` writer - Flash Size"]
pub type FLASH_PP_SIZE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u16, u16, 16, 0>;
#[doc = "Flash Sector Size of the physical bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_PP_MAINSS_A {
    #[doc = "0: 1 KB"]
    FLASH_PP_MAINSS_1KB = 0,
    #[doc = "1: 2 KB"]
    FLASH_PP_MAINSS_2KB = 1,
    #[doc = "2: 4 KB"]
    FLASH_PP_MAINSS_4KB = 2,
    #[doc = "3: 8 KB"]
    FLASH_PP_MAINSS_8KB = 3,
    #[doc = "4: 16 KB"]
    FLASH_PP_MAINSS_16KB = 4,
}
impl From<FLASH_PP_MAINSS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_PP_MAINSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_PP_MAINSS` reader - Flash Sector Size of the physical bank"]
pub type FLASH_PP_MAINSS_R = crate::FieldReader<u8, FLASH_PP_MAINSS_A>;
impl FLASH_PP_MAINSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_PP_MAINSS_A> {
        match self.bits {
            0 => Some(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_1KB),
            1 => Some(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_2KB),
            2 => Some(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_4KB),
            3 => Some(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_8KB),
            4 => Some(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_16KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_1KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_1kb(&self) -> bool {
        *self == FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_1KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_2KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_2kb(&self) -> bool {
        *self == FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_2KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_4KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_4kb(&self) -> bool {
        *self == FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_4KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_8KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_8kb(&self) -> bool {
        *self == FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_8KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_MAINSS_16KB`"]
    #[inline(always)]
    pub fn is_flash_pp_mainss_16kb(&self) -> bool {
        *self == FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_16KB
    }
}
#[doc = "Field `FLASH_PP_MAINSS` writer - Flash Sector Size of the physical bank"]
pub type FLASH_PP_MAINSS_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, FLASH_PP_MAINSS_A, 3, 16>;
impl<'a> FLASH_PP_MAINSS_W<'a> {
    #[doc = "1 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_1kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_1KB)
    }
    #[doc = "2 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_2kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_2KB)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_4kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_4KB)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_8kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_8KB)
    }
    #[doc = "16 KB"]
    #[inline(always)]
    pub fn flash_pp_mainss_16kb(self) -> &'a mut W {
        self.variant(FLASH_PP_MAINSS_A::FLASH_PP_MAINSS_16KB)
    }
}
#[doc = "EEPROM Sector Size of the physical bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_PP_EESS_A {
    #[doc = "0: 1 KB"]
    FLASH_PP_EESS_1KB = 0,
    #[doc = "1: 2 KB"]
    FLASH_PP_EESS_2KB = 1,
    #[doc = "2: 4 KB"]
    FLASH_PP_EESS_4KB = 2,
    #[doc = "3: 8 KB"]
    FLASH_PP_EESS_8KB = 3,
}
impl From<FLASH_PP_EESS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_PP_EESS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_PP_EESS` reader - EEPROM Sector Size of the physical bank"]
pub type FLASH_PP_EESS_R = crate::FieldReader<u8, FLASH_PP_EESS_A>;
impl FLASH_PP_EESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_PP_EESS_A> {
        match self.bits {
            0 => Some(FLASH_PP_EESS_A::FLASH_PP_EESS_1KB),
            1 => Some(FLASH_PP_EESS_A::FLASH_PP_EESS_2KB),
            2 => Some(FLASH_PP_EESS_A::FLASH_PP_EESS_4KB),
            3 => Some(FLASH_PP_EESS_A::FLASH_PP_EESS_8KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_1KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_1kb(&self) -> bool {
        *self == FLASH_PP_EESS_A::FLASH_PP_EESS_1KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_2KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_2kb(&self) -> bool {
        *self == FLASH_PP_EESS_A::FLASH_PP_EESS_2KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_4KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_4kb(&self) -> bool {
        *self == FLASH_PP_EESS_A::FLASH_PP_EESS_4KB
    }
    #[doc = "Checks if the value of the field is `FLASH_PP_EESS_8KB`"]
    #[inline(always)]
    pub fn is_flash_pp_eess_8kb(&self) -> bool {
        *self == FLASH_PP_EESS_A::FLASH_PP_EESS_8KB
    }
}
#[doc = "Field `FLASH_PP_EESS` writer - EEPROM Sector Size of the physical bank"]
pub type FLASH_PP_EESS_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, FLASH_PP_EESS_A, 4, 19>;
impl<'a> FLASH_PP_EESS_W<'a> {
    #[doc = "1 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_1kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESS_A::FLASH_PP_EESS_1KB)
    }
    #[doc = "2 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_2kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESS_A::FLASH_PP_EESS_2KB)
    }
    #[doc = "4 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_4kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESS_A::FLASH_PP_EESS_4KB)
    }
    #[doc = "8 KB"]
    #[inline(always)]
    pub fn flash_pp_eess_8kb(self) -> &'a mut W {
        self.variant(FLASH_PP_EESS_A::FLASH_PP_EESS_8KB)
    }
}
#[doc = "Field `FLASH_PP_DFA` reader - DMA Flash Access"]
pub type FLASH_PP_DFA_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PP_DFA` writer - DMA Flash Access"]
pub type FLASH_PP_DFA_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 28>;
#[doc = "Field `FLASH_PP_FMM` reader - Flash Mirror Mode"]
pub type FLASH_PP_FMM_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PP_FMM` writer - Flash Mirror Mode"]
pub type FLASH_PP_FMM_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 29>;
#[doc = "Field `FLASH_PP_PFC` reader - Prefetch Buffer Mode"]
pub type FLASH_PP_PFC_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_PP_PFC` writer - Prefetch Buffer Mode"]
pub type FLASH_PP_PFC_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 30>;
impl R {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash_pp_size(&self) -> FLASH_PP_SIZE_R {
        FLASH_PP_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_mainss(&self) -> FLASH_PP_MAINSS_R {
        FLASH_PP_MAINSS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_eess(&self) -> FLASH_PP_EESS_R {
        FLASH_PP_EESS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline(always)]
    pub fn flash_pp_dfa(&self) -> FLASH_PP_DFA_R {
        FLASH_PP_DFA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline(always)]
    pub fn flash_pp_fmm(&self) -> FLASH_PP_FMM_R {
        FLASH_PP_FMM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline(always)]
    pub fn flash_pp_pfc(&self) -> FLASH_PP_PFC_R {
        FLASH_PP_PFC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Size"]
    #[inline(always)]
    pub fn flash_pp_size(&mut self) -> FLASH_PP_SIZE_W {
        FLASH_PP_SIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Flash Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_mainss(&mut self) -> FLASH_PP_MAINSS_W {
        FLASH_PP_MAINSS_W::new(self)
    }
    #[doc = "Bits 19:22 - EEPROM Sector Size of the physical bank"]
    #[inline(always)]
    pub fn flash_pp_eess(&mut self) -> FLASH_PP_EESS_W {
        FLASH_PP_EESS_W::new(self)
    }
    #[doc = "Bit 28 - DMA Flash Access"]
    #[inline(always)]
    pub fn flash_pp_dfa(&mut self) -> FLASH_PP_DFA_W {
        FLASH_PP_DFA_W::new(self)
    }
    #[doc = "Bit 29 - Flash Mirror Mode"]
    #[inline(always)]
    pub fn flash_pp_fmm(&mut self) -> FLASH_PP_FMM_W {
        FLASH_PP_FMM_W::new(self)
    }
    #[doc = "Bit 30 - Prefetch Buffer Mode"]
    #[inline(always)]
    pub fn flash_pp_pfc(&mut self) -> FLASH_PP_PFC_W {
        FLASH_PP_PFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
