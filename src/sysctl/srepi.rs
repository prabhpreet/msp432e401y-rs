#[doc = "Register `SREPI` reader"]
pub struct R(crate::R<SREPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREPI` writer"]
pub struct W(crate::W<SREPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREPI_SPEC>;
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
impl From<crate::W<SREPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SREPI_R0` reader - EPI Module Software Reset"]
pub type SYSCTL_SREPI_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SREPI_R0` writer - EPI Module Software Reset"]
pub type SYSCTL_SREPI_R0_W<'a> = crate::BitWriter<'a, u32, SREPI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EPI Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srepi_r0(&self) -> SYSCTL_SREPI_R0_R {
        SYSCTL_SREPI_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EPI Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srepi_r0(&mut self) -> SYSCTL_SREPI_R0_W {
        SYSCTL_SREPI_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srepi](index.html) module"]
pub struct SREPI_SPEC;
impl crate::RegisterSpec for SREPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srepi::R](R) reader structure"]
impl crate::Readable for SREPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srepi::W](W) writer structure"]
impl crate::Writable for SREPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREPI to value 0"]
impl crate::Resettable for SREPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
