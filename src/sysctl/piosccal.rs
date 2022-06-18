#[doc = "Register `PIOSCCAL` reader"]
pub struct R(crate::R<PIOSCCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOSCCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOSCCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOSCCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIOSCCAL` writer"]
pub struct W(crate::W<PIOSCCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIOSCCAL_SPEC>;
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
impl From<crate::W<PIOSCCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIOSCCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PIOSCCAL_UT` reader - User Trim Value"]
pub type SYSCTL_PIOSCCAL_UT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_PIOSCCAL_UT` writer - User Trim Value"]
pub type SYSCTL_PIOSCCAL_UT_W<'a> = crate::FieldWriter<'a, u32, PIOSCCAL_SPEC, u8, u8, 7, 0>;
#[doc = "Field `SYSCTL_PIOSCCAL_UPDATE` reader - Update Trim"]
pub type SYSCTL_PIOSCCAL_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PIOSCCAL_UPDATE` writer - Update Trim"]
pub type SYSCTL_PIOSCCAL_UPDATE_W<'a> = crate::BitWriter<'a, u32, PIOSCCAL_SPEC, bool, 8>;
#[doc = "Field `SYSCTL_PIOSCCAL_CAL` reader - Start Calibration"]
pub type SYSCTL_PIOSCCAL_CAL_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PIOSCCAL_CAL` writer - Start Calibration"]
pub type SYSCTL_PIOSCCAL_CAL_W<'a> = crate::BitWriter<'a, u32, PIOSCCAL_SPEC, bool, 9>;
#[doc = "Field `SYSCTL_PIOSCCAL_UTEN` reader - Use User Trim Value"]
pub type SYSCTL_PIOSCCAL_UTEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PIOSCCAL_UTEN` writer - Use User Trim Value"]
pub type SYSCTL_PIOSCCAL_UTEN_W<'a> = crate::BitWriter<'a, u32, PIOSCCAL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_ut(&self) -> SYSCTL_PIOSCCAL_UT_R {
        SYSCTL_PIOSCCAL_UT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    pub fn sysctl_piosccal_update(&self) -> SYSCTL_PIOSCCAL_UPDATE_R {
        SYSCTL_PIOSCCAL_UPDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    pub fn sysctl_piosccal_cal(&self) -> SYSCTL_PIOSCCAL_CAL_R {
        SYSCTL_PIOSCCAL_CAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_uten(&self) -> SYSCTL_PIOSCCAL_UTEN_R {
        SYSCTL_PIOSCCAL_UTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_ut(&mut self) -> SYSCTL_PIOSCCAL_UT_W {
        SYSCTL_PIOSCCAL_UT_W::new(self)
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    pub fn sysctl_piosccal_update(&mut self) -> SYSCTL_PIOSCCAL_UPDATE_W {
        SYSCTL_PIOSCCAL_UPDATE_W::new(self)
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    pub fn sysctl_piosccal_cal(&mut self) -> SYSCTL_PIOSCCAL_CAL_W {
        SYSCTL_PIOSCCAL_CAL_W::new(self)
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_uten(&mut self) -> SYSCTL_PIOSCCAL_UTEN_W {
        SYSCTL_PIOSCCAL_UTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Precision Internal Oscillator Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piosccal](index.html) module"]
pub struct PIOSCCAL_SPEC;
impl crate::RegisterSpec for PIOSCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [piosccal::R](R) reader structure"]
impl crate::Readable for PIOSCCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [piosccal::W](W) writer structure"]
impl crate::Writable for PIOSCCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIOSCCAL to value 0"]
impl crate::Resettable for PIOSCCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
