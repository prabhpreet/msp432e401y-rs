#[doc = "Register `EEHIDE1` reader"]
pub struct R(crate::R<EEHIDE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEHIDE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEHIDE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEHIDE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEHIDE1` writer"]
pub struct W(crate::W<EEHIDE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEHIDE1_SPEC>;
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
impl From<crate::W<EEHIDE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEHIDE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEHIDE1_HN` reader - Hide Block"]
pub type EEPROM_EEHIDE1_HN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EEPROM_EEHIDE1_HN` writer - Hide Block"]
pub type EEPROM_EEHIDE1_HN_W<'a> = crate::FieldWriter<'a, u32, EEHIDE1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hide Block"]
    #[inline(always)]
    pub fn eeprom_eehide1_hn(&self) -> EEPROM_EEHIDE1_HN_R {
        EEPROM_EEHIDE1_HN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hide Block"]
    #[inline(always)]
    pub fn eeprom_eehide1_hn(&mut self) -> EEPROM_EEHIDE1_HN_W {
        EEPROM_EEHIDE1_HN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Block Hide 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eehide1](index.html) module"]
pub struct EEHIDE1_SPEC;
impl crate::RegisterSpec for EEHIDE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eehide1::R](R) reader structure"]
impl crate::Readable for EEHIDE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eehide1::W](W) writer structure"]
impl crate::Writable for EEHIDE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEHIDE1 to value 0"]
impl crate::Resettable for EEHIDE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
