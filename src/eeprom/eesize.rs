#[doc = "Register `EESIZE` reader"]
pub struct R(crate::R<EESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EESIZE` writer"]
pub struct W(crate::W<EESIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EESIZE_SPEC>;
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
impl From<crate::W<EESIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EESIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EESIZE_WORDCNT` reader - Number of 32-Bit Words"]
pub type EEPROM_EESIZE_WORDCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EEPROM_EESIZE_WORDCNT` writer - Number of 32-Bit Words"]
pub type EEPROM_EESIZE_WORDCNT_W<'a> = crate::FieldWriter<'a, u32, EESIZE_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EEPROM_EESIZE_BLKCNT` reader - Number of 16-Word Blocks"]
pub type EEPROM_EESIZE_BLKCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EEPROM_EESIZE_BLKCNT` writer - Number of 16-Word Blocks"]
pub type EEPROM_EESIZE_BLKCNT_W<'a> = crate::FieldWriter<'a, u32, EESIZE_SPEC, u16, u16, 11, 16>;
impl R {
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn eeprom_eesize_wordcnt(&self) -> EEPROM_EESIZE_WORDCNT_R {
        EEPROM_EESIZE_WORDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn eeprom_eesize_blkcnt(&self) -> EEPROM_EESIZE_BLKCNT_R {
        EEPROM_EESIZE_BLKCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of 32-Bit Words"]
    #[inline(always)]
    pub fn eeprom_eesize_wordcnt(&mut self) -> EEPROM_EESIZE_WORDCNT_W {
        EEPROM_EESIZE_WORDCNT_W::new(self)
    }
    #[doc = "Bits 16:26 - Number of 16-Word Blocks"]
    #[inline(always)]
    pub fn eeprom_eesize_blkcnt(&mut self) -> EEPROM_EESIZE_BLKCNT_W {
        EEPROM_EESIZE_BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Size Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eesize](index.html) module"]
pub struct EESIZE_SPEC;
impl crate::RegisterSpec for EESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eesize::R](R) reader structure"]
impl crate::Readable for EESIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eesize::W](W) writer structure"]
impl crate::Writable for EESIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EESIZE to value 0"]
impl crate::Resettable for EESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
