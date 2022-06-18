#[doc = "Register `ADDR0H` reader"]
pub struct R(crate::R<ADDR0H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR0H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR0H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR0H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR0H` writer"]
pub struct W(crate::W<ADDR0H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR0H_SPEC>;
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
impl From<crate::W<ADDR0H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR0H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_ADDR0H_ADDRHI` reader - MAC Address0 \\[47:32\\]"]
pub type EMAC_ADDR0H_ADDRHI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_ADDR0H_ADDRHI` writer - MAC Address0 \\[47:32\\]"]
pub type EMAC_ADDR0H_ADDRHI_W<'a> = crate::FieldWriter<'a, u32, ADDR0H_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EMAC_ADDR0H_AE` reader - Address Enable"]
pub type EMAC_ADDR0H_AE_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_ADDR0H_AE` writer - Address Enable"]
pub type EMAC_ADDR0H_AE_W<'a> = crate::BitWriter<'a, u32, ADDR0H_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr0h_addrhi(&self) -> EMAC_ADDR0H_ADDRHI_R {
        EMAC_ADDR0H_ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr0h_ae(&self) -> EMAC_ADDR0H_AE_R {
        EMAC_ADDR0H_AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\]"]
    #[inline(always)]
    pub fn emac_addr0h_addrhi(&mut self) -> EMAC_ADDR0H_ADDRHI_W {
        EMAC_ADDR0H_ADDRHI_W::new(self)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn emac_addr0h_ae(&mut self) -> EMAC_ADDR0H_AE_W {
        EMAC_ADDR0H_AE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Address 0 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0h](index.html) module"]
pub struct ADDR0H_SPEC;
impl crate::RegisterSpec for ADDR0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr0h::R](R) reader structure"]
impl crate::Readable for ADDR0H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr0h::W](W) writer structure"]
impl crate::Writable for ADDR0H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR0H to value 0"]
impl crate::Resettable for ADDR0H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
