#[doc = "Register `PCEEPROM` reader"]
pub struct R(crate::R<PCEEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCEEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCEEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCEEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCEEPROM` writer"]
pub struct W(crate::W<PCEEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCEEPROM_SPEC>;
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
impl From<crate::W<PCEEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCEEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCEEPROM_P0` reader - EEPROM Module 0 Power Control"]
pub type SYSCTL_PCEEPROM_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCEEPROM_P0` writer - EEPROM Module 0 Power Control"]
pub type SYSCTL_PCEEPROM_P0_W<'a> = crate::BitWriter<'a, u32, PCEEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pceeprom_p0(&self) -> SYSCTL_PCEEPROM_P0_R {
        SYSCTL_PCEEPROM_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pceeprom_p0(&mut self) -> SYSCTL_PCEEPROM_P0_W {
        SYSCTL_PCEEPROM_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pceeprom](index.html) module"]
pub struct PCEEPROM_SPEC;
impl crate::RegisterSpec for PCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pceeprom::R](R) reader structure"]
impl crate::Readable for PCEEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pceeprom::W](W) writer structure"]
impl crate::Writable for PCEEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCEEPROM to value 0"]
impl crate::Resettable for PCEEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
