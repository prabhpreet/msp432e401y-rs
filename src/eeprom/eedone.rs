#[doc = "Register `EEDONE` reader"]
pub struct R(crate::R<EEDONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEDONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEDONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEDONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEDONE` writer"]
pub struct W(crate::W<EEDONE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEDONE_SPEC>;
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
impl From<crate::W<EEDONE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEDONE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEDONE_WORKING` reader - EEPROM Working"]
pub type EEPROM_EEDONE_WORKING_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDONE_WORKING` writer - EEPROM Working"]
pub type EEPROM_EEDONE_WORKING_W<'a> = crate::BitWriter<'a, u32, EEDONE_SPEC, bool, 0>;
#[doc = "Field `EEPROM_EEDONE_WKERASE` reader - Working on an Erase"]
pub type EEPROM_EEDONE_WKERASE_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDONE_WKERASE` writer - Working on an Erase"]
pub type EEPROM_EEDONE_WKERASE_W<'a> = crate::BitWriter<'a, u32, EEDONE_SPEC, bool, 2>;
#[doc = "Field `EEPROM_EEDONE_WKCOPY` reader - Working on a Copy"]
pub type EEPROM_EEDONE_WKCOPY_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDONE_WKCOPY` writer - Working on a Copy"]
pub type EEPROM_EEDONE_WKCOPY_W<'a> = crate::BitWriter<'a, u32, EEDONE_SPEC, bool, 3>;
#[doc = "Field `EEPROM_EEDONE_NOPERM` reader - Write Without Permission"]
pub type EEPROM_EEDONE_NOPERM_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDONE_NOPERM` writer - Write Without Permission"]
pub type EEPROM_EEDONE_NOPERM_W<'a> = crate::BitWriter<'a, u32, EEDONE_SPEC, bool, 4>;
#[doc = "Field `EEPROM_EEDONE_WRBUSY` reader - Write Busy"]
pub type EEPROM_EEDONE_WRBUSY_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEDONE_WRBUSY` writer - Write Busy"]
pub type EEPROM_EEDONE_WRBUSY_W<'a> = crate::BitWriter<'a, u32, EEDONE_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn eeprom_eedone_working(&self) -> EEPROM_EEDONE_WORKING_R {
        EEPROM_EEDONE_WORKING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn eeprom_eedone_wkerase(&self) -> EEPROM_EEDONE_WKERASE_R {
        EEPROM_EEDONE_WKERASE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn eeprom_eedone_wkcopy(&self) -> EEPROM_EEDONE_WKCOPY_R {
        EEPROM_EEDONE_WKCOPY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn eeprom_eedone_noperm(&self) -> EEPROM_EEDONE_NOPERM_R {
        EEPROM_EEDONE_NOPERM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn eeprom_eedone_wrbusy(&self) -> EEPROM_EEDONE_WRBUSY_R {
        EEPROM_EEDONE_WRBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Working"]
    #[inline(always)]
    pub fn eeprom_eedone_working(&mut self) -> EEPROM_EEDONE_WORKING_W {
        EEPROM_EEDONE_WORKING_W::new(self)
    }
    #[doc = "Bit 2 - Working on an Erase"]
    #[inline(always)]
    pub fn eeprom_eedone_wkerase(&mut self) -> EEPROM_EEDONE_WKERASE_W {
        EEPROM_EEDONE_WKERASE_W::new(self)
    }
    #[doc = "Bit 3 - Working on a Copy"]
    #[inline(always)]
    pub fn eeprom_eedone_wkcopy(&mut self) -> EEPROM_EEDONE_WKCOPY_W {
        EEPROM_EEDONE_WKCOPY_W::new(self)
    }
    #[doc = "Bit 4 - Write Without Permission"]
    #[inline(always)]
    pub fn eeprom_eedone_noperm(&mut self) -> EEPROM_EEDONE_NOPERM_W {
        EEPROM_EEDONE_NOPERM_W::new(self)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn eeprom_eedone_wrbusy(&mut self) -> EEPROM_EEDONE_WRBUSY_W {
        EEPROM_EEDONE_WRBUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Done Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eedone](index.html) module"]
pub struct EEDONE_SPEC;
impl crate::RegisterSpec for EEDONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eedone::R](R) reader structure"]
impl crate::Readable for EEDONE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eedone::W](W) writer structure"]
impl crate::Writable for EEDONE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEDONE to value 0"]
impl crate::Resettable for EEDONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
