#[doc = "Register `LPITIMERCTL` reader"]
pub struct R(crate::R<LPITIMERCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPITIMERCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPITIMERCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPITIMERCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPITIMERCTL` writer"]
pub struct W(crate::W<LPITIMERCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPITIMERCTL_SPEC>;
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
impl From<crate::W<LPITIMERCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPITIMERCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_LPITIMERCTL_TWT` reader - LPI TW Timer"]
pub type EMAC_LPITIMERCTL_TWT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_LPITIMERCTL_TWT` writer - LPI TW Timer"]
pub type EMAC_LPITIMERCTL_TWT_W<'a> =
    crate::FieldWriter<'a, u32, LPITIMERCTL_SPEC, u16, u16, 16, 0>;
#[doc = "Field `EMAC_LPITIMERCTL_LST` reader - LPI LS Timer"]
pub type EMAC_LPITIMERCTL_LST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EMAC_LPITIMERCTL_LST` writer - LPI LS Timer"]
pub type EMAC_LPITIMERCTL_LST_W<'a> =
    crate::FieldWriter<'a, u32, LPITIMERCTL_SPEC, u16, u16, 10, 16>;
impl R {
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_twt(&self) -> EMAC_LPITIMERCTL_TWT_R {
        EMAC_LPITIMERCTL_TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_lst(&self) -> EMAC_LPITIMERCTL_LST_R {
        EMAC_LPITIMERCTL_LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPI TW Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_twt(&mut self) -> EMAC_LPITIMERCTL_TWT_W {
        EMAC_LPITIMERCTL_TWT_W::new(self)
    }
    #[doc = "Bits 16:25 - LPI LS Timer"]
    #[inline(always)]
    pub fn emac_lpitimerctl_lst(&mut self) -> EMAC_LPITIMERCTL_LST_W {
        EMAC_LPITIMERCTL_LST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Low Power Idle Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpitimerctl](index.html) module"]
pub struct LPITIMERCTL_SPEC;
impl crate::RegisterSpec for LPITIMERCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpitimerctl::R](R) reader structure"]
impl crate::Readable for LPITIMERCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpitimerctl::W](W) writer structure"]
impl crate::Writable for LPITIMERCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPITIMERCTL to value 0"]
impl crate::Resettable for LPITIMERCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
