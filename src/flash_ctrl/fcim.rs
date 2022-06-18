#[doc = "Register `FCIM` reader"]
pub struct R(crate::R<FCIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCIM` writer"]
pub struct W(crate::W<FCIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCIM_SPEC>;
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
impl From<crate::W<FCIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_FCIM_AMASK` reader - Access Interrupt Mask"]
pub type FLASH_FCIM_AMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_AMASK` writer - Access Interrupt Mask"]
pub type FLASH_FCIM_AMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 0>;
#[doc = "Field `FLASH_FCIM_PMASK` reader - Programming Interrupt Mask"]
pub type FLASH_FCIM_PMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_PMASK` writer - Programming Interrupt Mask"]
pub type FLASH_FCIM_PMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 1>;
#[doc = "Field `FLASH_FCIM_EMASK` reader - EEPROM Interrupt Mask"]
pub type FLASH_FCIM_EMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_EMASK` writer - EEPROM Interrupt Mask"]
pub type FLASH_FCIM_EMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 2>;
#[doc = "Field `FLASH_FCIM_VOLTMASK` reader - VOLT Interrupt Mask"]
pub type FLASH_FCIM_VOLTMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_VOLTMASK` writer - VOLT Interrupt Mask"]
pub type FLASH_FCIM_VOLTMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 9>;
#[doc = "Field `FLASH_FCIM_INVDMASK` reader - Invalid Data Interrupt Mask"]
pub type FLASH_FCIM_INVDMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_INVDMASK` writer - Invalid Data Interrupt Mask"]
pub type FLASH_FCIM_INVDMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 10>;
#[doc = "Field `FLASH_FCIM_ERMASK` reader - ERVER Interrupt Mask"]
pub type FLASH_FCIM_ERMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_ERMASK` writer - ERVER Interrupt Mask"]
pub type FLASH_FCIM_ERMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 11>;
#[doc = "Field `FLASH_FCIM_PROGMASK` reader - PROGVER Interrupt Mask"]
pub type FLASH_FCIM_PROGMASK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_FCIM_PROGMASK` writer - PROGVER Interrupt Mask"]
pub type FLASH_FCIM_PROGMASK_W<'a> = crate::BitWriter<'a, u32, FCIM_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&self) -> FLASH_FCIM_AMASK_R {
        FLASH_FCIM_AMASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&self) -> FLASH_FCIM_PMASK_R {
        FLASH_FCIM_PMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_emask(&self) -> FLASH_FCIM_EMASK_R {
        FLASH_FCIM_EMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_voltmask(&self) -> FLASH_FCIM_VOLTMASK_R {
        FLASH_FCIM_VOLTMASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_invdmask(&self) -> FLASH_FCIM_INVDMASK_R {
        FLASH_FCIM_INVDMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_ermask(&self) -> FLASH_FCIM_ERMASK_R {
        FLASH_FCIM_ERMASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_progmask(&self) -> FLASH_FCIM_PROGMASK_R {
        FLASH_FCIM_PROGMASK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_amask(&mut self) -> FLASH_FCIM_AMASK_W {
        FLASH_FCIM_AMASK_W::new(self)
    }
    #[doc = "Bit 1 - Programming Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_pmask(&mut self) -> FLASH_FCIM_PMASK_W {
        FLASH_FCIM_PMASK_W::new(self)
    }
    #[doc = "Bit 2 - EEPROM Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_emask(&mut self) -> FLASH_FCIM_EMASK_W {
        FLASH_FCIM_EMASK_W::new(self)
    }
    #[doc = "Bit 9 - VOLT Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_voltmask(&mut self) -> FLASH_FCIM_VOLTMASK_W {
        FLASH_FCIM_VOLTMASK_W::new(self)
    }
    #[doc = "Bit 10 - Invalid Data Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_invdmask(&mut self) -> FLASH_FCIM_INVDMASK_W {
        FLASH_FCIM_INVDMASK_W::new(self)
    }
    #[doc = "Bit 11 - ERVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_ermask(&mut self) -> FLASH_FCIM_ERMASK_W {
        FLASH_FCIM_ERMASK_W::new(self)
    }
    #[doc = "Bit 13 - PROGVER Interrupt Mask"]
    #[inline(always)]
    pub fn flash_fcim_progmask(&mut self) -> FLASH_FCIM_PROGMASK_W {
        FLASH_FCIM_PROGMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcim](index.html) module"]
pub struct FCIM_SPEC;
impl crate::RegisterSpec for FCIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcim::R](R) reader structure"]
impl crate::Readable for FCIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcim::W](W) writer structure"]
impl crate::Writable for FCIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCIM to value 0"]
impl crate::Resettable for FCIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
