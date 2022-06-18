#[doc = "Register `NVMSTAT` reader"]
pub struct R(crate::R<NVMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVMSTAT` writer"]
pub struct W(crate::W<NVMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVMSTAT_SPEC>;
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
impl From<crate::W<NVMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_NVMSTAT_FWB` reader - 32 Word Flash Write Buffer Available"]
pub type SYSCTL_NVMSTAT_FWB_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_NVMSTAT_FWB` writer - 32 Word Flash Write Buffer Available"]
pub type SYSCTL_NVMSTAT_FWB_W<'a> = crate::BitWriter<'a, u32, NVMSTAT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - 32 Word Flash Write Buffer Available"]
    #[inline(always)]
    pub fn sysctl_nvmstat_fwb(&self) -> SYSCTL_NVMSTAT_FWB_R {
        SYSCTL_NVMSTAT_FWB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32 Word Flash Write Buffer Available"]
    #[inline(always)]
    pub fn sysctl_nvmstat_fwb(&mut self) -> SYSCTL_NVMSTAT_FWB_W {
        SYSCTL_NVMSTAT_FWB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Volatile Memory Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmstat](index.html) module"]
pub struct NVMSTAT_SPEC;
impl crate::RegisterSpec for NVMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvmstat::R](R) reader structure"]
impl crate::Readable for NVMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvmstat::W](W) writer structure"]
impl crate::Writable for NVMSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVMSTAT to value 0"]
impl crate::Resettable for NVMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
