#[doc = "Register `MIIADDR` reader"]
pub struct R(crate::R<MIIADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIADDR` writer"]
pub struct W(crate::W<MIIADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIADDR_SPEC>;
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
impl From<crate::W<MIIADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_MIIADDR_MIIB` reader - MII Busy"]
pub type EMAC_MIIADDR_MIIB_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MIIADDR_MIIB` writer - MII Busy"]
pub type EMAC_MIIADDR_MIIB_W<'a> = crate::BitWriter<'a, u32, MIIADDR_SPEC, bool, 0>;
#[doc = "Field `EMAC_MIIADDR_MIIW` reader - MII Write"]
pub type EMAC_MIIADDR_MIIW_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_MIIADDR_MIIW` writer - MII Write"]
pub type EMAC_MIIADDR_MIIW_W<'a> = crate::BitWriter<'a, u32, MIIADDR_SPEC, bool, 1>;
#[doc = "Clock Reference Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_MIIADDR_CR_A {
    #[doc = "0: The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    EMAC_MIIADDR_CR_60_100 = 0,
    #[doc = "1: The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    EMAC_MIIADDR_CR_100_150 = 1,
    #[doc = "2: The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    EMAC_MIIADDR_CR_20_35 = 2,
    #[doc = "3: The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    EMAC_MIIADDR_CR_35_60 = 3,
}
impl From<EMAC_MIIADDR_CR_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_MIIADDR_CR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_MIIADDR_CR` reader - Clock Reference Frequency Selection"]
pub type EMAC_MIIADDR_CR_R = crate::FieldReader<u8, EMAC_MIIADDR_CR_A>;
impl EMAC_MIIADDR_CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_MIIADDR_CR_A> {
        match self.bits {
            0 => Some(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_60_100),
            1 => Some(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_100_150),
            2 => Some(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_20_35),
            3 => Some(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_35_60),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_60_100`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_60_100(&self) -> bool {
        *self == EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_60_100
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_100_150`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_100_150(&self) -> bool {
        *self == EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_100_150
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_20_35`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_20_35(&self) -> bool {
        *self == EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_20_35
    }
    #[doc = "Checks if the value of the field is `EMAC_MIIADDR_CR_35_60`"]
    #[inline(always)]
    pub fn is_emac_miiaddr_cr_35_60(&self) -> bool {
        *self == EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_35_60
    }
}
#[doc = "Field `EMAC_MIIADDR_CR` writer - Clock Reference Frequency Selection"]
pub type EMAC_MIIADDR_CR_W<'a> =
    crate::FieldWriter<'a, u32, MIIADDR_SPEC, u8, EMAC_MIIADDR_CR_A, 4, 2>;
impl<'a> EMAC_MIIADDR_CR_W<'a> {
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_60_100(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_60_100)
    }
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_100_150(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_100_150)
    }
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_20_35(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_20_35)
    }
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    #[inline(always)]
    pub fn emac_miiaddr_cr_35_60(self) -> &'a mut W {
        self.variant(EMAC_MIIADDR_CR_A::EMAC_MIIADDR_CR_35_60)
    }
}
#[doc = "Field `EMAC_MIIADDR_MII` reader - MII Register"]
pub type EMAC_MIIADDR_MII_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_MIIADDR_MII` writer - MII Register"]
pub type EMAC_MIIADDR_MII_W<'a> = crate::FieldWriter<'a, u32, MIIADDR_SPEC, u8, u8, 5, 6>;
#[doc = "Field `EMAC_MIIADDR_PLA` reader - Physical Layer Address"]
pub type EMAC_MIIADDR_PLA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_MIIADDR_PLA` writer - Physical Layer Address"]
pub type EMAC_MIIADDR_PLA_W<'a> = crate::FieldWriter<'a, u32, MIIADDR_SPEC, u8, u8, 5, 11>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn emac_miiaddr_miib(&self) -> EMAC_MIIADDR_MIIB_R {
        EMAC_MIIADDR_MIIB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn emac_miiaddr_miiw(&self) -> EMAC_MIIADDR_MIIW_R {
        EMAC_MIIADDR_MIIW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn emac_miiaddr_cr(&self) -> EMAC_MIIADDR_CR_R {
        EMAC_MIIADDR_CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn emac_miiaddr_mii(&self) -> EMAC_MIIADDR_MII_R {
        EMAC_MIIADDR_MII_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn emac_miiaddr_pla(&self) -> EMAC_MIIADDR_PLA_R {
        EMAC_MIIADDR_PLA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn emac_miiaddr_miib(&mut self) -> EMAC_MIIADDR_MIIB_W {
        EMAC_MIIADDR_MIIB_W::new(self)
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline(always)]
    pub fn emac_miiaddr_miiw(&mut self) -> EMAC_MIIADDR_MIIW_W {
        EMAC_MIIADDR_MIIW_W::new(self)
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline(always)]
    pub fn emac_miiaddr_cr(&mut self) -> EMAC_MIIADDR_CR_W {
        EMAC_MIIADDR_CR_W::new(self)
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline(always)]
    pub fn emac_miiaddr_mii(&mut self) -> EMAC_MIIADDR_MII_W {
        EMAC_MIIADDR_MII_W::new(self)
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn emac_miiaddr_pla(&mut self) -> EMAC_MIIADDR_PLA_W {
        EMAC_MIIADDR_PLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miiaddr](index.html) module"]
pub struct MIIADDR_SPEC;
impl crate::RegisterSpec for MIIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miiaddr::R](R) reader structure"]
impl crate::Readable for MIIADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miiaddr::W](W) writer structure"]
impl crate::Writable for MIIADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIIADDR to value 0"]
impl crate::Resettable for MIIADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
