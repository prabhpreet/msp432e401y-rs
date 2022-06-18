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
#[doc = "Field `QEI_STAT_ERROR` reader - Error Detected"]
pub type QEI_STAT_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `QEI_STAT_ERROR` writer - Error Detected"]
pub type QEI_STAT_ERROR_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 0>;
#[doc = "Field `QEI_STAT_DIRECTION` reader - Direction of Rotation"]
pub type QEI_STAT_DIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `QEI_STAT_DIRECTION` writer - Direction of Rotation"]
pub type QEI_STAT_DIRECTION_W<'a> = crate::BitWriter<'a, u32, STAT_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn qei_stat_error(&self) -> QEI_STAT_ERROR_R {
        QEI_STAT_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn qei_stat_direction(&self) -> QEI_STAT_DIRECTION_R {
        QEI_STAT_DIRECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn qei_stat_error(&mut self) -> QEI_STAT_ERROR_W {
        QEI_STAT_ERROR_W::new(self)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn qei_stat_direction(&mut self) -> QEI_STAT_DIRECTION_W {
        QEI_STAT_DIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
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
