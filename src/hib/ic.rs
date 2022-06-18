#[doc = "Register `IC` reader"]
pub struct R(crate::R<IC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC` writer"]
pub struct W(crate::W<IC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SPEC>;
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
impl From<crate::W<IC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_IC_RTCALT0` reader - RTC Alert0 Masked Interrupt Clear"]
pub type HIB_IC_RTCALT0_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_RTCALT0` writer - RTC Alert0 Masked Interrupt Clear"]
pub type HIB_IC_RTCALT0_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 0>;
#[doc = "Field `HIB_IC_LOWBAT` reader - Low Battery Voltage Interrupt Clear"]
pub type HIB_IC_LOWBAT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_LOWBAT` writer - Low Battery Voltage Interrupt Clear"]
pub type HIB_IC_LOWBAT_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 2>;
#[doc = "Field `HIB_IC_EXTW` reader - External Wake-Up Interrupt Clear"]
pub type HIB_IC_EXTW_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_EXTW` writer - External Wake-Up Interrupt Clear"]
pub type HIB_IC_EXTW_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 3>;
#[doc = "Field `HIB_IC_WC` reader - Write Complete/Capable Interrupt Clear"]
pub type HIB_IC_WC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_WC` writer - Write Complete/Capable Interrupt Clear"]
pub type HIB_IC_WC_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 4>;
#[doc = "Field `HIB_IC_PADIOWK` reader - Pad I/O Wake-Up Interrupt Clear"]
pub type HIB_IC_PADIOWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_PADIOWK` writer - Pad I/O Wake-Up Interrupt Clear"]
pub type HIB_IC_PADIOWK_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 5>;
#[doc = "Field `HIB_IC_RSTWK` reader - Reset Pad I/O Wake-Up Interrupt Clear"]
pub type HIB_IC_RSTWK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_RSTWK` writer - Reset Pad I/O Wake-Up Interrupt Clear"]
pub type HIB_IC_RSTWK_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 6>;
#[doc = "Field `HIB_IC_VDDFAIL` reader - VDD Fail Interrupt Clear"]
pub type HIB_IC_VDDFAIL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_IC_VDDFAIL` writer - VDD Fail Interrupt Clear"]
pub type HIB_IC_VDDFAIL_W<'a> = crate::BitWriter<'a, u32, IC_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_rtcalt0(&self) -> HIB_IC_RTCALT0_R {
        HIB_IC_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_lowbat(&self) -> HIB_IC_LOWBAT_R {
        HIB_IC_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_extw(&self) -> HIB_IC_EXTW_R {
        HIB_IC_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_wc(&self) -> HIB_IC_WC_R {
        HIB_IC_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_padiowk(&self) -> HIB_IC_PADIOWK_R {
        HIB_IC_PADIOWK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_rstwk(&self) -> HIB_IC_RSTWK_R {
        HIB_IC_RSTWK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_vddfail(&self) -> HIB_IC_VDDFAIL_R {
        HIB_IC_VDDFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_rtcalt0(&mut self) -> HIB_IC_RTCALT0_W {
        HIB_IC_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_lowbat(&mut self) -> HIB_IC_LOWBAT_W {
        HIB_IC_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_extw(&mut self) -> HIB_IC_EXTW_W {
        HIB_IC_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_wc(&mut self) -> HIB_IC_WC_W {
        HIB_IC_WC_W::new(self)
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_padiowk(&mut self) -> HIB_IC_PADIOWK_W {
        HIB_IC_PADIOWK_W::new(self)
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_rstwk(&mut self) -> HIB_IC_RSTWK_W {
        HIB_IC_RSTWK_W::new(self)
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_vddfail(&mut self) -> HIB_IC_VDDFAIL_W {
        HIB_IC_VDDFAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](index.html) module"]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic::R](R) reader structure"]
impl crate::Readable for IC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic::W](W) writer structure"]
impl crate::Writable for IC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
