#[doc = "Register `DCGCEEPROM` reader"]
pub struct R(crate::R<DCGCEEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCEEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCEEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCEEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCEEPROM` writer"]
pub struct W(crate::W<DCGCEEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCEEPROM_SPEC>;
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
impl From<crate::W<DCGCEEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCEEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCEEPROM_D0` reader - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCEEPROM_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCEEPROM_D0` writer - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCEEPROM_D0_W<'a> = crate::BitWriter<'a, u32, DCGCEEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgceeprom_d0(&self) -> SYSCTL_DCGCEEPROM_D0_R {
        SYSCTL_DCGCEEPROM_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgceeprom_d0(&mut self) -> SYSCTL_DCGCEEPROM_D0_W {
        SYSCTL_DCGCEEPROM_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgceeprom](index.html) module"]
pub struct DCGCEEPROM_SPEC;
impl crate::RegisterSpec for DCGCEEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgceeprom::R](R) reader structure"]
impl crate::Readable for DCGCEEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgceeprom::W](W) writer structure"]
impl crate::Writable for DCGCEEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCEEPROM to value 0"]
impl crate::Resettable for DCGCEEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
