#[doc = "Register `EERDWRINC` reader"]
pub struct R(crate::R<EERDWRINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EERDWRINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EERDWRINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EERDWRINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EERDWRINC` writer"]
pub struct W(crate::W<EERDWRINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EERDWRINC_SPEC>;
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
impl From<crate::W<EERDWRINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EERDWRINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EERDWRINC_VALUE` reader - EEPROM Read or Write Data with Increment"]
pub type EEPROM_EERDWRINC_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EEPROM_EERDWRINC_VALUE` writer - EEPROM Read or Write Data with Increment"]
pub type EEPROM_EERDWRINC_VALUE_W<'a> =
    crate::FieldWriter<'a, u32, EERDWRINC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data with Increment"]
    #[inline(always)]
    pub fn eeprom_eerdwrinc_value(&self) -> EEPROM_EERDWRINC_VALUE_R {
        EEPROM_EERDWRINC_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data with Increment"]
    #[inline(always)]
    pub fn eeprom_eerdwrinc_value(&mut self) -> EEPROM_EERDWRINC_VALUE_W {
        EEPROM_EERDWRINC_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Read-Write with Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eerdwrinc](index.html) module"]
pub struct EERDWRINC_SPEC;
impl crate::RegisterSpec for EERDWRINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eerdwrinc::R](R) reader structure"]
impl crate::Readable for EERDWRINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eerdwrinc::W](W) writer structure"]
impl crate::Writable for EERDWRINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EERDWRINC to value 0"]
impl crate::Resettable for EERDWRINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
