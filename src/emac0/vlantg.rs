#[doc = "Register `VLANTG` reader"]
pub struct R(crate::R<VLANTG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLANTG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLANTG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLANTG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLANTG` writer"]
pub struct W(crate::W<VLANTG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLANTG_SPEC>;
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
impl From<crate::W<VLANTG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLANTG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_VLANTG_VL` reader - VLAN Tag Identifier for Receive Frames"]
pub type EMAC_VLANTG_VL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_VLANTG_VL` writer - VLAN Tag Identifier for Receive Frames"]
pub type EMAC_VLANTG_VL_W<'a> = crate::FieldWriter<'a, u32, VLANTG_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EMAC_VLANTG_ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub type EMAC_VLANTG_ETV_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLANTG_ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub type EMAC_VLANTG_ETV_W<'a> = crate::BitWriter<'a, u32, VLANTG_SPEC, bool, 16>;
#[doc = "Field `EMAC_VLANTG_VTIM` reader - VLAN Tag Inverse Match Enable"]
pub type EMAC_VLANTG_VTIM_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLANTG_VTIM` writer - VLAN Tag Inverse Match Enable"]
pub type EMAC_VLANTG_VTIM_W<'a> = crate::BitWriter<'a, u32, VLANTG_SPEC, bool, 17>;
#[doc = "Field `EMAC_VLANTG_ESVL` reader - Enable S-VLAN"]
pub type EMAC_VLANTG_ESVL_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLANTG_ESVL` writer - Enable S-VLAN"]
pub type EMAC_VLANTG_ESVL_W<'a> = crate::BitWriter<'a, u32, VLANTG_SPEC, bool, 18>;
#[doc = "Field `EMAC_VLANTG_VTHM` reader - VLAN Tag Hash Table Match Enable"]
pub type EMAC_VLANTG_VTHM_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_VLANTG_VTHM` writer - VLAN Tag Hash Table Match Enable"]
pub type EMAC_VLANTG_VTHM_W<'a> = crate::BitWriter<'a, u32, VLANTG_SPEC, bool, 19>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn emac_vlantg_vl(&self) -> EMAC_VLANTG_VL_R {
        EMAC_VLANTG_VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn emac_vlantg_etv(&self) -> EMAC_VLANTG_ETV_R {
        EMAC_VLANTG_ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn emac_vlantg_vtim(&self) -> EMAC_VLANTG_VTIM_R {
        EMAC_VLANTG_VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn emac_vlantg_esvl(&self) -> EMAC_VLANTG_ESVL_R {
        EMAC_VLANTG_ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn emac_vlantg_vthm(&self) -> EMAC_VLANTG_VTHM_R {
        EMAC_VLANTG_VTHM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames"]
    #[inline(always)]
    pub fn emac_vlantg_vl(&mut self) -> EMAC_VLANTG_VL_W {
        EMAC_VLANTG_VL_W::new(self)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn emac_vlantg_etv(&mut self) -> EMAC_VLANTG_ETV_W {
        EMAC_VLANTG_ETV_W::new(self)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn emac_vlantg_vtim(&mut self) -> EMAC_VLANTG_VTIM_W {
        EMAC_VLANTG_VTIM_W::new(self)
    }
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn emac_vlantg_esvl(&mut self) -> EMAC_VLANTG_ESVL_W {
        EMAC_VLANTG_ESVL_W::new(self)
    }
    #[doc = "Bit 19 - VLAN Tag Hash Table Match Enable"]
    #[inline(always)]
    pub fn emac_vlantg_vthm(&mut self) -> EMAC_VLANTG_VTHM_W {
        EMAC_VLANTG_VTHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN Tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlantg](index.html) module"]
pub struct VLANTG_SPEC;
impl crate::RegisterSpec for VLANTG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlantg::R](R) reader structure"]
impl crate::Readable for VLANTG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlantg::W](W) writer structure"]
impl crate::Writable for VLANTG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLANTG to value 0"]
impl crate::Resettable for VLANTG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
