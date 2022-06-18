#[doc = "Register `SCGCHIB` reader"]
pub struct R(crate::R<SCGCHIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCHIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCHIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCHIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCHIB` writer"]
pub struct W(crate::W<SCGCHIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCHIB_SPEC>;
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
impl From<crate::W<SCGCHIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCHIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SCGCHIB_S0` reader - Hibernation Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCHIB_S0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SCGCHIB_S0` writer - Hibernation Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCHIB_S0_W<'a> = crate::BitWriter<'a, u32, SCGCHIB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgchib_s0(&self) -> SYSCTL_SCGCHIB_S0_R {
        SYSCTL_SCGCHIB_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgchib_s0(&mut self) -> SYSCTL_SCGCHIB_S0_W {
        SYSCTL_SCGCHIB_S0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgchib](index.html) module"]
pub struct SCGCHIB_SPEC;
impl crate::RegisterSpec for SCGCHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgchib::R](R) reader structure"]
impl crate::Readable for SCGCHIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgchib::W](W) writer structure"]
impl crate::Writable for SCGCHIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCGCHIB to value 0"]
impl crate::Resettable for SCGCHIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
