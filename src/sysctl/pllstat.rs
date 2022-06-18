#[doc = "Register `PLLSTAT` reader"]
pub struct R(crate::R<PLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSTAT` writer"]
pub struct W(crate::W<PLLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSTAT_SPEC>;
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
impl From<crate::W<PLLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PLLSTAT_LOCK` reader - PLL Lock"]
pub type SYSCTL_PLLSTAT_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PLLSTAT_LOCK` writer - PLL Lock"]
pub type SYSCTL_PLLSTAT_LOCK_W<'a> = crate::BitWriter<'a, u32, PLLSTAT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - PLL Lock"]
    #[inline(always)]
    pub fn sysctl_pllstat_lock(&self) -> SYSCTL_PLLSTAT_LOCK_R {
        SYSCTL_PLLSTAT_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock"]
    #[inline(always)]
    pub fn sysctl_pllstat_lock(&mut self) -> SYSCTL_PLLSTAT_LOCK_W {
        SYSCTL_PLLSTAT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllstat](index.html) module"]
pub struct PLLSTAT_SPEC;
impl crate::RegisterSpec for PLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllstat::R](R) reader structure"]
impl crate::Readable for PLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllstat::W](W) writer structure"]
impl crate::Writable for PLLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSTAT to value 0"]
impl crate::Resettable for PLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
