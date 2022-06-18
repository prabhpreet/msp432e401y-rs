#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPI_STAT_ACTIVE` reader - Register Active"]
pub type EPI_STAT_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_ACTIVE` writer - Register Active"]
pub type EPI_STAT_ACTIVE_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 0>;
#[doc = "Field `EPI_STAT_NBRBUSY` reader - Non-Blocking Read Busy"]
pub type EPI_STAT_NBRBUSY_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_NBRBUSY` writer - Non-Blocking Read Busy"]
pub type EPI_STAT_NBRBUSY_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 4>;
#[doc = "Field `EPI_STAT_WBUSY` reader - Write Busy"]
pub type EPI_STAT_WBUSY_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_WBUSY` writer - Write Busy"]
pub type EPI_STAT_WBUSY_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 5>;
#[doc = "Field `EPI_STAT_INITSEQ` reader - Initialization Sequence"]
pub type EPI_STAT_INITSEQ_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_INITSEQ` writer - Initialization Sequence"]
pub type EPI_STAT_INITSEQ_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 6>;
#[doc = "Field `EPI_STAT_XFEMPTY` reader - External FIFO Empty"]
pub type EPI_STAT_XFEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_XFEMPTY` writer - External FIFO Empty"]
pub type EPI_STAT_XFEMPTY_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 7>;
#[doc = "Field `EPI_STAT_XFFULL` reader - External FIFO Full"]
pub type EPI_STAT_XFFULL_R = crate::BitReader<bool>;
#[doc = "Field `EPI_STAT_XFFULL` writer - External FIFO Full"]
pub type EPI_STAT_XFFULL_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn epi_stat_active(&self) -> EPI_STAT_ACTIVE_R {
        EPI_STAT_ACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn epi_stat_nbrbusy(&self) -> EPI_STAT_NBRBUSY_R {
        EPI_STAT_NBRBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn epi_stat_wbusy(&self) -> EPI_STAT_WBUSY_R {
        EPI_STAT_WBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn epi_stat_initseq(&self) -> EPI_STAT_INITSEQ_R {
        EPI_STAT_INITSEQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn epi_stat_xfempty(&self) -> EPI_STAT_XFEMPTY_R {
        EPI_STAT_XFEMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn epi_stat_xffull(&self) -> EPI_STAT_XFFULL_R {
        EPI_STAT_XFFULL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Active"]
    #[inline(always)]
    pub fn epi_stat_active(&mut self) -> EPI_STAT_ACTIVE_W {
        EPI_STAT_ACTIVE_W::new(self)
    }
    #[doc = "Bit 4 - Non-Blocking Read Busy"]
    #[inline(always)]
    pub fn epi_stat_nbrbusy(&mut self) -> EPI_STAT_NBRBUSY_W {
        EPI_STAT_NBRBUSY_W::new(self)
    }
    #[doc = "Bit 5 - Write Busy"]
    #[inline(always)]
    pub fn epi_stat_wbusy(&mut self) -> EPI_STAT_WBUSY_W {
        EPI_STAT_WBUSY_W::new(self)
    }
    #[doc = "Bit 6 - Initialization Sequence"]
    #[inline(always)]
    pub fn epi_stat_initseq(&mut self) -> EPI_STAT_INITSEQ_W {
        EPI_STAT_INITSEQ_W::new(self)
    }
    #[doc = "Bit 7 - External FIFO Empty"]
    #[inline(always)]
    pub fn epi_stat_xfempty(&mut self) -> EPI_STAT_XFEMPTY_W {
        EPI_STAT_XFEMPTY_W::new(self)
    }
    #[doc = "Bit 8 - External FIFO Full"]
    #[inline(always)]
    pub fn epi_stat_xffull(&mut self) -> EPI_STAT_XFFULL_W {
        EPI_STAT_XFFULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EPI Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
