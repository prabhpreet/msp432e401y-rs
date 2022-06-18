#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_INTEN_INDEX` reader - Index Pulse Detected Interrupt Enable"]
pub type QEI_INTEN_INDEX_R = crate::BitReader<bool>;
#[doc = "Field `QEI_INTEN_INDEX` writer - Index Pulse Detected Interrupt Enable"]
pub type QEI_INTEN_INDEX_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `QEI_INTEN_TIMER` reader - Timer Expires Interrupt Enable"]
pub type QEI_INTEN_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `QEI_INTEN_TIMER` writer - Timer Expires Interrupt Enable"]
pub type QEI_INTEN_TIMER_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `QEI_INTEN_DIR` reader - Direction Change Interrupt Enable"]
pub type QEI_INTEN_DIR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_INTEN_DIR` writer - Direction Change Interrupt Enable"]
pub type QEI_INTEN_DIR_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `QEI_INTEN_ERROR` reader - Phase Error Interrupt Enable"]
pub type QEI_INTEN_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_INTEN_ERROR` writer - Phase Error Interrupt Enable"]
pub type QEI_INTEN_ERROR_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_index(&self) -> QEI_INTEN_INDEX_R {
        QEI_INTEN_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_timer(&self) -> QEI_INTEN_TIMER_R {
        QEI_INTEN_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_dir(&self) -> QEI_INTEN_DIR_R {
        QEI_INTEN_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_error(&self) -> QEI_INTEN_ERROR_R {
        QEI_INTEN_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_index(&mut self) -> QEI_INTEN_INDEX_W {
        QEI_INTEN_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_timer(&mut self) -> QEI_INTEN_TIMER_W {
        QEI_INTEN_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_dir(&mut self) -> QEI_INTEN_DIR_W {
        QEI_INTEN_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_error(&mut self) -> QEI_INTEN_ERROR_W {
        QEI_INTEN_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
