#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_ICR_GPIO` writer - GPIO Interrupt Clear"]
pub type GPIO_ICR_GPIO_W<'a> = crate::FieldWriter<'a, u32, ICR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GPIO_ICR_DMAIC` writer - GPIO uDMA Interrupt Clear"]
pub type GPIO_ICR_DMAIC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 8>;
impl W {
    #[doc = "Bits 0:7 - GPIO Interrupt Clear"]
    #[inline(always)]
    pub fn gpio_icr_gpio(&mut self) -> GPIO_ICR_GPIO_W {
        GPIO_ICR_GPIO_W::new(self)
    }
    #[doc = "Bit 8 - GPIO uDMA Interrupt Clear"]
    #[inline(always)]
    pub fn gpio_icr_dmaic(&mut self) -> GPIO_ICR_DMAIC_W {
        GPIO_ICR_DMAIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
