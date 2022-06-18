#[doc = "Register `PTBOCTL` reader"]
pub struct R(crate::R<PTBOCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTBOCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTBOCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTBOCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTBOCTL` writer"]
pub struct W(crate::W<PTBOCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTBOCTL_SPEC>;
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
impl From<crate::W<PTBOCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTBOCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VDD (VDDS) under BOR Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_PTBOCTL_VDD_UBOR_A {
    #[doc = "0: No Action"]
    SYSCTL_PTBOCTL_VDD_UBOR_NONE = 0,
    #[doc = "1: System control interrupt"]
    SYSCTL_PTBOCTL_VDD_UBOR_SYSINT = 1,
    #[doc = "2: NMI"]
    SYSCTL_PTBOCTL_VDD_UBOR_NMI = 2,
    #[doc = "3: Reset"]
    SYSCTL_PTBOCTL_VDD_UBOR_RST = 3,
}
impl From<SYSCTL_PTBOCTL_VDD_UBOR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_PTBOCTL_VDD_UBOR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_PTBOCTL_VDD_UBOR` reader - VDD (VDDS) under BOR Event Action"]
pub type SYSCTL_PTBOCTL_VDD_UBOR_R = crate::FieldReader<u8, SYSCTL_PTBOCTL_VDD_UBOR_A>;
impl SYSCTL_PTBOCTL_VDD_UBOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_PTBOCTL_VDD_UBOR_A {
        match self.bits {
            0 => SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NONE,
            1 => SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT,
            2 => SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NMI,
            3 => SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_NONE`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_none(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_SYSINT`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_sysint(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_NMI`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_nmi(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NMI
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_RST`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_rst(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_RST
    }
}
#[doc = "Field `SYSCTL_PTBOCTL_VDD_UBOR` writer - VDD (VDDS) under BOR Event Action"]
pub type SYSCTL_PTBOCTL_VDD_UBOR_W<'a> =
    crate::FieldWriterSafe<'a, u32, PTBOCTL_SPEC, u8, SYSCTL_PTBOCTL_VDD_UBOR_A, 2, 0>;
impl<'a> SYSCTL_PTBOCTL_VDD_UBOR_W<'a> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_none(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_sysint(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_nmi(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_rst(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBOR_A::SYSCTL_PTBOCTL_VDD_UBOR_RST)
    }
}
#[doc = "VDDA under BOR Event Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_PTBOCTL_VDDA_UBOR_A {
    #[doc = "0: No Action"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NONE = 0,
    #[doc = "1: System control interrupt"]
    SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT = 1,
    #[doc = "2: NMI"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NMI = 2,
    #[doc = "3: Reset"]
    SYSCTL_PTBOCTL_VDDA_UBOR_RST = 3,
}
impl From<SYSCTL_PTBOCTL_VDDA_UBOR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_PTBOCTL_VDDA_UBOR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_PTBOCTL_VDDA_UBOR` reader - VDDA under BOR Event Action"]
pub type SYSCTL_PTBOCTL_VDDA_UBOR_R = crate::FieldReader<u8, SYSCTL_PTBOCTL_VDDA_UBOR_A>;
impl SYSCTL_PTBOCTL_VDDA_UBOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_PTBOCTL_VDDA_UBOR_A {
        match self.bits {
            0 => SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NONE,
            1 => SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT,
            2 => SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NMI,
            3 => SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_NONE`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_none(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_sysint(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_NMI`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_nmi(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NMI
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_RST`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_rst(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_RST
    }
}
#[doc = "Field `SYSCTL_PTBOCTL_VDDA_UBOR` writer - VDDA under BOR Event Action"]
pub type SYSCTL_PTBOCTL_VDDA_UBOR_W<'a> =
    crate::FieldWriterSafe<'a, u32, PTBOCTL_SPEC, u8, SYSCTL_PTBOCTL_VDDA_UBOR_A, 2, 8>;
impl<'a> SYSCTL_PTBOCTL_VDDA_UBOR_W<'a> {
    #[doc = "No Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_none(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_sysint(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_nmi(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_rst(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBOR_A::SYSCTL_PTBOCTL_VDDA_UBOR_RST)
    }
}
impl R {
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor(&self) -> SYSCTL_PTBOCTL_VDD_UBOR_R {
        SYSCTL_PTBOCTL_VDD_UBOR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor(&self) -> SYSCTL_PTBOCTL_VDDA_UBOR_R {
        SYSCTL_PTBOCTL_VDDA_UBOR_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor(&mut self) -> SYSCTL_PTBOCTL_VDD_UBOR_W {
        SYSCTL_PTBOCTL_VDD_UBOR_W::new(self)
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor(&mut self) -> SYSCTL_PTBOCTL_VDDA_UBOR_W {
        SYSCTL_PTBOCTL_VDDA_UBOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-Temp Brown Out Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptboctl](index.html) module"]
pub struct PTBOCTL_SPEC;
impl crate::RegisterSpec for PTBOCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptboctl::R](R) reader structure"]
impl crate::Readable for PTBOCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptboctl::W](W) writer structure"]
impl crate::Writable for PTBOCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTBOCTL to value 0"]
impl crate::Resettable for PTBOCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
