#[doc = "Register `WAKELVL` reader"]
pub struct R(crate::R<WAKELVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKELVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKELVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKELVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKELVL` writer"]
pub struct W(crate::W<WAKELVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKELVL_SPEC>;
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
impl From<crate::W<WAKELVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKELVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKELVL_WAKELVL4` reader - P\\[4\\]
Wake Level"]
pub type GPIO_WAKELVL_WAKELVL4_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WAKELVL_WAKELVL4` writer - P\\[4\\]
Wake Level"]
pub type GPIO_WAKELVL_WAKELVL4_W<'a> = crate::BitWriter<'a, u32, WAKELVL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 4 - P\\[4\\]
Wake Level"]
    #[inline(always)]
    pub fn gpio_wakelvl_wakelvl4(&self) -> GPIO_WAKELVL_WAKELVL4_R {
        GPIO_WAKELVL_WAKELVL4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - P\\[4\\]
Wake Level"]
    #[inline(always)]
    pub fn gpio_wakelvl_wakelvl4(&mut self) -> GPIO_WAKELVL_WAKELVL4_W {
        GPIO_WAKELVL_WAKELVL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Wake Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakelvl](index.html) module"]
pub struct WAKELVL_SPEC;
impl crate::RegisterSpec for WAKELVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakelvl::R](R) reader structure"]
impl crate::Readable for WAKELVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakelvl::W](W) writer structure"]
impl crate::Writable for WAKELVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKELVL to value 0"]
impl crate::Resettable for WAKELVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
