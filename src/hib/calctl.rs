#[doc = "Register `CALCTL` reader"]
pub struct R(crate::R<CALCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCTL` writer"]
pub struct W(crate::W<CALCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCTL_SPEC>;
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
impl From<crate::W<CALCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CALCTL_CALEN` reader - RTC Calendar/Counter Mode Select"]
pub type HIB_CALCTL_CALEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CALCTL_CALEN` writer - RTC Calendar/Counter Mode Select"]
pub type HIB_CALCTL_CALEN_W<'a> = crate::BitWriter<'a, u32, CALCTL_SPEC, bool, 0>;
#[doc = "Field `HIB_CALCTL_CAL24` reader - Calendar Mode"]
pub type HIB_CALCTL_CAL24_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CALCTL_CAL24` writer - Calendar Mode"]
pub type HIB_CALCTL_CAL24_W<'a> = crate::BitWriter<'a, u32, CALCTL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - RTC Calendar/Counter Mode Select"]
    #[inline(always)]
    pub fn hib_calctl_calen(&self) -> HIB_CALCTL_CALEN_R {
        HIB_CALCTL_CALEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Calendar Mode"]
    #[inline(always)]
    pub fn hib_calctl_cal24(&self) -> HIB_CALCTL_CAL24_R {
        HIB_CALCTL_CAL24_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Calendar/Counter Mode Select"]
    #[inline(always)]
    pub fn hib_calctl_calen(&mut self) -> HIB_CALCTL_CALEN_W {
        HIB_CALCTL_CALEN_W::new(self)
    }
    #[doc = "Bit 2 - Calendar Mode"]
    #[inline(always)]
    pub fn hib_calctl_cal24(&mut self) -> HIB_CALCTL_CAL24_W {
        HIB_CALCTL_CAL24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Calendar Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctl](index.html) module"]
pub struct CALCTL_SPEC;
impl crate::RegisterSpec for CALCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calctl::R](R) reader structure"]
impl crate::Readable for CALCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calctl::W](W) writer structure"]
impl crate::Writable for CALCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALCTL to value 0"]
impl crate::Resettable for CALCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
