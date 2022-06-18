#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Ethernet PHY Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_PP_PHYTYPE_A {
    #[doc = "0: No PHY"]
    EMAC_PP_PHYTYPE_NONE = 0,
    #[doc = "3: Snowflake class PHY"]
    EMAC_PP_PHYTYPE_1 = 3,
}
impl From<EMAC_PP_PHYTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_PP_PHYTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_PP_PHYTYPE` reader - Ethernet PHY Type"]
pub type EMAC_PP_PHYTYPE_R = crate::FieldReader<u8, EMAC_PP_PHYTYPE_A>;
impl EMAC_PP_PHYTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EMAC_PP_PHYTYPE_A> {
        match self.bits {
            0 => Some(EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_NONE),
            3 => Some(EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PP_PHYTYPE_NONE`"]
    #[inline(always)]
    pub fn is_emac_pp_phytype_none(&self) -> bool {
        *self == EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_PP_PHYTYPE_1`"]
    #[inline(always)]
    pub fn is_emac_pp_phytype_1(&self) -> bool {
        *self == EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_1
    }
}
#[doc = "Field `EMAC_PP_PHYTYPE` writer - Ethernet PHY Type"]
pub type EMAC_PP_PHYTYPE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, EMAC_PP_PHYTYPE_A, 3, 0>;
impl<'a> EMAC_PP_PHYTYPE_W<'a> {
    #[doc = "No PHY"]
    #[inline(always)]
    pub fn emac_pp_phytype_none(self) -> &'a mut W {
        self.variant(EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_NONE)
    }
    #[doc = "Snowflake class PHY"]
    #[inline(always)]
    pub fn emac_pp_phytype_1(self) -> &'a mut W {
        self.variant(EMAC_PP_PHYTYPE_A::EMAC_PP_PHYTYPE_1)
    }
}
#[doc = "Field `EMAC_PP_MACTYPE` reader - Ethernet MAC Type"]
pub type EMAC_PP_MACTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_PP_MACTYPE` writer - Ethernet MAC Type"]
pub type EMAC_PP_MACTYPE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, u8, 3, 8>;
impl R {
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline(always)]
    pub fn emac_pp_phytype(&self) -> EMAC_PP_PHYTYPE_R {
        EMAC_PP_PHYTYPE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline(always)]
    pub fn emac_pp_mactype(&self) -> EMAC_PP_MACTYPE_R {
        EMAC_PP_MACTYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline(always)]
    pub fn emac_pp_phytype(&mut self) -> EMAC_PP_PHYTYPE_W {
        EMAC_PP_PHYTYPE_W::new(self)
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline(always)]
    pub fn emac_pp_mactype(&mut self) -> EMAC_PP_MACTYPE_W {
        EMAC_PP_MACTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Peripheral Property Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
