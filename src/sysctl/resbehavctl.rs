#[doc = "Register `RESBEHAVCTL` reader"]
pub struct R(crate::R<RESBEHAVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESBEHAVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESBEHAVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESBEHAVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESBEHAVCTL` writer"]
pub struct W(crate::W<RESBEHAVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESBEHAVCTL_SPEC>;
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
impl From<crate::W<RESBEHAVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESBEHAVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External RST Pin Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RESBEHAVCTL_EXTRES_A {
    #[doc = "2: External RST assertion issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_EXTRES_SYSRST = 2,
    #[doc = "3: External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_EXTRES_POR = 3,
}
impl From<SYSCTL_RESBEHAVCTL_EXTRES_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RESBEHAVCTL_EXTRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_EXTRES` reader - External RST Pin Operation"]
pub type SYSCTL_RESBEHAVCTL_EXTRES_R = crate::FieldReader<u8, SYSCTL_RESBEHAVCTL_EXTRES_A>;
impl SYSCTL_RESBEHAVCTL_EXTRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RESBEHAVCTL_EXTRES_A> {
        match self.bits {
            2 => Some(SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST),
            3 => Some(SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_POR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_EXTRES_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_extres_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_EXTRES_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_extres_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_POR
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_EXTRES` writer - External RST Pin Operation"]
pub type SYSCTL_RESBEHAVCTL_EXTRES_W<'a> =
    crate::FieldWriter<'a, u32, RESBEHAVCTL_SPEC, u8, SYSCTL_RESBEHAVCTL_EXTRES_A, 2, 0>;
impl<'a> SYSCTL_RESBEHAVCTL_EXTRES_W<'a> {
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_SYSRST)
    }
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_EXTRES_A::SYSCTL_RESBEHAVCTL_EXTRES_POR)
    }
}
#[doc = "BOR Reset operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RESBEHAVCTL_BOR_A {
    #[doc = "2: Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_BOR_SYSRST = 2,
    #[doc = "3: Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_BOR_POR = 3,
}
impl From<SYSCTL_RESBEHAVCTL_BOR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RESBEHAVCTL_BOR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_BOR` reader - BOR Reset operation"]
pub type SYSCTL_RESBEHAVCTL_BOR_R = crate::FieldReader<u8, SYSCTL_RESBEHAVCTL_BOR_A>;
impl SYSCTL_RESBEHAVCTL_BOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RESBEHAVCTL_BOR_A> {
        match self.bits {
            2 => Some(SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_SYSRST),
            3 => Some(SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_POR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_BOR_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_bor_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_BOR_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_bor_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_POR
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_BOR` writer - BOR Reset operation"]
pub type SYSCTL_RESBEHAVCTL_BOR_W<'a> =
    crate::FieldWriter<'a, u32, RESBEHAVCTL_SPEC, u8, SYSCTL_RESBEHAVCTL_BOR_A, 2, 2>;
impl<'a> SYSCTL_RESBEHAVCTL_BOR_W<'a> {
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_SYSRST)
    }
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_BOR_A::SYSCTL_RESBEHAVCTL_BOR_POR)
    }
}
#[doc = "Watchdog 0 Reset Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RESBEHAVCTL_WDOG0_A {
    #[doc = "2: Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG0_SYSRST = 2,
    #[doc = "3: Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG0_POR = 3,
}
impl From<SYSCTL_RESBEHAVCTL_WDOG0_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RESBEHAVCTL_WDOG0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_WDOG0` reader - Watchdog 0 Reset Operation"]
pub type SYSCTL_RESBEHAVCTL_WDOG0_R = crate::FieldReader<u8, SYSCTL_RESBEHAVCTL_WDOG0_A>;
impl SYSCTL_RESBEHAVCTL_WDOG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RESBEHAVCTL_WDOG0_A> {
        match self.bits {
            2 => Some(SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST),
            3 => Some(SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_POR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG0_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog0_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG0_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog0_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_POR
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_WDOG0` writer - Watchdog 0 Reset Operation"]
pub type SYSCTL_RESBEHAVCTL_WDOG0_W<'a> =
    crate::FieldWriter<'a, u32, RESBEHAVCTL_SPEC, u8, SYSCTL_RESBEHAVCTL_WDOG0_A, 2, 4>;
impl<'a> SYSCTL_RESBEHAVCTL_WDOG0_W<'a> {
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_SYSRST)
    }
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG0_A::SYSCTL_RESBEHAVCTL_WDOG0_POR)
    }
}
#[doc = "Watchdog 1 Reset Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_RESBEHAVCTL_WDOG1_A {
    #[doc = "2: Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSCTL_RESBEHAVCTL_WDOG1_SYSRST = 2,
    #[doc = "3: Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    SYSCTL_RESBEHAVCTL_WDOG1_POR = 3,
}
impl From<SYSCTL_RESBEHAVCTL_WDOG1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RESBEHAVCTL_WDOG1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_WDOG1` reader - Watchdog 1 Reset Operation"]
pub type SYSCTL_RESBEHAVCTL_WDOG1_R = crate::FieldReader<u8, SYSCTL_RESBEHAVCTL_WDOG1_A>;
impl SYSCTL_RESBEHAVCTL_WDOG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_RESBEHAVCTL_WDOG1_A> {
        match self.bits {
            2 => Some(SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST),
            3 => Some(SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_POR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG1_SYSRST`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog1_sysrst(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST
    }
    #[doc = "Checks if the value of the field is `SYSCTL_RESBEHAVCTL_WDOG1_POR`"]
    #[inline(always)]
    pub fn is_sysctl_resbehavctl_wdog1_por(&self) -> bool {
        *self == SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_POR
    }
}
#[doc = "Field `SYSCTL_RESBEHAVCTL_WDOG1` writer - Watchdog 1 Reset Operation"]
pub type SYSCTL_RESBEHAVCTL_WDOG1_W<'a> =
    crate::FieldWriter<'a, u32, RESBEHAVCTL_SPEC, u8, SYSCTL_RESBEHAVCTL_WDOG1_A, 2, 6>;
impl<'a> SYSCTL_RESBEHAVCTL_WDOG1_W<'a> {
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1_sysrst(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_SYSRST)
    }
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1_por(self) -> &'a mut W {
        self.variant(SYSCTL_RESBEHAVCTL_WDOG1_A::SYSCTL_RESBEHAVCTL_WDOG1_POR)
    }
}
impl R {
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres(&self) -> SYSCTL_RESBEHAVCTL_EXTRES_R {
        SYSCTL_RESBEHAVCTL_EXTRES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor(&self) -> SYSCTL_RESBEHAVCTL_BOR_R {
        SYSCTL_RESBEHAVCTL_BOR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0(&self) -> SYSCTL_RESBEHAVCTL_WDOG0_R {
        SYSCTL_RESBEHAVCTL_WDOG0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1(&self) -> SYSCTL_RESBEHAVCTL_WDOG1_R {
        SYSCTL_RESBEHAVCTL_WDOG1_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_extres(&mut self) -> SYSCTL_RESBEHAVCTL_EXTRES_W {
        SYSCTL_RESBEHAVCTL_EXTRES_W::new(self)
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_bor(&mut self) -> SYSCTL_RESBEHAVCTL_BOR_W {
        SYSCTL_RESBEHAVCTL_BOR_W::new(self)
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog0(&mut self) -> SYSCTL_RESBEHAVCTL_WDOG0_W {
        SYSCTL_RESBEHAVCTL_WDOG0_W::new(self)
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline(always)]
    pub fn sysctl_resbehavctl_wdog1(&mut self) -> SYSCTL_RESBEHAVCTL_WDOG1_W {
        SYSCTL_RESBEHAVCTL_WDOG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Behavior Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resbehavctl](index.html) module"]
pub struct RESBEHAVCTL_SPEC;
impl crate::RegisterSpec for RESBEHAVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resbehavctl::R](R) reader structure"]
impl crate::Readable for RESBEHAVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resbehavctl::W](W) writer structure"]
impl crate::Writable for RESBEHAVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESBEHAVCTL to value 0"]
impl crate::Resettable for RESBEHAVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
