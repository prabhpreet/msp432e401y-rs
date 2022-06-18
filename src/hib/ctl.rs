#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_CTL_RTCEN` reader - RTC Timer Enable"]
pub type HIB_CTL_RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_RTCEN` writer - RTC Timer Enable"]
pub type HIB_CTL_RTCEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 0>;
#[doc = "Field `HIB_CTL_HIBREQ` reader - Hibernation Request"]
pub type HIB_CTL_HIBREQ_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_HIBREQ` writer - Hibernation Request"]
pub type HIB_CTL_HIBREQ_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 1>;
#[doc = "Field `HIB_CTL_RTCWEN` reader - RTC Wake-up Enable"]
pub type HIB_CTL_RTCWEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_RTCWEN` writer - RTC Wake-up Enable"]
pub type HIB_CTL_RTCWEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 3>;
#[doc = "Field `HIB_CTL_PINWEN` reader - External Wake Pin Enable"]
pub type HIB_CTL_PINWEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_PINWEN` writer - External Wake Pin Enable"]
pub type HIB_CTL_PINWEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 4>;
#[doc = "Field `HIB_CTL_CLK32EN` reader - Clocking Enable"]
pub type HIB_CTL_CLK32EN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_CLK32EN` writer - Clocking Enable"]
pub type HIB_CTL_CLK32EN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 6>;
#[doc = "Field `HIB_CTL_VABORT` reader - Power Cut Abort Enable"]
pub type HIB_CTL_VABORT_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_VABORT` writer - Power Cut Abort Enable"]
pub type HIB_CTL_VABORT_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 7>;
#[doc = "Field `HIB_CTL_VDD3ON` reader - VDD Powered"]
pub type HIB_CTL_VDD3ON_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_VDD3ON` writer - VDD Powered"]
pub type HIB_CTL_VDD3ON_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 8>;
#[doc = "Field `HIB_CTL_BATWKEN` reader - Wake on Low Battery"]
pub type HIB_CTL_BATWKEN_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_BATWKEN` writer - Wake on Low Battery"]
pub type HIB_CTL_BATWKEN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 9>;
#[doc = "Field `HIB_CTL_BATCHK` reader - Check Battery Status"]
pub type HIB_CTL_BATCHK_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_BATCHK` writer - Check Battery Status"]
pub type HIB_CTL_BATCHK_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 10>;
#[doc = "Select for Low-Battery Comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HIB_CTL_VBATSEL_A {
    #[doc = "0: 1.9 Volts"]
    HIB_CTL_VBATSEL_1_9V = 0,
    #[doc = "1: 2.1 Volts (default)"]
    HIB_CTL_VBATSEL_2_1V = 1,
    #[doc = "2: 2.3 Volts"]
    HIB_CTL_VBATSEL_2_3V = 2,
    #[doc = "3: 2.5 Volts"]
    HIB_CTL_VBATSEL_2_5V = 3,
}
impl From<HIB_CTL_VBATSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIB_CTL_VBATSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HIB_CTL_VBATSEL` reader - Select for Low-Battery Comparator"]
pub type HIB_CTL_VBATSEL_R = crate::FieldReader<u8, HIB_CTL_VBATSEL_A>;
impl HIB_CTL_VBATSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIB_CTL_VBATSEL_A {
        match self.bits {
            0 => HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_1_9V,
            1 => HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_1V,
            2 => HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_3V,
            3 => HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_5V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_1_9V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_1_9v(&self) -> bool {
        *self == HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_1_9V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_1V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_1v(&self) -> bool {
        *self == HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_1V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_3V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_3v(&self) -> bool {
        *self == HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_3V
    }
    #[doc = "Checks if the value of the field is `HIB_CTL_VBATSEL_2_5V`"]
    #[inline(always)]
    pub fn is_hib_ctl_vbatsel_2_5v(&self) -> bool {
        *self == HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_5V
    }
}
#[doc = "Field `HIB_CTL_VBATSEL` writer - Select for Low-Battery Comparator"]
pub type HIB_CTL_VBATSEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, HIB_CTL_VBATSEL_A, 2, 13>;
impl<'a> HIB_CTL_VBATSEL_W<'a> {
    #[doc = "1.9 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_1_9v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_1_9V)
    }
    #[doc = "2.1 Volts (default)"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_1v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_1V)
    }
    #[doc = "2.3 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_3v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_3V)
    }
    #[doc = "2.5 Volts"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel_2_5v(self) -> &'a mut W {
        self.variant(HIB_CTL_VBATSEL_A::HIB_CTL_VBATSEL_2_5V)
    }
}
#[doc = "Field `HIB_CTL_OSCBYP` reader - Oscillator Bypass"]
pub type HIB_CTL_OSCBYP_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_OSCBYP` writer - Oscillator Bypass"]
pub type HIB_CTL_OSCBYP_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 16>;
#[doc = "Field `HIB_CTL_OSCDRV` reader - Oscillator Drive Capability"]
pub type HIB_CTL_OSCDRV_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_OSCDRV` writer - Oscillator Drive Capability"]
pub type HIB_CTL_OSCDRV_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 17>;
#[doc = "Field `HIB_CTL_OSCSEL` reader - Oscillator Select"]
pub type HIB_CTL_OSCSEL_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_OSCSEL` writer - Oscillator Select"]
pub type HIB_CTL_OSCSEL_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 19>;
#[doc = "Field `HIB_CTL_RETCLR` reader - GPIO Retention/Clear"]
pub type HIB_CTL_RETCLR_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_RETCLR` writer - GPIO Retention/Clear"]
pub type HIB_CTL_RETCLR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 30>;
#[doc = "Field `HIB_CTL_WRC` reader - Write Complete/Capable"]
pub type HIB_CTL_WRC_R = crate::BitReader<bool>;
#[doc = "Field `HIB_CTL_WRC` writer - Write Complete/Capable"]
pub type HIB_CTL_WRC_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&self) -> HIB_CTL_RTCEN_R {
        HIB_CTL_RTCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&self) -> HIB_CTL_HIBREQ_R {
        HIB_CTL_HIBREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&self) -> HIB_CTL_RTCWEN_R {
        HIB_CTL_RTCWEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&self) -> HIB_CTL_PINWEN_R {
        HIB_CTL_PINWEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&self) -> HIB_CTL_CLK32EN_R {
        HIB_CTL_CLK32EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&self) -> HIB_CTL_VABORT_R {
        HIB_CTL_VABORT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn hib_ctl_vdd3on(&self) -> HIB_CTL_VDD3ON_R {
        HIB_CTL_VDD3ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn hib_ctl_batwken(&self) -> HIB_CTL_BATWKEN_R {
        HIB_CTL_BATWKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn hib_ctl_batchk(&self) -> HIB_CTL_BATCHK_R {
        HIB_CTL_BATCHK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel(&self) -> HIB_CTL_VBATSEL_R {
        HIB_CTL_VBATSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn hib_ctl_oscbyp(&self) -> HIB_CTL_OSCBYP_R {
        HIB_CTL_OSCBYP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn hib_ctl_oscdrv(&self) -> HIB_CTL_OSCDRV_R {
        HIB_CTL_OSCDRV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Oscillator Select"]
    #[inline(always)]
    pub fn hib_ctl_oscsel(&self) -> HIB_CTL_OSCSEL_R {
        HIB_CTL_OSCSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO Retention/Clear"]
    #[inline(always)]
    pub fn hib_ctl_retclr(&self) -> HIB_CTL_RETCLR_R {
        HIB_CTL_RETCLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn hib_ctl_wrc(&self) -> HIB_CTL_WRC_R {
        HIB_CTL_WRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcen(&mut self) -> HIB_CTL_RTCEN_W {
        HIB_CTL_RTCEN_W::new(self)
    }
    #[doc = "Bit 1 - Hibernation Request"]
    #[inline(always)]
    pub fn hib_ctl_hibreq(&mut self) -> HIB_CTL_HIBREQ_W {
        HIB_CTL_HIBREQ_W::new(self)
    }
    #[doc = "Bit 3 - RTC Wake-up Enable"]
    #[inline(always)]
    pub fn hib_ctl_rtcwen(&mut self) -> HIB_CTL_RTCWEN_W {
        HIB_CTL_RTCWEN_W::new(self)
    }
    #[doc = "Bit 4 - External Wake Pin Enable"]
    #[inline(always)]
    pub fn hib_ctl_pinwen(&mut self) -> HIB_CTL_PINWEN_W {
        HIB_CTL_PINWEN_W::new(self)
    }
    #[doc = "Bit 6 - Clocking Enable"]
    #[inline(always)]
    pub fn hib_ctl_clk32en(&mut self) -> HIB_CTL_CLK32EN_W {
        HIB_CTL_CLK32EN_W::new(self)
    }
    #[doc = "Bit 7 - Power Cut Abort Enable"]
    #[inline(always)]
    pub fn hib_ctl_vabort(&mut self) -> HIB_CTL_VABORT_W {
        HIB_CTL_VABORT_W::new(self)
    }
    #[doc = "Bit 8 - VDD Powered"]
    #[inline(always)]
    pub fn hib_ctl_vdd3on(&mut self) -> HIB_CTL_VDD3ON_W {
        HIB_CTL_VDD3ON_W::new(self)
    }
    #[doc = "Bit 9 - Wake on Low Battery"]
    #[inline(always)]
    pub fn hib_ctl_batwken(&mut self) -> HIB_CTL_BATWKEN_W {
        HIB_CTL_BATWKEN_W::new(self)
    }
    #[doc = "Bit 10 - Check Battery Status"]
    #[inline(always)]
    pub fn hib_ctl_batchk(&mut self) -> HIB_CTL_BATCHK_W {
        HIB_CTL_BATCHK_W::new(self)
    }
    #[doc = "Bits 13:14 - Select for Low-Battery Comparator"]
    #[inline(always)]
    pub fn hib_ctl_vbatsel(&mut self) -> HIB_CTL_VBATSEL_W {
        HIB_CTL_VBATSEL_W::new(self)
    }
    #[doc = "Bit 16 - Oscillator Bypass"]
    #[inline(always)]
    pub fn hib_ctl_oscbyp(&mut self) -> HIB_CTL_OSCBYP_W {
        HIB_CTL_OSCBYP_W::new(self)
    }
    #[doc = "Bit 17 - Oscillator Drive Capability"]
    #[inline(always)]
    pub fn hib_ctl_oscdrv(&mut self) -> HIB_CTL_OSCDRV_W {
        HIB_CTL_OSCDRV_W::new(self)
    }
    #[doc = "Bit 19 - Oscillator Select"]
    #[inline(always)]
    pub fn hib_ctl_oscsel(&mut self) -> HIB_CTL_OSCSEL_W {
        HIB_CTL_OSCSEL_W::new(self)
    }
    #[doc = "Bit 30 - GPIO Retention/Clear"]
    #[inline(always)]
    pub fn hib_ctl_retclr(&mut self) -> HIB_CTL_RETCLR_W {
        HIB_CTL_RETCLR_W::new(self)
    }
    #[doc = "Bit 31 - Write Complete/Capable"]
    #[inline(always)]
    pub fn hib_ctl_wrc(&mut self) -> HIB_CTL_WRC_W {
        HIB_CTL_WRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
