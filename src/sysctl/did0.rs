#[doc = "Register `DID0` reader"]
pub struct R(crate::R<DID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DID0` writer"]
pub struct W(crate::W<DID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DID0_SPEC>;
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
impl From<crate::W<DID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MIN_A {
    #[doc = "0: Initial device, or a major revision update"]
    SYSCTL_DID0_MIN_0 = 0,
    #[doc = "1: First metal layer change"]
    SYSCTL_DID0_MIN_1 = 1,
    #[doc = "2: Second metal layer change"]
    SYSCTL_DID0_MIN_2 = 2,
}
impl From<SYSCTL_DID0_MIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID0_MIN` reader - Minor Revision"]
pub type SYSCTL_DID0_MIN_R = crate::FieldReader<u8, SYSCTL_DID0_MIN_A>;
impl SYSCTL_DID0_MIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID0_MIN_A> {
        match self.bits {
            0 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0),
            1 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1),
            2 => Some(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_0`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_0(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_1(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MIN_2`"]
    #[inline(always)]
    pub fn is_sysctl_did0_min_2(&self) -> bool {
        *self == SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2
    }
}
#[doc = "Field `SYSCTL_DID0_MIN` writer - Minor Revision"]
pub type SYSCTL_DID0_MIN_W<'a> =
    crate::FieldWriter<'a, u32, DID0_SPEC, u8, SYSCTL_DID0_MIN_A, 8, 0>;
impl<'a> SYSCTL_DID0_MIN_W<'a> {
    #[doc = "Initial device, or a major revision update"]
    #[inline(always)]
    pub fn sysctl_did0_min_0(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_0)
    }
    #[doc = "First metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_1)
    }
    #[doc = "Second metal layer change"]
    #[inline(always)]
    pub fn sysctl_did0_min_2(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MIN_A::SYSCTL_DID0_MIN_2)
    }
}
#[doc = "Major Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_MAJ_A {
    #[doc = "0: Revision A (initial device)"]
    SYSCTL_DID0_MAJ_REVA = 0,
    #[doc = "1: Revision B (first base layer revision)"]
    SYSCTL_DID0_MAJ_REVB = 1,
    #[doc = "2: Revision C (second base layer revision)"]
    SYSCTL_DID0_MAJ_REVC = 2,
}
impl From<SYSCTL_DID0_MAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_MAJ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID0_MAJ` reader - Major Revision"]
pub type SYSCTL_DID0_MAJ_R = crate::FieldReader<u8, SYSCTL_DID0_MAJ_A>;
impl SYSCTL_DID0_MAJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID0_MAJ_A> {
        match self.bits {
            0 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA),
            1 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB),
            2 => Some(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVA`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_reva(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVB`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revb(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_MAJ_REVC`"]
    #[inline(always)]
    pub fn is_sysctl_did0_maj_revc(&self) -> bool {
        *self == SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC
    }
}
#[doc = "Field `SYSCTL_DID0_MAJ` writer - Major Revision"]
pub type SYSCTL_DID0_MAJ_W<'a> =
    crate::FieldWriter<'a, u32, DID0_SPEC, u8, SYSCTL_DID0_MAJ_A, 8, 8>;
impl<'a> SYSCTL_DID0_MAJ_W<'a> {
    #[doc = "Revision A (initial device)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_reva(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVA)
    }
    #[doc = "Revision B (first base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revb(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVB)
    }
    #[doc = "Revision C (second base layer revision)"]
    #[inline(always)]
    pub fn sysctl_did0_maj_revc(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_MAJ_A::SYSCTL_DID0_MAJ_REVC)
    }
}
#[doc = "Device Class\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_CLASS_A {
    #[doc = "12: MSP432E4 class microcontrollers"]
    SYSCTL_DID0_CLASS_MSP432E4 = 12,
}
impl From<SYSCTL_DID0_CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_CLASS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID0_CLASS` reader - Device Class"]
pub type SYSCTL_DID0_CLASS_R = crate::FieldReader<u8, SYSCTL_DID0_CLASS_A>;
impl SYSCTL_DID0_CLASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID0_CLASS_A> {
        match self.bits {
            12 => Some(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_MSP432E4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_CLASS_MSP432E4`"]
    #[inline(always)]
    pub fn is_sysctl_did0_class_msp432e4(&self) -> bool {
        *self == SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_MSP432E4
    }
}
#[doc = "Field `SYSCTL_DID0_CLASS` writer - Device Class"]
pub type SYSCTL_DID0_CLASS_W<'a> =
    crate::FieldWriter<'a, u32, DID0_SPEC, u8, SYSCTL_DID0_CLASS_A, 8, 16>;
impl<'a> SYSCTL_DID0_CLASS_W<'a> {
    #[doc = "MSP432E4 class microcontrollers"]
    #[inline(always)]
    pub fn sysctl_did0_class_msp432e4(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_CLASS_A::SYSCTL_DID0_CLASS_MSP432E4)
    }
}
#[doc = "DID0 Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_DID0_VER_A {
    #[doc = "1: Second version of the DID0 register format."]
    SYSCTL_DID0_VER_1 = 1,
}
impl From<SYSCTL_DID0_VER_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DID0_VER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_DID0_VER` reader - DID0 Version"]
pub type SYSCTL_DID0_VER_R = crate::FieldReader<u8, SYSCTL_DID0_VER_A>;
impl SYSCTL_DID0_VER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_DID0_VER_A> {
        match self.bits {
            1 => Some(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DID0_VER_1`"]
    #[inline(always)]
    pub fn is_sysctl_did0_ver_1(&self) -> bool {
        *self == SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1
    }
}
#[doc = "Field `SYSCTL_DID0_VER` writer - DID0 Version"]
pub type SYSCTL_DID0_VER_W<'a> =
    crate::FieldWriter<'a, u32, DID0_SPEC, u8, SYSCTL_DID0_VER_A, 3, 28>;
impl<'a> SYSCTL_DID0_VER_W<'a> {
    #[doc = "Second version of the DID0 register format."]
    #[inline(always)]
    pub fn sysctl_did0_ver_1(self) -> &'a mut W {
        self.variant(SYSCTL_DID0_VER_A::SYSCTL_DID0_VER_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&self) -> SYSCTL_DID0_MIN_R {
        SYSCTL_DID0_MIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&self) -> SYSCTL_DID0_MAJ_R {
        SYSCTL_DID0_MAJ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&self) -> SYSCTL_DID0_CLASS_R {
        SYSCTL_DID0_CLASS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&self) -> SYSCTL_DID0_VER_R {
        SYSCTL_DID0_VER_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minor Revision"]
    #[inline(always)]
    pub fn sysctl_did0_min(&mut self) -> SYSCTL_DID0_MIN_W {
        SYSCTL_DID0_MIN_W::new(self)
    }
    #[doc = "Bits 8:15 - Major Revision"]
    #[inline(always)]
    pub fn sysctl_did0_maj(&mut self) -> SYSCTL_DID0_MAJ_W {
        SYSCTL_DID0_MAJ_W::new(self)
    }
    #[doc = "Bits 16:23 - Device Class"]
    #[inline(always)]
    pub fn sysctl_did0_class(&mut self) -> SYSCTL_DID0_CLASS_W {
        SYSCTL_DID0_CLASS_W::new(self)
    }
    #[doc = "Bits 28:30 - DID0 Version"]
    #[inline(always)]
    pub fn sysctl_did0_ver(&mut self) -> SYSCTL_DID0_VER_W {
        SYSCTL_DID0_VER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did0](index.html) module"]
pub struct DID0_SPEC;
impl crate::RegisterSpec for DID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [did0::R](R) reader structure"]
impl crate::Readable for DID0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [did0::W](W) writer structure"]
impl crate::Writable for DID0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DID0 to value 0"]
impl crate::Resettable for DID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
