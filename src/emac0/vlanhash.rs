#[doc = "Register `VLANHASH` reader"]
pub struct R(crate::R<VLANHASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLANHASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLANHASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLANHASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLANHASH` writer"]
pub struct W(crate::W<VLANHASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLANHASH_SPEC>;
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
impl From<crate::W<VLANHASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLANHASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_VLANHASH_VLHT` reader - VLAN Hash Table"]
pub type EMAC_VLANHASH_VLHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_VLANHASH_VLHT` writer - VLAN Hash Table"]
pub type EMAC_VLANHASH_VLHT_W<'a> = crate::FieldWriter<'a, u32, VLANHASH_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn emac_vlanhash_vlht(&self) -> EMAC_VLANHASH_VLHT_R {
        EMAC_VLANHASH_VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn emac_vlanhash_vlht(&mut self) -> EMAC_VLANHASH_VLHT_W {
        EMAC_VLANHASH_VLHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN Hash Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlanhash](index.html) module"]
pub struct VLANHASH_SPEC;
impl crate::RegisterSpec for VLANHASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlanhash::R](R) reader structure"]
impl crate::Readable for VLANHASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlanhash::W](W) writer structure"]
impl crate::Writable for VLANHASH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLANHASH to value 0"]
impl crate::Resettable for VLANHASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
