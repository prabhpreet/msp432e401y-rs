#[doc = "Register `_2_CTL` reader"]
pub struct R(crate::R<_2_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_2_CTL` writer"]
pub struct W(crate::W<_2_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_2_CTL_SPEC>;
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
impl From<crate::W<_2_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_2_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM_2_CTL_ENABLE` reader - PWM Block Enable"]
pub type PWM_2_CTL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_ENABLE` writer - PWM Block Enable"]
pub type PWM_2_CTL_ENABLE_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 0>;
#[doc = "Field `PWM_2_CTL_MODE` reader - Counter Mode"]
pub type PWM_2_CTL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_MODE` writer - Counter Mode"]
pub type PWM_2_CTL_MODE_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 1>;
#[doc = "Field `PWM_2_CTL_DEBUG` reader - Debug Mode"]
pub type PWM_2_CTL_DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_DEBUG` writer - Debug Mode"]
pub type PWM_2_CTL_DEBUG_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 2>;
#[doc = "Field `PWM_2_CTL_LOADUPD` reader - Load Register Update Mode"]
pub type PWM_2_CTL_LOADUPD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_LOADUPD` writer - Load Register Update Mode"]
pub type PWM_2_CTL_LOADUPD_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 3>;
#[doc = "Field `PWM_2_CTL_CMPAUPD` reader - Comparator A Update Mode"]
pub type PWM_2_CTL_CMPAUPD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_CMPAUPD` writer - Comparator A Update Mode"]
pub type PWM_2_CTL_CMPAUPD_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 4>;
#[doc = "Field `PWM_2_CTL_CMPBUPD` reader - Comparator B Update Mode"]
pub type PWM_2_CTL_CMPBUPD_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_CMPBUPD` writer - Comparator B Update Mode"]
pub type PWM_2_CTL_CMPBUPD_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 5>;
#[doc = "PWMnGENA Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_CTL_GENAUPD_A {
    #[doc = "0: Immediate"]
    PWM_2_CTL_GENAUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_2_CTL_GENAUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_2_CTL_GENAUPD_GS = 3,
}
impl From<PWM_2_CTL_GENAUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_CTL_GENAUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_CTL_GENAUPD` reader - PWMnGENA Update Mode"]
pub type PWM_2_CTL_GENAUPD_R = crate::FieldReader<u8, PWM_2_CTL_GENAUPD_A>;
impl PWM_2_CTL_GENAUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_2_CTL_GENAUPD_A> {
        match self.bits {
            0 => Some(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_I),
            2 => Some(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_LS),
            3 => Some(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_GS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_i(&self) -> bool {
        *self == PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_ls(&self) -> bool {
        *self == PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_gs(&self) -> bool {
        *self == PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_GS
    }
}
#[doc = "Field `PWM_2_CTL_GENAUPD` writer - PWMnGENA Update Mode"]
pub type PWM_2_CTL_GENAUPD_W<'a> =
    crate::FieldWriter<'a, u32, _2_CTL_SPEC, u8, PWM_2_CTL_GENAUPD_A, 2, 6>;
impl<'a> PWM_2_CTL_GENAUPD_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPD_A::PWM_2_CTL_GENAUPD_GS)
    }
}
#[doc = "PWMnGENB Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_CTL_GENBUPD_A {
    #[doc = "0: Immediate"]
    PWM_2_CTL_GENBUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_2_CTL_GENBUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_2_CTL_GENBUPD_GS = 3,
}
impl From<PWM_2_CTL_GENBUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_CTL_GENBUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_CTL_GENBUPD` reader - PWMnGENB Update Mode"]
pub type PWM_2_CTL_GENBUPD_R = crate::FieldReader<u8, PWM_2_CTL_GENBUPD_A>;
impl PWM_2_CTL_GENBUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_2_CTL_GENBUPD_A> {
        match self.bits {
            0 => Some(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_I),
            2 => Some(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_LS),
            3 => Some(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_GS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_i(&self) -> bool {
        *self == PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_ls(&self) -> bool {
        *self == PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_gs(&self) -> bool {
        *self == PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_GS
    }
}
#[doc = "Field `PWM_2_CTL_GENBUPD` writer - PWMnGENB Update Mode"]
pub type PWM_2_CTL_GENBUPD_W<'a> =
    crate::FieldWriter<'a, u32, _2_CTL_SPEC, u8, PWM_2_CTL_GENBUPD_A, 2, 8>;
impl<'a> PWM_2_CTL_GENBUPD_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPD_A::PWM_2_CTL_GENBUPD_GS)
    }
}
#[doc = "PWMnDBCTL Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_CTL_DBCTLUPD_A {
    #[doc = "0: Immediate"]
    PWM_2_CTL_DBCTLUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_2_CTL_DBCTLUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_2_CTL_DBCTLUPD_GS = 3,
}
impl From<PWM_2_CTL_DBCTLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_CTL_DBCTLUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_CTL_DBCTLUPD` reader - PWMnDBCTL Update Mode"]
pub type PWM_2_CTL_DBCTLUPD_R = crate::FieldReader<u8, PWM_2_CTL_DBCTLUPD_A>;
impl PWM_2_CTL_DBCTLUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_2_CTL_DBCTLUPD_A> {
        match self.bits {
            0 => Some(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_I),
            2 => Some(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_LS),
            3 => Some(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_GS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_GS
    }
}
#[doc = "Field `PWM_2_CTL_DBCTLUPD` writer - PWMnDBCTL Update Mode"]
pub type PWM_2_CTL_DBCTLUPD_W<'a> =
    crate::FieldWriter<'a, u32, _2_CTL_SPEC, u8, PWM_2_CTL_DBCTLUPD_A, 2, 10>;
impl<'a> PWM_2_CTL_DBCTLUPD_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPD_A::PWM_2_CTL_DBCTLUPD_GS)
    }
}
#[doc = "PWMnDBRISE Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_CTL_DBRISEUPD_A {
    #[doc = "0: Immediate"]
    PWM_2_CTL_DBRISEUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_2_CTL_DBRISEUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_2_CTL_DBRISEUPD_GS = 3,
}
impl From<PWM_2_CTL_DBRISEUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_CTL_DBRISEUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_CTL_DBRISEUPD` reader - PWMnDBRISE Update Mode"]
pub type PWM_2_CTL_DBRISEUPD_R = crate::FieldReader<u8, PWM_2_CTL_DBRISEUPD_A>;
impl PWM_2_CTL_DBRISEUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_2_CTL_DBRISEUPD_A> {
        match self.bits {
            0 => Some(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_I),
            2 => Some(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_LS),
            3 => Some(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_GS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_GS
    }
}
#[doc = "Field `PWM_2_CTL_DBRISEUPD` writer - PWMnDBRISE Update Mode"]
pub type PWM_2_CTL_DBRISEUPD_W<'a> =
    crate::FieldWriter<'a, u32, _2_CTL_SPEC, u8, PWM_2_CTL_DBRISEUPD_A, 2, 12>;
impl<'a> PWM_2_CTL_DBRISEUPD_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPD_A::PWM_2_CTL_DBRISEUPD_GS)
    }
}
#[doc = "PWMnDBFALL Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM_2_CTL_DBFALLUPD_A {
    #[doc = "0: Immediate"]
    PWM_2_CTL_DBFALLUPD_I = 0,
    #[doc = "2: Locally Synchronized"]
    PWM_2_CTL_DBFALLUPD_LS = 2,
    #[doc = "3: Globally Synchronized"]
    PWM_2_CTL_DBFALLUPD_GS = 3,
}
impl From<PWM_2_CTL_DBFALLUPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM_2_CTL_DBFALLUPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM_2_CTL_DBFALLUPD` reader - PWMnDBFALL Update Mode"]
pub type PWM_2_CTL_DBFALLUPD_R = crate::FieldReader<u8, PWM_2_CTL_DBFALLUPD_A>;
impl PWM_2_CTL_DBFALLUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWM_2_CTL_DBFALLUPD_A> {
        match self.bits {
            0 => Some(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_I),
            2 => Some(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_LS),
            3 => Some(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_GS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_GS
    }
}
#[doc = "Field `PWM_2_CTL_DBFALLUPD` writer - PWMnDBFALL Update Mode"]
pub type PWM_2_CTL_DBFALLUPD_W<'a> =
    crate::FieldWriter<'a, u32, _2_CTL_SPEC, u8, PWM_2_CTL_DBFALLUPD_A, 2, 14>;
impl<'a> PWM_2_CTL_DBFALLUPD_W<'a> {
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPD_A::PWM_2_CTL_DBFALLUPD_GS)
    }
}
#[doc = "Field `PWM_2_CTL_FLTSRC` reader - Fault Condition Source"]
pub type PWM_2_CTL_FLTSRC_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_FLTSRC` writer - Fault Condition Source"]
pub type PWM_2_CTL_FLTSRC_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 16>;
#[doc = "Field `PWM_2_CTL_MINFLTPER` reader - Minimum Fault Period"]
pub type PWM_2_CTL_MINFLTPER_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_MINFLTPER` writer - Minimum Fault Period"]
pub type PWM_2_CTL_MINFLTPER_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 17>;
#[doc = "Field `PWM_2_CTL_LATCH` reader - Latch Fault Input"]
pub type PWM_2_CTL_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `PWM_2_CTL_LATCH` writer - Latch Fault Input"]
pub type PWM_2_CTL_LATCH_W<'a> = crate::BitWriter<'a, u32, _2_CTL_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_2_ctl_enable(&self) -> PWM_2_CTL_ENABLE_R {
        PWM_2_CTL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_mode(&self) -> PWM_2_CTL_MODE_R {
        PWM_2_CTL_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_debug(&self) -> PWM_2_CTL_DEBUG_R {
        PWM_2_CTL_DEBUG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_loadupd(&self) -> PWM_2_CTL_LOADUPD_R {
        PWM_2_CTL_LOADUPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpaupd(&self) -> PWM_2_CTL_CMPAUPD_R {
        PWM_2_CTL_CMPAUPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpbupd(&self) -> PWM_2_CTL_CMPBUPD_R {
        PWM_2_CTL_CMPBUPD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd(&self) -> PWM_2_CTL_GENAUPD_R {
        PWM_2_CTL_GENAUPD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd(&self) -> PWM_2_CTL_GENBUPD_R {
        PWM_2_CTL_GENBUPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd(&self) -> PWM_2_CTL_DBCTLUPD_R {
        PWM_2_CTL_DBCTLUPD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd(&self) -> PWM_2_CTL_DBRISEUPD_R {
        PWM_2_CTL_DBRISEUPD_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd(&self) -> PWM_2_CTL_DBFALLUPD_R {
        PWM_2_CTL_DBFALLUPD_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn pwm_2_ctl_fltsrc(&self) -> PWM_2_CTL_FLTSRC_R {
        PWM_2_CTL_FLTSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_2_ctl_minfltper(&self) -> PWM_2_CTL_MINFLTPER_R {
        PWM_2_CTL_MINFLTPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn pwm_2_ctl_latch(&self) -> PWM_2_CTL_LATCH_R {
        PWM_2_CTL_LATCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_2_ctl_enable(&mut self) -> PWM_2_CTL_ENABLE_W {
        PWM_2_CTL_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_mode(&mut self) -> PWM_2_CTL_MODE_W {
        PWM_2_CTL_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_debug(&mut self) -> PWM_2_CTL_DEBUG_W {
        PWM_2_CTL_DEBUG_W::new(self)
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_loadupd(&mut self) -> PWM_2_CTL_LOADUPD_W {
        PWM_2_CTL_LOADUPD_W::new(self)
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpaupd(&mut self) -> PWM_2_CTL_CMPAUPD_W {
        PWM_2_CTL_CMPAUPD_W::new(self)
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpbupd(&mut self) -> PWM_2_CTL_CMPBUPD_W {
        PWM_2_CTL_CMPBUPD_W::new(self)
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd(&mut self) -> PWM_2_CTL_GENAUPD_W {
        PWM_2_CTL_GENAUPD_W::new(self)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd(&mut self) -> PWM_2_CTL_GENBUPD_W {
        PWM_2_CTL_GENBUPD_W::new(self)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd(&mut self) -> PWM_2_CTL_DBCTLUPD_W {
        PWM_2_CTL_DBCTLUPD_W::new(self)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd(&mut self) -> PWM_2_CTL_DBRISEUPD_W {
        PWM_2_CTL_DBRISEUPD_W::new(self)
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd(&mut self) -> PWM_2_CTL_DBFALLUPD_W {
        PWM_2_CTL_DBFALLUPD_W::new(self)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn pwm_2_ctl_fltsrc(&mut self) -> PWM_2_CTL_FLTSRC_W {
        PWM_2_CTL_FLTSRC_W::new(self)
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_2_ctl_minfltper(&mut self) -> PWM_2_CTL_MINFLTPER_W {
        PWM_2_CTL_MINFLTPER_W::new(self)
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn pwm_2_ctl_latch(&mut self) -> PWM_2_CTL_LATCH_W {
        PWM_2_CTL_LATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_ctl](index.html) module"]
pub struct _2_CTL_SPEC;
impl crate::RegisterSpec for _2_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_ctl::R](R) reader structure"]
impl crate::Readable for _2_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_2_ctl::W](W) writer structure"]
impl crate::Writable for _2_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _2_CTL to value 0"]
impl crate::Resettable for _2_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
