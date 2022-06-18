#[doc = "Register `EEINT` reader"]
pub struct R(crate::R<EEINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEINT` writer"]
pub struct W(crate::W<EEINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEINT_SPEC>;
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
impl From<crate::W<EEINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEINT_INT` reader - Interrupt Enable"]
pub type EEPROM_EEINT_INT_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EEINT_INT` writer - Interrupt Enable"]
pub type EEPROM_EEINT_INT_W<'a> = crate::BitWriter<'a, u32, EEINT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn eeprom_eeint_int(&self) -> EEPROM_EEINT_INT_R {
        EEPROM_EEINT_INT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn eeprom_eeint_int(&mut self) -> EEPROM_EEINT_INT_W {
        EEPROM_EEINT_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeint](index.html) module"]
pub struct EEINT_SPEC;
impl crate::RegisterSpec for EEINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeint::R](R) reader structure"]
impl crate::Readable for EEINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeint::W](W) writer structure"]
impl crate::Writable for EEINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEINT to value 0"]
impl crate::Resettable for EEINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
