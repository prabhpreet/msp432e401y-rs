#[doc = "Register `SCGCEEPROM` reader"]
pub struct R(crate::R<SCGCEEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCEEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCEEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCEEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCEEPROM` writer"]
pub struct W(crate::W<SCGCEEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCEEPROM_SPEC>;
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
impl From<crate::W<SCGCEEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCEEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCEEPROM_S0` reader - EEPROM Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEEPROM_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCEEPROM_S0` writer - EEPROM Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCEEPROM_S0_W<'a> = crate::BitWriter<'a, u32, SCGCEEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgceeprom_s0(&self) -> SYSCTL_SCGCEEPROM_S0_R {
        SYSCTL_SCGCEEPROM_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgceeprom_s0(&mut self) -> SYSCTL_SCGCEEPROM_S0_W {
        SYSCTL_SCGCEEPROM_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgceeprom](index.html) module"]
pub struct SCGCEEPROM_SPEC;
impl crate::RegisterSpec for SCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgceeprom::R](R) reader structure"]
impl crate::Readable for SCGCEEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgceeprom::W](W) writer structure"]
impl crate::Writable for SCGCEEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCEEPROM to value 0"]
impl crate::Resettable for SCGCEEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
