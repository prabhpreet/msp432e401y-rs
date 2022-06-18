#[doc = "Register `PWRTC` reader"]
pub struct R(crate::R<PWRTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRTC` writer"]
pub struct W(crate::W<PWRTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRTC_SPEC>;
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
impl From<crate::W<PWRTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PWRTC_VDD_UBOR` reader - VDD Under BOR Status"]
pub type SYSCTL_PWRTC_VDD_UBOR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PWRTC_VDD_UBOR` writer - VDD Under BOR Status"]
pub type SYSCTL_PWRTC_VDD_UBOR_W<'a> = crate::BitWriter<'a, u32, PWRTC_SPEC, bool, 0>;
#[doc = "Field `SYSCTL_PWRTC_VDDA_UBOR` reader - VDDA Under BOR Status"]
pub type SYSCTL_PWRTC_VDDA_UBOR_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PWRTC_VDDA_UBOR` writer - VDDA Under BOR Status"]
pub type SYSCTL_PWRTC_VDDA_UBOR_W<'a> = crate::BitWriter<'a, u32, PWRTC_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdd_ubor(&self) -> SYSCTL_PWRTC_VDD_UBOR_R {
        SYSCTL_PWRTC_VDD_UBOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdda_ubor(&self) -> SYSCTL_PWRTC_VDDA_UBOR_R {
        SYSCTL_PWRTC_VDDA_UBOR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDD Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdd_ubor(&mut self) -> SYSCTL_PWRTC_VDD_UBOR_W {
        SYSCTL_PWRTC_VDD_UBOR_W::new(self)
    }
    #[doc = "Bit 4 - VDDA Under BOR Status"]
    #[inline(always)]
    pub fn sysctl_pwrtc_vdda_ubor(&mut self) -> SYSCTL_PWRTC_VDDA_UBOR_W {
        SYSCTL_PWRTC_VDDA_UBOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-Temperature Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrtc](index.html) module"]
pub struct PWRTC_SPEC;
impl crate::RegisterSpec for PWRTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrtc::R](R) reader structure"]
impl crate::Readable for PWRTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrtc::W](W) writer structure"]
impl crate::Writable for PWRTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRTC to value 0"]
impl crate::Resettable for PWRTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
