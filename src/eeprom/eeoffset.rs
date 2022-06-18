#[doc = "Register `EEOFFSET` reader"]
pub struct R(crate::R<EEOFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEOFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEOFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEOFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEOFFSET` writer"]
pub struct W(crate::W<EEOFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEOFFSET_SPEC>;
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
impl From<crate::W<EEOFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEOFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEOFFSET_OFFSET` reader - Current Address Offset"]
pub type EEPROM_EEOFFSET_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EEPROM_EEOFFSET_OFFSET` writer - Current Address Offset"]
pub type EEPROM_EEOFFSET_OFFSET_W<'a> = crate::FieldWriter<'a, u32, EEOFFSET_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Current Address Offset"]
    #[inline(always)]
    pub fn eeprom_eeoffset_offset(&self) -> EEPROM_EEOFFSET_OFFSET_R {
        EEPROM_EEOFFSET_OFFSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current Address Offset"]
    #[inline(always)]
    pub fn eeprom_eeoffset_offset(&mut self) -> EEPROM_EEOFFSET_OFFSET_W {
        EEPROM_EEOFFSET_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Current Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeoffset](index.html) module"]
pub struct EEOFFSET_SPEC;
impl crate::RegisterSpec for EEOFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeoffset::R](R) reader structure"]
impl crate::Readable for EEOFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeoffset::W](W) writer structure"]
impl crate::Writable for EEOFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEOFFSET to value 0"]
impl crate::Resettable for EEOFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
