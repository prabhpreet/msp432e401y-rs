#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_RIS_RTCALT0` reader - RTC Alert 0 Raw Interrupt Status"]
pub type HIB_RIS_RTCALT0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_RTCALT0` writer - RTC Alert 0 Raw Interrupt Status"]
pub type HIB_RIS_RTCALT0_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `HIB_RIS_LOWBAT` reader - Low Battery Voltage Raw Interrupt Status"]
pub type HIB_RIS_LOWBAT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_LOWBAT` writer - Low Battery Voltage Raw Interrupt Status"]
pub type HIB_RIS_LOWBAT_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `HIB_RIS_EXTW` reader - External Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_EXTW_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_EXTW` writer - External Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_EXTW_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
#[doc = "Field `HIB_RIS_WC` reader - Write Complete/Capable Raw Interrupt Status"]
pub type HIB_RIS_WC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_WC` writer - Write Complete/Capable Raw Interrupt Status"]
pub type HIB_RIS_WC_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 4>;
#[doc = "Field `HIB_RIS_PADIOWK` reader - Pad I/O Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_PADIOWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_PADIOWK` writer - Pad I/O Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_PADIOWK_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 5>;
#[doc = "Field `HIB_RIS_RSTWK` reader - Reset Pad I/O Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_RSTWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_RSTWK` writer - Reset Pad I/O Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_RSTWK_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 6>;
#[doc = "Field `HIB_RIS_VDDFAIL` reader - VDD Fail Raw Interrupt Status"]
pub type HIB_RIS_VDDFAIL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_RIS_VDDFAIL` writer - VDD Fail Raw Interrupt Status"]
pub type HIB_RIS_VDDFAIL_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_rtcalt0(&self) -> HIB_RIS_RTCALT0_R {
        HIB_RIS_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_lowbat(&self) -> HIB_RIS_LOWBAT_R {
        HIB_RIS_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_extw(&self) -> HIB_RIS_EXTW_R {
        HIB_RIS_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_wc(&self) -> HIB_RIS_WC_R {
        HIB_RIS_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_padiowk(&self) -> HIB_RIS_PADIOWK_R {
        HIB_RIS_PADIOWK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_rstwk(&self) -> HIB_RIS_RSTWK_R {
        HIB_RIS_RSTWK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_vddfail(&self) -> HIB_RIS_VDDFAIL_R {
        HIB_RIS_VDDFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_rtcalt0(&mut self) -> HIB_RIS_RTCALT0_W {
        HIB_RIS_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_lowbat(&mut self) -> HIB_RIS_LOWBAT_W {
        HIB_RIS_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_extw(&mut self) -> HIB_RIS_EXTW_W {
        HIB_RIS_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_wc(&mut self) -> HIB_RIS_WC_W {
        HIB_RIS_WC_W::new(self)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_padiowk(&mut self) -> HIB_RIS_PADIOWK_W {
        HIB_RIS_PADIOWK_W::new(self)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_rstwk(&mut self) -> HIB_RIS_RSTWK_W {
        HIB_RIS_RSTWK_W::new(self)
    }
    #[doc = "Bit 7 - VDD Fail Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_vddfail(&mut self) -> HIB_RIS_VDDFAIL_W {
        HIB_RIS_VDDFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
