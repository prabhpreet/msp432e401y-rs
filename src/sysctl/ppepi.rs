#[doc = "Register `PPEPI` reader"]
pub struct R(crate::R<PPEPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPEPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPEPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPEPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPEPI` writer"]
pub struct W(crate::W<PPEPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPEPI_SPEC>;
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
impl From<crate::W<PPEPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPEPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPEPI_P0` reader - EPI Module Present"]
pub type SYSCTL_PPEPI_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPEPI_P0` writer - EPI Module Present"]
pub type SYSCTL_PPEPI_P0_W<'a> = crate::BitWriter<'a, u32, PPEPI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EPI Module Present"]
    #[inline(always)]
    pub fn sysctl_ppepi_p0(&self) -> SYSCTL_PPEPI_P0_R {
        SYSCTL_PPEPI_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EPI Module Present"]
    #[inline(always)]
    pub fn sysctl_ppepi_p0(&mut self) -> SYSCTL_PPEPI_P0_W {
        SYSCTL_PPEPI_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppepi](index.html) module"]
pub struct PPEPI_SPEC;
impl crate::RegisterSpec for PPEPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppepi::R](R) reader structure"]
impl crate::Readable for PPEPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppepi::W](W) writer structure"]
impl crate::Writable for PPEPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPEPI to value 0"]
impl crate::Resettable for PPEPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
