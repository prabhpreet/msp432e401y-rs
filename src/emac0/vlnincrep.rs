#[doc = "Register `VLNINCREP` reader"]
pub struct R(crate::R<VLNINCREP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLNINCREP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLNINCREP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLNINCREP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLNINCREP` writer"]
pub struct W(crate::W<VLNINCREP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLNINCREP_SPEC>;
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
impl From<crate::W<VLNINCREP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLNINCREP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_VLNINCREP_VLT` reader - VLAN Tag for Transmit Frames"]
pub type EMAC_VLNINCREP_VLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_VLNINCREP_VLT` writer - VLAN Tag for Transmit Frames"]
pub type EMAC_VLNINCREP_VLT_W<'a> = crate::FieldWriter<'a, u32, VLNINCREP_SPEC, u16, u16, 16, 0>;
#[doc = "VLAN Tag Control in Transmit Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMAC_VLNINCREP_VLC_A {
    #[doc = "0: No VLAN tag deletion, insertion, or replacement"]
    EMAC_VLNINCREP_VLC_NONE = 0,
    #[doc = "1: VLAN tag deletion"]
    EMAC_VLNINCREP_VLC_TAGDEL = 1,
    #[doc = "2: VLAN tag insertion"]
    EMAC_VLNINCREP_VLC_TAGINS = 2,
    #[doc = "3: VLAN tag replacement"]
    EMAC_VLNINCREP_VLC_TAGREP = 3,
}
impl From<EMAC_VLNINCREP_VLC_A> for u8 {
    #[inline(always)]
    fn from(variant: EMAC_VLNINCREP_VLC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMAC_VLNINCREP_VLC` reader - VLAN Tag Control in Transmit Frames"]
pub type EMAC_VLNINCREP_VLC_R = crate::FieldReader<u8, EMAC_VLNINCREP_VLC_A>;
impl EMAC_VLNINCREP_VLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMAC_VLNINCREP_VLC_A {
        match self.bits {
            0 => EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_NONE,
            1 => EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGDEL,
            2 => EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGINS,
            3 => EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGREP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_NONE`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_none(&self) -> bool {
        *self == EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGDEL`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagdel(&self) -> bool {
        *self == EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGDEL
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGINS`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagins(&self) -> bool {
        *self == EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGINS
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGREP`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagrep(&self) -> bool {
        *self == EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGREP
    }
}
#[doc = "Field `EMAC_VLNINCREP_VLC` writer - VLAN Tag Control in Transmit Frames"]
pub type EMAC_VLNINCREP_VLC_W<'a> =
    crate::FieldWriterSafe<'a, u32, VLNINCREP_SPEC, u8, EMAC_VLNINCREP_VLC_A, 2, 16>;
impl<'a> EMAC_VLNINCREP_VLC_W<'a> {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_none(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_NONE)
    }
    #[doc = "VLAN tag deletion"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagdel(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGDEL)
    }
    #[doc = "VLAN tag insertion"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagins(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGINS)
    }
    #[doc = "VLAN tag replacement"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagrep(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLC_A::EMAC_VLNINCREP_VLC_TAGREP)
    }
}
#[doc = "Field `EMAC_VLNINCREP_VLP` reader - VLAN Priority Control"]
pub type EMAC_VLNINCREP_VLP_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLNINCREP_VLP` writer - VLAN Priority Control"]
pub type EMAC_VLNINCREP_VLP_W<'a> = crate::BitWriter<'a, u32, VLNINCREP_SPEC, bool, 18>;
#[doc = "Field `EMAC_VLNINCREP_CSVL` reader - C-VLAN or S-VLAN"]
pub type EMAC_VLNINCREP_CSVL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLNINCREP_CSVL` writer - C-VLAN or S-VLAN"]
pub type EMAC_VLNINCREP_CSVL_W<'a> = crate::BitWriter<'a, u32, VLNINCREP_SPEC, bool, 19>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlt(&self) -> EMAC_VLNINCREP_VLT_R {
        EMAC_VLNINCREP_VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc(&self) -> EMAC_VLNINCREP_VLC_R {
        EMAC_VLNINCREP_VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlp(&self) -> EMAC_VLNINCREP_VLP_R {
        EMAC_VLNINCREP_VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn emac_vlnincrep_csvl(&self) -> EMAC_VLNINCREP_CSVL_R {
        EMAC_VLNINCREP_CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlt(&mut self) -> EMAC_VLNINCREP_VLT_W {
        EMAC_VLNINCREP_VLT_W::new(self)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc(&mut self) -> EMAC_VLNINCREP_VLC_W {
        EMAC_VLNINCREP_VLC_W::new(self)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlp(&mut self) -> EMAC_VLNINCREP_VLP_W {
        EMAC_VLNINCREP_VLP_W::new(self)
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn emac_vlnincrep_csvl(&mut self) -> EMAC_VLNINCREP_CSVL_W {
        EMAC_VLNINCREP_CSVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN Tag Inclusion or Replacement\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlnincrep](index.html) module"]
pub struct VLNINCREP_SPEC;
impl crate::RegisterSpec for VLNINCREP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlnincrep::R](R) reader structure"]
impl crate::Readable for VLNINCREP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlnincrep::W](W) writer structure"]
impl crate::Writable for VLNINCREP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLNINCREP to value 0"]
impl crate::Resettable for VLNINCREP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
