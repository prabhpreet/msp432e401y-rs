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
#[doc = "Field `SSI_PP_HSCLK` reader - High Speed Capability"]
pub type SSI_PP_HSCLK_R = crate::BitReader<bool>;
#[doc = "Field `SSI_PP_HSCLK` writer - High Speed Capability"]
pub type SSI_PP_HSCLK_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 0>;
#[doc = "Mode of Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSI_PP_MODE_A {
    #[doc = "0: Legacy SSI mode"]
    SSI_PP_MODE_LEGACY = 0,
    #[doc = "1: Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    SSI_PP_MODE_ADVBI = 1,
    #[doc = "2: Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    SSI_PP_MODE_ADVBIQUAD = 2,
}
impl From<SSI_PP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSI_PP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSI_PP_MODE` reader - Mode of Operation"]
pub type SSI_PP_MODE_R = crate::FieldReader<u8, SSI_PP_MODE_A>;
impl SSI_PP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSI_PP_MODE_A> {
        match self.bits {
            0 => Some(SSI_PP_MODE_A::SSI_PP_MODE_LEGACY),
            1 => Some(SSI_PP_MODE_A::SSI_PP_MODE_ADVBI),
            2 => Some(SSI_PP_MODE_A::SSI_PP_MODE_ADVBIQUAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_LEGACY`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_legacy(&self) -> bool {
        *self == SSI_PP_MODE_A::SSI_PP_MODE_LEGACY
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_ADVBI`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_advbi(&self) -> bool {
        *self == SSI_PP_MODE_A::SSI_PP_MODE_ADVBI
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_ADVBIQUAD`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_advbiquad(&self) -> bool {
        *self == SSI_PP_MODE_A::SSI_PP_MODE_ADVBIQUAD
    }
}
#[doc = "Field `SSI_PP_MODE` writer - Mode of Operation"]
pub type SSI_PP_MODE_W<'a> = crate::FieldWriter<'a, u32, PP_SPEC, u8, SSI_PP_MODE_A, 2, 1>;
impl<'a> SSI_PP_MODE_W<'a> {
    #[doc = "Legacy SSI mode"]
    #[inline(always)]
    pub fn ssi_pp_mode_legacy(self) -> &'a mut W {
        self.variant(SSI_PP_MODE_A::SSI_PP_MODE_LEGACY)
    }
    #[doc = "Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    #[inline(always)]
    pub fn ssi_pp_mode_advbi(self) -> &'a mut W {
        self.variant(SSI_PP_MODE_A::SSI_PP_MODE_ADVBI)
    }
    #[doc = "Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    #[inline(always)]
    pub fn ssi_pp_mode_advbiquad(self) -> &'a mut W {
        self.variant(SSI_PP_MODE_A::SSI_PP_MODE_ADVBIQUAD)
    }
}
#[doc = "Field `SSI_PP_FSSHLDFRM` reader - FSS Hold Frame Capability"]
pub type SSI_PP_FSSHLDFRM_R = crate::BitReader<bool>;
#[doc = "Field `SSI_PP_FSSHLDFRM` writer - FSS Hold Frame Capability"]
pub type SSI_PP_FSSHLDFRM_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline(always)]
    pub fn ssi_pp_hsclk(&self) -> SSI_PP_HSCLK_R {
        SSI_PP_HSCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline(always)]
    pub fn ssi_pp_mode(&self) -> SSI_PP_MODE_R {
        SSI_PP_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline(always)]
    pub fn ssi_pp_fsshldfrm(&self) -> SSI_PP_FSSHLDFRM_R {
        SSI_PP_FSSHLDFRM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline(always)]
    pub fn ssi_pp_hsclk(&mut self) -> SSI_PP_HSCLK_W {
        SSI_PP_HSCLK_W::new(self)
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline(always)]
    pub fn ssi_pp_mode(&mut self) -> SSI_PP_MODE_W {
        SSI_PP_MODE_W::new(self)
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline(always)]
    pub fn ssi_pp_fsshldfrm(&mut self) -> SSI_PP_FSSHLDFRM_W {
        SSI_PP_FSSHLDFRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
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
