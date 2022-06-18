#[doc = "Register `LDODPCTL` reader"]
pub struct R(crate::R<LDODPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDODPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDODPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDODPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDODPCTL` writer"]
pub struct W(crate::W<LDODPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDODPCTL_SPEC>;
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
impl From<crate::W<LDODPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDODPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LDO Output Voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_LDODPCTL_VLDO_A {
    #[doc = "18: 0.90 V"]
    SYSCTL_LDODPCTL_VLDO_0_90V = 18,
    #[doc = "19: 0.95 V"]
    SYSCTL_LDODPCTL_VLDO_0_95V = 19,
    #[doc = "20: 1.00 V"]
    SYSCTL_LDODPCTL_VLDO_1_00V = 20,
    #[doc = "21: 1.05 V"]
    SYSCTL_LDODPCTL_VLDO_1_05V = 21,
    #[doc = "22: 1.10 V"]
    SYSCTL_LDODPCTL_VLDO_1_10V = 22,
    #[doc = "23: 1.15 V"]
    SYSCTL_LDODPCTL_VLDO_1_15V = 23,
    #[doc = "24: 1.20 V"]
    SYSCTL_LDODPCTL_VLDO_1_20V = 24,
    #[doc = "25: 1.25 V"]
    SYSCTL_LDODPCTL_VLDO_1_25V = 25,
    #[doc = "26: 1.30 V"]
    SYSCTL_LDODPCTL_VLDO_1_30V = 26,
    #[doc = "27: 1.35 V"]
    SYSCTL_LDODPCTL_VLDO_1_35V = 27,
}
impl From<SYSCTL_LDODPCTL_VLDO_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_LDODPCTL_VLDO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_LDODPCTL_VLDO` reader - LDO Output Voltage"]
pub type SYSCTL_LDODPCTL_VLDO_R = crate::FieldReader<u8, SYSCTL_LDODPCTL_VLDO_A>;
impl SYSCTL_LDODPCTL_VLDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_LDODPCTL_VLDO_A> {
        match self.bits {
            18 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_90V),
            19 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_95V),
            20 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_00V),
            21 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_05V),
            22 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_10V),
            23 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_15V),
            24 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_20V),
            25 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_25V),
            26 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_30V),
            27 => Some(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_35V),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_0_90V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_0_90v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_90V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_0_95V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_0_95v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_95V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_00V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_00v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_00V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_05V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_05v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_05V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_10V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_10v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_10V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_15V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_15v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_15V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_20V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_20v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_20V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_25V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_25v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_25V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_30V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_30v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_30V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_35V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_35v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_35V
    }
}
#[doc = "Field `SYSCTL_LDODPCTL_VLDO` writer - LDO Output Voltage"]
pub type SYSCTL_LDODPCTL_VLDO_W<'a> =
    crate::FieldWriter<'a, u32, LDODPCTL_SPEC, u8, SYSCTL_LDODPCTL_VLDO_A, 8, 0>;
impl<'a> SYSCTL_LDODPCTL_VLDO_W<'a> {
    #[doc = "0.90 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_0_90v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_90V)
    }
    #[doc = "0.95 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_0_95v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_0_95V)
    }
    #[doc = "1.00 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_00v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_00V)
    }
    #[doc = "1.05 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_05v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_05V)
    }
    #[doc = "1.10 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_10v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_15v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_15V)
    }
    #[doc = "1.20 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_20v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_25v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_25V)
    }
    #[doc = "1.30 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_30v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_30V)
    }
    #[doc = "1.35 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_35v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDO_A::SYSCTL_LDODPCTL_VLDO_1_35V)
    }
}
#[doc = "Field `SYSCTL_LDODPCTL_VADJEN` reader - Voltage Adjust Enable"]
pub type SYSCTL_LDODPCTL_VADJEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_LDODPCTL_VADJEN` writer - Voltage Adjust Enable"]
pub type SYSCTL_LDODPCTL_VADJEN_W<'a> = crate::BitWriter<'a, u32, LDODPCTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo(&self) -> SYSCTL_LDODPCTL_VLDO_R {
        SYSCTL_LDODPCTL_VLDO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vadjen(&self) -> SYSCTL_LDODPCTL_VADJEN_R {
        SYSCTL_LDODPCTL_VADJEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo(&mut self) -> SYSCTL_LDODPCTL_VLDO_W {
        SYSCTL_LDODPCTL_VLDO_W::new(self)
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vadjen(&mut self) -> SYSCTL_LDODPCTL_VADJEN_W {
        SYSCTL_LDODPCTL_VADJEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Deep-Sleep Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldodpctl](index.html) module"]
pub struct LDODPCTL_SPEC;
impl crate::RegisterSpec for LDODPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldodpctl::R](R) reader structure"]
impl crate::Readable for LDODPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldodpctl::W](W) writer structure"]
impl crate::Writable for LDODPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDODPCTL to value 0"]
impl crate::Resettable for LDODPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
