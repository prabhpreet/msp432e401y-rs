#[doc = "Register `TPSTAT` reader"]
pub struct R(crate::R<TPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPSTAT` writer"]
pub struct W(crate::W<TPSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPSTAT_SPEC>;
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
impl From<crate::W<TPSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_TPSTAT_XOSCFAIL` reader - External Oscillator Failure"]
pub type HIB_TPSTAT_XOSCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPSTAT_XOSCFAIL` writer - External Oscillator Failure"]
pub type HIB_TPSTAT_XOSCFAIL_W<'a> = crate::BitWriter<'a, u32, TPSTAT_SPEC, bool, 0>;
#[doc = "Field `HIB_TPSTAT_XOSCST` reader - External Oscillator Status"]
pub type HIB_TPSTAT_XOSCST_R = crate::BitReader<bool>;
#[doc = "Field `HIB_TPSTAT_XOSCST` writer - External Oscillator Status"]
pub type HIB_TPSTAT_XOSCST_W<'a> = crate::BitWriter<'a, u32, TPSTAT_SPEC, bool, 1>;
#[doc = "Tamper Module Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HIB_TPSTAT_STATE_A {
    #[doc = "0: Tamper disabled"]
    HIB_TPSTAT_STATE_DISABLED = 0,
    #[doc = "1: Tamper configured"]
    HIB_TPSTAT_STATE_CONFIGED = 1,
    #[doc = "2: Tamper pin event occurred"]
    HIB_TPSTAT_STATE_ERROR = 2,
}
impl From<HIB_TPSTAT_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: HIB_TPSTAT_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HIB_TPSTAT_STATE` reader - Tamper Module Status"]
pub type HIB_TPSTAT_STATE_R = crate::FieldReader<u8, HIB_TPSTAT_STATE_A>;
impl HIB_TPSTAT_STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIB_TPSTAT_STATE_A> {
        match self.bits {
            0 => Some(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_DISABLED),
            1 => Some(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_CONFIGED),
            2 => Some(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_ERROR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_DISABLED`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_disabled(&self) -> bool {
        *self == HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_DISABLED
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_CONFIGED`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_configed(&self) -> bool {
        *self == HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_CONFIGED
    }
    #[doc = "Checks if the value of the field is `HIB_TPSTAT_STATE_ERROR`"]
    #[inline(always)]
    pub fn is_hib_tpstat_state_error(&self) -> bool {
        *self == HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_ERROR
    }
}
#[doc = "Field `HIB_TPSTAT_STATE` writer - Tamper Module Status"]
pub type HIB_TPSTAT_STATE_W<'a> =
    crate::FieldWriter<'a, u32, TPSTAT_SPEC, u8, HIB_TPSTAT_STATE_A, 2, 2>;
impl<'a> HIB_TPSTAT_STATE_W<'a> {
    #[doc = "Tamper disabled"]
    #[inline(always)]
    pub fn hib_tpstat_state_disabled(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_DISABLED)
    }
    #[doc = "Tamper configured"]
    #[inline(always)]
    pub fn hib_tpstat_state_configed(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_CONFIGED)
    }
    #[doc = "Tamper pin event occurred"]
    #[inline(always)]
    pub fn hib_tpstat_state_error(self) -> &'a mut W {
        self.variant(HIB_TPSTAT_STATE_A::HIB_TPSTAT_STATE_ERROR)
    }
}
impl R {
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn hib_tpstat_xoscfail(&self) -> HIB_TPSTAT_XOSCFAIL_R {
        HIB_TPSTAT_XOSCFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn hib_tpstat_xoscst(&self) -> HIB_TPSTAT_XOSCST_R {
        HIB_TPSTAT_XOSCST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn hib_tpstat_state(&self) -> HIB_TPSTAT_STATE_R {
        HIB_TPSTAT_STATE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline(always)]
    pub fn hib_tpstat_xoscfail(&mut self) -> HIB_TPSTAT_XOSCFAIL_W {
        HIB_TPSTAT_XOSCFAIL_W::new(self)
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline(always)]
    pub fn hib_tpstat_xoscst(&mut self) -> HIB_TPSTAT_XOSCST_W {
        HIB_TPSTAT_XOSCST_W::new(self)
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline(always)]
    pub fn hib_tpstat_state(&mut self) -> HIB_TPSTAT_STATE_W {
        HIB_TPSTAT_STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIB Tamper Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpstat](index.html) module"]
pub struct TPSTAT_SPEC;
impl crate::RegisterSpec for TPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpstat::R](R) reader structure"]
impl crate::Readable for TPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpstat::W](W) writer structure"]
impl crate::Writable for TPSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPSTAT to value 0"]
impl crate::Resettable for TPSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
