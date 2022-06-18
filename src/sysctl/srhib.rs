#[doc = "Register `SRHIB` reader"]
pub struct R(crate::R<SRHIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRHIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRHIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRHIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRHIB` writer"]
pub struct W(crate::W<SRHIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRHIB_SPEC>;
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
impl From<crate::W<SRHIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRHIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRHIB_R0` reader - Hibernation Module Software Reset"]
pub type SYSCTL_SRHIB_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRHIB_R0` writer - Hibernation Module Software Reset"]
pub type SYSCTL_SRHIB_R0_W<'a> = crate::BitWriter<'a, u32, SRHIB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srhib_r0(&self) -> SYSCTL_SRHIB_R0_R {
        SYSCTL_SRHIB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srhib_r0(&mut self) -> SYSCTL_SRHIB_R0_W {
        SYSCTL_SRHIB_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srhib](index.html) module"]
pub struct SRHIB_SPEC;
impl crate::RegisterSpec for SRHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srhib::R](R) reader structure"]
impl crate::Readable for SRHIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srhib::W](W) writer structure"]
impl crate::Writable for SRHIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRHIB to value 0"]
impl crate::Resettable for SRHIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
