#[doc = "Register `DCGCUSB` reader"]
pub struct R(crate::R<DCGCUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCUSB` writer"]
pub struct W(crate::W<DCGCUSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCUSB_SPEC>;
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
impl From<crate::W<DCGCUSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCUSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_DCGCUSB_D0` reader - USB Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUSB_D0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_DCGCUSB_D0` writer - USB Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUSB_D0_W<'a> = crate::BitWriter<'a, u32, DCGCUSB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcusb_d0(&self) -> SYSCTL_DCGCUSB_D0_R {
        SYSCTL_DCGCUSB_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcusb_d0(&mut self) -> SYSCTL_DCGCUSB_D0_W {
        SYSCTL_DCGCUSB_D0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgcusb](index.html) module"]
pub struct DCGCUSB_SPEC;
impl crate::RegisterSpec for DCGCUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgcusb::R](R) reader structure"]
impl crate::Readable for DCGCUSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgcusb::W](W) writer structure"]
impl crate::Writable for DCGCUSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCUSB to value 0"]
impl crate::Resettable for DCGCUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
