#[doc = "Register `EERDWR` reader"]
pub struct R(crate::R<EERDWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EERDWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EERDWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EERDWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EERDWR` writer"]
pub struct W(crate::W<EERDWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EERDWR_SPEC>;
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
impl From<crate::W<EERDWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EERDWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EERDWR_VALUE` reader - EEPROM Read or Write Data"]
pub type EEPROM_EERDWR_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EEPROM_EERDWR_VALUE` writer - EEPROM Read or Write Data"]
pub type EEPROM_EERDWR_VALUE_W<'a> = crate::FieldWriter<'a, u32, EERDWR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data"]
    #[inline(always)]
    pub fn eeprom_eerdwr_value(&self) -> EEPROM_EERDWR_VALUE_R {
        EEPROM_EERDWR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - EEPROM Read or Write Data"]
    #[inline(always)]
    pub fn eeprom_eerdwr_value(&mut self) -> EEPROM_EERDWR_VALUE_W {
        EEPROM_EERDWR_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Read-Write\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eerdwr](index.html) module"]
pub struct EERDWR_SPEC;
impl crate::RegisterSpec for EERDWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eerdwr::R](R) reader structure"]
impl crate::Readable for EERDWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eerdwr::W](W) writer structure"]
impl crate::Writable for EERDWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EERDWR to value 0"]
impl crate::Resettable for EERDWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
