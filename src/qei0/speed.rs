#[doc = "Register `SPEED` reader"]
pub struct R(crate::R<SPEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPEED` writer"]
pub struct W(crate::W<SPEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPEED_SPEC>;
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
impl From<crate::W<SPEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_SPEED` reader - Velocity"]
pub type QEI_SPEED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QEI_SPEED` writer - Velocity"]
pub type QEI_SPEED_W<'a> = crate::FieldWriter<'a, u32, SPEED_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    pub fn qei_speed(&self) -> QEI_SPEED_R {
        QEI_SPEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    pub fn qei_speed(&mut self) -> QEI_SPEED_W {
        QEI_SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Velocity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [speed](index.html) module"]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [speed::R](R) reader structure"]
impl crate::Readable for SPEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [speed::W](W) writer structure"]
impl crate::Writable for SPEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPEED to value 0"]
impl crate::Resettable for SPEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
