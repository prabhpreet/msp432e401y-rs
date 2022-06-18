#[doc = "Register `LPICTLSTAT` reader"]
pub struct R(crate::R<LPICTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPICTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPICTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPICTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPICTLSTAT` writer"]
pub struct W(crate::W<LPICTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPICTLSTAT_SPEC>;
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
impl From<crate::W<LPICTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPICTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_LPICTLSTAT_TLPIEN` reader - Transmit LPI Entry"]
pub type EMAC_LPICTLSTAT_TLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_TLPIEN` writer - Transmit LPI Entry"]
pub type EMAC_LPICTLSTAT_TLPIEN_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 0>;
#[doc = "Field `EMAC_LPICTLSTAT_TLPIEX` reader - Transmit LPI Exit"]
pub type EMAC_LPICTLSTAT_TLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_TLPIEX` writer - Transmit LPI Exit"]
pub type EMAC_LPICTLSTAT_TLPIEX_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 1>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIEN` reader - Receive LPI Entry"]
pub type EMAC_LPICTLSTAT_RLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIEN` writer - Receive LPI Entry"]
pub type EMAC_LPICTLSTAT_RLPIEN_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 2>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIEX` reader - Receive LPI Exit"]
pub type EMAC_LPICTLSTAT_RLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIEX` writer - Receive LPI Exit"]
pub type EMAC_LPICTLSTAT_RLPIEX_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 3>;
#[doc = "Field `EMAC_LPICTLSTAT_TLPIST` reader - Transmit LPI State"]
pub type EMAC_LPICTLSTAT_TLPIST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_TLPIST` writer - Transmit LPI State"]
pub type EMAC_LPICTLSTAT_TLPIST_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 8>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIST` reader - Receive LPI State"]
pub type EMAC_LPICTLSTAT_RLPIST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_RLPIST` writer - Receive LPI State"]
pub type EMAC_LPICTLSTAT_RLPIST_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 9>;
#[doc = "Field `EMAC_LPICTLSTAT_LPIEN` reader - LPI Enable"]
pub type EMAC_LPICTLSTAT_LPIEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_LPIEN` writer - LPI Enable"]
pub type EMAC_LPICTLSTAT_LPIEN_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 16>;
#[doc = "Field `EMAC_LPICTLSTAT_PLS` reader - PHY Link Status"]
pub type EMAC_LPICTLSTAT_PLS_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_PLS` writer - PHY Link Status"]
pub type EMAC_LPICTLSTAT_PLS_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 17>;
#[doc = "Field `EMAC_LPICTLSTAT_PLSEN` reader - PHY Link Status Enable"]
pub type EMAC_LPICTLSTAT_PLSEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_PLSEN` writer - PHY Link Status Enable"]
pub type EMAC_LPICTLSTAT_PLSEN_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 18>;
#[doc = "Field `EMAC_LPICTLSTAT_LPITXA` reader - LPI TX Automate"]
pub type EMAC_LPICTLSTAT_LPITXA_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_LPICTLSTAT_LPITXA` writer - LPI TX Automate"]
pub type EMAC_LPICTLSTAT_LPITXA_W<'a> = crate::BitWriter<'a, u32, LPICTLSTAT_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpien(&self) -> EMAC_LPICTLSTAT_TLPIEN_R {
        EMAC_LPICTLSTAT_TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpiex(&self) -> EMAC_LPICTLSTAT_TLPIEX_R {
        EMAC_LPICTLSTAT_TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpien(&self) -> EMAC_LPICTLSTAT_RLPIEN_R {
        EMAC_LPICTLSTAT_RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpiex(&self) -> EMAC_LPICTLSTAT_RLPIEX_R {
        EMAC_LPICTLSTAT_RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpist(&self) -> EMAC_LPICTLSTAT_TLPIST_R {
        EMAC_LPICTLSTAT_TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpist(&self) -> EMAC_LPICTLSTAT_RLPIST_R {
        EMAC_LPICTLSTAT_RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpien(&self) -> EMAC_LPICTLSTAT_LPIEN_R {
        EMAC_LPICTLSTAT_LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn emac_lpictlstat_pls(&self) -> EMAC_LPICTLSTAT_PLS_R {
        EMAC_LPICTLSTAT_PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_plsen(&self) -> EMAC_LPICTLSTAT_PLSEN_R {
        EMAC_LPICTLSTAT_PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpitxa(&self) -> EMAC_LPICTLSTAT_LPITXA_R {
        EMAC_LPICTLSTAT_LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpien(&mut self) -> EMAC_LPICTLSTAT_TLPIEN_W {
        EMAC_LPICTLSTAT_TLPIEN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpiex(&mut self) -> EMAC_LPICTLSTAT_TLPIEX_W {
        EMAC_LPICTLSTAT_TLPIEX_W::new(self)
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpien(&mut self) -> EMAC_LPICTLSTAT_RLPIEN_W {
        EMAC_LPICTLSTAT_RLPIEN_W::new(self)
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpiex(&mut self) -> EMAC_LPICTLSTAT_RLPIEX_W {
        EMAC_LPICTLSTAT_RLPIEX_W::new(self)
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_tlpist(&mut self) -> EMAC_LPICTLSTAT_TLPIST_W {
        EMAC_LPICTLSTAT_TLPIST_W::new(self)
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn emac_lpictlstat_rlpist(&mut self) -> EMAC_LPICTLSTAT_RLPIST_W {
        EMAC_LPICTLSTAT_RLPIST_W::new(self)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpien(&mut self) -> EMAC_LPICTLSTAT_LPIEN_W {
        EMAC_LPICTLSTAT_LPIEN_W::new(self)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn emac_lpictlstat_pls(&mut self) -> EMAC_LPICTLSTAT_PLS_W {
        EMAC_LPICTLSTAT_PLS_W::new(self)
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn emac_lpictlstat_plsen(&mut self) -> EMAC_LPICTLSTAT_PLSEN_W {
        EMAC_LPICTLSTAT_PLSEN_W::new(self)
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    pub fn emac_lpictlstat_lpitxa(&mut self) -> EMAC_LPICTLSTAT_LPITXA_W {
        EMAC_LPICTLSTAT_LPITXA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Low Power Idle Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpictlstat](index.html) module"]
pub struct LPICTLSTAT_SPEC;
impl crate::RegisterSpec for LPICTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpictlstat::R](R) reader structure"]
impl crate::Readable for LPICTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpictlstat::W](W) writer structure"]
impl crate::Writable for LPICTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPICTLSTAT to value 0"]
impl crate::Resettable for LPICTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
