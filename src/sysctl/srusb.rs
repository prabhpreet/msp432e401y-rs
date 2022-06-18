#[doc = "Register `SRUSB` reader"]
pub struct R(crate::R<SRUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRUSB` writer"]
pub struct W(crate::W<SRUSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRUSB_SPEC>;
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
impl From<crate::W<SRUSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRUSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_SRUSB_R0` reader - USB Module Software Reset"]
pub type SYSCTL_SRUSB_R0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_SRUSB_R0` writer - USB Module Software Reset"]
pub type SYSCTL_SRUSB_R0_W<'a> = crate::BitWriter<'a, u32, SRUSB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srusb_r0(&self) -> SYSCTL_SRUSB_R0_R {
        SYSCTL_SRUSB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srusb_r0(&mut self) -> SYSCTL_SRUSB_R0_W {
        SYSCTL_SRUSB_R0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Serial Bus Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srusb](index.html) module"]
pub struct SRUSB_SPEC;
impl crate::RegisterSpec for SRUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srusb::R](R) reader structure"]
impl crate::Readable for SRUSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srusb::W](W) writer structure"]
impl crate::Writable for SRUSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRUSB to value 0"]
impl crate::Resettable for SRUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
