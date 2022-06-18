#[doc = "Register `EMACPDS` reader"]
pub struct R(crate::R<EMACPDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMACPDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMACPDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMACPDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMACPDS` writer"]
pub struct W(crate::W<EMACPDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMACPDS_SPEC>;
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
impl From<crate::W<EMACPDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMACPDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power Domain Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_EMACPDS_PWRSTAT_A {
    #[doc = "0: OFF"]
    SYSCTL_EMACPDS_PWRSTAT_OFF = 0,
    #[doc = "3: ON"]
    SYSCTL_EMACPDS_PWRSTAT_ON = 3,
}
impl From<SYSCTL_EMACPDS_PWRSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_EMACPDS_PWRSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_EMACPDS_PWRSTAT` reader - Power Domain Status"]
pub type SYSCTL_EMACPDS_PWRSTAT_R = crate::FieldReader<u8, SYSCTL_EMACPDS_PWRSTAT_A>;
impl SYSCTL_EMACPDS_PWRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_EMACPDS_PWRSTAT_A> {
        match self.bits {
            0 => Some(SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_OFF),
            3 => Some(SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_ON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_PWRSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_pwrstat_off(&self) -> bool {
        *self == SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_PWRSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_pwrstat_on(&self) -> bool {
        *self == SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_ON
    }
}
#[doc = "Field `SYSCTL_EMACPDS_PWRSTAT` writer - Power Domain Status"]
pub type SYSCTL_EMACPDS_PWRSTAT_W<'a> =
    crate::FieldWriter<'a, u32, EMACPDS_SPEC, u8, SYSCTL_EMACPDS_PWRSTAT_A, 2, 0>;
impl<'a> SYSCTL_EMACPDS_PWRSTAT_W<'a> {
    #[doc = "OFF"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_OFF)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_PWRSTAT_A::SYSCTL_EMACPDS_PWRSTAT_ON)
    }
}
#[doc = "Memory Array Power Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_EMACPDS_MEMSTAT_A {
    #[doc = "0: Array OFF"]
    SYSCTL_EMACPDS_MEMSTAT_OFF = 0,
    #[doc = "3: Array On"]
    SYSCTL_EMACPDS_MEMSTAT_ON = 3,
}
impl From<SYSCTL_EMACPDS_MEMSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_EMACPDS_MEMSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_EMACPDS_MEMSTAT` reader - Memory Array Power Status"]
pub type SYSCTL_EMACPDS_MEMSTAT_R = crate::FieldReader<u8, SYSCTL_EMACPDS_MEMSTAT_A>;
impl SYSCTL_EMACPDS_MEMSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_EMACPDS_MEMSTAT_A> {
        match self.bits {
            0 => Some(SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_OFF),
            3 => Some(SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_ON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_MEMSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_memstat_off(&self) -> bool {
        *self == SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_EMACPDS_MEMSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_emacpds_memstat_on(&self) -> bool {
        *self == SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_ON
    }
}
#[doc = "Field `SYSCTL_EMACPDS_MEMSTAT` writer - Memory Array Power Status"]
pub type SYSCTL_EMACPDS_MEMSTAT_W<'a> =
    crate::FieldWriter<'a, u32, EMACPDS_SPEC, u8, SYSCTL_EMACPDS_MEMSTAT_A, 2, 2>;
impl<'a> SYSCTL_EMACPDS_MEMSTAT_W<'a> {
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_OFF)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_EMACPDS_MEMSTAT_A::SYSCTL_EMACPDS_MEMSTAT_ON)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat(&self) -> SYSCTL_EMACPDS_PWRSTAT_R {
        SYSCTL_EMACPDS_PWRSTAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat(&self) -> SYSCTL_EMACPDS_MEMSTAT_R {
        SYSCTL_EMACPDS_MEMSTAT_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_pwrstat(&mut self) -> SYSCTL_EMACPDS_PWRSTAT_W {
        SYSCTL_EMACPDS_PWRSTAT_W::new(self)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_emacpds_memstat(&mut self) -> SYSCTL_EMACPDS_MEMSTAT_W {
        SYSCTL_EMACPDS_MEMSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emacpds](index.html) module"]
pub struct EMACPDS_SPEC;
impl crate::RegisterSpec for EMACPDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emacpds::R](R) reader structure"]
impl crate::Readable for EMACPDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emacpds::W](W) writer structure"]
impl crate::Writable for EMACPDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMACPDS to value 0"]
impl crate::Resettable for EMACPDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
