#[doc = "Register `EMACMPC` reader"]
pub struct R(crate::R<EMACMPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMACMPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMACMPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMACMPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMACMPC` writer"]
pub struct W(crate::W<EMACMPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMACMPC_SPEC>;
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
impl From<crate::W<EMACMPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMACMPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory Array Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_EMACMPC_PWRCTL_A {
    #[doc = "0: Array OFF"]
    SYSCTL_EMACMPC_PWRCTL_OFF = 0,
    #[doc = "3: Array On"]
    SYSCTL_EMACMPC_PWRCTL_ON = 3,
}
impl From<SYSCTL_EMACMPC_PWRCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_EMACMPC_PWRCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_EMACMPC_PWRCTL` reader - Memory Array Power Control"]
pub type SYSCTL_EMACMPC_PWRCTL_R = crate::FieldReader<u8, SYSCTL_EMACMPC_PWRCTL_A>;
impl SYSCTL_EMACMPC_PWRCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_EMACMPC_PWRCTL_A> {
        match self.bits {
            0 => Some(SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_OFF),
            3 => Some(SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_ON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACMPC_PWRCTL_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacmpc_pwrctl_off(&self) -> bool {
        *self == SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACMPC_PWRCTL_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacmpc_pwrctl_on(&self) -> bool {
        *self == SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_ON
    }
}
#[doc = "Field `SYSCTL_EMACMPC_PWRCTL` writer - Memory Array Power Control"]
pub type SYSCTL_EMACMPC_PWRCTL_W<'a> =
    crate::FieldWriter<'a, u32, EMACMPC_SPEC, u8, SYSCTL_EMACMPC_PWRCTL_A, 2, 0>;
impl<'a> SYSCTL_EMACMPC_PWRCTL_W<'a> {
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_OFF)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACMPC_PWRCTL_A::SYSCTL_EMACMPC_PWRCTL_ON)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl(&self) -> SYSCTL_EMACMPC_PWRCTL_R {
        SYSCTL_EMACMPC_PWRCTL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline(always)]
    pub fn sysctl_emacmpc_pwrctl(&mut self) -> SYSCTL_EMACMPC_PWRCTL_W {
        SYSCTL_EMACMPC_PWRCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Memory Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emacmpc](index.html) module"]
pub struct EMACMPC_SPEC;
impl crate::RegisterSpec for EMACMPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emacmpc::R](R) reader structure"]
impl crate::Readable for EMACMPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emacmpc::W](W) writer structure"]
impl crate::Writable for EMACMPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMACMPC to value 0"]
impl crate::Resettable for EMACMPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
