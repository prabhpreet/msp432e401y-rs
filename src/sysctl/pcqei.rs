#[doc = "Register `PCQEI` reader"]
pub struct R(crate::R<PCQEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCQEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCQEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCQEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCQEI` writer"]
pub struct W(crate::W<PCQEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCQEI_SPEC>;
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
impl From<crate::W<PCQEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCQEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCQEI_P0` reader - QEI Module 0 Power Control"]
pub type SYSCTL_PCQEI_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCQEI_P0` writer - QEI Module 0 Power Control"]
pub type SYSCTL_PCQEI_P0_W<'a> = crate::BitWriter<'a, u32, PCQEI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcqei_p0(&self) -> SYSCTL_PCQEI_P0_R {
        SYSCTL_PCQEI_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pcqei_p0(&mut self) -> SYSCTL_PCQEI_P0_W {
        SYSCTL_PCQEI_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Encoder Interface Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcqei](index.html) module"]
pub struct PCQEI_SPEC;
impl crate::RegisterSpec for PCQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcqei::R](R) reader structure"]
impl crate::Readable for PCQEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcqei::W](W) writer structure"]
impl crate::Writable for PCQEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCQEI to value 0"]
impl crate::Resettable for PCQEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
