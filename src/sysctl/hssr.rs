#[doc = "Register `HSSR` reader"]
pub struct R(crate::R<HSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSSR` writer"]
pub struct W(crate::W<HSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSSR_SPEC>;
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
impl From<crate::W<HSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCTL_HSSR_CDOFF` reader - Command Descriptor Pointer"]
pub type SYSCTL_HSSR_CDOFF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYSCTL_HSSR_CDOFF` writer - Command Descriptor Pointer"]
pub type SYSCTL_HSSR_CDOFF_W<'a> = crate::FieldWriter<'a, u32, HSSR_SPEC, u32, u32, 24, 0>;
#[doc = "Field `SYSCTL_HSSR_KEY` reader - Write Key"]
pub type SYSCTL_HSSR_KEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCTL_HSSR_KEY` writer - Write Key"]
pub type SYSCTL_HSSR_KEY_W<'a> = crate::FieldWriter<'a, u32, HSSR_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn sysctl_hssr_cdoff(&self) -> SYSCTL_HSSR_CDOFF_R {
        SYSCTL_HSSR_CDOFF_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn sysctl_hssr_key(&self) -> SYSCTL_HSSR_KEY_R {
        SYSCTL_HSSR_KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn sysctl_hssr_cdoff(&mut self) -> SYSCTL_HSSR_CDOFF_W {
        SYSCTL_HSSR_CDOFF_W::new(self)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn sysctl_hssr_key(&mut self) -> SYSCTL_HSSR_KEY_W {
        SYSCTL_HSSR_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware System Service Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hssr](index.html) module"]
pub struct HSSR_SPEC;
impl crate::RegisterSpec for HSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hssr::R](R) reader structure"]
impl crate::Readable for HSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hssr::W](W) writer structure"]
impl crate::Writable for HSSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSSR to value 0"]
impl crate::Resettable for HSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
