#[doc = "Register `USBPDS` reader"]
pub struct R(crate::R<USBPDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPDS` writer"]
pub struct W(crate::W<USBPDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPDS_SPEC>;
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
impl From<crate::W<USBPDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power Domain Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_USBPDS_PWRSTAT_A {
    #[doc = "0: OFF"]
    SYSCTL_USBPDS_PWRSTAT_OFF = 0,
    #[doc = "3: ON"]
    SYSCTL_USBPDS_PWRSTAT_ON = 3,
}
impl From<SYSCTL_USBPDS_PWRSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_USBPDS_PWRSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_USBPDS_PWRSTAT` reader - Power Domain Status"]
pub type SYSCTL_USBPDS_PWRSTAT_R = crate::FieldReader<u8, SYSCTL_USBPDS_PWRSTAT_A>;
impl SYSCTL_USBPDS_PWRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_USBPDS_PWRSTAT_A> {
        match self.bits {
            0 => Some(SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_OFF),
            3 => Some(SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_ON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBPDS_PWRSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_usbpds_pwrstat_off(&self) -> bool {
        *self == SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBPDS_PWRSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_usbpds_pwrstat_on(&self) -> bool {
        *self == SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_ON
    }
}
#[doc = "Field `SYSCTL_USBPDS_PWRSTAT` writer - Power Domain Status"]
pub type SYSCTL_USBPDS_PWRSTAT_W<'a> =
    crate::FieldWriter<'a, u32, USBPDS_SPEC, u8, SYSCTL_USBPDS_PWRSTAT_A, 2, 0>;
impl<'a> SYSCTL_USBPDS_PWRSTAT_W<'a> {
    #[doc = "OFF"]
    #[inline(always)]
    pub fn sysctl_usbpds_pwrstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_OFF)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn sysctl_usbpds_pwrstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_USBPDS_PWRSTAT_A::SYSCTL_USBPDS_PWRSTAT_ON)
    }
}
#[doc = "Memory Array Power Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_USBPDS_MEMSTAT_A {
    #[doc = "0: Array OFF"]
    SYSCTL_USBPDS_MEMSTAT_OFF = 0,
    #[doc = "1: SRAM Retention"]
    SYSCTL_USBPDS_MEMSTAT_RETAIN = 1,
    #[doc = "3: Array On"]
    SYSCTL_USBPDS_MEMSTAT_ON = 3,
}
impl From<SYSCTL_USBPDS_MEMSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_USBPDS_MEMSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_USBPDS_MEMSTAT` reader - Memory Array Power Status"]
pub type SYSCTL_USBPDS_MEMSTAT_R = crate::FieldReader<u8, SYSCTL_USBPDS_MEMSTAT_A>;
impl SYSCTL_USBPDS_MEMSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_USBPDS_MEMSTAT_A> {
        match self.bits {
            0 => Some(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_OFF),
            1 => Some(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_RETAIN),
            3 => Some(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_ON),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBPDS_MEMSTAT_OFF`"]
    #[inline(always)]
    pub fn is_sysctl_usbpds_memstat_off(&self) -> bool {
        *self == SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_OFF
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBPDS_MEMSTAT_RETAIN`"]
    #[inline(always)]
    pub fn is_sysctl_usbpds_memstat_retain(&self) -> bool {
        *self == SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_RETAIN
    }
    #[doc = "Checks if the value of the field is `SYSCTL_USBPDS_MEMSTAT_ON`"]
    #[inline(always)]
    pub fn is_sysctl_usbpds_memstat_on(&self) -> bool {
        *self == SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_ON
    }
}
#[doc = "Field `SYSCTL_USBPDS_MEMSTAT` writer - Memory Array Power Status"]
pub type SYSCTL_USBPDS_MEMSTAT_W<'a> =
    crate::FieldWriter<'a, u32, USBPDS_SPEC, u8, SYSCTL_USBPDS_MEMSTAT_A, 2, 2>;
impl<'a> SYSCTL_USBPDS_MEMSTAT_W<'a> {
    #[doc = "Array OFF"]
    #[inline(always)]
    pub fn sysctl_usbpds_memstat_off(self) -> &'a mut W {
        self.variant(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_OFF)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub fn sysctl_usbpds_memstat_retain(self) -> &'a mut W {
        self.variant(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_RETAIN)
    }
    #[doc = "Array On"]
    #[inline(always)]
    pub fn sysctl_usbpds_memstat_on(self) -> &'a mut W {
        self.variant(SYSCTL_USBPDS_MEMSTAT_A::SYSCTL_USBPDS_MEMSTAT_ON)
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_usbpds_pwrstat(&self) -> SYSCTL_USBPDS_PWRSTAT_R {
        SYSCTL_USBPDS_PWRSTAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_usbpds_memstat(&self) -> SYSCTL_USBPDS_MEMSTAT_R {
        SYSCTL_USBPDS_MEMSTAT_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Domain Status"]
    #[inline(always)]
    pub fn sysctl_usbpds_pwrstat(&mut self) -> SYSCTL_USBPDS_PWRSTAT_W {
        SYSCTL_USBPDS_PWRSTAT_W::new(self)
    }
    #[doc = "Bits 2:3 - Memory Array Power Status"]
    #[inline(always)]
    pub fn sysctl_usbpds_memstat(&mut self) -> SYSCTL_USBPDS_MEMSTAT_W {
        SYSCTL_USBPDS_MEMSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Power Domain Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpds](index.html) module"]
pub struct USBPDS_SPEC;
impl crate::RegisterSpec for USBPDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpds::R](R) reader structure"]
impl crate::Readable for USBPDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbpds::W](W) writer structure"]
impl crate::Writable for USBPDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBPDS to value 0"]
impl crate::Resettable for USBPDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
