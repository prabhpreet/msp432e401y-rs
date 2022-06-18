#[doc = "Register `PIOSCSTAT` reader"]
pub struct R(crate::R<PIOSCSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOSCSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOSCSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOSCSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIOSCSTAT` writer"]
pub struct W(crate::W<PIOSCSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIOSCSTAT_SPEC>;
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
impl From<crate::W<PIOSCSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIOSCSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_CT` reader - Calibration Trim Value"]
pub type SYSCTL_PIOSCSTAT_CT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_PIOSCSTAT_CT` writer - Calibration Trim Value"]
pub type SYSCTL_PIOSCSTAT_CT_W<'a> = crate::FieldWriter<'a, u32, PIOSCSTAT_SPEC, u8, u8, 7, 0>;
#[doc = "Calibration Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCTL_PIOSCSTAT_CR_A {
    #[doc = "0: Calibration has not been attempted"]
    SYSCTL_PIOSCSTAT_CRNONE = 0,
    #[doc = "1: The last calibration operation completed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRPASS = 1,
    #[doc = "2: The last calibration operation failed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRFAIL = 2,
}
impl From<SYSCTL_PIOSCSTAT_CR_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_PIOSCSTAT_CR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_CR` reader - Calibration Result"]
pub type SYSCTL_PIOSCSTAT_CR_R = crate::FieldReader<u8, SYSCTL_PIOSCSTAT_CR_A>;
impl SYSCTL_PIOSCSTAT_CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCTL_PIOSCSTAT_CR_A> {
        match self.bits {
            0 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE),
            1 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS),
            2 => Some(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRNONE`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crnone(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRPASS`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crpass(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRFAIL`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crfail(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_CR` writer - Calibration Result"]
pub type SYSCTL_PIOSCSTAT_CR_W<'a> =
    crate::FieldWriter<'a, u32, PIOSCSTAT_SPEC, u8, SYSCTL_PIOSCSTAT_CR_A, 2, 8>;
impl<'a> SYSCTL_PIOSCSTAT_CR_W<'a> {
    #[doc = "Calibration has not been attempted"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crnone(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRNONE)
    }
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crpass(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRPASS)
    }
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crfail(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CR_A::SYSCTL_PIOSCSTAT_CRFAIL)
    }
}
#[doc = "Field `SYSCTL_PIOSCSTAT_DT` reader - Default Trim Value"]
pub type SYSCTL_PIOSCSTAT_DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_PIOSCSTAT_DT` writer - Default Trim Value"]
pub type SYSCTL_PIOSCSTAT_DT_W<'a> = crate::FieldWriter<'a, u32, PIOSCSTAT_SPEC, u8, u8, 7, 16>;
impl R {
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_ct(&self) -> SYSCTL_PIOSCSTAT_CT_R {
        SYSCTL_PIOSCSTAT_CT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn sysctl_pioscstat_cr(&self) -> SYSCTL_PIOSCSTAT_CR_R {
        SYSCTL_PIOSCSTAT_CR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_dt(&self) -> SYSCTL_PIOSCSTAT_DT_R {
        SYSCTL_PIOSCSTAT_DT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_ct(&mut self) -> SYSCTL_PIOSCSTAT_CT_W {
        SYSCTL_PIOSCSTAT_CT_W::new(self)
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn sysctl_pioscstat_cr(&mut self) -> SYSCTL_PIOSCSTAT_CR_W {
        SYSCTL_PIOSCSTAT_CR_W::new(self)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_dt(&mut self) -> SYSCTL_PIOSCSTAT_DT_W {
        SYSCTL_PIOSCSTAT_DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Precision Internal Oscillator Statistics\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioscstat](index.html) module"]
pub struct PIOSCSTAT_SPEC;
impl crate::RegisterSpec for PIOSCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pioscstat::R](R) reader structure"]
impl crate::Readable for PIOSCSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pioscstat::W](W) writer structure"]
impl crate::Writable for PIOSCSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIOSCSTAT to value 0"]
impl crate::Resettable for PIOSCSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
