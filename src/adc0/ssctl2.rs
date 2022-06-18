#[doc = "Register `SSCTL2` reader"]
pub struct R(crate::R<SSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCTL2` writer"]
pub struct W(crate::W<SSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCTL2_SPEC>;
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
impl From<crate::W<SSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SSCTL2_D0` reader - 1st Sample Differential Input Select"]
pub type ADC_SSCTL2_D0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_D0` writer - 1st Sample Differential Input Select"]
pub type ADC_SSCTL2_D0_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 0>;
#[doc = "Field `ADC_SSCTL2_END0` reader - 1st Sample is End of Sequence"]
pub type ADC_SSCTL2_END0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_END0` writer - 1st Sample is End of Sequence"]
pub type ADC_SSCTL2_END0_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 1>;
#[doc = "Field `ADC_SSCTL2_IE0` reader - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_IE0` writer - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE0_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 2>;
#[doc = "Field `ADC_SSCTL2_TS0` reader - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS0_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_TS0` writer - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS0_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 3>;
#[doc = "Field `ADC_SSCTL2_D1` reader - 2nd Sample Differential Input Select"]
pub type ADC_SSCTL2_D1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_D1` writer - 2nd Sample Differential Input Select"]
pub type ADC_SSCTL2_D1_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 4>;
#[doc = "Field `ADC_SSCTL2_END1` reader - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL2_END1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_END1` writer - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL2_END1_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 5>;
#[doc = "Field `ADC_SSCTL2_IE1` reader - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_IE1` writer - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE1_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 6>;
#[doc = "Field `ADC_SSCTL2_TS1` reader - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS1_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_TS1` writer - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS1_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 7>;
#[doc = "Field `ADC_SSCTL2_D2` reader - 3rd Sample Differential Input Select"]
pub type ADC_SSCTL2_D2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_D2` writer - 3rd Sample Differential Input Select"]
pub type ADC_SSCTL2_D2_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 8>;
#[doc = "Field `ADC_SSCTL2_END2` reader - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL2_END2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_END2` writer - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL2_END2_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 9>;
#[doc = "Field `ADC_SSCTL2_IE2` reader - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_IE2` writer - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE2_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 10>;
#[doc = "Field `ADC_SSCTL2_TS2` reader - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS2_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_TS2` writer - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS2_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 11>;
#[doc = "Field `ADC_SSCTL2_D3` reader - 4th Sample Differential Input Select"]
pub type ADC_SSCTL2_D3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_D3` writer - 4th Sample Differential Input Select"]
pub type ADC_SSCTL2_D3_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 12>;
#[doc = "Field `ADC_SSCTL2_END3` reader - 4th Sample is End of Sequence"]
pub type ADC_SSCTL2_END3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_END3` writer - 4th Sample is End of Sequence"]
pub type ADC_SSCTL2_END3_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 13>;
#[doc = "Field `ADC_SSCTL2_IE3` reader - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_IE3` writer - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE3_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 14>;
#[doc = "Field `ADC_SSCTL2_TS3` reader - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS3_R = crate::BitReader<bool>;
#[doc = "Field `ADC_SSCTL2_TS3` writer - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS3_W<'a> = crate::BitWriter<'a, u32, SSCTL2_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d0(&self) -> ADC_SSCTL2_D0_R {
        ADC_SSCTL2_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end0(&self) -> ADC_SSCTL2_END0_R {
        ADC_SSCTL2_END0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie0(&self) -> ADC_SSCTL2_IE0_R {
        ADC_SSCTL2_IE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts0(&self) -> ADC_SSCTL2_TS0_R {
        ADC_SSCTL2_TS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d1(&self) -> ADC_SSCTL2_D1_R {
        ADC_SSCTL2_D1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end1(&self) -> ADC_SSCTL2_END1_R {
        ADC_SSCTL2_END1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie1(&self) -> ADC_SSCTL2_IE1_R {
        ADC_SSCTL2_IE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts1(&self) -> ADC_SSCTL2_TS1_R {
        ADC_SSCTL2_TS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d2(&self) -> ADC_SSCTL2_D2_R {
        ADC_SSCTL2_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end2(&self) -> ADC_SSCTL2_END2_R {
        ADC_SSCTL2_END2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie2(&self) -> ADC_SSCTL2_IE2_R {
        ADC_SSCTL2_IE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts2(&self) -> ADC_SSCTL2_TS2_R {
        ADC_SSCTL2_TS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d3(&self) -> ADC_SSCTL2_D3_R {
        ADC_SSCTL2_D3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end3(&self) -> ADC_SSCTL2_END3_R {
        ADC_SSCTL2_END3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie3(&self) -> ADC_SSCTL2_IE3_R {
        ADC_SSCTL2_IE3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts3(&self) -> ADC_SSCTL2_TS3_R {
        ADC_SSCTL2_TS3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d0(&mut self) -> ADC_SSCTL2_D0_W {
        ADC_SSCTL2_D0_W::new(self)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end0(&mut self) -> ADC_SSCTL2_END0_W {
        ADC_SSCTL2_END0_W::new(self)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie0(&mut self) -> ADC_SSCTL2_IE0_W {
        ADC_SSCTL2_IE0_W::new(self)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts0(&mut self) -> ADC_SSCTL2_TS0_W {
        ADC_SSCTL2_TS0_W::new(self)
    }
    #[doc = "Bit 4 - 2nd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d1(&mut self) -> ADC_SSCTL2_D1_W {
        ADC_SSCTL2_D1_W::new(self)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end1(&mut self) -> ADC_SSCTL2_END1_W {
        ADC_SSCTL2_END1_W::new(self)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie1(&mut self) -> ADC_SSCTL2_IE1_W {
        ADC_SSCTL2_IE1_W::new(self)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts1(&mut self) -> ADC_SSCTL2_TS1_W {
        ADC_SSCTL2_TS1_W::new(self)
    }
    #[doc = "Bit 8 - 3rd Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d2(&mut self) -> ADC_SSCTL2_D2_W {
        ADC_SSCTL2_D2_W::new(self)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end2(&mut self) -> ADC_SSCTL2_END2_W {
        ADC_SSCTL2_END2_W::new(self)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie2(&mut self) -> ADC_SSCTL2_IE2_W {
        ADC_SSCTL2_IE2_W::new(self)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts2(&mut self) -> ADC_SSCTL2_TS2_W {
        ADC_SSCTL2_TS2_W::new(self)
    }
    #[doc = "Bit 12 - 4th Sample Differential Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d3(&mut self) -> ADC_SSCTL2_D3_W {
        ADC_SSCTL2_D3_W::new(self)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end3(&mut self) -> ADC_SSCTL2_END3_W {
        ADC_SSCTL2_END3_W::new(self)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie3(&mut self) -> ADC_SSCTL2_IE3_W {
        ADC_SSCTL2_IE3_W::new(self)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts3(&mut self) -> ADC_SSCTL2_TS3_W {
        ADC_SSCTL2_TS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Sample Sequence Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl2](index.html) module"]
pub struct SSCTL2_SPEC;
impl crate::RegisterSpec for SSCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssctl2::R](R) reader structure"]
impl crate::Readable for SSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssctl2::W](W) writer structure"]
impl crate::Writable for SSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCTL2 to value 0"]
impl crate::Resettable for SSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
