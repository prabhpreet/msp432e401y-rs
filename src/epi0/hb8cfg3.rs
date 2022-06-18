#[doc = "Register `HB8CFG3` reader"]
pub struct R(crate::R<HB8CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HB8CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HB8CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HB8CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB8CFG3` writer"]
pub struct W(crate::W<HB8CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HB8CFG3_SPEC>;
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
impl From<crate::W<HB8CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HB8CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CS2n Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG3_MODE_A {
    #[doc = "0: ADMUX - AD\\[7:0\\]"]
    EPI_HB8CFG3_MODE_ADMUX = 0,
    #[doc = "1: ADNONMUX - D\\[7:0\\]"]
    EPI_HB8CFG3_MODE_AD = 1,
}
impl From<EPI_HB8CFG3_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG3_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG3_MODE` reader - CS2n Host Bus Sub-Mode"]
pub type EPI_HB8CFG3_MODE_R = crate::FieldReader<u8, EPI_HB8CFG3_MODE_A>;
impl EPI_HB8CFG3_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_HB8CFG3_MODE_A> {
        match self.bits {
            0 => Some(EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_ADMUX),
            1 => Some(EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_AD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_mode_admux(&self) -> bool {
        *self == EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_MODE_AD`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_mode_ad(&self) -> bool {
        *self == EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_AD
    }
}
#[doc = "Field `EPI_HB8CFG3_MODE` writer - CS2n Host Bus Sub-Mode"]
pub type EPI_HB8CFG3_MODE_W<'a> =
    crate::FieldWriter<'a, u32, HB8CFG3_SPEC, u8, EPI_HB8CFG3_MODE_A, 2, 0>;
impl<'a> EPI_HB8CFG3_MODE_W<'a> {
    #[doc = "ADMUX - AD\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg3_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[7:0\\]"]
    #[inline(always)]
    pub fn epi_hb8cfg3_mode_ad(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_MODE_A::EPI_HB8CFG3_MODE_AD)
    }
}
#[doc = "CS2n Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG3_RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    EPI_HB8CFG3_RDWS_2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    EPI_HB8CFG3_RDWS_4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    EPI_HB8CFG3_RDWS_6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    EPI_HB8CFG3_RDWS_8 = 3,
}
impl From<EPI_HB8CFG3_RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG3_RDWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG3_RDWS` reader - CS2n Read Wait States"]
pub type EPI_HB8CFG3_RDWS_R = crate::FieldReader<u8, EPI_HB8CFG3_RDWS_A>;
impl EPI_HB8CFG3_RDWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG3_RDWS_A {
        match self.bits {
            0 => EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_2,
            1 => EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_4,
            2 => EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_6,
            3 => EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_rdws_2(&self) -> bool {
        *self == EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_rdws_4(&self) -> bool {
        *self == EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_rdws_6(&self) -> bool {
        *self == EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_rdws_8(&self) -> bool {
        *self == EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_8
    }
}
#[doc = "Field `EPI_HB8CFG3_RDWS` writer - CS2n Read Wait States"]
pub type EPI_HB8CFG3_RDWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, HB8CFG3_SPEC, u8, EPI_HB8CFG3_RDWS_A, 2, 4>;
impl<'a> EPI_HB8CFG3_RDWS_W<'a> {
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_RDWS_A::EPI_HB8CFG3_RDWS_8)
    }
}
#[doc = "CS2n Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB8CFG3_WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    EPI_HB8CFG3_WRWS_2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    EPI_HB8CFG3_WRWS_4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    EPI_HB8CFG3_WRWS_6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    EPI_HB8CFG3_WRWS_8 = 3,
}
impl From<EPI_HB8CFG3_WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB8CFG3_WRWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB8CFG3_WRWS` reader - CS2n Write Wait States"]
pub type EPI_HB8CFG3_WRWS_R = crate::FieldReader<u8, EPI_HB8CFG3_WRWS_A>;
impl EPI_HB8CFG3_WRWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB8CFG3_WRWS_A {
        match self.bits {
            0 => EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_2,
            1 => EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_4,
            2 => EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_6,
            3 => EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_wrws_2(&self) -> bool {
        *self == EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_wrws_4(&self) -> bool {
        *self == EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_wrws_6(&self) -> bool {
        *self == EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB8CFG3_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb8cfg3_wrws_8(&self) -> bool {
        *self == EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_8
    }
}
#[doc = "Field `EPI_HB8CFG3_WRWS` writer - CS2n Write Wait States"]
pub type EPI_HB8CFG3_WRWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, HB8CFG3_SPEC, u8, EPI_HB8CFG3_WRWS_A, 2, 6>;
impl<'a> EPI_HB8CFG3_WRWS_W<'a> {
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB8CFG3_WRWS_A::EPI_HB8CFG3_WRWS_8)
    }
}
#[doc = "Field `EPI_HB8CFG3_ALEHIGH` reader - CS2n ALE Strobe Polarity"]
pub type EPI_HB8CFG3_ALEHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG3_ALEHIGH` writer - CS2n ALE Strobe Polarity"]
pub type EPI_HB8CFG3_ALEHIGH_W<'a> = crate::BitWriter<'a, u32, HB8CFG3_SPEC, bool, 19>;
#[doc = "Field `EPI_HB8CFG3_RDHIGH` reader - CS2n READ Strobe Polarity"]
pub type EPI_HB8CFG3_RDHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG3_RDHIGH` writer - CS2n READ Strobe Polarity"]
pub type EPI_HB8CFG3_RDHIGH_W<'a> = crate::BitWriter<'a, u32, HB8CFG3_SPEC, bool, 20>;
#[doc = "Field `EPI_HB8CFG3_WRHIGH` reader - CS2n WRITE Strobe Polarity"]
pub type EPI_HB8CFG3_WRHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB8CFG3_WRHIGH` writer - CS2n WRITE Strobe Polarity"]
pub type EPI_HB8CFG3_WRHIGH_W<'a> = crate::BitWriter<'a, u32, HB8CFG3_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg3_mode(&self) -> EPI_HB8CFG3_MODE_R {
        EPI_HB8CFG3_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws(&self) -> EPI_HB8CFG3_RDWS_R {
        EPI_HB8CFG3_RDWS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws(&self) -> EPI_HB8CFG3_WRWS_R {
        EPI_HB8CFG3_WRWS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_alehigh(&self) -> EPI_HB8CFG3_ALEHIGH_R {
        EPI_HB8CFG3_ALEHIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdhigh(&self) -> EPI_HB8CFG3_RDHIGH_R {
        EPI_HB8CFG3_RDHIGH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrhigh(&self) -> EPI_HB8CFG3_WRHIGH_R {
        EPI_HB8CFG3_WRHIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb8cfg3_mode(&mut self) -> EPI_HB8CFG3_MODE_W {
        EPI_HB8CFG3_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdws(&mut self) -> EPI_HB8CFG3_RDWS_W {
        EPI_HB8CFG3_RDWS_W::new(self)
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrws(&mut self) -> EPI_HB8CFG3_WRWS_W {
        EPI_HB8CFG3_WRWS_W::new(self)
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_alehigh(&mut self) -> EPI_HB8CFG3_ALEHIGH_W {
        EPI_HB8CFG3_ALEHIGH_W::new(self)
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_rdhigh(&mut self) -> EPI_HB8CFG3_RDHIGH_W {
        EPI_HB8CFG3_RDHIGH_W::new(self)
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb8cfg3_wrhigh(&mut self) -> EPI_HB8CFG3_WRHIGH_W {
        EPI_HB8CFG3_WRHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 8 Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb8cfg3](index.html) module"]
pub struct HB8CFG3_SPEC;
impl crate::RegisterSpec for HB8CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hb8cfg3::R](R) reader structure"]
impl crate::Readable for HB8CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hb8cfg3::W](W) writer structure"]
impl crate::Writable for HB8CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB8CFG3 to value 0"]
impl crate::Resettable for HB8CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
