#[doc = "Register `EEBLOCK` reader"]
pub struct R(crate::R<EEBLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEBLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEBLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEBLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEBLOCK` writer"]
pub struct W(crate::W<EEBLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEBLOCK_SPEC>;
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
impl From<crate::W<EEBLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEBLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEPROM_EEBLOCK_BLOCK` reader - Current Block"]
pub type EEPROM_EEBLOCK_BLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EEPROM_EEBLOCK_BLOCK` writer - Current Block"]
pub type EEPROM_EEBLOCK_BLOCK_W<'a> = crate::FieldWriter<'a, u32, EEBLOCK_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    pub fn eeprom_eeblock_block(&self) -> EEPROM_EEBLOCK_BLOCK_R {
        EEPROM_EEBLOCK_BLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current Block"]
    #[inline(always)]
    pub fn eeprom_eeblock_block(&mut self) -> EEPROM_EEBLOCK_BLOCK_W {
        EEPROM_EEBLOCK_BLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Current Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeblock](index.html) module"]
pub struct EEBLOCK_SPEC;
impl crate::RegisterSpec for EEBLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeblock::R](R) reader structure"]
impl crate::Readable for EEBLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeblock::W](W) writer structure"]
impl crate::Writable for EEBLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEBLOCK to value 0"]
impl crate::Resettable for EEBLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
