#[doc = "Register `MAXPOS` reader"]
pub struct R(crate::R<MAXPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAXPOS` writer"]
pub struct W(crate::W<MAXPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXPOS_SPEC>;
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
impl From<crate::W<MAXPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_MAXPOS` reader - Maximum Position Integrator Value"]
pub type QEI_MAXPOS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QEI_MAXPOS` writer - Maximum Position Integrator Value"]
pub type QEI_MAXPOS_W<'a> = crate::FieldWriter<'a, u32, MAXPOS_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Maximum Position Integrator Value"]
    #[inline(always)]
    pub fn qei_maxpos(&self) -> QEI_MAXPOS_R {
        QEI_MAXPOS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Maximum Position Integrator Value"]
    #[inline(always)]
    pub fn qei_maxpos(&mut self) -> QEI_MAXPOS_W {
        QEI_MAXPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Maximum Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpos](index.html) module"]
pub struct MAXPOS_SPEC;
impl crate::RegisterSpec for MAXPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maxpos::R](R) reader structure"]
impl crate::Readable for MAXPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maxpos::W](W) writer structure"]
impl crate::Writable for MAXPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAXPOS to value 0"]
impl crate::Resettable for MAXPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
