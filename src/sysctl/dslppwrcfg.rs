#[doc = "Register `DSLPPWRCFG` reader"]
pub struct R(crate::R<DSLPPWRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSLPPWRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSLPPWRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSLPPWRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSLPPWRCFG` writer"]
pub struct W(crate::W<DSLPPWRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSLPPWRCFG_SPEC>;
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
impl From<crate::W<DSLPPWRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSLPPWRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SRAM Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DSLPPWRCFG_SRAMPM_A {
    #[doc = "0: Active Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_NRM = 0,
    #[doc = "1: Standby Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_SBY = 1,
    #[doc = "3: Low Power Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_LP = 3,
}
impl From<SYSCTL_DSLPPWRCFG_SRAMPM_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSLPPWRCFG_SRAMPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DSLPPWRCFG_SRAMPM` reader - SRAM Power Modes"]
pub type SYSCTL_DSLPPWRCFG_SRAMPM_R = crate::FieldReader<u8, SYSCTL_DSLPPWRCFG_SRAMPM_A>;
impl SYSCTL_DSLPPWRCFG_SRAMPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DSLPPWRCFG_SRAMPM_A> {
        match self.bits {
            0 => Some(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_NRM),
            1 => Some(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_SBY),
            3 => Some(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_LP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_nrm(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_SBY`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_sby(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_SBY
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_LP`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_lp(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_LP
    }
}
#[doc = "Field `SYSCTL_DSLPPWRCFG_SRAMPM` writer - SRAM Power Modes"]
pub type SYSCTL_DSLPPWRCFG_SRAMPM_W<'a> =
    crate::FieldWriter<'a, u32, DSLPPWRCFG_SPEC, u8, SYSCTL_DSLPPWRCFG_SRAMPM_A, 2, 0>;
impl<'a> SYSCTL_DSLPPWRCFG_SRAMPM_W<'a> {
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_NRM)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_sby(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_lp(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPM_A::SYSCTL_DSLPPWRCFG_SRAMPM_LP)
    }
}
#[doc = "Flash Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DSLPPWRCFG_FLASHPM_A {
    #[doc = "0: Active Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_NRM = 0,
    #[doc = "2: Low Power Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_SLP = 2,
}
impl From<SYSCTL_DSLPPWRCFG_FLASHPM_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DSLPPWRCFG_FLASHPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DSLPPWRCFG_FLASHPM` reader - Flash Power Modes"]
pub type SYSCTL_DSLPPWRCFG_FLASHPM_R = crate::FieldReader<u8, SYSCTL_DSLPPWRCFG_FLASHPM_A>;
impl SYSCTL_DSLPPWRCFG_FLASHPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DSLPPWRCFG_FLASHPM_A> {
        match self.bits {
            0 => Some(SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_NRM),
            2 => Some(SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_SLP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_FLASHPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_flashpm_nrm(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_FLASHPM_SLP`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_flashpm_slp(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_SLP
    }
}
#[doc = "Field `SYSCTL_DSLPPWRCFG_FLASHPM` writer - Flash Power Modes"]
pub type SYSCTL_DSLPPWRCFG_FLASHPM_W<'a> =
    crate::FieldWriter<'a, u32, DSLPPWRCFG_SPEC, u8, SYSCTL_DSLPPWRCFG_FLASHPM_A, 2, 4>;
impl<'a> SYSCTL_DSLPPWRCFG_FLASHPM_W<'a> {
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm_slp(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_FLASHPM_A::SYSCTL_DSLPPWRCFG_FLASHPM_SLP)
    }
}
#[doc = "Field `SYSCTL_DSLPPWRCFG_TSPD` reader - Temperature Sense Power Down"]
pub type SYSCTL_DSLPPWRCFG_TSPD_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DSLPPWRCFG_TSPD` writer - Temperature Sense Power Down"]
pub type SYSCTL_DSLPPWRCFG_TSPD_W<'a> = crate::BitWriter<'a, u32, DSLPPWRCFG_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_DSLPPWRCFG_LDOSM` reader - LDO Sleep Mode"]
pub type SYSCTL_DSLPPWRCFG_LDOSM_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DSLPPWRCFG_LDOSM` writer - LDO Sleep Mode"]
pub type SYSCTL_DSLPPWRCFG_LDOSM_W<'a> = crate::BitWriter<'a, u32, DSLPPWRCFG_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm(&self) -> SYSCTL_DSLPPWRCFG_SRAMPM_R {
        SYSCTL_DSLPPWRCFG_SRAMPM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm(&self) -> SYSCTL_DSLPPWRCFG_FLASHPM_R {
        SYSCTL_DSLPPWRCFG_FLASHPM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Temperature Sense Power Down"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_tspd(&self) -> SYSCTL_DSLPPWRCFG_TSPD_R {
        SYSCTL_DSLPPWRCFG_TSPD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LDO Sleep Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_ldosm(&self) -> SYSCTL_DSLPPWRCFG_LDOSM_R {
        SYSCTL_DSLPPWRCFG_LDOSM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm(&mut self) -> SYSCTL_DSLPPWRCFG_SRAMPM_W {
        SYSCTL_DSLPPWRCFG_SRAMPM_W::new(self)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm(&mut self) -> SYSCTL_DSLPPWRCFG_FLASHPM_W {
        SYSCTL_DSLPPWRCFG_FLASHPM_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Sense Power Down"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_tspd(&mut self) -> SYSCTL_DSLPPWRCFG_TSPD_W {
        SYSCTL_DSLPPWRCFG_TSPD_W::new(self)
    }
    #[doc = "Bit 9 - LDO Sleep Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_ldosm(&mut self) -> SYSCTL_DSLPPWRCFG_LDOSM_W {
        SYSCTL_DSLPPWRCFG_LDOSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep-Sleep Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dslppwrcfg](index.html) module"]
pub struct DSLPPWRCFG_SPEC;
impl crate::RegisterSpec for DSLPPWRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dslppwrcfg::R](R) reader structure"]
impl crate::Readable for DSLPPWRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dslppwrcfg::W](W) writer structure"]
impl crate::Writable for DSLPPWRCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSLPPWRCFG to value 0"]
impl crate::Resettable for DSLPPWRCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
