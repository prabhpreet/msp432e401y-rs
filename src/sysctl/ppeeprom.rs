#[doc = "Register `PPEEPROM` reader"]
pub struct R(crate::R<PPEEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPEEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPEEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPEEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPEEPROM` writer"]
pub struct W(crate::W<PPEEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPEEPROM_SPEC>;
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
impl From<crate::W<PPEEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPEEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPEEPROM_P0` reader - EEPROM Module Present"]
pub type SYSCTL_PPEEPROM_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPEEPROM_P0` writer - EEPROM Module Present"]
pub type SYSCTL_PPEEPROM_P0_W<'a> = crate::BitWriter<'a, u32, PPEEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Present"]
    #[inline(always)]
    pub fn sysctl_ppeeprom_p0(&self) -> SYSCTL_PPEEPROM_P0_R {
        SYSCTL_PPEEPROM_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Present"]
    #[inline(always)]
    pub fn sysctl_ppeeprom_p0(&mut self) -> SYSCTL_PPEEPROM_P0_W {
        SYSCTL_PPEEPROM_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppeeprom](index.html) module"]
pub struct PPEEPROM_SPEC;
impl crate::RegisterSpec for PPEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppeeprom::R](R) reader structure"]
impl crate::Readable for PPEEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppeeprom::W](W) writer structure"]
impl crate::Writable for PPEEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPEEPROM to value 0"]
impl crate::Resettable for PPEEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
