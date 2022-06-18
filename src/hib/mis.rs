#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_MIS_RTCALT0` reader - RTC Alert 0 Masked Interrupt Status"]
pub type HIB_MIS_RTCALT0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_RTCALT0` writer - RTC Alert 0 Masked Interrupt Status"]
pub type HIB_MIS_RTCALT0_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 0>;
#[doc = "Field `HIB_MIS_LOWBAT` reader - Low Battery Voltage Masked Interrupt Status"]
pub type HIB_MIS_LOWBAT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_LOWBAT` writer - Low Battery Voltage Masked Interrupt Status"]
pub type HIB_MIS_LOWBAT_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 2>;
#[doc = "Field `HIB_MIS_EXTW` reader - External Wake-Up Masked Interrupt Status"]
pub type HIB_MIS_EXTW_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_EXTW` writer - External Wake-Up Masked Interrupt Status"]
pub type HIB_MIS_EXTW_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 3>;
#[doc = "Field `HIB_MIS_WC` reader - Write Complete/Capable Masked Interrupt Status"]
pub type HIB_MIS_WC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_WC` writer - Write Complete/Capable Masked Interrupt Status"]
pub type HIB_MIS_WC_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 4>;
#[doc = "Field `HIB_MIS_PADIOWK` reader - Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_MIS_PADIOWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_PADIOWK` writer - Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_MIS_PADIOWK_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 5>;
#[doc = "Field `HIB_MIS_RSTWK` reader - Reset Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_MIS_RSTWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_RSTWK` writer - Reset Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_MIS_RSTWK_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 6>;
#[doc = "Field `HIB_MIS_VDDFAIL` reader - VDD Fail Interrupt Mask"]
pub type HIB_MIS_VDDFAIL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_MIS_VDDFAIL` writer - VDD Fail Interrupt Mask"]
pub type HIB_MIS_VDDFAIL_W<'a> = crate::BitWriter<'a, u32, MIS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&self) -> HIB_MIS_RTCALT0_R {
        HIB_MIS_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&self) -> HIB_MIS_LOWBAT_R {
        HIB_MIS_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&self) -> HIB_MIS_EXTW_R {
        HIB_MIS_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_wc(&self) -> HIB_MIS_WC_R {
        HIB_MIS_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_padiowk(&self) -> HIB_MIS_PADIOWK_R {
        HIB_MIS_PADIOWK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_rstwk(&self) -> HIB_MIS_RSTWK_R {
        HIB_MIS_RSTWK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_vddfail(&self) -> HIB_MIS_VDDFAIL_R {
        HIB_MIS_VDDFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&mut self) -> HIB_MIS_RTCALT0_W {
        HIB_MIS_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&mut self) -> HIB_MIS_LOWBAT_W {
        HIB_MIS_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&mut self) -> HIB_MIS_EXTW_W {
        HIB_MIS_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_wc(&mut self) -> HIB_MIS_WC_W {
        HIB_MIS_WC_W::new(self)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_padiowk(&mut self) -> HIB_MIS_PADIOWK_W {
        HIB_MIS_PADIOWK_W::new(self)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_rstwk(&mut self) -> HIB_MIS_RSTWK_W {
        HIB_MIS_RSTWK_W::new(self)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_vddfail(&mut self) -> HIB_MIS_VDDFAIL_W {
        HIB_MIS_VDDFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
