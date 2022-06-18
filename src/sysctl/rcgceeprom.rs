#[doc = "Register `RCGCEEPROM` reader"]
pub struct R(crate::R<RCGCEEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCEEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCEEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCEEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCEEPROM` writer"]
pub struct W(crate::W<RCGCEEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCEEPROM_SPEC>;
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
impl From<crate::W<RCGCEEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCEEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCEEPROM_R0` reader - EEPROM Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCEEPROM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCEEPROM_R0` writer - EEPROM Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCEEPROM_R0_W<'a> = crate::BitWriter<'a, u32, RCGCEEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgceeprom_r0(&self) -> SYSCTL_RCGCEEPROM_R0_R {
        SYSCTL_RCGCEEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgceeprom_r0(&mut self) -> SYSCTL_RCGCEEPROM_R0_W {
        SYSCTL_RCGCEEPROM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgceeprom](index.html) module"]
pub struct RCGCEEPROM_SPEC;
impl crate::RegisterSpec for RCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgceeprom::R](R) reader structure"]
impl crate::Readable for RCGCEEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgceeprom::W](W) writer structure"]
impl crate::Writable for RCGCEEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCEEPROM to value 0"]
impl crate::Resettable for RCGCEEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
