#[doc = "Register `FCRIS` reader"]
pub struct R(crate::R<FCRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCRIS` writer"]
pub struct W(crate::W<FCRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCRIS_SPEC>;
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
impl From<crate::W<FCRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FCRIS_ARIS` reader - Access Raw Interrupt Status"]
pub type FLASH_FCRIS_ARIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_ARIS` writer - Access Raw Interrupt Status"]
pub type FLASH_FCRIS_ARIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 0>;
#[doc = "Field `FLASH_FCRIS_PRIS` reader - Programming Raw Interrupt Status"]
pub type FLASH_FCRIS_PRIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_PRIS` writer - Programming Raw Interrupt Status"]
pub type FLASH_FCRIS_PRIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 1>;
#[doc = "Field `FLASH_FCRIS_ERIS` reader - EEPROM Raw Interrupt Status"]
pub type FLASH_FCRIS_ERIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_ERIS` writer - EEPROM Raw Interrupt Status"]
pub type FLASH_FCRIS_ERIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 2>;
#[doc = "Field `FLASH_FCRIS_VOLTRIS` reader - Pump Voltage Raw Interrupt Status"]
pub type FLASH_FCRIS_VOLTRIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_VOLTRIS` writer - Pump Voltage Raw Interrupt Status"]
pub type FLASH_FCRIS_VOLTRIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 9>;
#[doc = "Field `FLASH_FCRIS_INVDRIS` reader - Invalid Data Raw Interrupt Status"]
pub type FLASH_FCRIS_INVDRIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_INVDRIS` writer - Invalid Data Raw Interrupt Status"]
pub type FLASH_FCRIS_INVDRIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 10>;
#[doc = "Field `FLASH_FCRIS_ERRIS` reader - Erase Verify Error Raw Interrupt Status"]
pub type FLASH_FCRIS_ERRIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_ERRIS` writer - Erase Verify Error Raw Interrupt Status"]
pub type FLASH_FCRIS_ERRIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 11>;
#[doc = "Field `FLASH_FCRIS_PROGRIS` reader - Program Verify Error Raw Interrupt Status"]
pub type FLASH_FCRIS_PROGRIS_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCRIS_PROGRIS` writer - Program Verify Error Raw Interrupt Status"]
pub type FLASH_FCRIS_PROGRIS_W<'a> = crate::BitWriter<'a, u32, FCRIS_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&self) -> FLASH_FCRIS_ARIS_R {
        FLASH_FCRIS_ARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&self) -> FLASH_FCRIS_PRIS_R {
        FLASH_FCRIS_PRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_eris(&self) -> FLASH_FCRIS_ERIS_R {
        FLASH_FCRIS_ERIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - Pump Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_voltris(&self) -> FLASH_FCRIS_VOLTRIS_R {
        FLASH_FCRIS_VOLTRIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_invdris(&self) -> FLASH_FCRIS_INVDRIS_R {
        FLASH_FCRIS_INVDRIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Erase Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_erris(&self) -> FLASH_FCRIS_ERRIS_R {
        FLASH_FCRIS_ERRIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Program Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_progris(&self) -> FLASH_FCRIS_PROGRIS_R {
        FLASH_FCRIS_PROGRIS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_aris(&mut self) -> FLASH_FCRIS_ARIS_W {
        FLASH_FCRIS_ARIS_W::new(self)
    }
    #[doc = "Bit 1 - Programming Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_pris(&mut self) -> FLASH_FCRIS_PRIS_W {
        FLASH_FCRIS_PRIS_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_eris(&mut self) -> FLASH_FCRIS_ERIS_W {
        FLASH_FCRIS_ERIS_W::new(self)
    }
    #[doc = "Bit 9 - Pump Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_voltris(&mut self) -> FLASH_FCRIS_VOLTRIS_W {
        FLASH_FCRIS_VOLTRIS_W::new(self)
    }
    #[doc = "Bit 10 - Invalid Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_invdris(&mut self) -> FLASH_FCRIS_INVDRIS_W {
        FLASH_FCRIS_INVDRIS_W::new(self)
    }
    #[doc = "Bit 11 - Erase Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_erris(&mut self) -> FLASH_FCRIS_ERRIS_W {
        FLASH_FCRIS_ERRIS_W::new(self)
    }
    #[doc = "Bit 13 - Program Verify Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn flash_fcris_progris(&mut self) -> FLASH_FCRIS_PROGRIS_W {
        FLASH_FCRIS_PROGRIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcris](index.html) module"]
pub struct FCRIS_SPEC;
impl crate::RegisterSpec for FCRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcris::R](R) reader structure"]
impl crate::Readable for FCRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcris::W](W) writer structure"]
impl crate::Writable for FCRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCRIS to value 0"]
impl crate::Resettable for FCRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
