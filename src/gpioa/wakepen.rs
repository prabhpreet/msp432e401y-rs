#[doc = "Register `WAKEPEN` reader"]
pub struct R(crate::R<WAKEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEPEN` writer"]
pub struct W(crate::W<WAKEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEPEN_SPEC>;
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
impl From<crate::W<WAKEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEPEN_WAKEP4` reader - P\\[4\\]
Wake Enable"]
pub type GPIO_WAKEPEN_WAKEP4_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WAKEPEN_WAKEP4` writer - P\\[4\\]
Wake Enable"]
pub type GPIO_WAKEPEN_WAKEP4_W<'a> = crate::BitWriter<'a, u32, WAKEPEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 4 - P\\[4\\]
Wake Enable"]
    #[inline(always)]
    pub fn gpio_wakepen_wakep4(&self) -> GPIO_WAKEPEN_WAKEP4_R {
        GPIO_WAKEPEN_WAKEP4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - P\\[4\\]
Wake Enable"]
    #[inline(always)]
    pub fn gpio_wakepen_wakep4(&mut self) -> GPIO_WAKEPEN_WAKEP4_W {
        GPIO_WAKEPEN_WAKEP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Pin Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakepen](index.html) module"]
pub struct WAKEPEN_SPEC;
impl crate::RegisterSpec for WAKEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakepen::R](R) reader structure"]
impl crate::Readable for WAKEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakepen::W](W) writer structure"]
impl crate::Writable for WAKEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEPEN to value 0"]
impl crate::Resettable for WAKEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
