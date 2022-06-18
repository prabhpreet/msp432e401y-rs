#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPI_CFG_MODE_A {
    #[doc = "0: General Purpose"]
    EPI_CFG_MODE_NONE = 0,
    #[doc = "1: SDRAM"]
    EPI_CFG_MODE_SDRAM = 1,
    #[doc = "2: 8-Bit Host-Bus (HB8)"]
    EPI_CFG_MODE_HB8 = 2,
    #[doc = "3: 16-Bit Host-Bus (HB16)"]
    EPI_CFG_MODE_HB16 = 3,
}
impl From<EPI_CFG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPI_CFG_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPI_CFG_MODE` reader - Mode Select"]
pub type EPI_CFG_MODE_R = crate::FieldReader<u8, EPI_CFG_MODE_A>;
impl EPI_CFG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPI_CFG_MODE_A> {
        match self.bits {
            0 => Some(EPI_CFG_MODE_A::EPI_CFG_MODE_NONE),
            1 => Some(EPI_CFG_MODE_A::EPI_CFG_MODE_SDRAM),
            2 => Some(EPI_CFG_MODE_A::EPI_CFG_MODE_HB8),
            3 => Some(EPI_CFG_MODE_A::EPI_CFG_MODE_HB16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_NONE`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_none(&self) -> bool {
        *self == EPI_CFG_MODE_A::EPI_CFG_MODE_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_SDRAM`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_sdram(&self) -> bool {
        *self == EPI_CFG_MODE_A::EPI_CFG_MODE_SDRAM
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_HB8`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_hb8(&self) -> bool {
        *self == EPI_CFG_MODE_A::EPI_CFG_MODE_HB8
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_HB16`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_hb16(&self) -> bool {
        *self == EPI_CFG_MODE_A::EPI_CFG_MODE_HB16
    }
}
#[doc = "Field `EPI_CFG_MODE` writer - Mode Select"]
pub type EPI_CFG_MODE_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, EPI_CFG_MODE_A, 4, 0>;
impl<'a> EPI_CFG_MODE_W<'a> {
    #[doc = "General Purpose"]
    #[inline(always)]
    pub fn epi_cfg_mode_none(self) -> &'a mut W {
        self.variant(EPI_CFG_MODE_A::EPI_CFG_MODE_NONE)
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn epi_cfg_mode_sdram(self) -> &'a mut W {
        self.variant(EPI_CFG_MODE_A::EPI_CFG_MODE_SDRAM)
    }
    #[doc = "8-Bit Host-Bus (HB8)"]
    #[inline(always)]
    pub fn epi_cfg_mode_hb8(self) -> &'a mut W {
        self.variant(EPI_CFG_MODE_A::EPI_CFG_MODE_HB8)
    }
    #[doc = "16-Bit Host-Bus (HB16)"]
    #[inline(always)]
    pub fn epi_cfg_mode_hb16(self) -> &'a mut W {
        self.variant(EPI_CFG_MODE_A::EPI_CFG_MODE_HB16)
    }
}
#[doc = "Field `EPI_CFG_BLKEN` reader - Block Enable"]
pub type EPI_CFG_BLKEN_R = crate::BitReader<bool>;
#[doc = "Field `EPI_CFG_BLKEN` writer - Block Enable"]
pub type EPI_CFG_BLKEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 4>;
#[doc = "Field `EPI_CFG_INTDIV` reader - Integer Clock Divider Enable"]
pub type EPI_CFG_INTDIV_R = crate::BitReader<bool>;
#[doc = "Field `EPI_CFG_INTDIV` writer - Integer Clock Divider Enable"]
pub type EPI_CFG_INTDIV_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn epi_cfg_mode(&self) -> EPI_CFG_MODE_R {
        EPI_CFG_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn epi_cfg_blken(&self) -> EPI_CFG_BLKEN_R {
        EPI_CFG_BLKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn epi_cfg_intdiv(&self) -> EPI_CFG_INTDIV_R {
        EPI_CFG_INTDIV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn epi_cfg_mode(&mut self) -> EPI_CFG_MODE_W {
        EPI_CFG_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn epi_cfg_blken(&mut self) -> EPI_CFG_BLKEN_W {
        EPI_CFG_BLKEN_W::new(self)
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn epi_cfg_intdiv(&mut self) -> EPI_CFG_INTDIV_W {
        EPI_CFG_INTDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
