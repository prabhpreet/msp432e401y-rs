#[doc = "Register `PRHIB` reader"]
pub struct R(crate::R<PRHIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRHIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRHIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRHIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRHIB` writer"]
pub struct W(crate::W<PRHIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRHIB_SPEC>;
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
impl From<crate::W<PRHIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRHIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PRHIB_R0` reader - Hibernation Module Peripheral Ready"]
pub type SYSCTL_PRHIB_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PRHIB_R0` writer - Hibernation Module Peripheral Ready"]
pub type SYSCTL_PRHIB_R0_W<'a> = crate::BitWriter<'a, u32, PRHIB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prhib_r0(&self) -> SYSCTL_PRHIB_R0_R {
        SYSCTL_PRHIB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prhib_r0(&mut self) -> SYSCTL_PRHIB_R0_W {
        SYSCTL_PRHIB_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Peripheral Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prhib](index.html) module"]
pub struct PRHIB_SPEC;
impl crate::RegisterSpec for PRHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prhib::R](R) reader structure"]
impl crate::Readable for PRHIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prhib::W](W) writer structure"]
impl crate::Writable for PRHIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRHIB to value 0"]
impl crate::Resettable for PRHIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
