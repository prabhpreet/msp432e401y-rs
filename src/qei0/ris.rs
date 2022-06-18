#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RIS` writer"]
pub struct W(crate::W<RIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RIS_SPEC>;
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
impl From<crate::W<RIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_RIS_INDEX` reader - Index Pulse Asserted"]
pub type QEI_RIS_INDEX_R = crate::BitReader<bool>;
#[doc = "Field `QEI_RIS_INDEX` writer - Index Pulse Asserted"]
pub type QEI_RIS_INDEX_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 0>;
#[doc = "Field `QEI_RIS_TIMER` reader - Velocity Timer Expired"]
pub type QEI_RIS_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `QEI_RIS_TIMER` writer - Velocity Timer Expired"]
pub type QEI_RIS_TIMER_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 1>;
#[doc = "Field `QEI_RIS_DIR` reader - Direction Change Detected"]
pub type QEI_RIS_DIR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_RIS_DIR` writer - Direction Change Detected"]
pub type QEI_RIS_DIR_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 2>;
#[doc = "Field `QEI_RIS_ERROR` reader - Phase Error Detected"]
pub type QEI_RIS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_RIS_ERROR` writer - Phase Error Detected"]
pub type QEI_RIS_ERROR_W<'a> = crate::BitWriter<'a, u32, RIS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Index Pulse Asserted"]
    #[inline(always)]
    pub fn qei_ris_index(&self) -> QEI_RIS_INDEX_R {
        QEI_RIS_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired"]
    #[inline(always)]
    pub fn qei_ris_timer(&self) -> QEI_RIS_TIMER_R {
        QEI_RIS_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detected"]
    #[inline(always)]
    pub fn qei_ris_dir(&self) -> QEI_RIS_DIR_R {
        QEI_RIS_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Detected"]
    #[inline(always)]
    pub fn qei_ris_error(&self) -> QEI_RIS_ERROR_R {
        QEI_RIS_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Asserted"]
    #[inline(always)]
    pub fn qei_ris_index(&mut self) -> QEI_RIS_INDEX_W {
        QEI_RIS_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Velocity Timer Expired"]
    #[inline(always)]
    pub fn qei_ris_timer(&mut self) -> QEI_RIS_TIMER_W {
        QEI_RIS_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Detected"]
    #[inline(always)]
    pub fn qei_ris_dir(&mut self) -> QEI_RIS_DIR_W {
        QEI_RIS_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Detected"]
    #[inline(always)]
    pub fn qei_ris_error(&mut self) -> QEI_RIS_ERROR_W {
        QEI_RIS_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ris::W](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
