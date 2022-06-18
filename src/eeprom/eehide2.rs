#[doc = "Register `EEHIDE2` reader"]
pub struct R(crate::R<EEHIDE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEHIDE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEHIDE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEHIDE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEHIDE2` writer"]
pub struct W(crate::W<EEHIDE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEHIDE2_SPEC>;
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
impl From<crate::W<EEHIDE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEHIDE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEHIDE2_HN` reader - Hide Block"]
pub type EEPROM_EEHIDE2_HN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EEPROM_EEHIDE2_HN` writer - Hide Block"]
pub type EEPROM_EEHIDE2_HN_W<'a> = crate::FieldWriter<'a, u32, EEHIDE2_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Hide Block"]
    #[inline(always)]
    pub fn eeprom_eehide2_hn(&self) -> EEPROM_EEHIDE2_HN_R {
        EEPROM_EEHIDE2_HN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hide Block"]
    #[inline(always)]
    pub fn eeprom_eehide2_hn(&mut self) -> EEPROM_EEHIDE2_HN_W {
        EEPROM_EEHIDE2_HN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Block Hide 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eehide2](index.html) module"]
pub struct EEHIDE2_SPEC;
impl crate::RegisterSpec for EEHIDE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eehide2::R](R) reader structure"]
impl crate::Readable for EEHIDE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eehide2::W](W) writer structure"]
impl crate::Writable for EEHIDE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEHIDE2 to value 0"]
impl crate::Resettable for EEHIDE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
