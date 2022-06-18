#[doc = "Register `HB16CFG4` reader"]
pub struct R(crate::R<HB16CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HB16CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HB16CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HB16CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HB16CFG4` writer"]
pub struct W(crate::W<HB16CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HB16CFG4_SPEC>;
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
impl From<crate::W<HB16CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HB16CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CS3n Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB16CFG4_MODE_A {
    #[doc = "0: ADMUX - AD\\[15:0\\]"]
    EPI_HB16CFG4_MODE_ADMUX = 0,
    #[doc = "1: ADNONMUX - D\\[15:0\\]"]
    EPI_HB16CFG4_MODE_AD = 1,
}
impl From<EPI_HB16CFG4_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB16CFG4_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB16CFG4_MODE` reader - CS3n Host Bus Sub-Mode"]
pub type EPI_HB16CFG4_MODE_R = crate::FieldReader<u8, EPI_HB16CFG4_MODE_A>;
impl EPI_HB16CFG4_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_HB16CFG4_MODE_A> {
        match self.bits {
            0 => Some(EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_ADMUX),
            1 => Some(EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_AD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_MODE_ADMUX`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_mode_admux(&self) -> bool {
        *self == EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_ADMUX
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_MODE_AD`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_mode_ad(&self) -> bool {
        *self == EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_AD
    }
}
#[doc = "Field `EPI_HB16CFG4_MODE` writer - CS3n Host Bus Sub-Mode"]
pub type EPI_HB16CFG4_MODE_W<'a> =
    crate::FieldWriter<'a, u32, HB16CFG4_SPEC, u8, EPI_HB16CFG4_MODE_A, 2, 0>;
impl<'a> EPI_HB16CFG4_MODE_W<'a> {
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg4_mode_admux(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn epi_hb16cfg4_mode_ad(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_MODE_A::EPI_HB16CFG4_MODE_AD)
    }
}
#[doc = "CS3n Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB16CFG4_RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    EPI_HB16CFG4_RDWS_2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    EPI_HB16CFG4_RDWS_4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    EPI_HB16CFG4_RDWS_6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    EPI_HB16CFG4_RDWS_8 = 3,
}
impl From<EPI_HB16CFG4_RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB16CFG4_RDWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB16CFG4_RDWS` reader - CS3n Read Wait States"]
pub type EPI_HB16CFG4_RDWS_R = crate::FieldReader<u8, EPI_HB16CFG4_RDWS_A>;
impl EPI_HB16CFG4_RDWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB16CFG4_RDWS_A {
        match self.bits {
            0 => EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_2,
            1 => EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_4,
            2 => EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_6,
            3 => EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_RDWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_rdws_2(&self) -> bool {
        *self == EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_RDWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_rdws_4(&self) -> bool {
        *self == EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_RDWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_rdws_6(&self) -> bool {
        *self == EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_RDWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_rdws_8(&self) -> bool {
        *self == EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_8
    }
}
#[doc = "Field `EPI_HB16CFG4_RDWS` writer - CS3n Read Wait States"]
pub type EPI_HB16CFG4_RDWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, HB16CFG4_SPEC, u8, EPI_HB16CFG4_RDWS_A, 2, 4>;
impl<'a> EPI_HB16CFG4_RDWS_W<'a> {
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_RDWS_A::EPI_HB16CFG4_RDWS_8)
    }
}
#[doc = "CS3n Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_HB16CFG4_WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    EPI_HB16CFG4_WRWS_2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    EPI_HB16CFG4_WRWS_4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    EPI_HB16CFG4_WRWS_6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    EPI_HB16CFG4_WRWS_8 = 3,
}
impl From<EPI_HB16CFG4_WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_HB16CFG4_WRWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_HB16CFG4_WRWS` reader - CS3n Write Wait States"]
pub type EPI_HB16CFG4_WRWS_R = crate::FieldReader<u8, EPI_HB16CFG4_WRWS_A>;
impl EPI_HB16CFG4_WRWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPI_HB16CFG4_WRWS_A {
        match self.bits {
            0 => EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_2,
            1 => EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_4,
            2 => EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_6,
            3 => EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_WRWS_2`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_wrws_2(&self) -> bool {
        *self == EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_2
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_WRWS_4`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_wrws_4(&self) -> bool {
        *self == EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_4
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_WRWS_6`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_wrws_6(&self) -> bool {
        *self == EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_6
    }
    #[doc = "Checks if the value of the field is `EPI_HB16CFG4_WRWS_8`"]
    #[inline(always)]
    pub fn is_epi_hb16cfg4_wrws_8(&self) -> bool {
        *self == EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_8
    }
}
#[doc = "Field `EPI_HB16CFG4_WRWS` writer - CS3n Write Wait States"]
pub type EPI_HB16CFG4_WRWS_W<'a> =
    crate::FieldWriterSafe<'a, u32, HB16CFG4_SPEC, u8, EPI_HB16CFG4_WRWS_A, 2, 6>;
impl<'a> EPI_HB16CFG4_WRWS_W<'a> {
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws_2(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws_4(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws_6(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws_8(self) -> &'a mut W {
        self.variant(EPI_HB16CFG4_WRWS_A::EPI_HB16CFG4_WRWS_8)
    }
}
#[doc = "Field `EPI_HB16CFG4_BURST` reader - CS3n Burst Mode"]
pub type EPI_HB16CFG4_BURST_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_BURST` writer - CS3n Burst Mode"]
pub type EPI_HB16CFG4_BURST_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 16>;
#[doc = "Field `EPI_HB16CFG4_RDCRE` reader - CS3n PSRAM Configuration Register Read"]
pub type EPI_HB16CFG4_RDCRE_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_RDCRE` writer - CS3n PSRAM Configuration Register Read"]
pub type EPI_HB16CFG4_RDCRE_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 17>;
#[doc = "Field `EPI_HB16CFG4_WRCRE` reader - CS3n PSRAM Configuration Register Write"]
pub type EPI_HB16CFG4_WRCRE_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_WRCRE` writer - CS3n PSRAM Configuration Register Write"]
pub type EPI_HB16CFG4_WRCRE_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 18>;
#[doc = "Field `EPI_HB16CFG4_ALEHIGH` reader - CS3n ALE Strobe Polarity"]
pub type EPI_HB16CFG4_ALEHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_ALEHIGH` writer - CS3n ALE Strobe Polarity"]
pub type EPI_HB16CFG4_ALEHIGH_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 19>;
#[doc = "Field `EPI_HB16CFG4_RDHIGH` reader - CS3n READ Strobe Polarity"]
pub type EPI_HB16CFG4_RDHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_RDHIGH` writer - CS3n READ Strobe Polarity"]
pub type EPI_HB16CFG4_RDHIGH_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 20>;
#[doc = "Field `EPI_HB16CFG4_WRHIGH` reader - CS3n WRITE Strobe Polarity"]
pub type EPI_HB16CFG4_WRHIGH_R = crate::BitReader<bool>;
#[doc = "Field `EPI_HB16CFG4_WRHIGH` writer - CS3n WRITE Strobe Polarity"]
pub type EPI_HB16CFG4_WRHIGH_W<'a> = crate::BitWriter<'a, u32, HB16CFG4_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:1 - CS3n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg4_mode(&self) -> EPI_HB16CFG4_MODE_R {
        EPI_HB16CFG4_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CS3n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws(&self) -> EPI_HB16CFG4_RDWS_R {
        EPI_HB16CFG4_RDWS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CS3n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws(&self) -> EPI_HB16CFG4_WRWS_R {
        EPI_HB16CFG4_WRWS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - CS3n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg4_burst(&self) -> EPI_HB16CFG4_BURST_R {
        EPI_HB16CFG4_BURST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CS3n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdcre(&self) -> EPI_HB16CFG4_RDCRE_R {
        EPI_HB16CFG4_RDCRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CS3n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrcre(&self) -> EPI_HB16CFG4_WRCRE_R {
        EPI_HB16CFG4_WRCRE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CS3n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_alehigh(&self) -> EPI_HB16CFG4_ALEHIGH_R {
        EPI_HB16CFG4_ALEHIGH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CS3n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdhigh(&self) -> EPI_HB16CFG4_RDHIGH_R {
        EPI_HB16CFG4_RDHIGH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CS3n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrhigh(&self) -> EPI_HB16CFG4_WRHIGH_R {
        EPI_HB16CFG4_WRHIGH_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS3n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg4_mode(&mut self) -> EPI_HB16CFG4_MODE_W {
        EPI_HB16CFG4_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - CS3n Read Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdws(&mut self) -> EPI_HB16CFG4_RDWS_W {
        EPI_HB16CFG4_RDWS_W::new(self)
    }
    #[doc = "Bits 6:7 - CS3n Write Wait States"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrws(&mut self) -> EPI_HB16CFG4_WRWS_W {
        EPI_HB16CFG4_WRWS_W::new(self)
    }
    #[doc = "Bit 16 - CS3n Burst Mode"]
    #[inline(always)]
    pub fn epi_hb16cfg4_burst(&mut self) -> EPI_HB16CFG4_BURST_W {
        EPI_HB16CFG4_BURST_W::new(self)
    }
    #[doc = "Bit 17 - CS3n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdcre(&mut self) -> EPI_HB16CFG4_RDCRE_W {
        EPI_HB16CFG4_RDCRE_W::new(self)
    }
    #[doc = "Bit 18 - CS3n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrcre(&mut self) -> EPI_HB16CFG4_WRCRE_W {
        EPI_HB16CFG4_WRCRE_W::new(self)
    }
    #[doc = "Bit 19 - CS3n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_alehigh(&mut self) -> EPI_HB16CFG4_ALEHIGH_W {
        EPI_HB16CFG4_ALEHIGH_W::new(self)
    }
    #[doc = "Bit 20 - CS3n READ Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_rdhigh(&mut self) -> EPI_HB16CFG4_RDHIGH_W {
        EPI_HB16CFG4_RDHIGH_W::new(self)
    }
    #[doc = "Bit 21 - CS3n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn epi_hb16cfg4_wrhigh(&mut self) -> EPI_HB16CFG4_WRHIGH_W {
        EPI_HB16CFG4_WRHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Host-Bus 16 Configuration 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hb16cfg4](index.html) module"]
pub struct HB16CFG4_SPEC;
impl crate::RegisterSpec for HB16CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hb16cfg4::R](R) reader structure"]
impl crate::Readable for HB16CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hb16cfg4::W](W) writer structure"]
impl crate::Writable for HB16CFG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HB16CFG4 to value 0"]
impl crate::Resettable for HB16CFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
