#[doc = "Register `SLPPWRCFG` reader"]
pub struct R(crate::R<SLPPWRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLPPWRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLPPWRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLPPWRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLPPWRCFG` writer"]
pub struct W(crate::W<SLPPWRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLPPWRCFG_SPEC>;
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
impl From<crate::W<SLPPWRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLPPWRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_SLPPWRCFG_SRAMPM_A {
    #[doc = "0: Active Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_NRM = 0,
    #[doc = "1: Standby Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_SBY = 1,
    #[doc = "3: Low Power Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_LP = 3,
}
impl From<SYSCTL_SLPPWRCFG_SRAMPM_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_SLPPWRCFG_SRAMPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_SLPPWRCFG_SRAMPM` reader - SRAM Power Modes"]
pub type SYSCTL_SLPPWRCFG_SRAMPM_R = crate::FieldReader<u8, SYSCTL_SLPPWRCFG_SRAMPM_A>;
impl SYSCTL_SLPPWRCFG_SRAMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_SLPPWRCFG_SRAMPM_A> {
        match self.bits {
            0 => Some(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_NRM),
            1 => Some(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_SBY),
            3 => Some(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_LP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_nrm(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_SBY`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_sby(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_SBY
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_LP`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_lp(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_LP
    }
}
#[doc = "Field `SYSCTL_SLPPWRCFG_SRAMPM` writer - SRAM Power Modes"]
pub type SYSCTL_SLPPWRCFG_SRAMPM_W<'a> =
    crate::FieldWriter<'a, u32, SLPPWRCFG_SPEC, u8, SYSCTL_SLPPWRCFG_SRAMPM_A, 2, 0>;
impl<'a> SYSCTL_SLPPWRCFG_SRAMPM_W<'a> {
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_NRM)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_sby(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_lp(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPM_A::SYSCTL_SLPPWRCFG_SRAMPM_LP)
    }
}
#[doc = "Flash Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_SLPPWRCFG_FLASHPM_A {
    #[doc = "0: Active Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_NRM = 0,
    #[doc = "2: Low Power Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_SLP = 2,
}
impl From<SYSCTL_SLPPWRCFG_FLASHPM_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_SLPPWRCFG_FLASHPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_SLPPWRCFG_FLASHPM` reader - Flash Power Modes"]
pub type SYSCTL_SLPPWRCFG_FLASHPM_R = crate::FieldReader<u8, SYSCTL_SLPPWRCFG_FLASHPM_A>;
impl SYSCTL_SLPPWRCFG_FLASHPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_SLPPWRCFG_FLASHPM_A> {
        match self.bits {
            0 => Some(SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_NRM),
            2 => Some(SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_SLP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_FLASHPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_flashpm_nrm(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_FLASHPM_SLP`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_flashpm_slp(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_SLP
    }
}
#[doc = "Field `SYSCTL_SLPPWRCFG_FLASHPM` writer - Flash Power Modes"]
pub type SYSCTL_SLPPWRCFG_FLASHPM_W<'a> =
    crate::FieldWriter<'a, u32, SLPPWRCFG_SPEC, u8, SYSCTL_SLPPWRCFG_FLASHPM_A, 2, 4>;
impl<'a> SYSCTL_SLPPWRCFG_FLASHPM_W<'a> {
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm_slp(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_FLASHPM_A::SYSCTL_SLPPWRCFG_FLASHPM_SLP)
    }
}
impl R {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm(&self) -> SYSCTL_SLPPWRCFG_SRAMPM_R {
        SYSCTL_SLPPWRCFG_SRAMPM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm(&self) -> SYSCTL_SLPPWRCFG_FLASHPM_R {
        SYSCTL_SLPPWRCFG_FLASHPM_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm(&mut self) -> SYSCTL_SLPPWRCFG_SRAMPM_W {
        SYSCTL_SLPPWRCFG_SRAMPM_W::new(self)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm(&mut self) -> SYSCTL_SLPPWRCFG_FLASHPM_W {
        SYSCTL_SLPPWRCFG_FLASHPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slppwrcfg](index.html) module"]
pub struct SLPPWRCFG_SPEC;
impl crate::RegisterSpec for SLPPWRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slppwrcfg::R](R) reader structure"]
impl crate::Readable for SLPPWRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slppwrcfg::W](W) writer structure"]
impl crate::Writable for SLPPWRCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLPPWRCFG to value 0"]
impl crate::Resettable for SLPPWRCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
