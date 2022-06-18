#[doc = "Register `BOOTCFG` reader"]
pub struct R(crate::R<BOOTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTCFG` writer"]
pub struct W(crate::W<BOOTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTCFG_SPEC>;
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
impl From<crate::W<BOOTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_BOOTCFG_DBG0` reader - Debug Control 0"]
pub type FLASH_BOOTCFG_DBG0_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_DBG0` writer - Debug Control 0"]
pub type FLASH_BOOTCFG_DBG0_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 0>;
#[doc = "Field `FLASH_BOOTCFG_DBG1` reader - Debug Control 1"]
pub type FLASH_BOOTCFG_DBG1_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_DBG1` writer - Debug Control 1"]
pub type FLASH_BOOTCFG_DBG1_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 1>;
#[doc = "Field `FLASH_BOOTCFG_KEY` reader - KEY Select"]
pub type FLASH_BOOTCFG_KEY_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_KEY` writer - KEY Select"]
pub type FLASH_BOOTCFG_KEY_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 4>;
#[doc = "Field `FLASH_BOOTCFG_EN` reader - Boot GPIO Enable"]
pub type FLASH_BOOTCFG_EN_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_EN` writer - Boot GPIO Enable"]
pub type FLASH_BOOTCFG_EN_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 8>;
#[doc = "Field `FLASH_BOOTCFG_POL` reader - Boot GPIO Polarity"]
pub type FLASH_BOOTCFG_POL_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_POL` writer - Boot GPIO Polarity"]
pub type FLASH_BOOTCFG_POL_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 9>;
#[doc = "Boot GPIO Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_BOOTCFG_PIN_A {
    #[doc = "0: Pin 0"]
    FLASH_BOOTCFG_PIN_0 = 0,
    #[doc = "1: Pin 1"]
    FLASH_BOOTCFG_PIN_1 = 1,
    #[doc = "2: Pin 2"]
    FLASH_BOOTCFG_PIN_2 = 2,
    #[doc = "3: Pin 3"]
    FLASH_BOOTCFG_PIN_3 = 3,
    #[doc = "4: Pin 4"]
    FLASH_BOOTCFG_PIN_4 = 4,
    #[doc = "5: Pin 5"]
    FLASH_BOOTCFG_PIN_5 = 5,
    #[doc = "6: Pin 6"]
    FLASH_BOOTCFG_PIN_6 = 6,
    #[doc = "7: Pin 7"]
    FLASH_BOOTCFG_PIN_7 = 7,
}
impl From<FLASH_BOOTCFG_PIN_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_BOOTCFG_PIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_BOOTCFG_PIN` reader - Boot GPIO Pin"]
pub type FLASH_BOOTCFG_PIN_R = crate::FieldReader<u8, FLASH_BOOTCFG_PIN_A>;
impl FLASH_BOOTCFG_PIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_BOOTCFG_PIN_A {
        match self.bits {
            0 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_0,
            1 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_1,
            2 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_2,
            3 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_3,
            4 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_4,
            5 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_5,
            6 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_6,
            7 => FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_0`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_0(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_0
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_1`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_1(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_1
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_2`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_2(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_2
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_3`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_3(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_3
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_4`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_4(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_4
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_5`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_5(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_5
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_6`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_6(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_6
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_7`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_7(&self) -> bool {
        *self == FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_7
    }
}
#[doc = "Field `FLASH_BOOTCFG_PIN` writer - Boot GPIO Pin"]
pub type FLASH_BOOTCFG_PIN_W<'a> =
    crate::FieldWriterSafe<'a, u32, BOOTCFG_SPEC, u8, FLASH_BOOTCFG_PIN_A, 3, 10>;
impl<'a> FLASH_BOOTCFG_PIN_W<'a> {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_0(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_1(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_2(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_3(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_3)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_4(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_5(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_6(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_7(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PIN_A::FLASH_BOOTCFG_PIN_7)
    }
}
#[doc = "Boot GPIO Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_BOOTCFG_PORT_A {
    #[doc = "0: Port A"]
    FLASH_BOOTCFG_PORT_A = 0,
    #[doc = "1: Port B"]
    FLASH_BOOTCFG_PORT_B = 1,
    #[doc = "2: Port C"]
    FLASH_BOOTCFG_PORT_C = 2,
    #[doc = "3: Port D"]
    FLASH_BOOTCFG_PORT_D = 3,
    #[doc = "4: Port E"]
    FLASH_BOOTCFG_PORT_E = 4,
    #[doc = "5: Port F"]
    FLASH_BOOTCFG_PORT_F = 5,
    #[doc = "6: Port G"]
    FLASH_BOOTCFG_PORT_G = 6,
    #[doc = "7: Port H"]
    FLASH_BOOTCFG_PORT_H = 7,
}
impl From<FLASH_BOOTCFG_PORT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_BOOTCFG_PORT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH_BOOTCFG_PORT` reader - Boot GPIO Port"]
pub type FLASH_BOOTCFG_PORT_R = crate::FieldReader<u8, FLASH_BOOTCFG_PORT_A>;
impl FLASH_BOOTCFG_PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_BOOTCFG_PORT_A {
        match self.bits {
            0 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_A,
            1 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_B,
            2 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_C,
            3 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_D,
            4 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_E,
            5 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_F,
            6 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_G,
            7 => FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_A`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_a(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_A
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_B`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_b(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_B
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_C`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_c(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_C
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_D`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_d(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_D
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_E`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_e(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_E
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_F`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_f(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_F
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_G`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_g(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_G
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_H`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_h(&self) -> bool {
        *self == FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_H
    }
}
#[doc = "Field `FLASH_BOOTCFG_PORT` writer - Boot GPIO Port"]
pub type FLASH_BOOTCFG_PORT_W<'a> =
    crate::FieldWriterSafe<'a, u32, BOOTCFG_SPEC, u8, FLASH_BOOTCFG_PORT_A, 3, 13>;
impl<'a> FLASH_BOOTCFG_PORT_W<'a> {
    #[doc = "Port A"]
    #[inline(always)]
    pub fn flash_bootcfg_port_a(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_A)
    }
    #[doc = "Port B"]
    #[inline(always)]
    pub fn flash_bootcfg_port_b(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_B)
    }
    #[doc = "Port C"]
    #[inline(always)]
    pub fn flash_bootcfg_port_c(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_C)
    }
    #[doc = "Port D"]
    #[inline(always)]
    pub fn flash_bootcfg_port_d(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_D)
    }
    #[doc = "Port E"]
    #[inline(always)]
    pub fn flash_bootcfg_port_e(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_E)
    }
    #[doc = "Port F"]
    #[inline(always)]
    pub fn flash_bootcfg_port_f(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_F)
    }
    #[doc = "Port G"]
    #[inline(always)]
    pub fn flash_bootcfg_port_g(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_G)
    }
    #[doc = "Port H"]
    #[inline(always)]
    pub fn flash_bootcfg_port_h(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORT_A::FLASH_BOOTCFG_PORT_H)
    }
}
#[doc = "Field `FLASH_BOOTCFG_NW` reader - Not Written"]
pub type FLASH_BOOTCFG_NW_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_BOOTCFG_NW` writer - Not Written"]
pub type FLASH_BOOTCFG_NW_W<'a> = crate::BitWriter<'a, u32, BOOTCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg0(&self) -> FLASH_BOOTCFG_DBG0_R {
        FLASH_BOOTCFG_DBG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg1(&self) -> FLASH_BOOTCFG_DBG1_R {
        FLASH_BOOTCFG_DBG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline(always)]
    pub fn flash_bootcfg_key(&self) -> FLASH_BOOTCFG_KEY_R {
        FLASH_BOOTCFG_KEY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline(always)]
    pub fn flash_bootcfg_en(&self) -> FLASH_BOOTCFG_EN_R {
        FLASH_BOOTCFG_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline(always)]
    pub fn flash_bootcfg_pol(&self) -> FLASH_BOOTCFG_POL_R {
        FLASH_BOOTCFG_POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline(always)]
    pub fn flash_bootcfg_pin(&self) -> FLASH_BOOTCFG_PIN_R {
        FLASH_BOOTCFG_PIN_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline(always)]
    pub fn flash_bootcfg_port(&self) -> FLASH_BOOTCFG_PORT_R {
        FLASH_BOOTCFG_PORT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_bootcfg_nw(&self) -> FLASH_BOOTCFG_NW_R {
        FLASH_BOOTCFG_NW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg0(&mut self) -> FLASH_BOOTCFG_DBG0_W {
        FLASH_BOOTCFG_DBG0_W::new(self)
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg1(&mut self) -> FLASH_BOOTCFG_DBG1_W {
        FLASH_BOOTCFG_DBG1_W::new(self)
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline(always)]
    pub fn flash_bootcfg_key(&mut self) -> FLASH_BOOTCFG_KEY_W {
        FLASH_BOOTCFG_KEY_W::new(self)
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline(always)]
    pub fn flash_bootcfg_en(&mut self) -> FLASH_BOOTCFG_EN_W {
        FLASH_BOOTCFG_EN_W::new(self)
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline(always)]
    pub fn flash_bootcfg_pol(&mut self) -> FLASH_BOOTCFG_POL_W {
        FLASH_BOOTCFG_POL_W::new(self)
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline(always)]
    pub fn flash_bootcfg_pin(&mut self) -> FLASH_BOOTCFG_PIN_W {
        FLASH_BOOTCFG_PIN_W::new(self)
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline(always)]
    pub fn flash_bootcfg_port(&mut self) -> FLASH_BOOTCFG_PORT_W {
        FLASH_BOOTCFG_PORT_W::new(self)
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_bootcfg_nw(&mut self) -> FLASH_BOOTCFG_NW_W {
        FLASH_BOOTCFG_NW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootcfg](index.html) module"]
pub struct BOOTCFG_SPEC;
impl crate::RegisterSpec for BOOTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootcfg::R](R) reader structure"]
impl crate::Readable for BOOTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootcfg::W](W) writer structure"]
impl crate::Writable for BOOTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOTCFG to value 0"]
impl crate::Resettable for BOOTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
