#[doc = "Register `SRQEI` reader"]
pub struct R(crate::R<SRQEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRQEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRQEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRQEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRQEI` writer"]
pub struct W(crate::W<SRQEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRQEI_SPEC>;
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
impl From<crate::W<SRQEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRQEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRQEI_R0` reader - QEI Module 0 Software Reset"]
pub type SYSCTL_SRQEI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRQEI_R0` writer - QEI Module 0 Software Reset"]
pub type SYSCTL_SRQEI_R0_W<'a> = crate::BitWriter<'a, u32, SRQEI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srqei_r0(&self) -> SYSCTL_SRQEI_R0_R {
        SYSCTL_SRQEI_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_srqei_r0(&mut self) -> SYSCTL_SRQEI_R0_W {
        SYSCTL_SRQEI_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Encoder Interface Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srqei](index.html) module"]
pub struct SRQEI_SPEC;
impl crate::RegisterSpec for SRQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srqei::R](R) reader structure"]
impl crate::Readable for SRQEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srqei::W](W) writer structure"]
impl crate::Writable for SRQEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRQEI to value 0"]
impl crate::Resettable for SRQEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
