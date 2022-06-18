#[doc = "Register `SRACMP` reader"]
pub struct R(crate::R<SRACMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRACMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRACMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRACMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRACMP` writer"]
pub struct W(crate::W<SRACMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRACMP_SPEC>;
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
impl From<crate::W<SRACMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRACMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRACMP_R0` reader - Analog Comparator Module 0 Software Reset"]
pub type SYSCTL_SRACMP_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRACMP_R0` writer - Analog Comparator Module 0 Software Reset"]
pub type SYSCTL_SRACMP_R0_W<'a> = crate::BitWriter<'a, u32, SRACMP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sracmp_r0(&self) -> SYSCTL_SRACMP_R0_R {
        SYSCTL_SRACMP_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sracmp_r0(&mut self) -> SYSCTL_SRACMP_R0_W {
        SYSCTL_SRACMP_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sracmp](index.html) module"]
pub struct SRACMP_SPEC;
impl crate::RegisterSpec for SRACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sracmp::R](R) reader structure"]
impl crate::Readable for SRACMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sracmp::W](W) writer structure"]
impl crate::Writable for SRACMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRACMP to value 0"]
impl crate::Resettable for SRACMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
