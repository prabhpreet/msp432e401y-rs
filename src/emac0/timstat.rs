#[doc = "Register `TIMSTAT` reader"]
pub struct R(crate::R<TIMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSTAT` writer"]
pub struct W(crate::W<TIMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSTAT_SPEC>;
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
impl From<crate::W<TIMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMAC_TIMSTAT_TSSOVF` reader - Timestamp Seconds Overflow"]
pub type EMAC_TIMSTAT_TSSOVF_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTAT_TSSOVF` writer - Timestamp Seconds Overflow"]
pub type EMAC_TIMSTAT_TSSOVF_W<'a> = crate::BitWriter<'a, u32, TIMSTAT_SPEC, bool, 0>;
#[doc = "Field `EMAC_TIMSTAT_TSTARGT` reader - Timestamp Target Time Reached"]
pub type EMAC_TIMSTAT_TSTARGT_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_TIMSTAT_TSTARGT` writer - Timestamp Target Time Reached"]
pub type EMAC_TIMSTAT_TSTARGT_W<'a> = crate::BitWriter<'a, u32, TIMSTAT_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn emac_timstat_tssovf(&self) -> EMAC_TIMSTAT_TSSOVF_R {
        EMAC_TIMSTAT_TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn emac_timstat_tstargt(&self) -> EMAC_TIMSTAT_TSTARGT_R {
        EMAC_TIMSTAT_TSTARGT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn emac_timstat_tssovf(&mut self) -> EMAC_TIMSTAT_TSSOVF_W {
        EMAC_TIMSTAT_TSSOVF_W::new(self)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn emac_timstat_tstargt(&mut self) -> EMAC_TIMSTAT_TSTARGT_W {
        EMAC_TIMSTAT_TSTARGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC Timestamp Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstat](index.html) module"]
pub struct TIMSTAT_SPEC;
impl crate::RegisterSpec for TIMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timstat::R](R) reader structure"]
impl crate::Readable for TIMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timstat::W](W) writer structure"]
impl crate::Writable for TIMSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMSTAT to value 0"]
impl crate::Resettable for TIMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
