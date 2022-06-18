#[doc = "Register `EEPASS0` reader"]
pub struct R(crate::R<EEPASS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEPASS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEPASS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEPASS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEPASS0` writer"]
pub struct W(crate::W<EEPASS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEPASS0_SPEC>;
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
impl From<crate::W<EEPASS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEPASS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEPASS0_PASS` reader - Password"]
pub type EEPROM_EEPASS0_PASS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EEPROM_EEPASS0_PASS` writer - Password"]
pub type EEPROM_EEPASS0_PASS_W<'a> = crate::FieldWriter<'a, u32, EEPASS0_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    pub fn eeprom_eepass0_pass(&self) -> EEPROM_EEPASS0_PASS_R {
        EEPROM_EEPASS0_PASS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Password"]
    #[inline(always)]
    pub fn eeprom_eepass0_pass(&mut self) -> EEPROM_EEPASS0_PASS_W {
        EEPROM_EEPASS0_PASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Password\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eepass0](index.html) module"]
pub struct EEPASS0_SPEC;
impl crate::RegisterSpec for EEPASS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eepass0::R](R) reader structure"]
impl crate::Readable for EEPASS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eepass0::W](W) writer structure"]
impl crate::Writable for EEPASS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEPASS0 to value 0"]
impl crate::Resettable for EEPASS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
