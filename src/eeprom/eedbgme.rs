#[doc = "Register `EEDBGME` reader"]
pub struct R(crate::R<EEDBGME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEDBGME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEDBGME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEDBGME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEDBGME` writer"]
pub struct W(crate::W<EEDBGME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEDBGME_SPEC>;
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
impl From<crate::W<EEDBGME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEDBGME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEDBGME_ME` reader - Mass Erase"]
pub type EEPROM_EEDBGME_ME_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDBGME_ME` writer - Mass Erase"]
pub type EEPROM_EEDBGME_ME_W<'a> = crate::BitWriter<'a, u32, EEDBGME_SPEC, bool, 0>;
#[doc = "Field `EEPROM_EEDBGME_KEY` reader - Erase Key"]
pub type EEPROM_EEDBGME_KEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EEPROM_EEDBGME_KEY` writer - Erase Key"]
pub type EEPROM_EEDBGME_KEY_W<'a> = crate::FieldWriter<'a, u32, EEDBGME_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn eeprom_eedbgme_me(&self) -> EEPROM_EEDBGME_ME_R {
        EEPROM_EEDBGME_ME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    pub fn eeprom_eedbgme_key(&self) -> EEPROM_EEDBGME_KEY_R {
        EEPROM_EEDBGME_KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase"]
    #[inline(always)]
    pub fn eeprom_eedbgme_me(&mut self) -> EEPROM_EEDBGME_ME_W {
        EEPROM_EEDBGME_ME_W::new(self)
    }
    #[doc = "Bits 16:31 - Erase Key"]
    #[inline(always)]
    pub fn eeprom_eedbgme_key(&mut self) -> EEPROM_EEDBGME_KEY_W {
        EEPROM_EEDBGME_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Debug Mass Erase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eedbgme](index.html) module"]
pub struct EEDBGME_SPEC;
impl crate::RegisterSpec for EEDBGME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eedbgme::R](R) reader structure"]
impl crate::Readable for EEDBGME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eedbgme::W](W) writer structure"]
impl crate::Writable for EEDBGME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEDBGME to value 0"]
impl crate::Resettable for EEDBGME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
