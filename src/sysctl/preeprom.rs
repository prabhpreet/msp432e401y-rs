#[doc = "Register `PREEPROM` reader"]
pub struct R(crate::R<PREEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREEPROM` writer"]
pub struct W(crate::W<PREEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREEPROM_SPEC>;
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
impl From<crate::W<PREEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PREEPROM_R0` reader - EEPROM Module Peripheral Ready"]
pub type SYSCTL_PREEPROM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PREEPROM_R0` writer - EEPROM Module Peripheral Ready"]
pub type SYSCTL_PREEPROM_R0_W<'a> = crate::BitWriter<'a, u32, PREEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_preeprom_r0(&self) -> SYSCTL_PREEPROM_R0_R {
        SYSCTL_PREEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_preeprom_r0(&mut self) -> SYSCTL_PREEPROM_R0_W {
        SYSCTL_PREEPROM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preeprom](index.html) module"]
pub struct PREEPROM_SPEC;
impl crate::RegisterSpec for PREEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [preeprom::R](R) reader structure"]
impl crate::Readable for PREEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [preeprom::W](W) writer structure"]
impl crate::Writable for PREEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PREEPROM to value 0"]
impl crate::Resettable for PREEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
