#[doc = "Register `PCUSB` reader"]
pub struct R(crate::R<PCUSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCUSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCUSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCUSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCUSB` writer"]
pub struct W(crate::W<PCUSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCUSB_SPEC>;
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
impl From<crate::W<PCUSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCUSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_PCUSB_P0` reader - USB Module Power Control"]
pub type SYSCTL_PCUSB_P0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCTL_PCUSB_P0` writer - USB Module Power Control"]
pub type SYSCTL_PCUSB_P0_W<'a> = crate::BitWriter<'a, u32, PCUSB_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - USB Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcusb_p0(&self) -> SYSCTL_PCUSB_P0_R {
        SYSCTL_PCUSB_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Power Control"]
    #[inline(always)]
    pub fn sysctl_pcusb_p0(&mut self) -> SYSCTL_PCUSB_P0_W {
        SYSCTL_PCUSB_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Universal Serial Bus Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcusb](index.html) module"]
pub struct PCUSB_SPEC;
impl crate::RegisterSpec for PCUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcusb::R](R) reader structure"]
impl crate::Readable for PCUSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcusb::W](W) writer structure"]
impl crate::Writable for PCUSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCUSB to value 0"]
impl crate::Resettable for PCUSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
