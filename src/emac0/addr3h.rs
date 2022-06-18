#[doc = "Register `ADDR3H` reader"]
pub struct R(crate::R<ADDR3H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR3H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR3H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR3H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR3H` writer"]
pub struct W(crate::W<ADDR3H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR3H_SPEC>;
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
impl From<crate::W<ADDR3H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR3H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_ADDR3H_ADDRHI` reader - MAC Address3 \\[47:32\\]"]
pub type EMAC_ADDR3H_ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_ADDR3H_ADDRHI` writer - MAC Address3 \\[47:32\\]"]
pub type EMAC_ADDR3H_ADDRHI_W<'a> = crate::FieldWriter<'a, u32, ADDR3H_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EMAC_ADDR3H_MBC` reader - Mask Byte Control"]
pub type EMAC_ADDR3H_MBC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_ADDR3H_MBC` writer - Mask Byte Control"]
pub type EMAC_ADDR3H_MBC_W<'a> = crate::FieldWriter<'a, u32, ADDR3H_SPEC, u8, u8, 6, 24>;
#[doc = "Field `EMAC_ADDR3H_SA` reader - Source Address"]
pub type EMAC_ADDR3H_SA_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_ADDR3H_SA` writer - Source Address"]
pub type EMAC_ADDR3H_SA_W<'a> = crate::BitWriter<'a, u32, ADDR3H_SPEC, bool, 30>;
#[doc = "Field `EMAC_ADDR3H_AE` reader - Address Enable"]
pub type EMAC_ADDR3H_AE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_ADDR3H_AE` writer - Address Enable"]
pub type EMAC_ADDR3H_AE_W<'a> = crate::BitWriter<'a, u32, ADDR3H_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - MAC Address3 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr3h_addrhi(&self) -> EMAC_ADDR3H_ADDRHI_R {
        EMAC_ADDR3H_ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn emac_addr3h_mbc(&self) -> EMAC_ADDR3H_MBC_R {
        EMAC_ADDR3H_MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn emac_addr3h_sa(&self) -> EMAC_ADDR3H_SA_R {
        EMAC_ADDR3H_SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr3h_ae(&self) -> EMAC_ADDR3H_AE_R {
        EMAC_ADDR3H_AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address3 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr3h_addrhi(&mut self) -> EMAC_ADDR3H_ADDRHI_W {
        EMAC_ADDR3H_ADDRHI_W::new(self)
    }
    #[doc = "Bits 24:29 - Mask Byte Control"]
    #[inline(always)]
    pub fn emac_addr3h_mbc(&mut self) -> EMAC_ADDR3H_MBC_W {
        EMAC_ADDR3H_MBC_W::new(self)
    }
    #[doc = "Bit 30 - Source Address"]
    #[inline(always)]
    pub fn emac_addr3h_sa(&mut self) -> EMAC_ADDR3H_SA_W {
        EMAC_ADDR3H_SA_W::new(self)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr3h_ae(&mut self) -> EMAC_ADDR3H_AE_W {
        EMAC_ADDR3H_AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Address 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr3h](index.html) module"]
pub struct ADDR3H_SPEC;
impl crate::RegisterSpec for ADDR3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr3h::R](R) reader structure"]
impl crate::Readable for ADDR3H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr3h::W](W) writer structure"]
impl crate::Writable for ADDR3H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR3H to value 0"]
impl crate::Resettable for ADDR3H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
