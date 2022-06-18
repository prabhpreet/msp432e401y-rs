#[doc = "Register `ISC` reader"]
pub struct R(crate::R<ISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISC` writer"]
pub struct W(crate::W<ISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISC_SPEC>;
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
impl From<crate::W<ISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_ISC_INDEX` reader - Index Pulse Interrupt"]
pub type QEI_ISC_INDEX_R = crate::BitReader<bool>;
#[doc = "Field `QEI_ISC_INDEX` writer - Index Pulse Interrupt"]
pub type QEI_ISC_INDEX_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 0>;
#[doc = "Field `QEI_ISC_TIMER` reader - Velocity Timer Expired Interrupt"]
pub type QEI_ISC_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `QEI_ISC_TIMER` writer - Velocity Timer Expired Interrupt"]
pub type QEI_ISC_TIMER_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 1>;
#[doc = "Field `QEI_ISC_DIR` reader - Direction Change Interrupt"]
pub type QEI_ISC_DIR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_ISC_DIR` writer - Direction Change Interrupt"]
pub type QEI_ISC_DIR_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 2>;
#[doc = "Field `QEI_ISC_ERROR` reader - Phase Error Interrupt"]
pub type QEI_ISC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_ISC_ERROR` writer - Phase Error Interrupt"]
pub type QEI_ISC_ERROR_W<'a> = crate::BitWriter<'a, u32, ISC_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&self) -> QEI_ISC_INDEX_R {
        QEI_ISC_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&self) -> QEI_ISC_TIMER_R {
        QEI_ISC_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&self) -> QEI_ISC_DIR_R {
        QEI_ISC_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&self) -> QEI_ISC_ERROR_R {
        QEI_ISC_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&mut self) -> QEI_ISC_INDEX_W {
        QEI_ISC_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&mut self) -> QEI_ISC_TIMER_W {
        QEI_ISC_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&mut self) -> QEI_ISC_DIR_W {
        QEI_ISC_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&mut self) -> QEI_ISC_ERROR_W {
        QEI_ISC_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](index.html) module"]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isc::R](R) reader structure"]
impl crate::Readable for ISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isc::W](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
