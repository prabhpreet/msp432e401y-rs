#[doc = "Register `EESUPP` reader"]
pub struct R(crate::R<EESUPP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESUPP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESUPP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESUPP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESUPP` writer"]
pub struct W(crate::W<EESUPP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESUPP_SPEC>;
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
impl From<crate::W<EESUPP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESUPP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EESUPP_ERETRY` reader - Erase Must Be Retried"]
pub type EEPROM_EESUPP_ERETRY_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EESUPP_ERETRY` writer - Erase Must Be Retried"]
pub type EEPROM_EESUPP_ERETRY_W<'a> = crate::BitWriter<'a, u32, EESUPP_SPEC, bool, 2>;
#[doc = "Field `EEPROM_EESUPP_PRETRY` reader - Programming Must Be Retried"]
pub type EEPROM_EESUPP_PRETRY_R = crate::BitReader<bool>;
#[doc = "Field `EEPROM_EESUPP_PRETRY` writer - Programming Must Be Retried"]
pub type EEPROM_EESUPP_PRETRY_W<'a> = crate::BitWriter<'a, u32, EESUPP_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_eretry(&self) -> EEPROM_EESUPP_ERETRY_R {
        EEPROM_EESUPP_ERETRY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_pretry(&self) -> EEPROM_EESUPP_PRETRY_R {
        EEPROM_EESUPP_PRETRY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Erase Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_eretry(&mut self) -> EEPROM_EESUPP_ERETRY_W {
        EEPROM_EESUPP_ERETRY_W::new(self)
    }
    #[doc = "Bit 3 - Programming Must Be Retried"]
    #[inline(always)]
    pub fn eeprom_eesupp_pretry(&mut self) -> EEPROM_EESUPP_PRETRY_W {
        EEPROM_EESUPP_PRETRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Support Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesupp](index.html) module"]
pub struct EESUPP_SPEC;
impl crate::RegisterSpec for EESUPP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesupp::R](R) reader structure"]
impl crate::Readable for EESUPP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesupp::W](W) writer structure"]
impl crate::Writable for EESUPP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EESUPP to value 0"]
impl crate::Resettable for EESUPP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
