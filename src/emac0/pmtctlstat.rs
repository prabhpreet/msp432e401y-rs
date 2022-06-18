#[doc = "Register `PMTCTLSTAT` reader"]
pub struct R(crate::R<PMTCTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMTCTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMTCTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMTCTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMTCTLSTAT` writer"]
pub struct W(crate::W<PMTCTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMTCTLSTAT_SPEC>;
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
impl From<crate::W<PMTCTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMTCTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_PMTCTLSTAT_PWRDWN` reader - Power Down"]
pub type EMAC_PMTCTLSTAT_PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_PWRDWN` writer - Power Down"]
pub type EMAC_PMTCTLSTAT_PWRDWN_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 0>;
#[doc = "Field `EMAC_PMTCTLSTAT_MGKPKTEN` reader - Magic Packet Enable"]
pub type EMAC_PMTCTLSTAT_MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_MGKPKTEN` writer - Magic Packet Enable"]
pub type EMAC_PMTCTLSTAT_MGKPKTEN_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 1>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPFREN` reader - Wake-Up Frame Enable"]
pub type EMAC_PMTCTLSTAT_WUPFREN_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPFREN` writer - Wake-Up Frame Enable"]
pub type EMAC_PMTCTLSTAT_WUPFREN_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 2>;
#[doc = "Field `EMAC_PMTCTLSTAT_MGKPRX` reader - Magic Packet Received"]
pub type EMAC_PMTCTLSTAT_MGKPRX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_MGKPRX` writer - Magic Packet Received"]
pub type EMAC_PMTCTLSTAT_MGKPRX_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 5>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPRX` reader - Wake-Up Frame Received"]
pub type EMAC_PMTCTLSTAT_WUPRX_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPRX` writer - Wake-Up Frame Received"]
pub type EMAC_PMTCTLSTAT_WUPRX_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 6>;
#[doc = "Field `EMAC_PMTCTLSTAT_GLBLUCAST` reader - Global Unicast"]
pub type EMAC_PMTCTLSTAT_GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_GLBLUCAST` writer - Global Unicast"]
pub type EMAC_PMTCTLSTAT_GLBLUCAST_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 9>;
#[doc = "Field `EMAC_PMTCTLSTAT_RWKPTR` reader - Remote Wake-Up FIFO Pointer"]
pub type EMAC_PMTCTLSTAT_RWKPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMAC_PMTCTLSTAT_RWKPTR` writer - Remote Wake-Up FIFO Pointer"]
pub type EMAC_PMTCTLSTAT_RWKPTR_W<'a> = crate::FieldWriter<'a, u32, PMTCTLSTAT_SPEC, u8, u8, 3, 24>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPFRRST` reader - Wake-Up Frame Filter Register Pointer Reset"]
pub type EMAC_PMTCTLSTAT_WUPFRRST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_PMTCTLSTAT_WUPFRRST` writer - Wake-Up Frame Filter Register Pointer Reset"]
pub type EMAC_PMTCTLSTAT_WUPFRRST_W<'a> = crate::BitWriter<'a, u32, PMTCTLSTAT_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn emac_pmtctlstat_pwrdwn(&self) -> EMAC_PMTCTLSTAT_PWRDWN_R {
        EMAC_PMTCTLSTAT_PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkpkten(&self) -> EMAC_PMTCTLSTAT_MGKPKTEN_R {
        EMAC_PMTCTLSTAT_MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfren(&self) -> EMAC_PMTCTLSTAT_WUPFREN_R {
        EMAC_PMTCTLSTAT_WUPFREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkprx(&self) -> EMAC_PMTCTLSTAT_MGKPRX_R {
        EMAC_PMTCTLSTAT_MGKPRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wuprx(&self) -> EMAC_PMTCTLSTAT_WUPRX_R {
        EMAC_PMTCTLSTAT_WUPRX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn emac_pmtctlstat_glblucast(&self) -> EMAC_PMTCTLSTAT_GLBLUCAST_R {
        EMAC_PMTCTLSTAT_GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn emac_pmtctlstat_rwkptr(&self) -> EMAC_PMTCTLSTAT_RWKPTR_R {
        EMAC_PMTCTLSTAT_RWKPTR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfrrst(&self) -> EMAC_PMTCTLSTAT_WUPFRRST_R {
        EMAC_PMTCTLSTAT_WUPFRRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn emac_pmtctlstat_pwrdwn(&mut self) -> EMAC_PMTCTLSTAT_PWRDWN_W {
        EMAC_PMTCTLSTAT_PWRDWN_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkpkten(&mut self) -> EMAC_PMTCTLSTAT_MGKPKTEN_W {
        EMAC_PMTCTLSTAT_MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfren(&mut self) -> EMAC_PMTCTLSTAT_WUPFREN_W {
        EMAC_PMTCTLSTAT_WUPFREN_W::new(self)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_mgkprx(&mut self) -> EMAC_PMTCTLSTAT_MGKPRX_W {
        EMAC_PMTCTLSTAT_MGKPRX_W::new(self)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wuprx(&mut self) -> EMAC_PMTCTLSTAT_WUPRX_W {
        EMAC_PMTCTLSTAT_WUPRX_W::new(self)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn emac_pmtctlstat_glblucast(&mut self) -> EMAC_PMTCTLSTAT_GLBLUCAST_W {
        EMAC_PMTCTLSTAT_GLBLUCAST_W::new(self)
    }
    #[doc = "Bits 24:26 - Remote Wake-Up FIFO Pointer"]
    #[inline(always)]
    pub fn emac_pmtctlstat_rwkptr(&mut self) -> EMAC_PMTCTLSTAT_RWKPTR_W {
        EMAC_PMTCTLSTAT_RWKPTR_W::new(self)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn emac_pmtctlstat_wupfrrst(&mut self) -> EMAC_PMTCTLSTAT_WUPFRRST_W {
        EMAC_PMTCTLSTAT_WUPFRRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtctlstat](index.html) module"]
pub struct PMTCTLSTAT_SPEC;
impl crate::RegisterSpec for PMTCTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmtctlstat::R](R) reader structure"]
impl crate::Readable for PMTCTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmtctlstat::W](W) writer structure"]
impl crate::Writable for PMTCTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMTCTLSTAT to value 0"]
impl crate::Resettable for PMTCTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
