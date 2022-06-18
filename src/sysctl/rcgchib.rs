#[doc = "Register `RCGCHIB` reader"]
pub struct R(crate::R<RCGCHIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCHIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCHIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCHIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCHIB` writer"]
pub struct W(crate::W<RCGCHIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCHIB_SPEC>;
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
impl From<crate::W<RCGCHIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCHIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_RCGCHIB_R0` reader - Hibernation Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCHIB_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_RCGCHIB_R0` writer - Hibernation Module Run Mode Clock Gating Control"]
pub type SYSCTL_RCGCHIB_R0_W<'a> = crate::BitWriter<'a, u32, RCGCHIB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgchib_r0(&self) -> SYSCTL_RCGCHIB_R0_R {
        SYSCTL_RCGCHIB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Run Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgchib_r0(&mut self) -> SYSCTL_RCGCHIB_R0_W {
        SYSCTL_RCGCHIB_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Run Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgchib](index.html) module"]
pub struct RCGCHIB_SPEC;
impl crate::RegisterSpec for RCGCHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgchib::R](R) reader structure"]
impl crate::Readable for RCGCHIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgchib::W](W) writer structure"]
impl crate::Writable for RCGCHIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCHIB to value 0"]
impl crate::Resettable for RCGCHIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
