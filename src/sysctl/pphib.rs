#[doc = "Register `PPHIB` reader"]
pub struct R(crate::R<PPHIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPHIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPHIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPHIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPHIB` writer"]
pub struct W(crate::W<PPHIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPHIB_SPEC>;
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
impl From<crate::W<PPHIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPHIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPHIB_P0` reader - Hibernation Module Present"]
pub type SYSCTL_PPHIB_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPHIB_P0` writer - Hibernation Module Present"]
pub type SYSCTL_PPHIB_P0_W<'a> = crate::BitWriter<'a, u32, PPHIB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Present"]
    #[inline(always)]
    pub fn sysctl_pphib_p0(&self) -> SYSCTL_PPHIB_P0_R {
        SYSCTL_PPHIB_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Present"]
    #[inline(always)]
    pub fn sysctl_pphib_p0(&mut self) -> SYSCTL_PPHIB_P0_W {
        SYSCTL_PPHIB_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pphib](index.html) module"]
pub struct PPHIB_SPEC;
impl crate::RegisterSpec for PPHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pphib::R](R) reader structure"]
impl crate::Readable for PPHIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pphib::W](W) writer structure"]
impl crate::Writable for PPHIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPHIB to value 0"]
impl crate::Resettable for PPHIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
