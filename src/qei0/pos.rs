#[doc = "Register `POS` reader"]
pub struct R(crate::R<POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POS` writer"]
pub struct W(crate::W<POS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POS_SPEC>;
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
impl From<crate::W<POS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEI_POS` reader - Current Position Integrator Value"]
pub type QEI_POS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `QEI_POS` writer - Current Position Integrator Value"]
pub type QEI_POS_W<'a> = crate::FieldWriter<'a, u32, POS_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    pub fn qei_pos(&self) -> QEI_POS_R {
        QEI_POS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    pub fn qei_pos(&mut self) -> QEI_POS_W {
        QEI_POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QEI Position\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pos](index.html) module"]
pub struct POS_SPEC;
impl crate::RegisterSpec for POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pos::R](R) reader structure"]
impl crate::Readable for POS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pos::W](W) writer structure"]
impl crate::Writable for POS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for POS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
