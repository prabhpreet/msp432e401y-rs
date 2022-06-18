#[doc = "Register `SSCTL0` reader"]
pub struct R(crate::R<SSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCTL0` writer"]
pub struct W(crate::W<SSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCTL0_SPEC>;
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
impl From<crate::W<SSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSCTL0_D0` reader - 1st Sample Differential Input Select"]
pub type ADC_SSCTL0_D0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D0` writer - 1st Sample Differential Input Select"]
pub type ADC_SSCTL0_D0_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 0>;
#[doc = "Field `ADC_SSCTL0_END0` reader - 1st Sample is End of Sequence"]
pub type ADC_SSCTL0_END0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END0` writer - 1st Sample is End of Sequence"]
pub type ADC_SSCTL0_END0_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 1>;
#[doc = "Field `ADC_SSCTL0_IE0` reader - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE0` writer - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE0_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 2>;
#[doc = "Field `ADC_SSCTL0_TS0` reader - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS0` writer - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS0_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 3>;
#[doc = "Field `ADC_SSCTL0_D1` reader - 2nd Sample Differential Input Select"]
pub type ADC_SSCTL0_D1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D1` writer - 2nd Sample Differential Input Select"]
pub type ADC_SSCTL0_D1_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 4>;
#[doc = "Field `ADC_SSCTL0_END1` reader - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL0_END1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END1` writer - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL0_END1_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 5>;
#[doc = "Field `ADC_SSCTL0_IE1` reader - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE1` writer - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE1_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 6>;
#[doc = "Field `ADC_SSCTL0_TS1` reader - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS1` writer - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS1_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 7>;
#[doc = "Field `ADC_SSCTL0_D2` reader - 3rd Sample Differential Input Select"]
pub type ADC_SSCTL0_D2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D2` writer - 3rd Sample Differential Input Select"]
pub type ADC_SSCTL0_D2_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 8>;
#[doc = "Field `ADC_SSCTL0_END2` reader - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL0_END2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END2` writer - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL0_END2_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 9>;
#[doc = "Field `ADC_SSCTL0_IE2` reader - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE2` writer - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE2_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 10>;
#[doc = "Field `ADC_SSCTL0_TS2` reader - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS2` writer - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS2_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 11>;
#[doc = "Field `ADC_SSCTL0_D3` reader - 4th Sample Differential Input Select"]
pub type ADC_SSCTL0_D3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D3` writer - 4th Sample Differential Input Select"]
pub type ADC_SSCTL0_D3_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 12>;
#[doc = "Field `ADC_SSCTL0_END3` reader - 4th Sample is End of Sequence"]
pub type ADC_SSCTL0_END3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END3` writer - 4th Sample is End of Sequence"]
pub type ADC_SSCTL0_END3_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 13>;
#[doc = "Field `ADC_SSCTL0_IE3` reader - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE3` writer - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE3_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 14>;
#[doc = "Field `ADC_SSCTL0_TS3` reader - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS3` writer - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS3_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 15>;
#[doc = "Field `ADC_SSCTL0_D4` reader - 5th Sample Differential Input Select"]
pub type ADC_SSCTL0_D4_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D4` writer - 5th Sample Differential Input Select"]
pub type ADC_SSCTL0_D4_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 16>;
#[doc = "Field `ADC_SSCTL0_END4` reader - 5th Sample is End of Sequence"]
pub type ADC_SSCTL0_END4_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END4` writer - 5th Sample is End of Sequence"]
pub type ADC_SSCTL0_END4_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 17>;
#[doc = "Field `ADC_SSCTL0_IE4` reader - 5th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE4_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE4` writer - 5th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE4_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 18>;
#[doc = "Field `ADC_SSCTL0_TS4` reader - 5th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS4_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS4` writer - 5th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS4_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 19>;
#[doc = "Field `ADC_SSCTL0_D5` reader - 6th Sample Differential Input Select"]
pub type ADC_SSCTL0_D5_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D5` writer - 6th Sample Differential Input Select"]
pub type ADC_SSCTL0_D5_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 20>;
#[doc = "Field `ADC_SSCTL0_END5` reader - 6th Sample is End of Sequence"]
pub type ADC_SSCTL0_END5_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END5` writer - 6th Sample is End of Sequence"]
pub type ADC_SSCTL0_END5_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 21>;
#[doc = "Field `ADC_SSCTL0_IE5` reader - 6th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE5_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE5` writer - 6th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE5_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 22>;
#[doc = "Field `ADC_SSCTL0_TS5` reader - 6th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS5_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS5` writer - 6th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS5_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 23>;
#[doc = "Field `ADC_SSCTL0_D6` reader - 7th Sample Differential Input Select"]
pub type ADC_SSCTL0_D6_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D6` writer - 7th Sample Differential Input Select"]
pub type ADC_SSCTL0_D6_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 24>;
#[doc = "Field `ADC_SSCTL0_END6` reader - 7th Sample is End of Sequence"]
pub type ADC_SSCTL0_END6_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END6` writer - 7th Sample is End of Sequence"]
pub type ADC_SSCTL0_END6_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 25>;
#[doc = "Field `ADC_SSCTL0_IE6` reader - 7th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE6_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE6` writer - 7th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE6_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 26>;
#[doc = "Field `ADC_SSCTL0_TS6` reader - 7th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS6_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS6` writer - 7th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS6_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 27>;
#[doc = "Field `ADC_SSCTL0_D7` reader - 8th Sample Differential Input Select"]
pub type ADC_SSCTL0_D7_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_D7` writer - 8th Sample Differential Input Select"]
pub type ADC_SSCTL0_D7_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 28>;
#[doc = "Field `ADC_SSCTL0_END7` reader - 8th Sample is End of Sequence"]
pub type ADC_SSCTL0_END7_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_END7` writer - 8th Sample is End of Sequence"]
pub type ADC_SSCTL0_END7_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 29>;
#[doc = "Field `ADC_SSCTL0_IE7` reader - 8th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE7_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_IE7` writer - 8th Sample Interrupt Enable"]
pub type ADC_SSCTL0_IE7_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 30>;
#[doc = "Field `ADC_SSCTL0_TS7` reader - 8th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS7_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL0_TS7` writer - 8th Sample Temp Sensor Select"]
pub type ADC_SSCTL0_TS7_W<'a> = crate::BitWriter<'a, u32, SSCTL0_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&self) -> ADC_SSCTL0_D0_R {
        ADC_SSCTL0_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&self) -> ADC_SSCTL0_END0_R {
        ADC_SSCTL0_END0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&self) -> ADC_SSCTL0_IE0_R {
        ADC_SSCTL0_IE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&self) -> ADC_SSCTL0_TS0_R {
        ADC_SSCTL0_TS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&self) -> ADC_SSCTL0_D1_R {
        ADC_SSCTL0_D1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&self) -> ADC_SSCTL0_END1_R {
        ADC_SSCTL0_END1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&self) -> ADC_SSCTL0_IE1_R {
        ADC_SSCTL0_IE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&self) -> ADC_SSCTL0_TS1_R {
        ADC_SSCTL0_TS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&self) -> ADC_SSCTL0_D2_R {
        ADC_SSCTL0_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&self) -> ADC_SSCTL0_END2_R {
        ADC_SSCTL0_END2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&self) -> ADC_SSCTL0_IE2_R {
        ADC_SSCTL0_IE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&self) -> ADC_SSCTL0_TS2_R {
        ADC_SSCTL0_TS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&self) -> ADC_SSCTL0_D3_R {
        ADC_SSCTL0_D3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&self) -> ADC_SSCTL0_END3_R {
        ADC_SSCTL0_END3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&self) -> ADC_SSCTL0_IE3_R {
        ADC_SSCTL0_IE3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&self) -> ADC_SSCTL0_TS3_R {
        ADC_SSCTL0_TS3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&self) -> ADC_SSCTL0_D4_R {
        ADC_SSCTL0_D4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&self) -> ADC_SSCTL0_END4_R {
        ADC_SSCTL0_END4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&self) -> ADC_SSCTL0_IE4_R {
        ADC_SSCTL0_IE4_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&self) -> ADC_SSCTL0_TS4_R {
        ADC_SSCTL0_TS4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&self) -> ADC_SSCTL0_D5_R {
        ADC_SSCTL0_D5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&self) -> ADC_SSCTL0_END5_R {
        ADC_SSCTL0_END5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&self) -> ADC_SSCTL0_IE5_R {
        ADC_SSCTL0_IE5_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&self) -> ADC_SSCTL0_TS5_R {
        ADC_SSCTL0_TS5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&self) -> ADC_SSCTL0_D6_R {
        ADC_SSCTL0_D6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&self) -> ADC_SSCTL0_END6_R {
        ADC_SSCTL0_END6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&self) -> ADC_SSCTL0_IE6_R {
        ADC_SSCTL0_IE6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&self) -> ADC_SSCTL0_TS6_R {
        ADC_SSCTL0_TS6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&self) -> ADC_SSCTL0_D7_R {
        ADC_SSCTL0_D7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&self) -> ADC_SSCTL0_END7_R {
        ADC_SSCTL0_END7_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&self) -> ADC_SSCTL0_IE7_R {
        ADC_SSCTL0_IE7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&self) -> ADC_SSCTL0_TS7_R {
        ADC_SSCTL0_TS7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d0(&mut self) -> ADC_SSCTL0_D0_W {
        ADC_SSCTL0_D0_W::new(self)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end0(&mut self) -> ADC_SSCTL0_END0_W {
        ADC_SSCTL0_END0_W::new(self)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie0(&mut self) -> ADC_SSCTL0_IE0_W {
        ADC_SSCTL0_IE0_W::new(self)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts0(&mut self) -> ADC_SSCTL0_TS0_W {
        ADC_SSCTL0_TS0_W::new(self)
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d1(&mut self) -> ADC_SSCTL0_D1_W {
        ADC_SSCTL0_D1_W::new(self)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end1(&mut self) -> ADC_SSCTL0_END1_W {
        ADC_SSCTL0_END1_W::new(self)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie1(&mut self) -> ADC_SSCTL0_IE1_W {
        ADC_SSCTL0_IE1_W::new(self)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts1(&mut self) -> ADC_SSCTL0_TS1_W {
        ADC_SSCTL0_TS1_W::new(self)
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d2(&mut self) -> ADC_SSCTL0_D2_W {
        ADC_SSCTL0_D2_W::new(self)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end2(&mut self) -> ADC_SSCTL0_END2_W {
        ADC_SSCTL0_END2_W::new(self)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie2(&mut self) -> ADC_SSCTL0_IE2_W {
        ADC_SSCTL0_IE2_W::new(self)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts2(&mut self) -> ADC_SSCTL0_TS2_W {
        ADC_SSCTL0_TS2_W::new(self)
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d3(&mut self) -> ADC_SSCTL0_D3_W {
        ADC_SSCTL0_D3_W::new(self)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end3(&mut self) -> ADC_SSCTL0_END3_W {
        ADC_SSCTL0_END3_W::new(self)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie3(&mut self) -> ADC_SSCTL0_IE3_W {
        ADC_SSCTL0_IE3_W::new(self)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts3(&mut self) -> ADC_SSCTL0_TS3_W {
        ADC_SSCTL0_TS3_W::new(self)
    }
    #[doc = "Bit 16 - 5th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d4(&mut self) -> ADC_SSCTL0_D4_W {
        ADC_SSCTL0_D4_W::new(self)
    }
    #[doc = "Bit 17 - 5th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end4(&mut self) -> ADC_SSCTL0_END4_W {
        ADC_SSCTL0_END4_W::new(self)
    }
    #[doc = "Bit 18 - 5th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie4(&mut self) -> ADC_SSCTL0_IE4_W {
        ADC_SSCTL0_IE4_W::new(self)
    }
    #[doc = "Bit 19 - 5th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts4(&mut self) -> ADC_SSCTL0_TS4_W {
        ADC_SSCTL0_TS4_W::new(self)
    }
    #[doc = "Bit 20 - 6th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d5(&mut self) -> ADC_SSCTL0_D5_W {
        ADC_SSCTL0_D5_W::new(self)
    }
    #[doc = "Bit 21 - 6th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end5(&mut self) -> ADC_SSCTL0_END5_W {
        ADC_SSCTL0_END5_W::new(self)
    }
    #[doc = "Bit 22 - 6th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie5(&mut self) -> ADC_SSCTL0_IE5_W {
        ADC_SSCTL0_IE5_W::new(self)
    }
    #[doc = "Bit 23 - 6th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts5(&mut self) -> ADC_SSCTL0_TS5_W {
        ADC_SSCTL0_TS5_W::new(self)
    }
    #[doc = "Bit 24 - 7th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d6(&mut self) -> ADC_SSCTL0_D6_W {
        ADC_SSCTL0_D6_W::new(self)
    }
    #[doc = "Bit 25 - 7th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end6(&mut self) -> ADC_SSCTL0_END6_W {
        ADC_SSCTL0_END6_W::new(self)
    }
    #[doc = "Bit 26 - 7th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie6(&mut self) -> ADC_SSCTL0_IE6_W {
        ADC_SSCTL0_IE6_W::new(self)
    }
    #[doc = "Bit 27 - 7th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts6(&mut self) -> ADC_SSCTL0_TS6_W {
        ADC_SSCTL0_TS6_W::new(self)
    }
    #[doc = "Bit 28 - 8th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl0_d7(&mut self) -> ADC_SSCTL0_D7_W {
        ADC_SSCTL0_D7_W::new(self)
    }
    #[doc = "Bit 29 - 8th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl0_end7(&mut self) -> ADC_SSCTL0_END7_W {
        ADC_SSCTL0_END7_W::new(self)
    }
    #[doc = "Bit 30 - 8th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl0_ie7(&mut self) -> ADC_SSCTL0_IE7_W {
        ADC_SSCTL0_IE7_W::new(self)
    }
    #[doc = "Bit 31 - 8th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl0_ts7(&mut self) -> ADC_SSCTL0_TS7_W {
        ADC_SSCTL0_TS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl0](index.html) module"]
pub struct SSCTL0_SPEC;
impl crate::RegisterSpec for SSCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssctl0::R](R) reader structure"]
impl crate::Readable for SSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssctl0::W](W) writer structure"]
impl crate::Writable for SSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCTL0 to value 0"]
impl crate::Resettable for SSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
