#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_IM_RTCALT0` reader - RTC Alert 0 Interrupt Mask"]
pub type HIB_IM_RTCALT0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_RTCALT0` writer - RTC Alert 0 Interrupt Mask"]
pub type HIB_IM_RTCALT0_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 0>;
#[doc = "Field `HIB_IM_LOWBAT` reader - Low Battery Voltage Interrupt Mask"]
pub type HIB_IM_LOWBAT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_LOWBAT` writer - Low Battery Voltage Interrupt Mask"]
pub type HIB_IM_LOWBAT_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 2>;
#[doc = "Field `HIB_IM_EXTW` reader - External Wake-Up Interrupt Mask"]
pub type HIB_IM_EXTW_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_EXTW` writer - External Wake-Up Interrupt Mask"]
pub type HIB_IM_EXTW_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 3>;
#[doc = "Field `HIB_IM_WC` reader - External Write Complete/Capable Interrupt Mask"]
pub type HIB_IM_WC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_WC` writer - External Write Complete/Capable Interrupt Mask"]
pub type HIB_IM_WC_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 4>;
#[doc = "Field `HIB_IM_PADIOWK` reader - Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_IM_PADIOWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_PADIOWK` writer - Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_IM_PADIOWK_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 5>;
#[doc = "Field `HIB_IM_RSTWK` reader - Reset Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_IM_RSTWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_RSTWK` writer - Reset Pad I/O Wake-Up Interrupt Mask"]
pub type HIB_IM_RSTWK_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 6>;
#[doc = "Field `HIB_IM_VDDFAIL` reader - VDD Fail Interrupt Mask"]
pub type HIB_IM_VDDFAIL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IM_VDDFAIL` writer - VDD Fail Interrupt Mask"]
pub type HIB_IM_VDDFAIL_W<'a> = crate::BitWriter<'a, u32, IM_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rtcalt0(&self) -> HIB_IM_RTCALT0_R {
        HIB_IM_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_lowbat(&self) -> HIB_IM_LOWBAT_R {
        HIB_IM_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_extw(&self) -> HIB_IM_EXTW_R {
        HIB_IM_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_wc(&self) -> HIB_IM_WC_R {
        HIB_IM_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_padiowk(&self) -> HIB_IM_PADIOWK_R {
        HIB_IM_PADIOWK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rstwk(&self) -> HIB_IM_RSTWK_R {
        HIB_IM_RSTWK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_vddfail(&self) -> HIB_IM_VDDFAIL_R {
        HIB_IM_VDDFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rtcalt0(&mut self) -> HIB_IM_RTCALT0_W {
        HIB_IM_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_lowbat(&mut self) -> HIB_IM_LOWBAT_W {
        HIB_IM_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_extw(&mut self) -> HIB_IM_EXTW_W {
        HIB_IM_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_wc(&mut self) -> HIB_IM_WC_W {
        HIB_IM_WC_W::new(self)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_padiowk(&mut self) -> HIB_IM_PADIOWK_W {
        HIB_IM_PADIOWK_W::new(self)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rstwk(&mut self) -> HIB_IM_RSTWK_W {
        HIB_IM_RSTWK_W::new(self)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_vddfail(&mut self) -> HIB_IM_VDDFAIL_W {
        HIB_IM_VDDFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
