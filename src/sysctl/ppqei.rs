#[doc = "Register `PPQEI` reader"]
pub struct R(crate::R<PPQEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPQEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPQEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPQEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPQEI` writer"]
pub struct W(crate::W<PPQEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPQEI_SPEC>;
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
impl From<crate::W<PPQEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPQEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PPQEI_P0` reader - QEI Module 0 Present"]
pub type SYSCTL_PPQEI_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PPQEI_P0` writer - QEI Module 0 Present"]
pub type SYSCTL_PPQEI_P0_W<'a> = crate::BitWriter<'a, u32, PPQEI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppqei_p0(&self) -> SYSCTL_PPQEI_P0_R {
        SYSCTL_PPQEI_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppqei_p0(&mut self) -> SYSCTL_PPQEI_P0_W {
        SYSCTL_PPQEI_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Encoder Interface Peripheral Present\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppqei](index.html) module"]
pub struct PPQEI_SPEC;
impl crate::RegisterSpec for PPQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppqei::R](R) reader structure"]
impl crate::Readable for PPQEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppqei::W](W) writer structure"]
impl crate::Writable for PPQEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPQEI to value 0"]
impl crate::Resettable for PPQEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
