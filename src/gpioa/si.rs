#[doc = "Register `SI` reader"]
pub struct R(crate::R<SI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SI` writer"]
pub struct W(crate::W<SI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SI_SPEC>;
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
impl From<crate::W<SI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SI_SUM` reader - Summary Interrupt"]
pub type GPIO_SI_SUM_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_SI_SUM` writer - Summary Interrupt"]
pub type GPIO_SI_SUM_W<'a> = crate::BitWriter<'a, u32, SI_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Summary Interrupt"]
    #[inline(always)]
    pub fn gpio_si_sum(&self) -> GPIO_SI_SUM_R {
        GPIO_SI_SUM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Summary Interrupt"]
    #[inline(always)]
    pub fn gpio_si_sum(&mut self) -> GPIO_SI_SUM_W {
        GPIO_SI_SUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Select Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [si](index.html) module"]
pub struct SI_SPEC;
impl crate::RegisterSpec for SI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [si::R](R) reader structure"]
impl crate::Readable for SI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [si::W](W) writer structure"]
impl crate::Writable for SI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SI to value 0"]
impl crate::Resettable for SI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
