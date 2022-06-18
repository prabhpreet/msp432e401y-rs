#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PP` writer"]
pub struct W(crate::W<PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PP_SPEC>;
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
impl From<crate::W<PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PP_EDE` reader - Extended Drive Enable"]
pub type GPIO_PP_EDE_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_PP_EDE` writer - Extended Drive Enable"]
pub type GPIO_PP_EDE_W<'a> = crate::BitWriter<'a, u32, PP_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Extended Drive Enable"]
    #[inline(always)]
    pub fn gpio_pp_ede(&self) -> GPIO_PP_EDE_R {
        GPIO_PP_EDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extended Drive Enable"]
    #[inline(always)]
    pub fn gpio_pp_ede(&mut self) -> GPIO_PP_EDE_W {
        GPIO_PP_EDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Peripheral Property\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pp::W](W) writer structure"]
impl crate::Writable for PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
