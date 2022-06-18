#[doc = "Register `SREEPROM` reader"]
pub struct R(crate::R<SREEPROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREEPROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREEPROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREEPROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREEPROM` writer"]
pub struct W(crate::W<SREEPROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREEPROM_SPEC>;
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
impl From<crate::W<SREEPROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREEPROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SREEPROM_R0` reader - EEPROM Module Software Reset"]
pub type SYSCTL_SREEPROM_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SREEPROM_R0` writer - EEPROM Module Software Reset"]
pub type SYSCTL_SREEPROM_R0_W<'a> = crate::BitWriter<'a, u32, SREEPROM_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EEPROM Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_sreeprom_r0(&self) -> SYSCTL_SREEPROM_R0_R {
        SYSCTL_SREEPROM_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_sreeprom_r0(&mut self) -> SYSCTL_SREEPROM_R0_W {
        SYSCTL_SREEPROM_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreeprom](index.html) module"]
pub struct SREEPROM_SPEC;
impl crate::RegisterSpec for SREEPROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sreeprom::R](R) reader structure"]
impl crate::Readable for SREEPROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sreeprom::W](W) writer structure"]
impl crate::Writable for SREEPROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREEPROM to value 0"]
impl crate::Resettable for SREEPROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
