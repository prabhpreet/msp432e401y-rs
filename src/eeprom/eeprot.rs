#[doc = "Register `EEPROT` reader"]
pub struct R(crate::R<EEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEPROT` writer"]
pub struct W(crate::W<EEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEPROT_SPEC>;
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
impl From<crate::W<EEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Protection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEPROM_EEPROT_PROT_A {
    #[doc = "0: This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    EEPROM_EEPROT_PROT_RWNPW = 0,
    #[doc = "1: If there is a password, the block is readable or writable only when unlocked"]
    EEPROM_EEPROT_PROT_RWPW = 1,
    #[doc = "2: If there is no password, the block is readable, not writable"]
    EEPROM_EEPROT_PROT_RONPW = 2,
}
impl From<EEPROM_EEPROT_PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEPROM_EEPROT_PROT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEPROM_EEPROT_PROT` reader - Protection Control"]
pub type EEPROM_EEPROT_PROT_R = crate::FieldReader<u8, EEPROM_EEPROT_PROT_A>;
impl EEPROM_EEPROT_PROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EEPROM_EEPROT_PROT_A> {
        match self.bits {
            0 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW),
            1 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW),
            2 => Some(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RWNPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwnpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RWPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_rwpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW
    }
    #[doc = "Checks if the value of the field is `EEPROM_EEPROT_PROT_RONPW`"]
    #[inline(always)]
    pub fn is_eeprom_eeprot_prot_ronpw(&self) -> bool {
        *self == EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW
    }
}
#[doc = "Field `EEPROM_EEPROT_PROT` writer - Protection Control"]
pub type EEPROM_EEPROT_PROT_W<'a> =
    crate::FieldWriter<'a, u32, EEPROT_SPEC, u8, EEPROM_EEPROT_PROT_A, 3, 0>;
impl<'a> EEPROM_EEPROT_PROT_W<'a> {
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwnpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWNPW)
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_rwpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RWPW)
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot_ronpw(self) -> &'a mut W {
        self.variant(EEPROM_EEPROT_PROT_A::EEPROM_EEPROT_PROT_RONPW)
    }
}
#[doc = "Field `EEPROM_EEPROT_ACC` reader - Access Control"]
pub type EEPROM_EEPROT_ACC_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEPROT_ACC` writer - Access Control"]
pub type EEPROM_EEPROT_ACC_W<'a> = crate::BitWriter<'a, u32, EEPROT_SPEC, bool, 3>;
impl R {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot(&self) -> EEPROM_EEPROT_PROT_R {
        EEPROM_EEPROT_PROT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_acc(&self) -> EEPROM_EEPROT_ACC_R {
        EEPROM_EEPROT_ACC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_prot(&mut self) -> EEPROM_EEPROT_PROT_W {
        EEPROM_EEPROT_PROT_W::new(self)
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline(always)]
    pub fn eeprom_eeprot_acc(&mut self) -> EEPROM_EEPROT_ACC_W {
        EEPROM_EEPROT_ACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeprot](index.html) module"]
pub struct EEPROT_SPEC;
impl crate::RegisterSpec for EEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeprot::R](R) reader structure"]
impl crate::Readable for EEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeprot::W](W) writer structure"]
impl crate::Writable for EEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEPROT to value 0"]
impl crate::Resettable for EEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
